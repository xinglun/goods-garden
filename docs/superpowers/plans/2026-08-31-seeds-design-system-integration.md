# SEEDS Web Design System Integration Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Project the external SEEDS Figma import into a validated, committed CSS theme and apply its semantic/material tokens to the read-only synthetic Goods State web presentation.

**Architecture:** A build-time ES module reads the external SEEDS_figma_plugin_import.json, validates the fixed six-mode contract, resolves aliases, and writes a deterministic CSS snapshot under the web package. React owns only a presentation-only mode selector; GoodsStateView continues to consume the existing synthetic view model and owns no design-system or domain behavior.

**Tech Stack:** React 19, TypeScript, Vite, Vitest, Testing Library, Node.js ES modules, CSS custom properties, Rust workspace checks through Makefile.ai.

**Spec:** docs/superpowers/specs/2026-08-31-seeds-design-system-integration-design.md

## Global Constraints

- The source of truth is /Users/sei-rinn/dev/workspace_typescript/seeds_design_system/docs/design-system/SEEDS_figma_plugin_import.json; SEEDS_design_system_spec.md supplies semantic/material meaning and SEEDS_figma_naming_convention.md supplies Figma path and mode rules.
- The supported mode order is exactly Light, Dark, Sakura, Momiji, NatureLaw, Disaster; Light is the default.
- Figma paths use /; CSS names replace / with - and convert camelCase segments to kebab-case, so semantic/text/primary becomes --seeds-semantic-text-primary.
- Application selectors consume only semantic/* and material/* custom properties. Primitive values may exist in the generated projection but are never referenced by styles.css or React components.
- The generated CSS is committed build input. Browser typecheck, test, and build do not read the external workspace at runtime.
- The current surface remains solid/fallback. No glass treatment, Figma mutation, component library, API, server, external data, or new runtime dependency is added.
- The browser remains a read-only synthetic State presentation outside the Cargo member list. Need, Care, Memory, Learning, autonomous action, POS, SEJ, database, and domain behavior remain out of scope.
- Invalid imports fail before the output file is replaced; missing modes, missing aliases, self aliases, cycles, duplicate paths, invalid layer paths, unsupported value kinds, and malformed values are errors.
- Human-facing documentation remains semantically equivalent in English, Japanese, and Chinese in that order. Protected Rust/domain and architecture documents are not modified.
- Every repository-bound AI Cockpit command uses --repo /Users/sei-rinn/dev/workspace_rust/goods-garden-worktrees/seeds-design-system; generated .ai records are written only by the Runtime.

---

## English

### Files and responsibilities

- Create apps/goods-garden-web/tests/seeds-theme-generator.test.mjs for path conversion, six-mode validation, count reporting, alias resolution, and fail-closed errors.
- Create apps/goods-garden-web/scripts/generate-seeds-theme.mjs as the build-time validator/resolver/serializer and CLI entry point.
- Modify apps/goods-garden-web/vite.config.ts so Vitest discovers the generator's .mjs test file.
- Create apps/goods-garden-web/src/design-system/modes.ts for the typed six-mode list and default.
- Create apps/goods-garden-web/src/design-system/seeds-theme.css as the committed generated projection; do not hand-edit it after generation.
- Modify apps/goods-garden-web/package.json to expose generate:seeds without adding a dependency.
- Modify apps/goods-garden-web/src/main.tsx to load the generated theme before application CSS.
- Modify apps/goods-garden-web/src/App.tsx to own presentation-only mode selection and document-root mode state.
- Modify apps/goods-garden-web/src/styles.css to consume semantic/material properties and retain the current read-only visual structure.
- Modify apps/goods-garden-web/tests/App.test.tsx to cover all modes and preserve healthy/unhealthy, synthetic, and no-action assertions.

### Task 1: Define the generator contract with failing tests

**Files:**

- Create: apps/goods-garden-web/tests/seeds-theme-generator.test.mjs
- Modify: apps/goods-garden-web/vite.config.ts
- Test: apps/goods-garden-web/tests/seeds-theme-generator.test.mjs

**Interfaces:**

- The tests import SEEDS_MODES, tokenToCssName, validateAndResolveImport, and generateCss from ../scripts/generate-seeds-theme.mjs.
- tokenToCssName(path) returns a CSS custom-property name.
- validateAndResolveImport(document) returns { modes, variables, counts }, where each returned variable has path, layer, resolvedType, modeStrategy, and resolved values.
- generateCss(document, metadata) returns the complete CSS string without writing a file.

- [ ] **Step 1: Add a six-mode fixture and path-name tests.**

Use a fixture with the exact mode list and these values: two invariant primitive colors, one per-mode semantic color alias, one invariant material spacing number, and one invariant material typography number. The fixture must use the import schema's path, resolvedType, modeStrategy, and values fields.

~~~js
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
    expect(result.variables.find(({ path }) => path === "semantic/text/primary").values.Dark).toEqual({
      kind: "color",
      value: "#ffffff",
    });

    const css = generateCss(validImport(), { sourceLabel: "fixture.json", sourceHash: "fixture" });
    expect(css).toContain(':root, :root[data-seeds-mode="Light"]');
    expect(css).toContain('[data-seeds-mode="Dark"]');
    expect(css).toContain("--seeds-semantic-text-primary: #ffffff;");
    expect(css).toContain("--seeds-material-spacing-16: 16px;");
  });
});

~~~
- [ ] **Step 2: Add fail-closed cases before implementation.**

Extend the same test file with a helper that mutates a fresh validImport() fixture and assert that validateAndResolveImport throws for each invalid contract:

~~~js
it("rejects a missing mode", () => {
  const document = validImport();
  document.figma.modes = document.figma.modes.slice(0, -1);
  expect(() => validateAndResolveImport(document)).toThrow(/modes/i);
});

it("rejects a missing alias target", () => {
  const document = validImport();
  document.variables[2].values.Light = { kind: "alias", path: "primitive/missing/1" };
  expect(() => validateAndResolveImport(document)).toThrow(/alias|missing/i);
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
~~~

- [ ] **Step 3: Expand Vitest's include pattern to execute the .mjs contract test.**

Change the existing Vitest include from tests/**/*.test.tsx to tests/**/*.test.{ts,tsx,mjs}. Do not change the test environment, setup file, or CSS handling.

