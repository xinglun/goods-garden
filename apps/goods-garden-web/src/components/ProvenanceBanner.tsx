interface ProvenanceBannerProps {
  label: string;
}

export function ProvenanceBanner({ label }: ProvenanceBannerProps) {
  return (
    <aside className="provenance-banner" aria-label="Data provenance">
      <span className="provenance-dot" aria-hidden="true" />
      <div>
        <strong>{label}</strong>
        <p>This read-only view uses local example data. It is not a POS or SEJ feed.</p>
      </div>
    </aside>
  );
}

