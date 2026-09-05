# Goods Garden Phase 6 — Seven Day Life Design Specification

**Status:** PROPOSED — requires human review before implementation

**Date:** 2026-09-05

**Scope:** `apps/goods-garden-cli` only. No `goods-domain`/`goods-application`/`goods-runtime`
changes.

## English

### Context and problem

Phases 2-5 built the full `State → Need → Care → Memory → Verification →
Learning` model, each exercised only through single-observation demos and
tests. `docs/phases/phase-6-seven-day-life.md` names a product milestone —
"one rice ball lives for seven days" — that must show normal, anomaly,
investigation, request for care, human feedback, improvement, verification
and memory in one coherent narrative. Unlike Phases 2-5, this milestone
needs no new domain semantics; it is purely a CLI-level orchestration of
existing, already-reviewed rules.

### Goals

1. Script seven synthetic days for one Goods individual using only existing
   `goods-domain`/`goods-application`/`goods-runtime` types, exposed as a new
   `seven-day-life` CLI subcommand.
2. Include at least two anomaly days, each raising a Need, a Care Request
   answered by synthetic Human Feedback, and a following verification day
   showing the Care Action resolved the Need.
3. Leave the existing `demo` subcommand's behavior and output completely
   unchanged.
4. Keep every value clearly synthetic and explainable, matching Phase 1's
   established labelling convention.

### Non-goals

This proposal does not implement or authorize any new domain type, any
automatic rule/threshold adjustment, Multiple Individuals (Phase 7),
Multiple Goods (Phase 8), or any frontend change.

### Decision: a "day" is a restocked observation of the same monitored slot

A literal single rice ball cannot safely age for a full week under Phase 1's
own freshness rule. The good's `GoodsIdentity` instead represents a
monitored retail slot: each day's `age_hours`/`quantity_on_hand` reflects
what a store clerk would observe that day, and a restock (recorded as a
Human-Feedback-driven Care Action) resets `age_hours` on a later day. This
is the same "Individual" concept Phase 1 already defined and does not
require any new type.

### Decision: two independent anomalies, one per Need dimension

Day 3 exercises the Freshness dimension (age exceeds expectation); Day 6
exercises the StockAvailability dimension (quantity falls below the
minimum). Using both of Phase 2's dimensions once each demonstrates the
full breadth of the existing Need model without inventing a third one, and
keeps each anomaly's cause-and-resolution easy to follow independently.

### Decision: `seven-day-life` is a separate subcommand, not a change to `demo`

`apps/goods-garden-cli/tests/phase_1_demo.rs` asserts on the existing
`demo` subcommand's output. Rather than touch that frozen contract, this
phase adds a new, additive subcommand and its own test file.

### Alternatives considered

**Driving the seven days from an external multi-day fixture file:**
rejected; it would require inventing a new fixture text format and parser
solely for a one-off milestone script, whereas Phase 2/3's tests already
establish the precedent of constructing `Observation`/`HumanFeedback`
values directly in Rust for scripted scenarios.

**Repurposing the `demo` subcommand to show the seven-day story instead of
the single-day one:** rejected; it would break the Phase 1 frozen test
contract for no necessary reason.

### Known facts, inferences and unknowns

- **KNOWN:** every Need/Care/Outcome/Learning value produced during the
  seven days comes from the unchanged Phase 2-5 rules; this phase adds no
  new comparison or decision logic.
- **INFERRED:** two anomalies (one per existing Need dimension) are enough
  to satisfy the phase document's checklist without adding scope.
- **UNKNOWN/UNAVAILABLE:** real day-over-day store data; the script's
  specific numbers are `example`/`synthetic` only.

### Review gate and acceptance

This document is `PROPOSED`. Approval should confirm:

1. No new domain type is introduced; the seven-day script only supplies
   input to Phase 2-5's existing rules.
2. The existing `demo` subcommand remains unchanged.
3. The two anomalies (Freshness, StockAvailability) and their verification
   days are an acceptable minimal script for this milestone.

## 日本語

### 背景と課題