- [ ] **Step 4: Run the focused test and confirm the expected pre-implementation failure.**

Run from apps/goods-garden-web/:

~~~bash
npm test -- tests/seeds-theme-generator.test.mjs
~~~

Expected: Vitest fails because scripts/generate-seeds-theme.mjs does not exist yet. Do not weaken the tests to make this failure pass.

- [ ] **Step 5: Commit the failing test contract.**

~~~bash
git add apps/goods-garden-web/tests/seeds-theme-generator.test.mjs
git commit -m "test: define SEEDS theme generator contract"
~~~

### Task 2: Implement validation, alias resolution, and deterministic CSS generation

**Files:**

- Create: apps/goods-garden-web/scripts/generate-seeds-theme.mjs
- Modify: apps/goods-garden-web/package.json
- Create: apps/goods-garden-web/src/design-system/seeds-theme.css
- Test: apps/goods-garden-web/tests/seeds-theme-generator.test.mjs

**Interfaces:**

- SEEDS_MODES is a frozen array: ['Light', 'Dark', 'Sakura', 'Momiji', 'NatureLaw', 'Disaster'].
- tokenToCssName(path: string): string converts every path segment with the two camelCase boundary regexes, lowercases it, joins segments with -, and prefixes --seeds-.
- validateAndResolveImport(document: unknown) returns { modes: string[], variables: ResolvedVariable[], counts: { primitive: number, semantic: number, material: number } }.
- generateCss(document: unknown, metadata?: { sourceLabel?: string, sourceHash?: string }): string validates first, then serializes all primitive, semantic, and material variables.
- The CLI accepts exactly --input <absolute-or-relative-json-path> and writes src/design-system/seeds-theme.css only after validation and serialization succeed.

- [ ] **Step 1: Implement the module's validation and resolver.**

Validate in this order so failures identify the first broken contract: object shape, collection name SEEDS, exact mode order, variables array, duplicate paths, supported top-level layer, resolvedType/value-kind pairing, strategy shape, and alias graph. Normalize invariant values as values.default; normalize per-mode values under all six mode names. Resolve aliases with a depth-first function keyed by (path, mode) and a visiting set; use default for invariant targets and the requested mode for per-mode targets. A resolved terminal value is { kind: "color" | "number" | "string", value: string | number }.

