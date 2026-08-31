import { describe, expect, it } from "vitest";
import {
  SEEDS_MODES,
  generateCss,
  tokenToCssName,
  validateAndResolveImport,
} from "../scripts/generate-seeds-theme.mjs";

const validImport = () => ({
  formatVersion: "1.0.0",
  figma: {
    collectionName: "SEEDS",
    modes: SEEDS_MODES.map((name) => ({ name })),
    groups: ["primitive", "semantic", "material", "component"],
  },
  variables: [
    {
      path: "primitive/neutral/900",
      resolvedType: "COLOR",
      modeStrategy: "invariant",
      values: { default: { kind: "color", value: "#111111" } },
    },
    {
      path: "primitive/neutral/0",
      resolvedType: "COLOR",
      modeStrategy: "invariant",
      values: { default: { kind: "color", value: "#ffffff" } },
    },
    {
      path: "semantic/text/primary",
      resolvedType: "COLOR",
      modeStrategy: "per-mode",
      values: {
        Light: { kind: "alias", path: "primitive/neutral/900" },
        Dark: { kind: "alias", path: "primitive/neutral/0" },
        Sakura: { kind: "alias", path: "primitive/neutral/900" },
        Momiji: { kind: "alias", path: "primitive/neutral/900" },
        NatureLaw: { kind: "alias", path: "primitive/neutral/900" },
        Disaster: { kind: "alias", path: "primitive/neutral/0" },
      },
    },
    {
      path: "material/spacing/16",
      resolvedType: "FLOAT",
      modeStrategy: "invariant",
      values: { default: { kind: "number", value: 16 } },
    },
    {
      path: "material/typography/bodyM/fontSize",
      resolvedType: "FLOAT",
      modeStrategy: "invariant",
      values: { default: { kind: "number", value: 14 } },
    },
  ],
});

