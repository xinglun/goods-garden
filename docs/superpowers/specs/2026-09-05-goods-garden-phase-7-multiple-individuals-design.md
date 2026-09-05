# Goods Garden Phase 7 — Multiple Individuals Design Specification

**Status:** PROPOSED — requires human review before implementation

**Date:** 2026-09-05

**Scope:** `apps/goods-garden-cli` only. No `goods-domain`/`goods-application`/`goods-runtime`
changes.

## English

### Context and problem

`docs/phases/phase-7-multiple-individuals.md` asks to "expand from
`TunaMayo@StoreA` to multiple store individuals" and "validate the
separation between Goods Species and Individual Memory." Like Phase 6, this
needs no new domain semantics: `GoodsIdentity.species`/`GoodsProfile` already
represent the shared, reusable Species-level data (per the existing
glossary definitions of Species and Individual), and `GoodsMemory` is
already a value owned by whoever constructs it, not a global store keyed by
species. What Phase 7 asks for is a demonstration that proves this
separation actually holds, not a new mechanism.

### Goals

1. Run two Goods individuals of the same species through the unchanged
   Phase 1-6 model, exposed as a new `multiple-individuals` CLI subcommand.
2. Share one cloned `GoodsProfile` between both individuals to represent
   the Species, while giving each a distinct `GoodsIdentity.individual_id`
   and its own separate `GoodsMemory` value.
3. Make at least one individual's observation raise a Need/Care episode
   (remembered in its own memory) while the other's does not, and print an
   explicit statement confirming neither individual's memory contains the
   other's records.
4. Leave the existing `demo` and `seven-day-life` subcommands' behavior and
   output completely unchanged.

### Non-goals

This proposal does not implement or authorize any new domain type
(specifically, no dedicated "Species" struct/registry), any persistence or
`MemoryStore` implementation, Multiple Goods (Phase 8), or any frontend
change.

### Decision: Species is demonstrated by a shared, cloned GoodsProfile

Rather than inventing a `Species` type or registry, this phase constructs
one `GoodsProfile` value and clones it into each individual's `Goods`. This
is exactly the existing "reusable class of goods profiles" from the
glossary's Species definition — no new abstraction is needed to show
sharing; Rust's `Clone` on an already-`Clone`-derived struct is sufficient.

### Decision: Individual Memory separation is demonstrated by construction, not asserted abstractly

Each individual gets its own `GoodsMemory::new()` inside the loop that
builds it; nothing threads a memory value between individuals. The demo
prints each individual's own record count and a closing sentence stating
they do not cross over. The accompanying test
(`phase_7_multiple_individuals.rs`) asserts on these concrete counts (1 for
Store A, 0 for Store B) rather than on an abstract claim, so the separation
is verified by an actual, checkable outcome.

### Alternatives considered

**Introducing a `Species` domain type that owns a registry of individuals
and profiles:** rejected as premature architecture for a demonstration
phase; `docs/phases/phase-8-multiple-goods.md` (the next phase) is where
"Goods Intelligence is a class of capability" gets its fuller treatment, and
inventing a registry now would pre-empt that phase's own scope decisions.

**A shared, mutable `GoodsMemory` passed to both individuals to show
per-record ownership by identity instead of type-level separation:**
rejected; that would actually demonstrate the opposite of what Phase 7
asks for (Species-level sharing without Individual-level leakage), and
would require inventing per-record identity filtering that does not exist
in Phase 4's minimal `GoodsMemory`.

### Known facts, inferences and unknowns

- **KNOWN:** `GoodsProfile` and `GoodsMemory` already derive/behave in a way
  that makes this demonstration possible without any code change to
  `goods-domain`; this phase only adds CLI-level orchestration.
- **INFERRED:** two individuals are enough to demonstrate the Species vs.
  Individual Memory separation; a third would not add a materially
  different property.
- **UNKNOWN/UNAVAILABLE:** real multi-store operational data; the specific
  ages/quantities in the script are `example`/`synthetic` only.

### Review gate and acceptance

This document is `PROPOSED`. Approval should confirm:

1. No new domain type is introduced; Species sharing is shown via a cloned
   `GoodsProfile`, and Individual Memory separation via distinct
   `GoodsMemory` instances.
2. The existing `demo` and `seven-day-life` subcommands remain unchanged.
3. Two individuals (Store A with a Care episode, Store B without) are an
   acceptable minimal script for this milestone.

