import { createHash } from "node:crypto";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

export const SEEDS_MODES = Object.freeze([
  "Light",
  "Dark",
  "Sakura",
  "Momiji",
  "NatureLaw",
  "Disaster",
]);

const CAMEL_BOUNDARY_LOWER_UPPER = /([a-z0-9])([A-Z])/g;
const CAMEL_BOUNDARY_ACRONYM = /([A-Z])([A-Z][a-z])/g;
const LAYERS = new Set(["primitive", "semantic", "material"]);
const TYPE_KIND_MAP = {
  COLOR: new Set(["color", "string"]),
  FLOAT: new Set(["number"]),
  STRING: new Set(["string"]),
};

function isRecord(value) {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function fail(message) {
  throw new Error(message);
}

function splitPathSegments(tokenPath) {
  return tokenPath.split("/").map((segment) =>
    segment
      .replace(CAMEL_BOUNDARY_ACRONYM, "$1-$2")
      .replace(CAMEL_BOUNDARY_LOWER_UPPER, "$1-$2")
      .toLowerCase(),
  );
}

function sanitizeCssValue(value) {
  if (value.includes("\n") || value.includes("\r")) {
    fail(`Unsafe CSS value contains a newline: ${value}`);
  }
  if (value.includes("/*") || value.includes("*/")) {
    fail(`Unsafe CSS value contains a comment terminator: ${value}`);
  }
  return value;
}

function serializeNumber(pathName, value) {
  if (pathName.startsWith("material/spacing/") || pathName.startsWith("material/radius/")) {
    return `${value}px`;
  }
  if (
    /^material\/typography\/[^/]+\/(fontSize|lineHeight|letterSpacing)$/.test(pathName) ||
    /^material\/glass\/[^/]+\/blur$/.test(pathName) ||
    /^material\/motion\/transform\/[^/]*OffsetY$/.test(pathName)
  ) {
    return `${value}px`;
  }
  if (
    /^material\/motion\/duration\/[^/]+$/.test(pathName) ||
    /^material\/motion\/role\/[^/]+\/duration$/.test(pathName)
  ) {
    return `${value}ms`;
  }
  return `${value}`;
}

function serializeTerminal(pathName, terminal) {
  if (terminal.kind === "number") {
    return serializeNumber(pathName, terminal.value);
  }
  return sanitizeCssValue(terminal.value);
}

function expectRecord(value, message) {
  if (!isRecord(value)) {
    fail(message);
  }
}

function normalizeValueSpec(valueSpec, variablePath, resolvedType) {
  expectRecord(valueSpec, `Invalid value spec for ${variablePath}`);

  const { kind } = valueSpec;
  if (kind === "alias") {
    if (typeof valueSpec.path !== "string" || valueSpec.path.length === 0) {
      fail(`Alias target path must be a non-empty string for ${variablePath}`);
    }
    if (valueSpec.path === variablePath) {
      fail(`Self alias detected at ${variablePath}`);
    }
    return { kind: "alias", path: valueSpec.path };
  }

  const expectedKinds = TYPE_KIND_MAP[resolvedType];
  if (!expectedKinds.has(kind)) {
    fail(
      `resolvedType ${resolvedType} must use ${Array.from(expectedKinds).join(" or ")} values for ${variablePath}`,
    );
  }

  if (kind === "number") {
    if (typeof valueSpec.value !== "number" || !Number.isFinite(valueSpec.value)) {
      fail(`Number value must be finite for ${variablePath}`);
    }
    return { kind, value: valueSpec.value };
  }

  if (typeof valueSpec.value !== "string") {
    fail(`${kind} value must be a string for ${variablePath}`);
  }

  return { kind, value: valueSpec.value };
}

function normalizeVariable(variable) {
  expectRecord(variable, "Each variable must be an object");

  const { path: variablePath, resolvedType, modeStrategy, values } = variable;
  if (typeof variablePath !== "string" || variablePath.length === 0) {
    fail("Variable path must be a non-empty string");
  }

  const [layer] = variablePath.split("/");
  if (layer === "component") {
    fail(`Unsupported component layer variable ${variablePath}`);
  }
  if (!LAYERS.has(layer)) {
    fail(`Unsupported top-level layer ${layer} for ${variablePath}`);
  }

  if (!Object.hasOwn(TYPE_KIND_MAP, resolvedType)) {
    fail(`Unsupported resolvedType ${resolvedType} for ${variablePath}`);
  }

  expectRecord(values, `Variable values must be an object for ${variablePath}`);

  if (modeStrategy === "invariant") {
    if (!Object.hasOwn(values, "default")) {
      fail(`Invariant variable ${variablePath} must define values.default`);
    }
    return {
      path: variablePath,
      layer,
      resolvedType,
      modeStrategy,
      values: {
        default: normalizeValueSpec(values.default, variablePath, resolvedType),
      },
    };
  }

  if (modeStrategy === "per-mode") {
    const normalizedValues = {};
    for (const mode of SEEDS_MODES) {
      if (!Object.hasOwn(values, mode)) {
        fail(`Per-mode variable ${variablePath} is missing ${mode}`);
      }
      normalizedValues[mode] = normalizeValueSpec(values[mode], variablePath, resolvedType);
    }
    return {
      path: variablePath,
      layer,
      resolvedType,
      modeStrategy,
      values: normalizedValues,
    };
  }

  fail(`Unsupported modeStrategy ${modeStrategy} for ${variablePath}`);
}

function resolveAliases(variables) {
  const byPath = new Map(variables.map((variable) => [variable.path, variable]));

  function resolve(variablePath, mode, visiting) {
    const key = `${variablePath}::${mode}`;
    if (visiting.has(key)) {
      fail(`Alias cycle detected at ${variablePath} for ${mode}`);
    }

    const variable = byPath.get(variablePath);
    if (!variable) {
      fail(`Missing alias target ${variablePath}`);
    }

    const spec =
      variable.modeStrategy === "invariant" ? variable.values.default : variable.values[mode];

    if (!spec) {
      fail(`Missing ${mode} value for ${variablePath}`);
    }

    if (spec.kind !== "alias") {
      return { kind: spec.kind, value: spec.value };
    }

    visiting.add(key);
    try {
      return resolve(spec.path, mode, visiting);
    } finally {
      visiting.delete(key);
    }
  }

  return variables.map((variable) => {
    if (variable.modeStrategy === "invariant") {
      return {
        ...variable,
        values: {
          default: resolve(variable.path, SEEDS_MODES[0], new Set()),
        },
      };
    }

    const resolvedValues = {};
    for (const mode of SEEDS_MODES) {
      resolvedValues[mode] = resolve(variable.path, mode, new Set());
    }
    return {
      ...variable,
      values: resolvedValues,
    };
  });
}

export function tokenToCssName(tokenPath) {
  return `--seeds-${splitPathSegments(tokenPath).join("-")}`;
}

export function validateAndResolveImport(document) {
  expectRecord(document, "Import document must be an object");
  expectRecord(document.figma, "Import document must include figma metadata");

  if (document.figma.collectionName !== "SEEDS") {
    fail("Figma collectionName must be SEEDS");
  }

  if (!Array.isArray(document.figma.modes)) {
    fail("Figma modes must be an array");
  }

  const modeNames = document.figma.modes.map((mode) => {
    expectRecord(mode, "Each mode must be an object");
    if (typeof mode.name !== "string") {
      fail("Each mode must include a string name");
    }
    return mode.name;
  });

  if (modeNames.length !== SEEDS_MODES.length || modeNames.some((mode, index) => mode !== SEEDS_MODES[index])) {
    fail(`Figma modes must match ${SEEDS_MODES.join(", ")}`);
  }

  if (!Array.isArray(document.variables)) {
    fail("Import document variables must be an array");
  }

  const seenPaths = new Set();
  const normalizedVariables = document.variables.map((variable) => {
    const normalized = normalizeVariable(variable);
    if (seenPaths.has(normalized.path)) {
      fail(`Duplicate variable path ${normalized.path}`);
    }
    seenPaths.add(normalized.path);
    return normalized;
  });

  const resolvedVariables = resolveAliases(normalizedVariables).sort((left, right) =>
    left.path.localeCompare(right.path),
  );

  const counts = {
    primitive: resolvedVariables.filter((variable) => variable.layer === "primitive").length,
    semantic: resolvedVariables.filter((variable) => variable.layer === "semantic").length,
    material: resolvedVariables.filter((variable) => variable.layer === "material").length,
  };

  return {
    modes: [...SEEDS_MODES],
    variables: resolvedVariables,
    counts,
  };
}

function buildBlock(selector, declarations) {
  const lines = declarations.map(
    ({ path: tokenPath, value }) => `  ${tokenToCssName(tokenPath)}: ${serializeTerminal(tokenPath, value)};`,
  );
  return `${selector} {\n${lines.join("\n")}\n}`;
}

export function generateCss(document, metadata = {}) {
  const { modes, variables, counts } = validateAndResolveImport(document);
  const primitiveAndMaterial = variables
    .filter((variable) => variable.layer === "primitive" || variable.layer === "material")
    .map((variable) => ({ path: variable.path, value: variable.values.default }));

  const semanticVariables = variables.filter((variable) => variable.layer === "semantic");

  const header = [
    "/*",
    " * Generated by scripts/generate-seeds-theme.mjs",
    ` * Source: ${metadata.sourceLabel ?? "UNKNOWN"}`,
    ` * SHA-256: ${metadata.sourceHash ?? "UNKNOWN"}`,
    ` * Modes: ${modes.join(", ")}`,
    ` * Counts: primitive=${counts.primitive} semantic=${counts.semantic} material=${counts.material}`,
    " */",
  ].join("\n");

  const blocks = [buildBlock(":root", primitiveAndMaterial)];

  if (semanticVariables.length > 0) {
    blocks.push(
      buildBlock(
        ':root, :root[data-seeds-mode="Light"]',
        semanticVariables.map((variable) => ({
          path: variable.path,
          value: variable.values.Light,
        })),
      ),
    );

    for (const mode of SEEDS_MODES.slice(1)) {
      blocks.push(
        buildBlock(
          `[data-seeds-mode="${mode}"]`,
          semanticVariables.map((variable) => ({
            path: variable.path,
            value: variable.values[mode],
          })),
        ),
      );
    }
  }

  return `${header}\n\n${blocks.join("\n\n")}\n`;
}

async function main(argv) {
  if (argv.length !== 2 || argv[0] !== "--input") {
    fail("Usage: node scripts/generate-seeds-theme.mjs --input <absolute-or-relative-json-path>");
  }

  const inputPath = path.resolve(process.cwd(), argv[1]);
  const outputPath = path.resolve(process.cwd(), "src/design-system/seeds-theme.css");
  const inputContent = await readFile(inputPath, "utf8");
  const sourceHash = createHash("sha256").update(inputContent).digest("hex");
  const document = JSON.parse(inputContent);
  const css = generateCss(document, {
    sourceLabel: inputPath,
    sourceHash,
  });

  await mkdir(path.dirname(outputPath), { recursive: true });
  await writeFile(outputPath, css, "utf8");
}

const invokedPath = process.argv[1] ? path.resolve(process.argv[1]) : null;
const currentPath = fileURLToPath(import.meta.url);

if (invokedPath === currentPath) {
  main(process.argv.slice(2)).catch((error) => {
    console.error(error.message);
    process.exitCode = 1;
  });
}