describe("SEEDS theme generator", () => {
  it("maps slash paths and camelCase to stable CSS names", () => {
    expect(tokenToCssName("semantic/text/primary")).toBe("--seeds-semantic-text-primary");
    expect(tokenToCssName("material/typography/bodyM/fontSize")).toBe(
      "--seeds-material-typography-body-m-font-size",
    );
  });

  it("validates modes, reports layer counts, resolves aliases, and emits mode selectors", () => {
    const result = validateAndResolveImport(validImport());
    expect(result.modes).toEqual([...SEEDS_MODES]);
    expect(result.counts).toEqual({ primitive: 2, semantic: 1, material: 2 });
    expect(result.variables.find(({ path }) => path === "semantic/text/primary").values.Dark).toEqual(
      {
        kind: "color",
        value: "#ffffff",
      },
    );

    const css = generateCss(validImport(), { sourceLabel: "fixture.json", sourceHash: "fixture" });
    expect(css).toContain(':root, :root[data-seeds-mode="Light"]');
    expect(css).toContain('[data-seeds-mode="Dark"]');
    expect(css).toContain("--seeds-semantic-text-primary: #ffffff;");
    expect(css).toContain("--seeds-material-spacing-16: 16px;");
  });

  it("sorts variables deterministically and prints header metadata", () => {
    const document = validImport();
    document.variables = [
      document.variables[4],
      document.variables[2],
      document.variables[0],
      document.variables[3],
      document.variables[1],
    ];

    const css = generateCss(document, {
      sourceLabel: "fixture.json",
      sourceHash: "abc123",
    });

    expect(css).toContain("Source: fixture.json");
    expect(css).toContain("SHA-256: abc123");
    expect(css).toContain("Modes: Light, Dark, Sakura, Momiji, NatureLaw, Disaster");
    expect(css).toContain("Counts: primitive=2 semantic=1 material=2");
    expect(css.indexOf("--seeds-primitive-neutral-0")).toBeLessThan(
      css.indexOf("--seeds-primitive-neutral-900"),
    );
    expect(css.indexOf("--seeds-material-spacing-16")).toBeLessThan(
      css.indexOf("--seeds-material-typography-body-m-font-size"),
    );
  });

  it("applies px and ms only to the required numeric token paths", () => {
    const document = validImport();
    document.variables.push(
      {
        path: "material/motion/duration/quick",
        resolvedType: "FLOAT",
        modeStrategy: "invariant",
        values: { default: { kind: "number", value: 120 } },
      },
      {
        path: "material/glass/strong/saturate",
        resolvedType: "FLOAT",
        modeStrategy: "invariant",
        values: { default: { kind: "number", value: 1.12 } },
      },
      {
        path: "material/motion/transform/riseOffsetY",
        resolvedType: "FLOAT",
        modeStrategy: "invariant",
        values: { default: { kind: "number", value: 6 } },
      },
    );

    const css = generateCss(document);
    expect(css).toContain("--seeds-material-motion-duration-quick: 120ms;");
    expect(css).toContain("--seeds-material-glass-strong-saturate: 1.12;");
    expect(css).toContain("--seeds-material-motion-transform-rise-offset-y: 6px;");
  });

  it("accepts string CSS colors for COLOR variables", () => {
    const document = validImport();
    document.variables[0].values.default = { kind: "string", value: "rgba(0,0,0,0.04)" };

    const result = validateAndResolveImport(document);
    expect(result.variables.find(({ path }) => path === "primitive/neutral/900").values.default).toEqual(
      {
        kind: "string",
        value: "rgba(0,0,0,0.04)",
      },
    );
  });

  it("rejects a missing mode", () => {
    const document = validImport();
    document.figma.modes = document.figma.modes.slice(0, -1);
    expect(() => validateAndResolveImport(document)).toThrow(/modes/i);
  });

  it("rejects duplicate paths and component variables", () => {
    const duplicate = validImport();
    duplicate.variables.push({
      path: "primitive/neutral/900",
      resolvedType: "COLOR",
      modeStrategy: "invariant",
      values: { default: { kind: "color", value: "#222222" } },
    });
    expect(() => validateAndResolveImport(duplicate)).toThrow(/duplicate|path/i);

    const component = validImport();
    component.variables.push({
      path: "component/button/background",
      resolvedType: "COLOR",
      modeStrategy: "invariant",
      values: { default: { kind: "color", value: "#222222" } },
    });
    expect(() => validateAndResolveImport(component)).toThrow(/component|layer/i);
  });

  it("rejects malformed SEEDS paths and aliases before CSS serialization", () => {
    const malformedPath = validImport();
    malformedPath.variables[2].path = "semantic/text/x\n} body { color";
    expect(() => validateAndResolveImport(malformedPath)).toThrow(/path|segment|unsafe/i);

    const malformedAlias = validImport();
    malformedAlias.variables[2].values.Light = {
      kind: "alias",
      path: "primitive/neutral/900; body { color: red",
    };
    expect(() => validateAndResolveImport(malformedAlias)).toThrow(/path|alias|unsafe/i);
  });

  it("rejects distinct SEEDS paths that normalize to the same CSS custom property", () => {
    const document = validImport();
    document.variables.push(
      {
        path: "material/spacing/urlValue",
        resolvedType: "FLOAT",
        modeStrategy: "invariant",
        values: { default: { kind: "number", value: 12 } },
      },
      {
        path: "material/spacing/urlVALUE",
        resolvedType: "FLOAT",
        modeStrategy: "invariant",
        values: { default: { kind: "number", value: 24 } },
      },
    );

    expect(() => validateAndResolveImport(document)).toThrow(/css.*duplicate|duplicate.*css|collision/i);
  });

  it("reports duplicate paths before later layer validation defects", () => {
    const document = validImport();
    document.variables = [
      document.variables[0],
      {
        path: "brokenLayer/value",
        resolvedType: "COLOR",
        modeStrategy: "invariant",
        values: { default: { kind: "color", value: "#222222" } },
      },
      {
        path: "primitive/neutral/900",
        resolvedType: "COLOR",
        modeStrategy: "invariant",
        values: { default: { kind: "color", value: "#333333" } },
      },
    ];

    expect(() => validateAndResolveImport(document)).toThrow(/duplicate|path/i);
  });

  it("rejects invalid terminal values and unsafe strings", () => {
    const nonFinite = validImport();
    nonFinite.variables[3].values.default = { kind: "number", value: Number.NaN };
    expect(() => validateAndResolveImport(nonFinite)).toThrow(/number|finite/i);

    const wrongColor = validImport();
    wrongColor.variables[0].values.default = { kind: "color", value: 42 };
    expect(() => validateAndResolveImport(wrongColor)).toThrow(/color|string/i);

    const unsafeCss = validImport();
    unsafeCss.variables.push({
      path: "material/motion/easing/unsafe",
      resolvedType: "STRING",
      modeStrategy: "invariant",
      values: { default: { kind: "string", value: "ease*/" } },
    });
    expect(() => generateCss(unsafeCss)).toThrow(/unsafe|comment|newline/i);
  });

  it("rejects CSS delimiters in terminal values and generated header metadata", () => {
    const unsafeValue = validImport();
    unsafeValue.variables[0].values.default = {
      kind: "color",
      value: "#111111; body { color: red",
    };
    expect(() => validateAndResolveImport(unsafeValue)).toThrow(/unsafe|delimiter|value/i);

    expect(() =>
      generateCss(validImport(), {
        sourceLabel: "fixture.json */ body { color: red",
        sourceHash: "abc123; body { color: red",
      }),
    ).toThrow(/unsafe|delimiter|metadata/i);
  });

  it("rejects mode strategies the serializer cannot represent", () => {
    const perModePrimitive = validImport();
    perModePrimitive.variables[0] = {
      path: "primitive/neutral/900",
      resolvedType: "COLOR",
      modeStrategy: "per-mode",
      values: Object.fromEntries(
        SEEDS_MODES.map((mode) => [mode, { kind: "color", value: "#111111" }]),
      ),
    };
    expect(() => validateAndResolveImport(perModePrimitive)).toThrow(/primitive.*invariant|invariant.*primitive/i);

    const invariantSemantic = validImport();
    invariantSemantic.variables[2] = {
      path: "semantic/text/primary",
      resolvedType: "COLOR",
      modeStrategy: "invariant",
      values: { default: { kind: "color", value: "#111111" } },
    };
    expect(() => validateAndResolveImport(invariantSemantic)).toThrow(/semantic.*per-mode|per-mode.*semantic/i);
  });

  it("rejects a missing alias target", () => {
    const document = validImport();
    document.variables[2].values.Light = { kind: "alias", path: "primitive/missing/1" };
    expect(() => validateAndResolveImport(document)).toThrow(/alias|missing/i);
  });

  it("rejects aliases that resolve to a terminal kind incompatible with the declared resolvedType", () => {
    const colorAliasToNumber = validImport();
    colorAliasToNumber.variables[2].values.Light = { kind: "alias", path: "material/spacing/16" };
    expect(() => validateAndResolveImport(colorAliasToNumber)).toThrow(/resolvedType|color|number/i);

    const floatAliasToColor = validImport();
    floatAliasToColor.variables[3].values.default = { kind: "alias", path: "primitive/neutral/900" };
    expect(() => validateAndResolveImport(floatAliasToColor)).toThrow(/resolvedType|float|number|color/i);
  });

  it("rejects self aliases and alias cycles", () => {
    const selfAlias = validImport();
    selfAlias.variables[0] = {
      path: "primitive/neutral/900",
      resolvedType: "COLOR",
      modeStrategy: "invariant",
      values: { default: { kind: "alias", path: "primitive/neutral/900" } },
    };
    expect(() => validateAndResolveImport(selfAlias)).toThrow(/cycle|self|alias/i);

    const cycle = validImport();
    cycle.variables[0].values.default = { kind: "alias", path: "primitive/neutral/0" };
    cycle.variables[1].values.default = { kind: "alias", path: "primitive/neutral/900" };
    expect(() => validateAndResolveImport(cycle)).toThrow(/cycle|alias/i);
  });
});
