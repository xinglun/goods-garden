import type { GoodsStateView } from "../view-models/goods-state-view";

export const syntheticGoodsStateView: GoodsStateView = {
  identity: {
    species: "tuna-mayo-onigiri",
    individualId: "tuna-mayo-demo-001",
  },
  profile: {
    displayName: "Tuna Mayo Onigiri",
    expectedLifetimeHours: 12,
  },
  observation: {
    source: "local synthetic observation",
    observedAt: "2026-08-28T09:00:00+09:00",
    observedAgeHours: 4,
  },
  expectation: {
    maximumAgeHours: 12,
  },
  healthAssessment: {
    status: "healthy",
    explanation: "The supplied observation is within the profile expectation.",
  },
  provenance: {
    kind: "synthetic",
    label: "SYNTHETIC EXAMPLE",
  },
};

