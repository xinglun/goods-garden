# Phase 2 — Need

## English

### Goal

Goods discovers what is uncomfortable and what it needs. Add Deviation, Need,
Urgency and Need Conflict on top of the unchanged Phase 1 State/Health
Assessment, so a KPI can become an explainable business Need, not only an
Alert. This phase does not implement asking for or receiving Care.

### Implemented boundary

Phase 2 includes only:

- Deviation: a signed, quantified gap between an observation and its
  expectation on one dimension. Two dimensions are implemented: Freshness
  (reusing the Phase 1 `Expectation`) and StockAvailability (a new dimension
  added by this phase).
- GoodsProfile gains `minimum_stock_quantity`; Observation gains
  `quantity_on_hand`, so StockAvailability can be evaluated alongside
  Freshness.
- Urgency: an explainable `Low`/`Medium`/`High` level derived from a
  Deviation's magnitude by fixed, dimension-specific thresholds. These
  thresholds are `example`/`hypothesis` values, not derived from real POS or
  SEJ data.
- GoodsNeed: a `NeedKind` (`FreshnessConcern` or `StockAvailabilityConcern`),
  its Urgency, its Deviation and a plain-language explanation. A Need is
  identified only when its Deviation is concerning (beyond expectation).
- Need Conflict: when a `FreshnessConcern` Need and a `StockAvailabilityConcern`
  Need coexist, Goods Garden surfaces an explainable contradiction between the
  two recommended directions. It does not resolve the contradiction and does
  not recommend either direction.
- NeedAssessment: the bounded result of identifying Needs and any Need
  Conflict from one observation.

The bounded rule is that a Deviation with a positive magnitude is concerning;
one at or below zero is not. This is a minimal, reviewable rule, not a
complete inventory, waste or replenishment model.

### Local demo and data boundary

Run the demo from the repository root:

```bash
cargo run -p goods-garden-cli -- demo
```

The path extends Phase 1's:

```text
examples/tuna-mayo/observation.example.txt
    ↓
DemoObservationSource
    ↓
ObservationSource port
    ↓
GoodsRuntime::observe_and_identify_needs
    ↓
Goods State + Health Assessment (unchanged)
Need Assessment (Needs + Need Conflict)
    ↓
CLI output
```

The fixture remains labelled `synthetic-example` and is tuned so the bundled
demo shows no Need (both dimensions are within expectation). A Need Conflict
is exercised only by the automated test suite with a crafted observation, not
by the bundled fixture.

### Exit criteria

- A local user can run the demo and see any identified Need, its Urgency, and
  any Need Conflict, alongside the unchanged Phase 1 State/Health output.
- The no-need, freshness-only, stock-only and conflicting cases are tested.
- Care, Memory, Learning, autonomous action and real external data are not
  implemented.

## 日本語

### Goal

Goods が何に問題があり何を必要とするかを発見する。不変の Phase 1 State/Health Assessment の上に、Deviation、Need、
Urgency、Need Conflict を追加し、KPI を単なる Alert ではなく explainable な business Need に変換する。この phase では
Care を求める・受け取る挙動は実装しない。

### Implemented boundary

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

### Local demo と data boundary

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

### Exit criteria

- local user が demo を実行し、不変の Phase 1 State/Health 出力に加え、識別された Need、その Urgency、
  Need Conflict の有無を確認できる。
- need無し・freshness単独・stock単独・conflict の case を test する。
- Care、Memory、Learning、自律 action、real external data は実装しない。

## 中文

### Goal

商品发现哪里不舒服以及需要什么。在不变的 Phase 1 State/Health Assessment 之上，增加 Deviation、Need、Urgency、
Need Conflict，使 KPI 能转换为可解释的经营 Need，而不只是 Alert。本阶段不实现求助或接受 Care 的行为。

### Implemented boundary

Phase 2 只包含：

- Deviation：某维度上 observation 与 expectation 之间带符号、可量化的 gap。实现两个维度：复用 Phase 1
  `Expectation` 的 Freshness，以及本阶段新增的 StockAvailability。
- 为使 StockAvailability 能与 Freshness 一同评估，GoodsProfile 新增 `minimum_stock_quantity`，
  Observation 新增 `quantity_on_hand`。
- Urgency：根据 Deviation 的 magnitude、按维度专属固定阈值得出的可解释 `Low`/`Medium`/`High` 级别。该阈值为
  `example`/`hypothesis` 数值，并非源自真实 POS 或 SEJ 数据。
- GoodsNeed：`NeedKind`（`FreshnessConcern` 或 `StockAvailabilityConcern`）、其 Urgency、Deviation 与
  自然语言 explanation。仅当 Deviation 令人担忧（超出 expectation）时才会识别出 Need。
- Need Conflict：当 `FreshnessConcern` 的 Need 与 `StockAvailabilityConcern` 的 Need 共存时，Goods
  Garden 呈现两个推荐方向之间可解释的矛盾，不解决矛盾，也不推荐任一方向。
- NeedAssessment：从一次 observation 中识别出的 Need 与是否存在 Need Conflict 的有边界结果。

有边界规则是：Deviation 的 magnitude 为正时视为令人担忧，小于等于零则不然。这是最小的、可 review 的规则，
不是完整的库存、损耗或补货模型。

### 本地 demo 与数据边界

从仓库根目录运行 demo：

```bash
cargo run -p goods-garden-cli -- demo
```

路径在 Phase 1 基础上扩展：

```text
examples/tuna-mayo/observation.example.txt
    ↓
DemoObservationSource
    ↓
ObservationSource port
    ↓
GoodsRuntime::observe_and_identify_needs
    ↓
Goods State + Health Assessment（不变）
Need Assessment（Need + Need Conflict）
    ↓
CLI output
```

fixture 继续明确标记为 `synthetic-example`，并调整为使内置 demo 不产生 Need（两个维度均在 expectation
之内）。Need Conflict 仅由自动化测试使用构造的 observation 验证，内置 fixture 不会触发。

### Exit criteria

- 本地用户运行 demo，能在不变的 Phase 1 State/Health 输出之外，看到已识别的 Need、其 Urgency 以及是否存在
  Need Conflict。
- 测试无 need、仅 freshness、仅 stock 与冲突四种情况。
- 不实现 Care、Memory、Learning、自主行动和真实外部数据。