The validator must reject component/* variables because this projection intentionally does not generate component contracts. It must accept the existing import's terminal kinds color, number, string, and alias, and must reject null, arrays, objects without a supported kind, non-finite numbers, and non-string color/string values.

~~~js
function resolve(path, mode, visiting) {
  const key = path + "::" + mode;
  if (visiting.has(key)) throw new Error("Alias cycle detected at " + path + " for " + mode);
  const variable = byPath.get(path);
  if (!variable) throw new Error("Missing alias target " + path);
  const spec = variable.modeStrategy === "invariant" ? variable.values.default : variable.values[mode];
  if (spec.kind !== "alias") return { kind: spec.kind, value: spec.value };
  visiting.add(key);
  const resolved = resolve(spec.path, mode, visiting);
  visiting.delete(key);
  return resolved;
}
~~~

- [ ] **Step 2: Implement CSS serialization and units.**

Serialize invariant primitive/material variables under :root; serialize semantic values under :root, :root[data-seeds-mode="Light"] and one [data-seeds-mode="..."] block for each remaining mode. Sort variables by path for deterministic output. Add a generated-file header containing the source label, source SHA-256, mode list, and counts; do not add a timestamp.

Use these exact unit rules for terminal numbers:

| Source path | CSS unit |
| --- | --- |
| material/spacing/*, material/radius/* | px |
| material/typography/*/fontSize, lineHeight, letterSpacing | px |
| material/glass/*/blur, material/motion/transform/*OffsetY | px |
| material/motion/duration/*, material/motion/role/*/duration | ms |
| all other numbers, including weights, scale, saturate, and tintAlpha | unitless |

Preserve color strings and easing strings as CSS values. Reject newline or comment terminators rather than emitting unsafe CSS. The generator must construct the full string before creating the output directory or replacing the CSS file, so invalid input leaves any previous snapshot untouched.

- [ ] **Step 3: Expose the generation command and generate the first snapshot.**

Add this package script without changing dependencies:

~~~json
"generate:seeds": "node scripts/generate-seeds-theme.mjs"
~~~

From apps/goods-garden-web/, run:

~~~bash
npm run generate:seeds -- --input /Users/sei-rinn/dev/workspace_typescript/seeds_design_system/docs/design-system/SEEDS_figma_plugin_import.json
~~~

