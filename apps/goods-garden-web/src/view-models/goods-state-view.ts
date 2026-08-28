export type HealthAssessmentStatus = "healthy" | "unhealthy";

export interface GoodsStateView {
  identity: {
    species: string;
    individualId: string;
  };
  profile: {
    displayName: string;
    expectedLifetimeHours: number;
  };
  observation: {
    source: string;
    observedAt: string;
    observedAgeHours: number;
  };
  expectation: {
    maximumAgeHours: number;
  };
  healthAssessment: {
    status: HealthAssessmentStatus;
    explanation: string;
  };
  provenance: {
    kind: "synthetic";
    label: "SYNTHETIC EXAMPLE";
  };
}