## 日本語

### 背景と課題

`docs/phases/phase-7-multiple-individuals.md` は「`TunaMayo@StoreA` から複数 store の
individual へ拡張する」こと、「Goods Species と Individual Memory の分離を検証する」ことを求めている。
Phase 6 と同様、新しい domain semantics は不要である：`GoodsIdentity.species`/`GoodsProfile` は
既に既存の glossary の Species/Individual 定義が指す、共有される再利用可能な Species レベルの
データを表しており、`GoodsMemory` は既に species でキー付けされたグローバルな store ではなく、
それを構築した者が所有する値である。Phase 7 が求めているのは、この分離が実際に成立していることを
証明するデモであり、新しい機構ではない。

### 目標

1. 不変の Phase 1-6 model を、同一 species の2つの Goods individual に対して実行し、新しい
   `multiple-individuals` CLI subcommand として公開する。
2. 両 individual で1つの clone された `GoodsProfile` を共有して Species を表現しつつ、それぞれに
   異なる `GoodsIdentity.individual_id` と独立した `GoodsMemory` 値を与える。
3. 少なくとも1つの individual の observation が Need/Care episode を発生させ（自身の memory に
   記憶される）、もう一方は発生させないようにし、どちらの individual の memory にも相手の記録が
   含まれないことを確認する明示的な statement を表示する。
4. 既存の `demo` と `seven-day-life` subcommand の挙動と出力は完全に変更しない。

### 非目標

本提案は新しい domain type（特に専用の「Species」struct/registry）、persistence や `MemoryStore`
の実装、Multiple Goods（Phase 8）、frontend の変更のいずれも実装・許可しない。

### 決定：Species は共有・clone された GoodsProfile によって示される

`Species` type や registry を発明する代わりに、本 phase は1つの `GoodsProfile` 値を構築し、それを
各 individual の `Goods` に clone する。これはまさに glossary の Species 定義にある既存の
「再利用可能な goods profile の class」であり、共有を示すのに新しい抽象化は不要である：既に
`Clone` を derive している struct に対する Rust の `Clone` で十分である。

### 決定：Individual Memory の分離は抽象的な主張ではなく構築によって示す

各 individual は、それを構築する loop の中で自身の `GoodsMemory::new()` を得る。individual 間で
memory 値を受け渡すことは一切ない。demo は各 individual 自身の record 件数と、それらが交差しない
ことを述べる締めの一文を表示する。付随する test（`phase_7_multiple_individuals.rs`）は、抽象的な
主張ではなく具体的な件数（Store A で1件、Store B で0件）を assert するため、分離は実際に検証可能な
結果によって確認される。

### 検討した代替案

**individual と profile の registry を所有する `Species` domain type を導入する**：demonstration
phase としては時期尚早な architecture として不採用。「Goods Intelligence が capability class
である」というより本格的な扱いは次の phase の `docs/phases/phase-8-multiple-goods.md` の役割であり、
今 registry を発明することはその phase 自身の scope 決定を先取りしてしまう。

**両 individual に渡す共有の mutable `GoodsMemory` を使い、type レベルの分離ではなく identity ごとの
record 所有を示す**：不採用。これは Phase 7 が求めていること（Individual レベルの漏洩なしの
Species レベルの共有）の逆を示すことになり、Phase 4 の最小限の `GoodsMemory` には存在しない
record ごとの identity filtering を発明する必要が生じる。

### Known、inference、unknown

- **KNOWN：** `GoodsProfile` と `GoodsMemory` は既に、`goods-domain` へのコード変更なしにこの
  demonstration を可能にする振る舞いを持つ。本 phase は CLI level の orchestration だけを追加する。
- **INFERRED：** 2つの individual で Species vs. Individual Memory の分離を示すのに十分であり、
  3つ目を追加しても本質的に異なる性質は加わらない。
- **UNKNOWN/UNAVAILABLE：** real な複数店舗の運営データ；script 内の具体的な age/quantity は
  `example`/`synthetic` のみである。

### Review gate と acceptance

この文書は `PROPOSED` である。承認では次を確認する。

1. 新しい domain type を導入せず、Species の共有は clone された `GoodsProfile` によって、
   Individual Memory の分離は独立した `GoodsMemory` instance によって示されること。
