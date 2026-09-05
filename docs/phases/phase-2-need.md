# Phase 2 — Need

## Goal

Goods が何に問題があり何を必要とするかを発見する。不変の Phase 1 State/Health Assessment の上に、Deviation、Need、
Urgency、Need Conflict を追加し、KPI を単なる Alert ではなく explainable な business Need に変換する。この phase では
Care を求める・受け取る挙動は実装しない。

## Implemented boundary

Phase 2 に含めるのは次だけである。

- Deviation: ある次元での observation と expectation の間の符号付き、定量化された gap。次元は Freshness
  （Phase 1 の `Expectation` を再利用）と StockAvailability（本 phase で新設）の2つを実装する。
- StockAvailability を Freshness と並んで評価できるよう、GoodsProfile に `minimum_stock_quantity`、
  Observation に `quantity_on_hand` を追加する。
- Urgency: Deviation の magnitude から次元別の固定閾値で導く explainable な `Low`/`Medium`/`High` level。
  この閾値は `example`/`hypothesis` 値であり、real POS や SEJ data から導いたものではない。
- GoodsNeed: `NeedKind`（`FreshnessConcern` または `StockAvailabilityConcern`）、その Urgency、Deviation、
  plain-language な explanation。Need は Deviation が懸念あり（expectation を超過）の場合にのみ識別される。
- Need Conflict: `FreshnessConcern` の Need と `StockAvailabilityConcern` の Need が共存する場合、Goods
  Garden は2つの推奨方向の間の explainable な矛盾を提示する。矛盾を解決せず、どちらの方向も推奨しない。
- NeedAssessment: 1つの observation から Need と Need Conflict の有無を識別した bounded な結果。

bounded rule は、Deviation の magnitude が正なら懸念あり、0以下なら懸念なしとする。これは minimal で
reviewable な rule であり、完全な inventory、waste、replenishment model ではない。

## Local demo と data boundary

repository root から demo を実行する。

```bash
cargo run -p goods-garden-cli -- demo
```

path は Phase 1 を拡張する。

```text
examples/tuna-mayo/observation.example.txt
    ↓
DemoObservationSource
    ↓
ObservationSource port
    ↓
GoodsRuntime::observe_and_identify_needs
    ↓
Goods State + Health Assessment（不変）
Need Assessment（Need + Need Conflict）
    ↓
CLI output
```

fixture は引き続き `synthetic-example` と明記し、同梱 demo が Need を発生させない値（両次元とも expectation 内）に
調整する。Need Conflict は自動テストが crafted な observation で検証するのみで、同梱 fixture では発生しない。

## Exit criteria

- local user が demo を実行し、不変の Phase 1 State/Health 出力に加え、識別された Need、その Urgency、
  Need Conflict の有無を確認できる。
- need無し・freshness単独・stock単独・conflict の case を test する。
- Care、Memory、Learning、自律 action、real external data は実装しない。