Expected: src/design-system/seeds-theme.css is created and its header reports primitive=123, semantic=77, material=111, and all six modes. The file contains the resolved aliases and no component/* output.

- [ ] **Step 4: Run the generator tests and inspect the generated projection.**

~~~bash
npm test -- tests/seeds-theme-generator.test.mjs
rg -n -- '--seeds-(semantic|material)-' src/design-system/seeds-theme.css | sed -n '1,40p'
~~~

Expected: all generator tests pass; the generated CSS contains semantic/material properties and mode selectors. The focused test output must show no failed assertions.

- [ ] **Step 5: Commit the generator and generated snapshot.**

~~~bash
git add apps/goods-garden-web/package.json apps/goods-garden-web/scripts/generate-seeds-theme.mjs apps/goods-garden-web/src/design-system/seeds-theme.css apps/goods-garden-web/tests/seeds-theme-generator.test.mjs
git commit -m "feat: generate SEEDS web theme tokens"
~~~

### Task 3: Add the typed presentation-only mode selector

**Files:**

- Create: apps/goods-garden-web/src/design-system/modes.ts
- Modify: apps/goods-garden-web/src/App.tsx
- Modify: apps/goods-garden-web/src/main.tsx
- Modify: apps/goods-garden-web/tests/App.test.tsx

**Interfaces:**

- SEEDS_MODES is the typed readonly tuple of the six exact mode names.
- SeedsMode is (typeof SEEDS_MODES)[number].
- DEFAULT_SEEDS_MODE is SeedsMode and equals "Light".
- App keeps AppProps.view?: GoodsStateViewModel; mode selection does not alter that prop or the rendered view model.

- [ ] **Step 1: Add failing mode-selector tests.**

Extend App.test.tsx with fireEvent, afterEach, and the mode tuple import. Keep the existing healthy/unhealthy/no-action tests unchanged except for shared cleanup required by the root data attribute.

~~~tsx
import { afterEach, fireEvent, render, screen } from "@testing-library/react";
import { SEEDS_MODES } from "../src/design-system/modes";

afterEach(() => {
  document.documentElement.removeAttribute("data-seeds-mode");
});

it("offers every SEEDS presentation mode and changes only the root mode attribute", () => {
  render(<App />);
  const selector = screen.getByRole("combobox", { name: "Display mode" });

  expect(Array.from(selector.querySelectorAll("option")).map((option) => option.value)).toEqual([
    ...SEEDS_MODES,
  ]);
  expect(selector).toHaveValue("Light");

  fireEvent.change(selector, { target: { value: "Dark" } });
  expect(document.documentElement).toHaveAttribute("data-seeds-mode", "Dark");
  expect(screen.getByText("SYNTHETIC EXAMPLE")).toBeInTheDocument();
  expect(screen.getByText("Healthy")).toBeInTheDocument();
  expect(screen.queryByRole("button")).not.toBeInTheDocument();
  expect(screen.queryByRole("form")).not.toBeInTheDocument();
  expect(screen.queryByRole("link")).not.toBeInTheDocument();
});
~~~

- [ ] **Step 2: Run the focused App test and confirm it fails for the missing selector.**

~~~bash
npm test -- tests/App.test.tsx
~~~

Expected: the existing projection assertions pass and the new mode-selector test fails because no Display mode combobox exists.

- [ ] **Step 3: Add the typed mode module and root attribute effect.**

Create modes.ts with:

~~~ts
export const SEEDS_MODES = [
  "Light",
  "Dark",
  "Sakura",
  "Momiji",
  "NatureLaw",
  "Disaster",
] as const;

export type SeedsMode = (typeof SEEDS_MODES)[number];

export const DEFAULT_SEEDS_MODE: SeedsMode = "Light";
~~~

In App.tsx, use useEffect and useState<SeedsMode>. Render a labelled native select with aria-label="Display mode" and one option per SEEDS_MODES, before GoodsStateView. The effect sets document.documentElement.dataset.seedsMode = mode and cleanup deletes the attribute. Do not wrap the selector in a form and do not add business-action controls.

In main.tsx, import ./design-system/seeds-theme.css before ./styles.css so application rules consume the generated variables.

- [ ] **Step 4: Run typecheck and App tests.**

~~~bash
npm run typecheck
npm test -- tests/App.test.tsx
~~~

Expected: TypeScript and all App tests pass; the root attribute is Light after initial render and changes to Dark after the event.

- [ ] **Step 5: Commit the typed mode integration.**

~~~bash
git add apps/goods-garden-web/src/design-system/modes.ts apps/goods-garden-web/src/App.tsx apps/goods-garden-web/src/main.tsx apps/goods-garden-web/tests/App.test.tsx
git commit -m "feat: add SEEDS presentation mode selector"
~~~

### Task 4: Refactor the existing screen CSS onto SEEDS semantic/material properties

**Files:**

- Modify: apps/goods-garden-web/src/styles.css
- Test: apps/goods-garden-web/tests/App.test.tsx

**Interfaces:**

- CSS selectors use the generated names from seeds-theme.css; no selector references a primitive custom property.
- The existing GoodsStateView, HealthAssessment, and ProvenanceBanner data contracts are unchanged.
- The screen uses solid/fallback surfaces and semantic state tokens; it does not consume glass or component-contract variables.

- [ ] **Step 1: Add and run the failing CSS token contract test.**

Keep the App test's existing read-only assertions and add this real-file assertion before changing styles.css. It must fail against the current raw CSS because the required generated properties are absent and raw colors are present:

~~~tsx
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const stylesSource = readFileSync(fileURLToPath(new URL("../src/styles.css", import.meta.url)), "utf8");

it("uses SEEDS semantic and material properties for the presentation surface", () => {
  expect(stylesSource).toContain("--seeds-semantic-surface-canvas");
  expect(stylesSource).toContain("--seeds-semantic-text-primary");
  expect(stylesSource).toContain("--seeds-semantic-border-default");
  expect(stylesSource).toContain("--seeds-semantic-state-success-bg");
  expect(stylesSource).toContain("--seeds-semantic-state-error-bg");
  expect(stylesSource).toContain("--seeds-semantic-state-warning-bg");
  expect(stylesSource).toContain("--seeds-material-spacing-16");
  expect(stylesSource).toContain("--seeds-material-radius-lg");
  expect(stylesSource).toContain("--seeds-material-typography-body-l-font-size");
  expect(stylesSource).not.toMatch(/#[0-9a-fA-F]{3,8}|rgba?\\(/);
  expect(stylesSource).not.toMatch(/--seeds-primitive-/);
});
~~~

Run npm test -- tests/App.test.tsx and record the expected failure before rewriting styles.css. After the rewrite, run the exact token-presence check rg -n -- '--seeds-(semantic|material)-' apps/goods-garden-web/src/styles.css and verify that its output includes every property named in the test.

- [ ] **Step 2: Replace global and layout values with generated properties.**

Use this concrete mapping in styles.css:

| Existing concern | Required property |
| --- | --- |
| page/body canvas | var(--seeds-semantic-surface-canvas) |
| primary headings and identity text | var(--seeds-semantic-text-primary) |
| secondary introduction/evidence copy | var(--seeds-semantic-text-secondary) |
| tertiary labels/kickers | var(--seeds-semantic-text-tertiary) |
| default card borders | var(--seeds-semantic-border-default) |
| subtle dividers | var(--seeds-semantic-border-subtle) |
| provenance background | var(--seeds-semantic-state-warning-bg) |
| provenance foreground | var(--seeds-semantic-state-warning-fg) |
| provenance border | var(--seeds-semantic-state-warning-border) |
| healthy background | var(--seeds-semantic-state-success-bg) |
| healthy foreground | var(--seeds-semantic-state-success-fg) |
| healthy border | var(--seeds-semantic-state-success-border) |
| unhealthy background | var(--seeds-semantic-state-error-bg) |
| unhealthy foreground | var(--seeds-semantic-state-error-fg) |
| unhealthy border | var(--seeds-semantic-state-error-border) |
| page surface | var(--seeds-semantic-surface-canvas) |
| card surface | var(--seeds-semantic-surface-elevated) |
| data surface | var(--seeds-semantic-surface-secondary) |
| padding, gap, margins | var(--seeds-material-spacing-*) |
| card and badge radii | var(--seeds-material-radius-lg/md/full) |
| type size/line-height/weight/letter-spacing | var(--seeds-material-typography-*-*) |
| shadow colors | var(--seeds-semantic-shadow-key) and var(--seeds-semantic-shadow-ambient) |

Remove the existing hex colors, rgb/rgba colors, Georgia/Avenir typography declarations, gradient grid, and large hard-coded spacing/radius values from component selectors. Retain layout-only constraints such as min-width, max-width, breakpoint width, and the 1px border geometry when no corresponding SEEDS token exists. Use material spacing tokens for all component padding, gaps, and margins. Use the source specification's level-1 geometry with semantic shadow colors:

~~~css
box-shadow:
  0 1px 2px var(--seeds-semantic-shadow-key),
  0 1px 3px var(--seeds-semantic-shadow-ambient);
~~~

For the mode selector, use --seeds-semantic-surface-primary, --seeds-semantic-text-primary, and --seeds-semantic-border-default; it remains a presentation control, not a domain action.

- [ ] **Step 3: Apply exact semantic state rules.**

The resulting state selectors must include the equivalent of:

~~~css
.health-card {
  background: var(--seeds-semantic-state-success-bg);
  border-color: var(--seeds-semantic-state-success-border);
}

.health-card--unhealthy {
  background: var(--seeds-semantic-state-error-bg);
  border-color: var(--seeds-semantic-state-error-border);
}

.health-card--healthy h2 {
  color: var(--seeds-semantic-state-success-fg);
}

.health-card--unhealthy h2 {
  color: var(--seeds-semantic-state-error-fg);
}
~~~

Keep the generated status class as the only status-dependent class; no new status or action concept is introduced.

- [ ] **Step 4: Run CSS-focused checks and the full frontend suite.**

~~~bash
if rg -n '#[0-9a-fA-F]{3,8}|rgba?\\(' apps/goods-garden-web/src/styles.css; then exit 1; fi
rg -n -- '--seeds-(semantic|material)-' apps/goods-garden-web/src/styles.css
npm run typecheck --prefix apps/goods-garden-web
npm test --prefix apps/goods-garden-web
npm run build --prefix apps/goods-garden-web
~~~

Expected: the raw-color scan produces no matches; CSS references semantic/material properties; all frontend tests and the production build pass.

- [ ] **Step 5: Commit the tokenized screen CSS.**

~~~bash
git add apps/goods-garden-web/src/styles.css apps/goods-garden-web/tests/App.test.tsx
git commit -m "refactor: apply SEEDS tokens to Goods State view"
~~~

### Task 5: Regenerate, verify, and hand off the Work Item

**Files:**

- Modify: apps/goods-garden-web/src/design-system/seeds-theme.css only through the generator command when the source changes
- Inspect: all files in this plan and the active AI Cockpit Contract/Summary

**Interfaces:**

- The final generated header must report the six modes and counts 123/77/111.
- The final frontend build consumes the committed CSS snapshot without reading the external source.
- AI Cockpit verification evidence must bind to the current Contract and actual command output; no .ai generated record is hand-edited.

- [ ] **Step 1: Re-run generation from the authoritative external JSON.**

From apps/goods-garden-web/:

~~~bash
npm run generate:seeds -- --input /Users/sei-rinn/dev/workspace_typescript/seeds_design_system/docs/design-system/SEEDS_figma_plugin_import.json
~~~

Then confirm the generated metadata and source bindings:

~~~bash
rg -n 'SEEDS_figma_plugin_import.json|primitive=123|semantic=77|material=111|Light.*Dark.*Sakura.*Momiji.*NatureLaw.*Disaster' src/design-system/seeds-theme.css
~~~

Expected: the header identifies the external JSON and reports the exact production counts and modes.

- [ ] **Step 2: Run the Contract's frontend and Rust verification commands.**

Run the generation command from apps/goods-garden-web/ as specified in Step 1. Then, from the repository root, execute each remaining command and retain its actual output for Runtime evidence:

~~~bash
git diff --check
cargo fmt --check
cargo check --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace
npm ci --prefix apps/goods-garden-web
npm run typecheck --prefix apps/goods-garden-web
npm test --prefix apps/goods-garden-web
npm run build --prefix apps/goods-garden-web
make -f Makefile.ai quality
~~~

Expected: every command exits successfully; the frontend tests retain healthy/unhealthy, synthetic, mode, and no-action coverage; no Rust file is changed.

- [ ] **Step 3: Revalidate the current Work Item before Runtime verification.**

~~~bash
ai-cockpit preflight --repo /Users/sei-rinn/dev/workspace_rust/goods-garden-worktrees/seeds-design-system --contract .ai/work-items/active/seeds-design-system.contract.json
ai-cockpit work-item validate --repo /Users/sei-rinn/dev/workspace_rust/goods-garden-worktrees/seeds-design-system --id seeds-design-system --json
~~~

Expected: the Contract remains authorized and aligned; any remaining yellow state must identify only verification evidence that the next command records. If a new humanDecisionRequest appears, stop and present it rather than inferring authorization.

- [ ] **Step 4: Record Runtime verification and finish/archive through AI Cockpit.**

Use the Runtime with the exact repository path and Work Item ID; do not edit .ai output files:

~~~bash
ai-cockpit verify --repo /Users/sei-rinn/dev/workspace_rust/goods-garden-worktrees/seeds-design-system --work-item seeds-design-system --stage task --command "make" --args "-f Makefile.ai quality"
ai-cockpit finish --repo /Users/sei-rinn/dev/workspace_rust/goods-garden-worktrees/seeds-design-system --id seeds-design-system
ai-cockpit archive --repo /Users/sei-rinn/dev/workspace_rust/goods-garden-worktrees/seeds-design-system --id seeds-design-system
~~~

The human Outcome must state the actual verification facts, issue count, blockers/stopping reason, resolved issues, risks, unknowns, evidence, human decision, impact, and next action. Do not claim an unproven user benefit as fact. Do not run close because this request does not include a reviewed PR, merge, default-branch synchronization, or branch cleanup authorization.

- [ ] **Step 5: Review the final diff and report the handoff.**

~~~bash
git status --short --branch
git diff --stat origin/main...HEAD
git diff --check
~~~

Expected: only the scoped plan, generator, generated theme, typed mode integration, tests, and tokenized CSS are present on codex/seeds-design-system; the base branch remains untouched. Report the branch/worktree path, verification results, unresolved unknowns, and that push/PR/merge were not performed.

## 日本語

### ファイル構成と責務

外部の SEEDS_figma_plugin_import.json を正本とし、apps/goods-garden-web/scripts/generate-seeds-theme.mjs が検証・alias 解決・決定的な CSS 生成を担当する。生成物は src/design-system/seeds-theme.css、6 mode の型は src/design-system/modes.ts に置く。App.tsx は presentation-only の mode selector と document root の data-seeds-mode だけを管理し、Goods State の view model は変更しない。styles.css は semantic/material property のみを消費し、Rust、domain、POS、SEJ、server、API、Need、Care、自律 action には触れない。

### 実装タスク

#### Task 1: Generator のテストを先に作る

apps/goods-garden-web/tests/seeds-theme-generator.test.mjs を作成し、SEEDS_MODES、tokenToCssName、validateAndResolveImport、generateCss の契約を検証する。fixture には正確な Light、Dark、Sakura、Momiji、NatureLaw、Disaster と、primitive 2、semantic 1、material 2 の変数を置く。semantic/text/primary の mode 別 alias 解決、camelCase の kebab-case 化、CSS selector 出力を検証する。mode 欠落、alias 先欠落、自参照、cycle は toThrow で検証する。

~~~bash
npm test -- tests/seeds-theme-generator.test.mjs
~~~

最初は generator file がないため失敗することを確認し、test: define SEEDS theme generator contract でテスト契約を保存する。

#### Task 2: Generator と generated CSS を実装する

JSON の形、SEEDS collection、6 mode の順序、重複 path、primitive/semantic/material の層、resolvedType と value kind、invariant/per-mode の値、alias graph を順に fail-closed で検証する。DFS で (path, mode) ごとに alias を解決し、missing target、self alias、cycle を出力前に拒否する。primitive/material は :root、semantic は :root と各 data-seeds-mode selector に出力し、path の / と camelCase を安定した CSS custom property に変換する。

数値の単位は spacing/radius/fontSize/lineHeight/letterSpacing/blur/OffsetY を px、duration を ms、weight・scale・saturate・tintAlpha を unitless とする。header に source label、SHA-256、mode、primitive=123 semantic=77 material=111 を入れ、timestamp は入れない。package script は generate:seeds の一つだけ追加する。生成に成功してからだけ seeds-theme.css を置換し、generator test と実 source の生成を確認する。

#### Task 3: 6 mode の presentation selector を追加する

modes.ts には exact tuple、SeedsMode、DEFAULT_SEEDS_MODE = Light を定義する。App.tsx は native select を6 optionで描画し、useEffect で root の data-seeds-mode を更新、cleanup で削除する。selector は form/button/link を追加せず、view model や GoodsStateView の値を変更しない。main.tsx は generated CSS を styles.css より先に import する。全 option、初期 Light、Dark への変更、synthetic/healthy 表示、button/form/link 不在をテストする。

#### Task 4: Screen CSS を token 化する

canvas は semantic/surface/canvas、text は primary/secondary/tertiary、border は default/subtle、state は success/error/warning の bg/fg/border、layout は material spacing/radius、typography は material typography、shadow color は semantic shadow key/ambient を使う。hex、rgb/rgba、gradient grid、Georgia/Avenir の直接指定を削除する。screen は solid/fallback のままとし、glass/component contract は使わない。elevation geometry は仕様の level-1 geometry を使い、色だけ semantic shadow property にする。

#### Task 5: 再生成、検証、handoff

生成 header、6 mode、123/77/111 の count、source path を確認する。その後、Contract に記載された git diff --check、Rust の fmt/check/clippy/test、frontend の npm ci/generate/typecheck/test/build、make -f Makefile.ai quality を実際に実行する。ai-cockpit preflight、work-item validate、verify、finish、archive を明示的な --repo と Work Item ID で実行し、.ai は手で変更しない。reviewed PR、merge、default branch 同期、branch cleanup の依頼はないため close は実行しない。

## 中文

### 文件结构与职责

以外部 /Users/sei-rinn/dev/workspace_typescript/seeds_design_system/docs/design-system/SEEDS_figma_plugin_import.json 为唯一来源。apps/goods-garden-web/scripts/generate-seeds-theme.mjs 负责校验、解析 alias、生成确定性的 CSS；src/design-system/seeds-theme.css 是提交的生成结果；src/design-system/modes.ts 定义六个模式和默认模式。App.tsx 只管理展示用 mode selector 与 document root 的 data-seeds-mode，不修改 Goods State view model。styles.css 只消费 semantic/material property，不引入 Rust、domain、POS、SEJ、server、API、Need、Care 或自主 action。

### 实现任务

#### Task 1：先编写 Generator 测试

创建 apps/goods-garden-web/tests/seeds-theme-generator.test.mjs，测试 SEEDS_MODES、tokenToCssName、validateAndResolveImport、generateCss。fixture 必须使用准确的 Light、Dark、Sakura、Momiji、NatureLaw、Disaster，并包含 primitive 2、semantic 1、material 2。验证 semantic/text/primary 的按模式 alias 解析、camelCase 到 kebab-case、CSS mode selector 输出。缺失模式、缺失 alias、自引用和 alias cycle 必须抛错。先确认因为 generator 文件不存在而失败，再提交 test: define SEEDS theme generator contract。

#### Task 2：实现校验、alias 解析和 CSS 生成

按顺序校验 JSON 结构、SEEDS collection、六模式及顺序、重复 path、primitive/semantic/material 层、resolvedType 与 value kind、invariant/per-mode 值和 alias graph。使用以 (path, mode) 为键的 DFS，遇到 missing target、self alias 或 cycle 时在写出前停止。primitive/material 写入 :root；semantic 写入 :root 和每个 data-seeds-mode selector；将 Figma / 路径和 camelCase 稳定地转换为 CSS custom property。

spacing/radius/fontSize/lineHeight/letterSpacing/blur/OffsetY 使用 px，duration 使用 ms，weight、scale、saturate、tintAlpha 不加单位。生成文件 header 必须包含 source label、SHA-256、模式和 primitive=123 semantic=77 material=111，不包含时间戳。只有校验和序列化成功后才替换 seeds-theme.css。

#### Task 3：增加展示用六模式选择器

modes.ts 定义 exact tuple、SeedsMode 和 DEFAULT_SEEDS_MODE = Light。App.tsx 渲染 select 与六个 option，使用 useEffect 更新 root 的 data-seeds-mode，cleanup 时删除。不得增加 form/button/link，也不得改变 view model 或 GoodsStateView 的数据。main.tsx 必须在 styles.css 前加载 generated CSS。测试全量 option、默认 Light、切换 Dark、synthetic/healthy 文本和 button/form/link 不存在。

#### Task 4：将 screen CSS 改为 token 消费

canvas 使用 semantic/surface/canvas；文字使用 primary/secondary/tertiary；border 使用 default/subtle；状态使用 success/error/warning 的 bg/fg/border；布局使用 material spacing/radius；排版使用 material typography；shadow 使用 semantic shadow key/ambient。删除 hex、rgb/rgba、gradient grid、Georgia/Avenir 直接值。界面继续使用 solid/fallback，不使用 glass 或 component contract。阴影几何使用规范 level-1，颜色使用 semantic shadow token。

#### Task 5：重新生成、验证并交付

检查生成 header、六模式、123/77/111 counts 和 source path。执行 Contract 中的 git diff --check、Rust fmt/check/clippy/test、frontend npm ci/generate/typecheck/test/build、make -f Makefile.ai quality。使用显式 --repo /Users/sei-rinn/dev/workspace_rust/goods-garden-worktrees/seeds-design-system 和 Work Item ID 执行 ai-cockpit preflight、work-item validate、verify、finish、archive，不手改 .ai 生成记录。用户没有要求 reviewed PR、merge、default branch 同步或清理 branch，因此不执行 close。
