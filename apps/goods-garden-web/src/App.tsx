import { useEffect, useState } from "react";
import { syntheticGoodsStateView } from "./demo/synthetic-goods-state";
import { GoodsStateView } from "./components/GoodsStateView";
import { DEFAULT_SEEDS_MODE, SEEDS_MODES, type SeedsMode } from "./design-system/modes";
import type { GoodsStateView as GoodsStateViewModel } from "./view-models/goods-state-view";

export interface AppProps {
  view?: GoodsStateViewModel;
}

export function App({ view = syntheticGoodsStateView }: AppProps = {}) {
  const [mode, setMode] = useState<SeedsMode>(DEFAULT_SEEDS_MODE);

  useEffect(() => {
    document.documentElement.dataset.seedsMode = mode;

    return () => {
      delete document.documentElement.dataset.seedsMode;
    };
  }, [mode]);

  return (
    <>
      <label>
        <span>Display mode</span>
        <select aria-label="Display mode" value={mode} onChange={(event) => setMode(event.target.value as SeedsMode)}>
          {SEEDS_MODES.map((seedsMode) => (
            <option key={seedsMode} value={seedsMode}>
              {seedsMode}
            </option>
          ))}
        </select>
      </label>
      <GoodsStateView view={view} />
    </>
  );
}
