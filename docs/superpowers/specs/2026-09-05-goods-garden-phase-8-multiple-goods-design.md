# Goods Garden Phase 8 — Multiple Goods Design Specification

**Status:** PROPOSED — requires human review before implementation

**Date:** 2026-09-05

**Scope:** `apps/goods-garden-cli` only. No `goods-domain`/`goods-application`/`goods-runtime`
changes.

## English

### Context and problem

`docs/phases/phase-8-multiple-goods.md` asks to "add salmon rice ball,
coffee, sandwich and bento only to prove that Goods Intelligence is a class
of capability while a concrete good is an object or instance," explicitly
warning against adding products "merely for quantity." Phase 1 already
proved this narrowly with a second `coffee` profile in
`the_same_runtime_supports_a_different_goods_profile`; Phase 8 broadens that
proof across four named products with a genuinely different spread of
profile values, through the unchanged Phase 1-7 model, needing no new
domain semantics.

### Goals

1. Run four named species — salmon rice ball, coffee, sandwich, bento —
   through the exact same `GoodsRuntime::request_care_and_remember` call,
   exposed as a new `multiple-goods` CLI subcommand.
2. Choose `GoodsProfile` values (expected lifetime, minimum stock) that
   genuinely differ across the four, rather than four near-identical
   copies, and confirm by inspection that no product-specific branch exists
   anywhere in `goods-domain`/`goods-application`/`goods-runtime`.
3. Have at least one product (coffee, the shortest-lived) raise a Need/Care
   episode while the others stay healthy, and print an explicit statement
   confirming the uniform code path.
4. Leave the existing `demo`, `seven-day-life` and `multiple-individuals`
   subcommands' behavior and output completely unchanged.

### Non-goals

This proposal does not implement or authorize any new domain type, any
product-specific branching, persistence, autonomous action, or any frontend
change.

### Decision: four species chosen for spread, not padding

The chosen values — coffee (4h/3 min stock), sandwich (6h/2), salmon rice
ball (8h/2), bento (10h/1) — span roughly a 2.5x range in expected lifetime
and a 3x range in minimum stock quantity. Coffee is deliberately observed
one hour past its expectation so exactly one product demonstrates the full
Need/Care path; the other three demonstrate the healthy path with
different, still-meaningful profile numbers. This directly answers the
phase document's warning against adding products "merely for quantity."

### Decision: verify genericity by absence, not by a new abstraction

Rather than adding a "capability registry" or similar mechanism to prove
genericity, this phase relies on a simple, checkable fact already true of
the codebase: grepping `goods-domain`, `goods-application` and
`goods-runtime` for any conditional on a species/product string returns
nothing. The demo's closing statement names this fact for the reader; the
design does not need to invent a new type to prove it.

### Alternatives considered

**Adding many more products "for coverage":** rejected; the phase document
explicitly warns against this, and four products already span a meaningful
range of profile values.

**Introducing a `ProductCatalog`/`Species` registry type:** rejected as
premature architecture for a demonstration phase, matching Phase 7's
reasoning; `apps/goods-garden-cli/src/main.rs`'s local `ProductSpecies`
struct is a CLI-only data table, not a new domain concept.

### Known facts, inferences and unknowns

- **KNOWN:** `goods-domain`, `goods-application` and `goods-runtime` contain
  no species/product-name conditional (confirmed by grep before writing this
  spec); Phase 8 adds none.
- **INFERRED:** four products with a genuine spread of profile values are
  enough to demonstrate the capability-vs-instance distinction without
  padding.
- **UNKNOWN/UNAVAILABLE:** real per-product shelf-life and stock data; the
  script's specific numbers are `example`/`synthetic` only.

### Review gate and acceptance

This document is `PROPOSED`. Approval should confirm:

1. No new domain type or product-specific branch is introduced.
2. The four chosen products and their profile values are an acceptable,
   non-padded minimal script for this milestone.
3. The existing `demo`, `seven-day-life` and `multiple-individuals`
   subcommands remain unchanged.

## 日本語

### 背景と課題

