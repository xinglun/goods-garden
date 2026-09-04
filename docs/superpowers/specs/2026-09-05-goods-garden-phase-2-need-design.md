# Goods Garden Phase 2 — Need Design Specification

**Status:** PROPOSED — requires human review before implementation

**Date:** 2026-09-05

**Scope:** `goods-domain`, `goods-application`, `goods-runtime`, `goods-infrastructure` and
`apps/goods-garden-cli` additions for Phase 2 (Deviation, Urgency, GoodsNeed, Need Conflict).

## English

### Context and problem

Phase 1 gave one synthetic reference good a bounded healthy/unhealthy State.
`crates/goods-domain/src/need/goods_need.rs` and
`crates/goods-application/src/use_cases/identify_need.rs` were left as empty
placeholders for the next phase. `docs/phases/phase-2-need.md` names Deviation,
Need, Urgency and Need Conflict but did not yet define their fields or
boundary.

A single evaluation dimension (Freshness/age) cannot produce a genuine Need
Conflict: there is nothing for a Freshness Need to conflict with. This
proposal adds one further evaluation dimension, StockAvailability, so that two
Needs can coexist and recommend opposite directions.

### Goals

1. Add `Deviation`, `Urgency`, `GoodsNeed` and `NeedConflict` to `goods-domain`
   without changing `state::goods_state` (Phase 1's archived contract).
2. Add a second, independent evaluation dimension (StockAvailability) so a
   Need Conflict can actually occur, not just be modeled in the abstract.
3. Implement `IdentifyNeed` in `goods-application` and expose it from
   `GoodsRuntime` alongside the unchanged `observe_and_assess`.
4. Keep the CLI demo's default, bundled run showing no Need, preserving the
   calm Phase 1 reference experience; exercise the Need/Conflict paths through
   tests with crafted observations.
5. Name the Need vocabulary as concern/state descriptions, not Care verbs, so
   Phase 2 does not pre-empt the Phase 3 Care boundary.

### Non-goals

This proposal does not implement or authorize Care, CareRequest, CareAction,
Memory, Learning, autonomous business action, real POS/SEJ data, an API
server, a database, or any frontend change (`apps/goods-garden-web/` is
untouched; Need remains explicitly out of scope for the frontend per
AGENTS.md).

### Decision: two dimensions, not one

`Observation` gains `quantity_on_hand: u32`; `GoodsProfile` gains
`minimum_stock_quantity: u32`. `Deviation::freshness` reuses the existing
`Expectation` (`age_hours - max_age_hours`); `Deviation::stock_availability`
is new (`minimum_stock_quantity - quantity_on_hand`). A positive magnitude on
either dimension is concerning; each concerning Deviation becomes one
`GoodsNeed` with a `NeedKind`:

```rust
pub enum NeedKind { FreshnessConcern, StockAvailabilityConcern }
```

`NeedConflict::detect` fires only when both kinds are present in the same
`NeedAssessment`, describing the contradiction (remove from shelf vs. keep
stocked) without resolving it or recommending either action.

### Decision: Urgency thresholds are fixed example values

```text
Freshness (hours beyond max_age_hours):    1-2 = Low, 3-5 = Medium, 6+ = High
StockAvailability (units below minimum):   1   = Low, 2-3 = Medium, 4+ = High
```

These are `example`/`hypothesis` values in the Trust Model sense (see
`docs/architecture/trust-model.md`), not derived from real POS, SEJ or
inventory data. A later, evidence-bearing Work Item may replace them.

### Decision: NeedKind names a concern, not a Care action

`FreshnessConcern`/`StockAvailabilityConcern` were chosen over verb-shaped
alternatives (e.g. `ReduceForFreshness`/`ReplenishForAvailability`) so that
Need names describe the concerning condition rather than a recommended
response, keeping the Need/Care boundary in `AGENTS.md` explicit.

### Alternatives considered

**Deferring Need Conflict to a later phase:** rejected because
`docs/phases/phase-2-need.md` explicitly names Need Conflict as in-scope for
Phase 2, and a single-dimension model cannot exercise it meaningfully.

**A generic `Vec<Deviation>` with dynamic dimension registration:** rejected
as premature abstraction; two concrete, named dimensions are enough to
demonstrate and test a real conflict without speculative extensibility.

**Verb-shaped `NeedKind` variants:** rejected per the naming decision above.

### Known facts, inferences and unknowns

- **KNOWN:** `GoodsNeed` and `IdentifyNeed` were empty placeholders reserved
  for this phase; `state::goods_state` is Phase 1's archived, frozen contract.
- **KNOWN:** `Observation`/`GoodsProfile` are constructed as struct literals in
  exactly five places in the repository (the simulator adapter, the CLI, and
  three call sites in `phase_1_demo.rs`); adding required fields is a
  compiler-enforced, fully auditable change.
- **INFERRED:** a stock-availability dimension is a plausible second Need
  source for a convenience-store-style good, illustrating a realistic
  freshness-vs-availability tension without inventing POS-specific behavior.
- **UNKNOWN:** the real thresholds a store would use for Urgency, and the real
  minimum stock policy; both remain example/hypothesis per the Trust Model.
- **UNAVAILABLE:** real external operational data in this repository.

### Review gate and acceptance

This document is `PROPOSED`. Approval should confirm:

1. Adding `quantity_on_hand`/`minimum_stock_quantity` as a second dimension is
   an acceptable way to make Need Conflict concrete in Phase 2.
2. The fixed Urgency thresholds are acceptable as example/hypothesis values.
3. `NeedKind::FreshnessConcern`/`StockAvailabilityConcern` (concern-shaped
   names) are acceptable over Care-verb-shaped alternatives.
4. `state::goods_state` and the frontend remain untouched.

## 日本語

### 背景と課題

Phase 1 は1つの synthetic reference good に bounded な healthy/unhealthy State を与えた。
`crates/goods-domain/src/need/goods_need.rs` と
`crates/goods-application/src/use_cases/identify_need.rs` は次 phase 用の空プレースホルダーとして
残されていた。`docs/phases/phase-2-need.md` は Deviation、Need、Urgency、Need Conflict という語を
挙げていたが、field や境界はまだ定義されていなかった。

単一の評価次元（Freshness/age）だけでは、Freshness の Need が矛盾する相手が存在しないため、実際の
Need Conflict を発生させられない。本提案は StockAvailability という追加の評価次元を導入し、2つの
Need が共存し相反する方向を示唆できるようにする。

### 目標

1. `state::goods_state`（Phase 1 の archived contract）を変更せず、`goods-domain` に `Deviation`、
   `Urgency`、`GoodsNeed`、`NeedConflict` を追加する。
2. 抽象的なモデルにとどまらず実際に Need Conflict が発生するよう、独立した第2の評価次元
   （StockAvailability）を追加する。
3. `goods-application` に `IdentifyNeed` を実装し、既存の `observe_and_assess` を変更せずに
   `GoodsRuntime` から公開する。
4. CLI demo の同梱デフォルト実行では Need が発生しないままにし、Phase 1 の落ち着いた reference
   体験を保つ。Need/Conflict の経路は crafted な observation を使う test で検証する。
5. Need の語彙を Care の動詞ではなく懸念/状態の記述として命名し、Phase 2 が Phase 3 の Care 境界を
   先取りしないようにする。

### 非目標

本提案は Care、CareRequest、CareAction、Memory、Learning、自律 business action、real POS/SEJ
data、API server、database、frontend の変更を実装・許可しない
（`apps/goods-garden-web/` は無変更。AGENTS.md により Need は frontend に対して明示的に対象外）。

### 決定：次元は2つ、1つではない

`Observation` に `quantity_on_hand: u32`、`GoodsProfile` に `minimum_stock_quantity: u32` を追加する。
`Deviation::freshness` は既存の `Expectation` を再利用し（`age_hours - max_age_hours`）、
`Deviation::stock_availability` は新設する（`minimum_stock_quantity - quantity_on_hand`）。
どちらの次元でも magnitude が正なら懸念ありとし、懸念ある Deviation はそれぞれ `NeedKind` を持つ
1つの `GoodsNeed` になる。

```rust
pub enum NeedKind { FreshnessConcern, StockAvailabilityConcern }
```

`NeedConflict::detect` は同じ `NeedAssessment` の中に両方の kind が存在する場合にのみ発火し、
矛盾（棚から下げる vs 棚に置き続ける）を説明する。解決はせず、どちらの行動も推奨しない。

### 決定：Urgency の閾値は固定の example 値

```text
Freshness（max_age_hours超過の時間）:       1-2=Low, 3-5=Medium, 6以上=High
StockAvailability（minimum不足の個数）:     1=Low, 2-3=Medium, 4以上=High
```

これらは Trust Model（`docs/architecture/trust-model.md`）の意味での `example`/`hypothesis` 値であり、
real POS、SEJ、inventory data から導いたものではない。後続の evidence を伴う Work Item が置き換えてよい。

### 決定：NeedKind は懸念を名付け、Care action を名付けない

`FreshnessConcern`/`StockAvailabilityConcern` は、動詞形の代替案（例:
`ReduceForFreshness`/`ReplenishForAvailability`）よりも採用された。Need の名前が推奨対応ではなく
懸念状態を記述するようにし、`AGENTS.md` の Need/Care 境界を明示的に保つため。

### 検討した代替案

**Need Conflict を後続 phase に先送りする**案は、`docs/phases/phase-2-need.md` が Phase 2 の scope として
明示的に Need Conflict を挙げており、単一次元モデルでは意味のある検証ができないため不採用。

**動的に次元を登録できる汎用 `Vec<Deviation>`**案は、過剰な抽象化として不採用。具体的で名前の付いた
2つの次元があれば、投機的な拡張性なしに実際の conflict を実証・検証するには十分である。

**動詞形の `NeedKind` variant** は上記の命名決定により不採用。

### Known、inference、unknown

- **KNOWN：** `GoodsNeed` と `IdentifyNeed` は本 phase 用に予約された空プレースホルダーであり、
  `state::goods_state` は Phase 1 の archived かつ frozen な contract である。
- **KNOWN：** `Observation`/`GoodsProfile` の struct literal 構築箇所は repository 内にちょうど5箇所
  （simulator adapter、CLI、`phase_1_demo.rs` 内の3箇所）のみであり、必須 field の追加は compiler が
  網羅性を保証する、完全に監査可能な変更である。
- **INFERRED：** stock-availability 次元は、コンビニ商品的な good にとって妥当な第2の Need 源であり、
  POS 固有の挙動を創作せずに freshness と availability の現実的な緊張関係を示せる。
- **UNKNOWN：** 実際の店舗が Urgency に使う閾値、実際の minimum stock policy。どちらも Trust Model に
  従い example/hypothesis のままとする。
- **UNAVAILABLE：** この repository 内の real external operational data。

### Review gate と acceptance

この文書は `PROPOSED` である。承認では次を確認する。

1. `quantity_on_hand`/`minimum_stock_quantity` を第2次元として追加し、Phase 2 で Need Conflict を
   具体化する方法が妥当であること。
2. 固定の Urgency 閾値が example/hypothesis 値として妥当であること。
3. `NeedKind::FreshnessConcern`/`StockAvailabilityConcern`（懸念形の命名）が Care 動詞形の代替案より
   妥当であること。
4. `state::goods_state` と frontend が無変更のままであること。

## 中文

### 背景与问题

Phase 1 为一个 synthetic reference good 提供了有边界的 healthy/unhealthy State。
`crates/goods-domain/src/need/goods_need.rs` 与
`crates/goods-application/src/use_cases/identify_need.rs` 被保留为下一阶段的空占位符。
`docs/phases/phase-2-need.md` 提到了 Deviation、Need、Urgency、Need Conflict，但尚未定义其字段与边界。

单一评估维度（Freshness/age）无法产生真正的 Need Conflict：Freshness 的 Need 没有可与之冲突的对象。
本提案新增 StockAvailability 这一评估维度，使两个 Need 能够共存并指向相反的方向。

### 目标

1. 在不修改 `state::goods_state`（Phase 1 已归档的 contract）的前提下，为 `goods-domain` 增加
   `Deviation`、`Urgency`、`GoodsNeed`、`NeedConflict`。
2. 新增独立的第二评估维度（StockAvailability），使 Need Conflict 能够真实发生，而不仅仅是抽象建模。
3. 在 `goods-application` 中实现 `IdentifyNeed`，并在不改动现有 `observe_and_assess` 的前提下从
   `GoodsRuntime` 公开。
4. 保持 CLI demo 默认内置运行不产生 Need，维持 Phase 1 平静的参考体验；通过带有构造 observation 的
   测试来验证 Need/Conflict 路径。
5. 将 Need 词汇命名为关注点/状态描述，而非 Care 动词，使 Phase 2 不会提前占用 Phase 3 的 Care 边界。

### 非目标

本提案不实现或授权 Care、CareRequest、CareAction、Memory、Learning、自主经营行动、真实 POS/SEJ
数据、API server、数据库，也不涉及任何前端改动（`apps/goods-garden-web/` 保持不变；根据 AGENTS.md，
Need 对前端明确不在范围内）。

### 决策：两个维度，而非一个

`Observation` 新增 `quantity_on_hand: u32`；`GoodsProfile` 新增 `minimum_stock_quantity: u32`。
`Deviation::freshness` 复用现有 `Expectation`（`age_hours - max_age_hours`）；
`Deviation::stock_availability` 为新增（`minimum_stock_quantity - quantity_on_hand`）。任一维度上
magnitude 为正即视为令人担忧；每个令人担忧的 Deviation 会成为一个带有 `NeedKind` 的 `GoodsNeed`。

```rust
pub enum NeedKind { FreshnessConcern, StockAvailabilityConcern }
```

`NeedConflict::detect` 仅在同一 `NeedAssessment` 中两种 kind 同时存在时触发，描述其矛盾（下架 vs
保持补货），不解决矛盾，也不推荐任一行动。

### 决策：Urgency 阈值为固定的示例值

```text
Freshness（超出 max_age_hours 的小时数）：   1-2=Low, 3-5=Medium, 6以上=High
StockAvailability（低于 minimum 的数量）：   1=Low, 2-3=Medium, 4以上=High
```

按照 Trust Model（`docs/architecture/trust-model.md`）的定义，这些是 `example`/`hypothesis` 数值，
并非源自真实 POS、SEJ 或库存数据。后续携带证据的 Work Item 可以替换它们。

### 决策：NeedKind 命名关注点，而非 Care 行动

相较于动词形的替代方案（例如 `ReduceForFreshness`/`ReplenishForAvailability`），选择了
`FreshnessConcern`/`StockAvailabilityConcern`，使 Need 的名称描述令人担忧的状态而非推荐的响应，
从而明确保持 `AGENTS.md` 中的 Need/Care 边界。

### 已考虑的替代方案

**将 Need Conflict 推迟到后续阶段**：不采用，因为 `docs/phases/phase-2-need.md` 明确将 Need
Conflict 列为 Phase 2 的范围，且单一维度模型无法对其进行有意义的验证。

**支持动态注册维度的通用 `Vec<Deviation>`**：不采用，属于过度设计；两个具体且命名清晰的维度已足以
在不引入投机性可扩展性的前提下演示并验证真实冲突。

**动词形的 `NeedKind` variant**：根据上述命名决策不采用。

### 已知、推断与未知

- **KNOWN：** `GoodsNeed` 与 `IdentifyNeed` 是为本阶段保留的空占位符；`state::goods_state` 是
  Phase 1 已归档且冻结的 contract。
- **KNOWN：** `Observation`/`GoodsProfile` 的 struct literal 构建位置在仓库中恰好只有5处
  （simulator adapter、CLI，以及 `phase_1_demo.rs` 中的3处）；新增必需字段是由编译器保证覆盖完整、
  完全可审计的变更。
- **INFERRED：** 库存充足度维度对于便利店风格的商品而言是合理的第二 Need 来源，能够在不臆造 POS
  专属行为的前提下展示新鲜度与可得性之间真实的张力。
- **UNKNOWN：** 门店实际会为 Urgency 使用的阈值，以及实际的最低库存策略；两者按照 Trust Model 均
  保持为 example/hypothesis。
- **UNAVAILABLE：** 本仓库中真实的外部运营数据。

### Review gate 与验收

本文档状态为 `PROPOSED`。批准应确认：

1. 新增 `quantity_on_hand`/`minimum_stock_quantity` 作为第二维度、以在 Phase 2 中具体化 Need
   Conflict 的方式是可接受的。
2. 固定的 Urgency 阈值作为 example/hypothesis 数值是可接受的。
3. `NeedKind::FreshnessConcern`/`StockAvailabilityConcern`（关注点形式的命名）优于 Care
   动词形式的替代方案。
4. `state::goods_state` 与前端保持不变。