Phase 2-5 は `State → Need → Care → Memory → Verification → Learning` の
model を一通り構築したが、いずれも単一 observation の demo と test でしか検証されていなかった。
`docs/phases/phase-6-seven-day-life.md` は「一つのおにぎりが7日間 live する」という product
milestone を挙げ、normal、anomaly、investigation、request for care、human feedback、
improvement、verification、memory を1つの一貫した narrative で示すことを求めている。Phase 2-5 と
異なり、この milestone は新しい domain semantics を必要としない：既存の、既に review された rule を
CLI level で orchestration するだけである。

### 目標

1. 既存の `goods-domain`/`goods-application`/`goods-runtime` の型だけを使い、1つの Goods
   individual に対して7日分の synthetic day をスクリプト化し、新しい `seven-day-life` CLI
   subcommand として公開する。
2. 少なくとも2つの anomaly day を含め、それぞれが Need、synthetic Human Feedback で応答される
   Care Request、続く verification day（Care Action が Need を解決したことを示す）を発生させる。
3. 既存の `demo` subcommand の挙動と出力は完全に変更しない。
4. Phase 1 が確立した labelling 規約に合わせ、すべての値を明確に synthetic かつ explainable に保つ。

### 非目標

本提案は新しい domain type、自動的な rule/threshold 調整、Multiple Individuals（Phase 7）、
Multiple Goods（Phase 8）、frontend の変更のいずれも実装・許可しない。

### 決定：「1日」とは同じ監視対象スロットの restock 後 observation である

文字通り単一のおにぎりは、Phase 1 自身の freshness rule の下では1週間安全に age できない。代わりに
good の `GoodsIdentity` は監視対象の小売スロットを表す：各日の `age_hours`/`quantity_on_hand` は
その日に店員が observe するであろう値を反映し、restock（Human-Feedback 駆動の Care Action として
記録される）が後の日で `age_hours` をリセットする。これは Phase 1 が既に定義した「Individual」概念と
同じであり、新しい type を必要としない。

### 決定：独立した2つの anomaly、Need 次元ごとに1つ

Day 3 は Freshness 次元（age が expectation を超過）、Day 6 は StockAvailability 次元
（quantity が minimum を下回る）を扱う。Phase 2 の両次元をそれぞれ1回ずつ使うことで、3つ目の次元を
発明することなく既存 Need model の全幅を示し、各 anomaly の原因と解決を独立して追いやすく保つ。

### 決定：`seven-day-life` は `demo` への変更ではなく独立した subcommand とする

`apps/goods-garden-cli/tests/phase_1_demo.rs` は既存の `demo` subcommand の出力を assert している。
この frozen contract に触れる代わりに、本 phase は新しい additive な subcommand と、それ専用の test
file を追加する。

### 検討した代替案

**7日分を外部の複数日 fixture file から駆動する**：不採用。一度限りの milestone script のためだけに
新しい fixture text format と parser を発明することになる一方、Phase 2/3 の test は既に、
スクリプト化された scenario のために `Observation`/`HumanFeedback` 値を Rust 内で直接構築するという
前例を確立している。

**`demo` subcommand を単一日の story ではなく7日間の story を示すものに作り替える**：不採用。
必要のない理由で Phase 1 の frozen test contract を壊すことになる。

### Known、inference、unknown

- **KNOWN：** 7日間で生成される全ての Need/Care/Outcome/Learning 値は不変の Phase 2-5 rule に
  由来する。本 phase は新しい比較や決定 logic を一切追加しない。
- **INFERRED：** 2つの anomaly（既存 Need 次元ごとに1つ）は、scope を広げることなく phase document
  の checklist を満たすのに十分である。
- **UNKNOWN/UNAVAILABLE：** real な日々の店舗データ；script の具体的な数値は `example`/`synthetic`
  のみである。

### Review gate と acceptance

この文書は `PROPOSED` である。承認では次を確認する。

1. 新しい domain type を導入せず、7日間の script は Phase 2-5 の既存 rule への input を供給する
   だけであること。
