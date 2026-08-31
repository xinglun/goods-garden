import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { App } from "../src/App";
import { SEEDS_MODES } from "../src/design-system/modes";
import type { GoodsStateView } from "../src/view-models/goods-state-view";

const stylesSource = readFileSync(
  fileURLToPath(new URL(["..", "src", "styles.css"].join("/"), import.meta.url)),
  "utf8",
);

afterEach(() => {
  document.documentElement.removeAttribute("data-seeds-mode");
});

const genericUnhealthyView: GoodsStateView = {
  identity: {
    species: "generic-demo-product",
    individualId: "generic-demo-001",
  },
  profile: {
    displayName: "Generic Demo Product",
    expectedLifetimeHours: 8,
  },
  observation: {
    source: "local synthetic observation",
    observedAt: "2026-08-28T10:00:00+09:00",
    observedAgeHours: 9,
  },
  expectation: {
    maximumAgeHours: 8,
  },
  healthAssessment: {
    status: "unhealthy",
    explanation: "The supplied observation is beyond the profile expectation.",
  },
  provenance: {
    kind: "synthetic",
    label: "SYNTHETIC EXAMPLE",
  },
};

describe("Goods Garden State view", () => {
  it("renders a synthetic read-only Goods State", () => {
    render(<App />);

    expect(screen.getByRole("heading", { name: "Goods Garden" })).toBeInTheDocument();
    expect(screen.getByText("SYNTHETIC EXAMPLE")).toBeInTheDocument();
    expect(screen.getByText("Healthy")).toBeInTheDocument();
    expect(screen.queryByRole("button")).not.toBeInTheDocument();
  });

  it("renders a generic unhealthy projection without an action surface", () => {
    render(<App view={genericUnhealthyView} />);

    expect(screen.getByText("Generic Demo Product")).toBeInTheDocument();
    expect(screen.getByText("Needs attention")).toBeInTheDocument();
    expect(screen.getByText("The supplied observation is beyond the profile expectation.")).toBeInTheDocument();
    expect(screen.getByText("SYNTHETIC EXAMPLE")).toBeInTheDocument();
    expect(screen.queryByRole("button")).not.toBeInTheDocument();
    expect(screen.queryByRole("form")).not.toBeInTheDocument();
    expect(screen.queryByRole("link")).not.toBeInTheDocument();
  });

  it("offers every SEEDS presentation mode and changes only the root mode attribute", () => {
    render(<App />);
    const selector = screen.getByRole("combobox", { name: "Display mode" });

    expect(Array.from(selector.querySelectorAll("option")).map((option) => option.value)).toEqual([
      ...SEEDS_MODES,
    ]);
    expect(selector).toHaveValue("Light");
    expect(document.documentElement).toHaveAttribute("data-seeds-mode", "Light");

    fireEvent.change(selector, { target: { value: "Dark" } });
    expect(document.documentElement).toHaveAttribute("data-seeds-mode", "Dark");
    expect(screen.getByText("SYNTHETIC EXAMPLE")).toBeInTheDocument();
    expect(screen.getByText("Healthy")).toBeInTheDocument();
    expect(screen.queryByRole("button")).not.toBeInTheDocument();
    expect(screen.queryByRole("form")).not.toBeInTheDocument();
    expect(screen.queryByRole("link")).not.toBeInTheDocument();
  });

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
    expect(stylesSource).not.toMatch(/#[0-9a-fA-F]{3,8}|rgba?\(/);
    expect(stylesSource).not.toMatch(/--seeds-primitive-/);
  });
});
