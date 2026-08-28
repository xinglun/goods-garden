import { syntheticGoodsStateView } from "./demo/synthetic-goods-state";
import { GoodsStateView } from "./components/GoodsStateView";
import type { GoodsStateView as GoodsStateViewModel } from "./view-models/goods-state-view";

export interface AppProps {
  view?: GoodsStateViewModel;
}

export function App({ view = syntheticGoodsStateView }: AppProps = {}) {
  return <GoodsStateView view={view} />;
}