`docs/phases/phase-8-multiple-goods.md` は「salmon rice ball、coffee、sandwich、bento を
追加する目的は数を増やすことではなく、Goods Intelligence が class of capability、concrete good が
object/instance であることを証明すること」を求め、商品を「数合わせのためだけに」追加することを
明示的に戒めている。Phase 1 は `the_same_runtime_supports_a_different_goods_profile` で2つ目の
`coffee` profile を使い、既にこれを狭い範囲で証明していた。Phase 8 はこの証明を、本当に異なる幅を
持つ profile 値を持つ4つの named product に広げるものであり、不変の Phase 1-7 model を使い、
新しい domain semantics を必要としない。

### 目標

1. salmon rice ball、coffee、sandwich、bento という4つの named species を、全く同じ
   `GoodsRuntime::request_care_and_remember` call で実行し、新しい `multiple-goods` CLI
   subcommand として公開する。
2. 4つの商品でほぼ同一の複製ではなく本当に異なる `GoodsProfile` 値（expected lifetime、
   minimum stock）を選び、`goods-domain`/`goods-application`/`goods-runtime` のどこにも商品固有の
   分岐が存在しないことを目視で確認する。
3. 少なくとも1つの商品（最も寿命の短い coffee）に Need/Care episode を発生させ、他は healthy の
   ままにし、統一された code path であることを確認する明示的な statement を表示する。
4. 既存の `demo`、`seven-day-life`、`multiple-individuals` subcommand の挙動と出力は完全に
   変更しない。

### 非目標

本提案は新しい domain type、商品固有の分岐、persistence、自律 action、frontend の変更のいずれも
実装・許可しない。

### 決定：4つの species は幅を持たせるために選び、水増しではない

選んだ値——coffee（4h/最低在庫3）、sandwich（6h/2）、salmon rice ball（8h/2）、bento（10h/1）——は
expected lifetime でおよそ2.5倍、minimum stock quantity でおよそ3倍の幅にまたがる。coffee は
意図的に expectation を1時間超えて observe され、正確に1つの商品だけが完全な Need/Care path を
実演する。残る3つは、それぞれ異なる意味のある profile 値で healthy path を実演する。これは
phase document の「数合わせのためだけに商品を追加しない」という警告に直接応えるものである。

### 決定：汎用性は新しい抽象化ではなく不在によって検証する

汎用性を証明するために「capability registry」のような機構を追加する代わりに、本 phase は既に
codebase に成り立っているシンプルで確認可能な事実に依拠する：`goods-domain`、
`goods-application`、`goods-runtime` を species/product 文字列に対する条件分岐で grep しても
何もヒットしない。demo の締めの statement はこの事実を読者に名指しするだけであり、design はそれを
証明するために新しい type を発明する必要はない。

### 検討した代替案

**「網羅性のために」さらに多くの商品を追加する**：不採用。phase document が明示的にこれを戒めており、
4つの商品で既に意味のある profile 値の幅をカバーしている。

**`ProductCatalog`/`Species` registry type を導入する**：demonstration phase としては時期尚早な
architecture として不採用。Phase 7 と同じ理由による；
`apps/goods-garden-cli/src/main.rs` のローカルな `ProductSpecies` struct は CLI のみの data
table であり、新しい domain concept ではない。

### Known、inference、unknown

- **KNOWN：** `goods-domain`、`goods-application`、`goods-runtime` には species/product 名に
  対する条件分岐が存在しない（本 spec 執筆前に grep で確認済み）。Phase 8 もこれを追加しない。
- **INFERRED：** 本当に幅のある profile 値を持つ4つの商品で、水増しなしに capability vs.
  instance の区別を示すのに十分である。
- **UNKNOWN/UNAVAILABLE：** real な商品ごとの保存期限と在庫データ；script の具体的な数値は
  `example`/`synthetic` のみである。

### Review gate と acceptance

この文書は `PROPOSED` である。承認では次を確認する。

1. 新しい domain type や商品固有の分岐を導入しないこと。
2. 選んだ4つの商品とその profile 値が、水増しの無い本 milestone の妥当な最小 script であること。
3. 既存の `demo`、`seven-day-life`、`multiple-individuals` subcommand が変更されないままであること。

