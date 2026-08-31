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
});
