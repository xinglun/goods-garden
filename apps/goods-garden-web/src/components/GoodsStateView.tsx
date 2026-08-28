import type { GoodsStateView as GoodsStateViewModel } from "../view-models/goods-state-view";
import { HealthAssessment } from "./HealthAssessment";
import { ProvenanceBanner } from "./ProvenanceBanner";

interface GoodsStateViewProps {
  view: GoodsStateViewModel;
}

function DataPoint({ label, value }: { label: string; value: string }) {
  return (
    <div className="data-point">
      <dt>{label}</dt>
      <dd>{value}</dd>
    </div>
  );
}

export function GoodsStateView({ view }: GoodsStateViewProps) {
  return (
    <main className="goods-page">
      <header className="page-header">
        <div>
          <p className="eyebrow">Goods Intelligence / State</p>
          <h1>Goods Garden</h1>
          <p className="page-introduction">
            A quiet view of what this product knows about its current state.
          </p>
        </div>
        <div className="identity-mark" aria-hidden="true">
          <span>GG</span>
        </div>
      </header>

      <ProvenanceBanner label={view.provenance.label} />

      <section className="state-card" aria-labelledby="product-heading">
        <div className="state-card__heading">
          <div>
            <div className="section-kicker">Product identity</div>
            <h2 id="product-heading">{view.profile.displayName}</h2>
            <p className="identity-line">
              {view.identity.species} <span aria-hidden="true">·</span> {view.identity.individualId}
            </p>
          </div>
          <span className="state-pill">State observed</span>
        </div>

        <dl className="data-grid">
          <DataPoint label="Observation source" value={view.observation.source} />
          <DataPoint label="Observed at" value={view.observation.observedAt} />
          <DataPoint label="Observed age" value={`${view.observation.observedAgeHours} hours`} />
          <DataPoint label="Expected lifetime" value={`${view.profile.expectedLifetimeHours} hours`} />
          <DataPoint label="Expectation ceiling" value={`${view.expectation.maximumAgeHours} hours`} />
        </dl>
      </section>

      <HealthAssessment assessment={view.healthAssessment} />

      <footer className="evidence-footer">
        <div>
          <div className="section-kicker">Evidence boundary</div>
          <p>
            This screen presents supplied observation, expectation and assessment values. It does
            not infer a Need or initiate Care.
          </p>
        </div>
        <span className="footer-label">Read-only</span>
      </footer>
    </main>
  );
}