## 中文

### 背景与问题

`docs/phases/phase-8-multiple-goods.md` 要求“加入 salmon rice ball、coffee、sandwich 和
bento 的目的不是堆商品，而是证明 Goods Intelligence 是能力 Class，具体商品是 Object 或
Instance”，并明确告诫不要“仅仅为了数量”而添加商品。Phase 1 已经通过
`the_same_runtime_supports_a_different_goods_profile` 中的第二个 `coffee` profile 在较窄的
范围内证明了这一点；Phase 8 将这一证明扩展到四个具名商品，它们的 profile 数值真正跨越了不同的
范围，并使用不变的 Phase 1-7 模型，无需任何新的领域语义。

### 目标

1. 将 salmon rice ball、coffee、sandwich、bento 这四个具名 species，通过完全相同的
   `GoodsRuntime::request_care_and_remember` 调用运行，作为新的 `multiple-goods` CLI
   子命令公开。
2. 为四个商品选择真正不同（而非近乎相同复制）的 `GoodsProfile` 数值（expected lifetime、
   minimum stock），并通过检视确认 `goods-domain`/`goods-application`/`goods-runtime` 中不存在
   任何商品专属分支。
3. 使至少一个商品（寿命最短的 coffee）引发 Need/Care 事件，其余保持健康，并打印明确的陈述确认
   统一的代码路径。
4. 保持既有 `demo`、`seven-day-life`、`multiple-individuals` 子命令的行为与输出完全不变。

### 非目标

本提案不实现或授权任何新的领域类型、任何商品专属分支、持久化、自主行动，也不涉及任何前端改动。

### 决策：四种 species 的选择是为了跨度，而非凑数

所选数值——coffee（4小时/最低库存3）、sandwich（6小时/2）、salmon rice ball（8小时/2）、
bento（10小时/1）——在 expected lifetime 上跨越约2.5倍，在 minimum stock quantity 上跨越约3倍。
coffee 被刻意观测为超出其 expectation 一小时，因此恰好只有一个商品演示完整的 Need/Care 路径；
其余三个商品以不同但同样有意义的 profile 数值演示健康路径。这直接回应了阶段文档中“不要仅仅为了
数量而添加商品”的告诫。

### 决策：通用性通过“不存在”而非新的抽象来验证

本阶段不添加“能力注册表”之类的机制来证明通用性，而是依赖代码库中已经成立、可检查的简单事实：
对 `goods-domain`、`goods-application`、`goods-runtime` 按 species/product 字符串搜索条件分支，
结果为空。演示结尾的陈述只是向读者指出这一事实；设计无需为此发明新的类型。

### 已考虑的替代方案

**“为了覆盖面”添加更多商品**：不采用；阶段文档明确告诫不要这样做，而四个商品已经覆盖了有意义的
profile 数值范围。

**引入 `ProductCatalog`/`Species` 注册表类型**：作为演示阶段过早的架构设计不采用，理由与
Phase 7 相同；`apps/goods-garden-cli/src/main.rs` 中本地的 `ProductSpecies` struct 只是
CLI 专属的数据表，而非新的领域概念。

### 已知、推断与未知

- **KNOWN：** `goods-domain`、`goods-application`、`goods-runtime` 中不存在针对 species/product
  名称的条件分支（在撰写本 spec 前已通过 grep 确认）。Phase 8 也不会添加。
- **INFERRED：** 四个具有真正跨度的 profile 数值的商品，足以在不凑数的情况下展示能力与实例的区别。
- **UNKNOWN/UNAVAILABLE：** 真实的各商品保质期与库存数据；脚本中的具体数值仅为
  `example`/`synthetic`。

### Review gate 与验收

本文档状态为 `PROPOSED`。批准应确认：

1. 不引入任何新的领域类型或商品专属分支。
2. 所选四种商品及其 profile 数值是该里程碑可接受的、未凑数的最小脚本。
3. 既有的 `demo`、`seven-day-life`、`multiple-individuals` 子命令保持不变。
