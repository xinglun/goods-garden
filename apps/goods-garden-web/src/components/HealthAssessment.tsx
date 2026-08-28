import type { GoodsStateView } from "../view-models/goods-state-view";

interface HealthAssessmentProps {
  assessment: GoodsStateView["healthAssessment"];
}

export function HealthAssessment({ assessment }: HealthAssessmentProps) {
  const statusLabel = assessment.status === "healthy" ? "Healthy" : "Needs attention";

  return (
    <section className={`health-card health-card--${assessment.status}`} aria-labelledby="health-heading">
      <div className="section-kicker">Current assessment</div>
      <h2 id="health-heading">{statusLabel}</h2>
      <p>{assessment.explanation}</p>
    </section>
  );
}
