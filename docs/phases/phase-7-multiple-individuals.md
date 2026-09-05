# Phase 7 — Multiple Individuals

## English

### Goal

Expand from `TunaMayo@StoreA` to multiple store individuals. Validate the
separation between Goods Species and Individual Memory.

### Implemented boundary

Phase 7 introduces no new domain type. It runs two Goods individuals of the
same species (`TunaMayo@StoreA` and `TunaMayo@StoreB`) through the unchanged
Phase 1-6 model, exposed as a new `multiple-individuals` CLI subcommand
alongside the unchanged `demo` and `seven-day-life` subcommands.

Both individuals share one cloned `GoodsProfile` value (the Species-level
data: display name, expected lifetime, minimum stock), while each owns a
distinct `GoodsIdentity.individual_id` and, crucially, its own separate
`GoodsMemory` instance. Store A's synthetic observation triggers a
`FreshnessConcern` Need, a Care Request and a Care Action, remembered only in
Store A's memory; Store B's observation stays within expectation and
remembers nothing. The demo's closing summary explicitly states that neither
individual's memory contains the other's records.

This validates, by direct construction rather than by adding a type, that
"Species" (the shared `GoodsProfile`/`species` label) and "Individual
Memory" (a `GoodsMemory` value owned by one `Goods` instance) are already
separate concerns in the Phase 1-6 architecture.

### Local demo and data boundary

Run the demo from the repository root:

```bash
cargo run -p goods-garden-cli -- multiple-individuals
```

Both individuals' observations and Human Feedback are constructed directly
in the CLI (matching Phase 6's precedent) and labelled `synthetic-example`.
The existing `demo` and `seven-day-life` subcommands and their output are
unchanged.

### Exit criteria

- A local user can run `multiple-individuals` and see two individuals of the
  same species produce independent Needs, Care episodes and Memory, with an
  explicit statement confirming their memories do not cross over.
- The existing `demo` and `seven-day-life` subcommands are unchanged.
- No new domain type, autonomous action or rule adjustment is introduced.

## 日本語

### Goal

`TunaMayo@StoreA` から複数 store の individual へ拡張し、Goods Species と Individual Memory の
分離を検証する。

### Implemented boundary

Phase 7 は新しい domain type を導入しない。不変の Phase 1-6 model を、同一 species の2つの Goods
individual（`TunaMayo@StoreA` と `TunaMayo@StoreB`）に対して実行し、既存の `demo` と
`seven-day-life` subcommand はそのままに、新しい `multiple-individuals` CLI subcommand として
公開する。

両 individual は1つの clone された `GoodsProfile` 値（Species レベルのデータ：display name、
expected lifetime、minimum stock）を共有する一方、それぞれ異なる `GoodsIdentity.individual_id` と、
決定的に、独立した `GoodsMemory` instance を持つ。Store A の synthetic observation は
`FreshnessConcern` Need、Care Request、Care Action を発生させ、それは Store A の memory にのみ
記憶される。Store B の observation は expectation 内にとどまり、何も記憶しない。demo の最後の
summary は、どちらの individual の memory にも相手の記録が含まれないことを明示する。

これは、型を追加するのではなく直接の構築によって、「Species」（共有される `GoodsProfile`/`species`
label）と「Individual Memory」（1つの `Goods` instance が所有する `GoodsMemory` 値）が、Phase 1-6
の architecture において既に分離された関心事であることを検証する。

### Local demo と data boundary

repository root から demo を実行する。

```bash
cargo run -p goods-garden-cli -- multiple-individuals
```

両 individual の observation と Human Feedback は（Phase 6 の前例に合わせ）CLI 内で直接構築され、
`synthetic-example` と明記される。既存の `demo` と `seven-day-life` subcommand とその出力は変更しない。

### Exit criteria

- local user が `multiple-individuals` を実行し、同一 species の2つの individual が独立した
  Need、Care episode、Memory を生成すること、そしてそれらの memory が交差しないことを明示する
  statement を確認できる。
- 既存の `demo` と `seven-day-life` subcommand は変更しない。
- 新しい domain type、自律 action、rule adjustment は導入しない。

## 中文

### Goal

从 `TunaMayo@StoreA` 扩展到多个门店个体，验证 Goods Species 与 Individual Memory 的分离。

### Implemented boundary

Phase 7 不引入任何新的领域类型。它将不变的 Phase 1-6 模型运行于同一 species 的两个 Goods
individual（`TunaMayo@StoreA` 与 `TunaMayo@StoreB`），作为新的 `multiple-individuals` CLI
子命令公开，既有的 `demo` 与 `seven-day-life` 子命令保持不变。

两个 individual 共享同一个克隆的 `GoodsProfile` 值（Species 级数据：display name、expected
lifetime、minimum stock），但各自拥有不同的 `GoodsIdentity.individual_id`，并且——至关重要地——各自
拥有独立的 `GoodsMemory` 实例。Store A 的 synthetic observation 触发 `FreshnessConcern` Need、
Care Request 与 Care Action，仅记录在 Store A 的 memory 中；Store B 的 observation 保持在
expectation 之内，不记住任何内容。demo 结尾的汇总明确说明任一方的 memory 都不包含对方的记录。

这通过直接构造而非新增类型，验证了“Species”（共享的 `GoodsProfile`/`species` 标签）与
“Individual Memory”（由单个 `Goods` 实例拥有的 `GoodsMemory` 值）在 Phase 1-6 架构中已经是
分离的关注点。

### 本地 demo 与数据边界

从仓库根目录运行 demo：

```bash
cargo run -p goods-garden-cli -- multiple-individuals
```

两个 individual 的 observation 与 Human Feedback 都直接在 CLI 中构造（与 Phase 6 的先例一致），
并标记为 `synthetic-example`。既有的 `demo` 与 `seven-day-life` 子命令及其输出保持不变。

### Exit criteria

- 本地用户运行 `multiple-individuals`，能看到同一 species 的两个 individual 产生独立的 Need、
  Care 事件与 Memory，并有明确的陈述确认它们的 memory 不会交叉。
- 既有的 `demo` 与 `seven-day-life` 子命令保持不变。
- 不引入任何新的领域类型、自主行动或规则调整。