2. 既存の `demo` と `seven-day-life` subcommand が変更されないままであること。
3. 2つの individual（Care episode のある Store A と無い Store B）が、本 milestone の妥当な
   最小 script であること。

## 中文

### 背景与问题

`docs/phases/phase-7-multiple-individuals.md` 要求“从 `TunaMayo@StoreA` 扩展到多个门店个体”，
并“验证 Goods Species 与 Individual Memory 的分离”。与 Phase 6 相同，这不需要任何新的领域语义：
`GoodsIdentity.species`/`GoodsProfile` 已经代表了既有 glossary 中 Species/Individual 定义所指的、
被共享的可复用 Species 级数据，而 `GoodsMemory` 已经是由构造它的一方所拥有的值，而非按 species
键控的全局存储。Phase 7 所要求的是证明这种分离确实成立的演示，而非新的机制。

### 目标

1. 将不变的 Phase 1-6 模型运行于同一 species 的两个 Goods individual，作为新的
   `multiple-individuals` CLI 子命令公开。
2. 让两个 individual 共享一个克隆的 `GoodsProfile` 以代表 Species，同时各自拥有不同的
   `GoodsIdentity.individual_id` 与独立的 `GoodsMemory` 值。
3. 使至少一个 individual 的 observation 引发 Need/Care 事件（记录在其自身的 memory 中），而另一个
   不引发，并打印明确的陈述确认任一方的 memory 都不包含对方的记录。
4. 保持既有 `demo` 与 `seven-day-life` 子命令的行为与输出完全不变。

### 非目标

本提案不实现或授权任何新的领域类型（尤其是专门的“Species” struct/registry）、任何持久化或
`MemoryStore` 实现、Multiple Goods（Phase 8），也不涉及任何前端改动。

### 决策：Species 通过共享的、克隆的 GoodsProfile 来演示

本阶段不发明 `Species` 类型或注册表，而是构造一个 `GoodsProfile` 值，并将其克隆到每个 individual
的 `Goods` 中。这正是 glossary 中 Species 定义所说的既有“可复用的商品 profile 类别”——展示共享
不需要新的抽象；对一个已经派生 `Clone` 的 struct 使用 Rust 的 `Clone` 就足够了。

### 决策：Individual Memory 的分离通过构造来演示，而非抽象断言

每个 individual 都在构建它的循环内部获得自己的 `GoodsMemory::new()`；不会在 individual 之间传递
任何 memory 值。演示会打印每个 individual 自身的记录数，并以一句总结说明它们不会交叉。配套的测试
（`phase_7_multiple_individuals.rs`）断言的是具体的数字（Store A 为1，Store B 为0），而非抽象
主张，因此分离性是通过实际可验证的结果来确认的。

### 已考虑的替代方案

**引入拥有 individual 与 profile 注册表的 `Species` 领域类型**：作为演示阶段过早的架构设计不采用；
“Goods Intelligence 是能力类”这一更完整的处理属于下一阶段
`docs/phases/phase-8-multiple-goods.md` 的职责，现在发明注册表会提前占用该阶段自身的范围决策。

**使用传递给两个 individual 的共享可变 `GoodsMemory`，通过按身份归属记录而非类型层面分离来演示**：
不采用；这实际上会演示 Phase 7 所要求内容的反面（Species 级共享且不发生 Individual 级泄漏），
并且需要发明 Phase 4 极简 `GoodsMemory` 中不存在的按记录身份过滤机制。

### 已知、推断与未知

- **KNOWN：** `GoodsProfile` 与 `GoodsMemory` 已经具备使这一演示无需对 `goods-domain` 做任何代码
  修改即可实现的行为；本阶段只新增 CLI 层面的编排。
- **INFERRED：** 两个 individual 足以演示 Species 与 Individual Memory 的分离；第三个不会带来
  本质上不同的性质。
- **UNKNOWN/UNAVAILABLE：** 真实的多门店运营数据；脚本中的具体 age/quantity 仅为
  `example`/`synthetic`。

### Review gate 与验收

本文档状态为 `PROPOSED`。批准应确认：

1. 不引入任何新的领域类型；Species 的共享通过克隆的 `GoodsProfile` 展示，Individual Memory 的
   分离通过独立的 `GoodsMemory` 实例展示。
2. 既有的 `demo` 与 `seven-day-life` 子命令保持不变。
3. 两个 individual（有 Care 事件的 Store A 与没有的 Store B）是该里程碑可接受的最小脚本。
