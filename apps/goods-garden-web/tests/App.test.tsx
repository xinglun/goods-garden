import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { App } from "../src/App";
import type { GoodsStateView } from "../src/view-models/goods-state-view";

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
});