2. 既存の `demo` subcommand が変更されないままであること。
3. 2つの anomaly（Freshness、StockAvailability）とその verification day が、本 milestone の
   妥当な最小 script であること。

## 中文

### 背景与问题

Phase 2-5 构建了完整的 `State → Need → Care → Memory → Verification → Learning` 模型，但都仅通过
单次观测的 demo 与测试进行验证。`docs/phases/phase-6-seven-day-life.md` 提出了一个产品里程碑——
“一个饭团连续活七天”——要求在一个连贯的叙事中展现正常、异常、调查、求助、人工反馈、改善、验证和记忆。
与 Phase 2-5 不同，这个里程碑不需要任何新的领域语义：它纯粹是对既有、已经评审过的规则进行 CLI 层面的编排。

### 目标

1. 仅使用既有的 `goods-domain`/`goods-application`/`goods-runtime` 类型，为一个 Goods
   individual 编排七天的 synthetic day 脚本，作为新的 `seven-day-life` CLI 子命令公开。
2. 至少包含两个异常日，每个都引发一个 Need、由 synthetic Human Feedback 回应的 Care Request，
   以及随后的验证日，显示 Care Action 解决了该 Need。
3. 保持既有 `demo` 子命令的行为与输出完全不变。
4. 遵循 Phase 1 已确立的标注惯例，使每个数值都明确标记为 synthetic 且可解释。

### 非目标

本提案不实现或授权任何新的领域类型、任何自动的规则/阈值调整、Multiple Individuals（Phase 7）、
Multiple Goods（Phase 8），也不涉及任何前端改动。

### 决策：“一天”是对同一被监控货位的补货后观测

字面意义上的单个饭团，在 Phase 1 自身的新鲜度规则下无法安全地连续老化一周。相反，商品的
`GoodsIdentity` 代表的是被监控的零售货位：每天的 `age_hours`/`quantity_on_hand` 反映的是店员当天
会观察到的值，而补货（记录为由 Human Feedback 驱动的 Care Action）会在后续某天重置 `age_hours`。
这与 Phase 1 已经定义的“Individual”概念相同，不需要任何新类型。

### 决策：两个独立的异常，每个 Need 维度各一个

第3天演示 Freshness 维度（age 超出 expectation）；第6天演示 StockAvailability 维度（quantity 低于
minimum）。将 Phase 2 的两个维度各使用一次，既展示了既有 Need 模型的完整广度，又不必发明第三个维度，
并使每个异常的起因与解决都易于独立追踪。

### 决策：`seven-day-life` 是独立的子命令，而非对 `demo` 的修改

`apps/goods-garden-cli/tests/phase_1_demo.rs` 对既有 `demo` 子命令的输出进行断言。本阶段不触碰
这一冻结的契约，而是新增一个纯追加式的子命令及其专属测试文件。

### 已考虑的替代方案

**从外部的多日 fixture 文件驱动七天**：不采用；这需要仅为一次性的里程碑脚本发明新的 fixture 文本
格式与解析器，而 Phase 2/3 的测试已经确立了为脚本化场景直接在 Rust 中构造
`Observation`/`HumanFeedback` 值的先例。

**将 `demo` 子命令改造为展示七天故事而非单日故事**：不采用；这会在没有必要的情况下破坏 Phase 1 冻结的
测试契约。

### 已知、推断与未知

- **KNOWN：** 七天中产生的所有 Need/Care/Outcome/Learning 值均来自不变的 Phase 2-5 规则；本阶段
  不新增任何比较或决策逻辑。
- **INFERRED：** 两个异常（既有 Need 维度各一个）足以满足阶段文档的检查清单，且不扩大范围。
- **UNKNOWN/UNAVAILABLE：** 真实的逐日门店数据；脚本中的具体数值仅为 `example`/`synthetic`。

### Review gate 与验收

本文档状态为 `PROPOSED`。批准应确认：

1. 不引入任何新的领域类型；七天脚本只为 Phase 2-5 的既有规则提供输入。
2. 既有 `demo` 子命令保持不变。
3. 两个异常（Freshness、StockAvailability）及其验证日是该里程碑可接受的最小脚本。
