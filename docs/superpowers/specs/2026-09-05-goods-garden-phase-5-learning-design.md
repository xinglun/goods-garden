# Goods Garden Phase 5 — Verification & Learning Design Specification

**Status:** PROPOSED — requires human review before implementation

**Date:** 2026-09-05

**Scope:** `goods-domain`, `goods-application`, `goods-runtime` and `apps/goods-garden-cli`
additions for Phase 5 (Outcome, OutcomeStatus, Learning).

## English

### Context and problem

Phase 4 gave the good an append-only Relationship Memory. The
`crates/goods-application/src/use_cases/{verify_outcome.rs,
learn_from_outcome.rs}` files and `crates/goods-domain/src/learning/learning.rs`
were left as empty placeholders. `docs/phases/phase-5-learning.md` names
`Care Action → Outcome → Verification → Learning`, with traceable evidence
and explicit authority.

The repository's own governance (`AGENTS.md`) treats learning policy as a
protected semantic area requiring a human decision. The central risk is that
"Learning" could be read as license for the system to start tuning its own
rules (Urgency thresholds, minimum stock quantities, Care role routing) from
observed outcomes — which would be exactly the kind of autonomous business
action the North Star forbids ("AI does not own business authority"). This
proposal is deliberately conservative: Learning is a reviewable record, never
a self-modifying rule.

### Goals

1. Add `Outcome`/`OutcomeStatus` (new `outcome` module) and implement the
   existing `Learning` placeholder in `goods-domain`, without changing the
   Phase 1-4 State/Need/Care/Memory model.
2. Make the Outcome comparison purely factual: whether the NeedKind(s) a
   CareAction addressed are still present in a follow-up Need Assessment.
   No qualitative scoring of the Caregiver's decision.
3. Make Learning strictly a reviewable statement: it carries the Outcome and
   a plain-language `String`, and has no method or field that mutates any
   threshold, profile value or other rule.
4. Implement `VerifyOutcome`/`LearnFromOutcome` in `goods-application` and
   expose `GoodsRuntime::verify_and_learn`, which takes a prior `CareAction`
   and observes again through the same `ObservationSource` port, without
   altering the four existing runtime methods.
5. Keep the CLI demo's default, bundled run showing nothing pending
   verification, preserving the calm Phase 1 reference experience; exercise
   both the resolved and unresolved paths through tests with two crafted
   observations (initial and follow-up).

### Non-goals

This proposal does not implement or authorize any automatic adjustment of
`Urgency` thresholds, `GoodsProfile` fields, Care role routing or any other
rule; the generic `Evidence` type; persistence; the Phase 6 multi-day
("Seven Day Life") loop; or any frontend change (`apps/goods-garden-web/` is
untouched).

### Decision: Outcome is a factual comparison, not a score

`Outcome::verify(action, new_state, new_needs)` collects the `NeedKind`s the
original `CareRequest` addressed and checks whether any of them still appear
in `new_needs`. `OutcomeStatus::Resolved` means none remain;
`OutcomeStatus::Unresolved` means at least one does. This is derived
entirely from data Phase 2 already computes (`NeedKind` identity), so no new
unevidenced business rule is introduced — it is the same kind of bounded,
explainable comparison Phase 1 used for healthy/unhealthy.

### Decision: Learning never adjusts a rule by itself

`Learning { outcome, statement }` has no other fields and no method that
writes back into `GoodsProfile`, `Urgency` thresholds or any other domain
rule. Its `statement` explicitly says "this is a reviewable observation, not
a rule change." This was confirmed with the repository owner as the correct
scope for Phase 5, given `AGENTS.md`'s learning-policy protection: an
alternative that kept a "provisional adjustment" field for future human
adoption was considered and explicitly rejected as broadening scope and risk
without an immediate need.

### Decision: verification reuses the existing ObservationSource port

`GoodsRuntime::verify_and_learn(goods, action)` observes again through
`self.source` (the same `ObservationSource` used by the other methods) and
calls `observe_and_identify_needs` internally. A "follow-up" observation is
simply a different `ObservationSource` value (e.g. a second
`DemoObservationSource::new(..)` in tests) — no new port is introduced, and
no notion of "time passing" is modeled beyond that.

### Alternatives considered

**Letting Learning propose a specific threshold/parameter change (with a
"provisional, not yet applied" marker):** rejected. Even unapplied, a
concrete proposed number is close enough to an invented rule that it risks
being treated as authoritative later without genuine evidence behind it;
the repository owner confirmed the plain-statement-only scope.

**Aggregating Learning across multiple episodes into a trend/score:**
rejected as premature; Phase 6 ("Seven Day Life") is the first phase that
puts a good through a genuine multi-day loop, and any aggregation here would
be built on too little evidence.

**Introducing the generic `Evidence` type now:** rejected; `Outcome`'s own
fields already provide full traceability (which CareAction, which follow-up
State/Need Assessment) for this phase's needs, so adding an abstract
wrapper would be premature.

### Known facts, inferences and unknowns

- **KNOWN:** `verify_outcome.rs`, `learn_from_outcome.rs` and
  `learning/learning.rs` were empty placeholders reserved for this phase;
  the Phase 1-4 types are unaffected (Outcome only reads `CareAction` and
  `NeedAssessment`, it does not extend them).
- **KNOWN:** the only way to obtain an `Outcome`/`Learning` value is through
  `GoodsRuntime::verify_and_learn`, which requires an already-recorded
  `CareAction`; there is no path that invents one.
- **INFERRED:** comparing `NeedKind` presence before/after is a defensible
  minimal proxy for "did the Need go away," without claiming to know the
  real-world reason it did or didn't.
- **UNKNOWN:** real learning/adaptation policy, whether any future phase
  should let a human explicitly approve a rule change from accumulated
  Learning records, and how Learning should behave across the multi-day
  scenario named in `docs/phases/phase-6-seven-day-life.md`; these remain
  `UNKNOWN`/out of scope until a later, evidence-bearing Work Item addresses
  them.
- **UNAVAILABLE:** real external outcome/effectiveness data in this
  repository.

### Review gate and acceptance

This document is `PROPOSED`. Approval should confirm:

1. Outcome is a factual NeedKind-presence comparison, not a qualitative
   score of the Caregiver's decision.
2. Learning is a plain reviewable statement only, with no field or method
   that adjusts a threshold, profile value or other rule.
3. `state::goods_state`, the Phase 2 Need model, the Phase 3 Care model, the
   Phase 4 Memory model and the frontend remain untouched.

## 日本語

### 背景と課題

Phase 4 は good に append-only な Relationship Memory を与えた。
`crates/goods-application/src/use_cases/{verify_outcome.rs,
learn_from_outcome.rs}` と `crates/goods-domain/src/learning/learning.rs`
は空プレースホルダーのまま残されていた。`docs/phases/phase-5-learning.md` は
`Care Action → Outcome → Verification → Learning` を、traceable evidence と explicit
authority とともに挙げている。

repository 自身の governance（`AGENTS.md`）は learning policy を human decision を要する
protected semantic area として扱う。中心的なリスクは、「Learning」が観測された outcome から
system 自身のルール（Urgency 閾値、minimum stock quantity、Care role routing）を tuning し始める
許可であるかのように読めてしまうことである——これはまさに North Star が禁じる自律 business action
（「AI does not own business authority」）そのものである。本提案は意図的に保守的である：Learning は
reviewable な記録であり、自己書き換えするルールでは決してない。

### 目標

1. `goods-domain` に `Outcome`/`OutcomeStatus`（新設 `outcome` module）を追加し、既存の `Learning`
   placeholder を実装する。Phase 1-4 の State/Need/Care/Memory model は変更しない。
2. Outcome の比較を純粋に事実ベースにする：CareAction が対象とした NeedKind が follow-up の Need
   Assessment にまだ存在するかどうか。Caregiver の決定を定性的に採点しない。
3. Learning を厳密に reviewable な statement に限定する：Outcome と plain-language な `String` を
   持つだけで、閾値、profile 値、その他のルールを変更する method や field を一切持たない。
4. `goods-application` に `VerifyOutcome`/`LearnFromOutcome` を実装し、以前の `CareAction` を受け取り
   同じ `ObservationSource` port から再度 observe する `GoodsRuntime::verify_and_learn` を公開する。
   既存4つの runtime method は変更しない。
5. CLI demo の同梱デフォルト実行では verify 待ちが無いままにし、Phase 1 の落ち着いた reference 体験を
   保つ。resolved と unresolved の両経路は、2つの crafted observation（初回と follow-up）を使う test
   で検証する。

### 非目標

本提案は `Urgency` 閾値、`GoodsProfile` field、Care role routing、その他ルールのいかなる自動調整も、
汎用 `Evidence` type も、persistence も、Phase 6 の複数日（「Seven Day Life」）loop も、frontend の
変更（`apps/goods-garden-web/` は無変更）も実装・許可しない。

### 決定：Outcome は事実比較であり、採点ではない

`Outcome::verify(action, new_state, new_needs)` は、元の `CareRequest` が対象とした `NeedKind` を
集め、それらのいずれかが `new_needs` にまだ現れるかを確認する。`OutcomeStatus::Resolved` は1つも
残っていないこと、`OutcomeStatus::Unresolved` は少なくとも1つ残っていることを意味する。これは Phase 2
が既に計算しているデータ（`NeedKind` の identity）から完全に導出されるため、新しい根拠のない business
rule は導入しない——Phase 1 が healthy/unhealthy に使ったのと同種の、bounded で explainable な比較である。

### 決定：Learning は自らルールを調整しない

`Learning { outcome, statement }` は他の field を持たず、`GoodsProfile`、`Urgency` 閾値、その他の
domain rule に書き戻す method も持たない。その `statement` は明示的に「これは reviewable な観測であり、
ルール変更ではない」と述べる。これは `AGENTS.md` の learning-policy 保護を踏まえ、Phase 5 の正しい
scope として repository owner と確認済みである：将来の human adoption 向けに「provisional adjustment」
field を残す代替案も検討したが、直近の必要性なしに scope とリスクを広げるものとして明示的に不採用とした。

### 決定：verification は既存の ObservationSource port を再利用する

`GoodsRuntime::verify_and_learn(goods, action)` は `self.source`（他の method と同じ
`ObservationSource`）を通じて再度 observe し、内部で `observe_and_identify_needs` を呼ぶ。
「follow-up」observation とは単に別の `ObservationSource` 値（例えば test での2つ目の
`DemoObservationSource::new(..)`）であり、新しい port は導入せず、「時間の経過」という概念もそれ以上には
モデル化しない。

### 検討した代替案

**Learning に具体的な閾値/パラメータ変更を提案させる**（「provisional、未適用」マーカー付き）：不採用。
未適用であっても、具体的な提案値は根拠のない rule の発明に近く、後で本当の evidence なしに authoritative
なものとして扱われるリスクがある。repository owner は plain-statement のみの scope を確認済みである。

**複数 episode をまたいで Learning を trend/score に集約する**：時期尚早として不採用。Phase 6
（「Seven Day Life」）が good を本当の複数日 loop に通す最初の phase であり、ここでの集約は
evidence が少なすぎる土台の上に構築されることになる。

**今 汎用の `Evidence` type を導入する**：不採用。`Outcome` 自身の field（どの CareAction、どの
follow-up State/Need Assessment）が本 phase の必要とする traceability を既に完全に提供しており、
抽象的な wrapper の追加は時期尚早である。

### Known、inference、unknown

- **KNOWN：** `verify_outcome.rs`、`learn_from_outcome.rs`、`learning/learning.rs` は本 phase 用に
  予約された空プレースホルダーであり、Phase 1-4 の type は本提案の影響を受けない（Outcome は
  `CareAction` と `NeedAssessment` を読むだけで、それらを拡張しない）。
- **KNOWN：** `Outcome`/`Learning` 値を得る唯一の方法は `GoodsRuntime::verify_and_learn` 経由であり、
  既に記録された `CareAction` を要求する。それを発明する経路は存在しない。
- **INFERRED：** 前後の `NeedKind` の有無を比較することは、「Need が無くなったか」の defensible な
  最小 proxy であり、実世界でそれが起きた/起きなかった本当の理由を知っていると主張しない。
- **UNKNOWN：** real な learning/adaptation policy、蓄積された Learning record から human が明示的に
  rule change を承認する仕組みを将来 phase で持たせるべきか、`docs/phases/phase-6-seven-day-life.md`
  が挙げる複数日 scenario で Learning がどう振る舞うべきか。これらは、後続の evidence を伴う Work Item
  が扱うまで `UNKNOWN`/対象外のままとする。
- **UNAVAILABLE：** この repository 内の real external outcome/effectiveness data。

### Review gate と acceptance

この文書は `PROPOSED` である。承認では次を確認する。

1. Outcome が Caregiver の決定を定性的に採点するものではなく、NeedKind の有無を比較する事実判定である
   こと。
2. Learning が単なる reviewable な statement であり、閾値、profile 値、その他ルールを調整する field
   や method を持たないこと。
3. `state::goods_state`、Phase 2 Need model、Phase 3 Care model、Phase 4 Memory model、frontend が
   無変更のままであること。

## 中文

### 背景与问题

Phase 4 为商品提供了仅追加的 Relationship Memory。
`crates/goods-application/src/use_cases/{verify_outcome.rs,
learn_from_outcome.rs}` 与 `crates/goods-domain/src/learning/learning.rs`
被保留为空占位符。`docs/phases/phase-5-learning.md` 列出了
`Care Action → Outcome → Verification → Learning`，并要求可追溯证据和明确权限。

仓库自身的治理（`AGENTS.md`）将学习策略视为需要人类决策的受保护语义区域。核心风险在于“Learning”可能被
解读为允许系统根据观察到的结果开始调整自身规则（Urgency 阈值、最低库存数量、Care 角色路由）——这正是
North Star 所禁止的自主经营行动（“AI does not own business authority”）。本提案刻意保持保守：Learning
是可 review 的记录，绝不是会自我修改的规则。

### 目标

1. 在 `goods-domain` 中新增 `Outcome`/`OutcomeStatus`（新的 `outcome` 模块），并实现既有的 `Learning`
   占位符，不修改 Phase 1-4 的 State/Need/Care/Memory 模型。
2. 使 Outcome 的比较纯粹基于事实：CareAction 所针对的 NeedKind 是否仍存在于后续的 Need Assessment 中。
   不对 Caregiver 的决定进行定性打分。
3. 使 Learning 严格限定为可 review 的陈述：只携带 Outcome 与一段自然语言 `String`，没有任何会修改
   阈值、profile 值或其他规则的方法或字段。
4. 在 `goods-application` 中实现 `VerifyOutcome`/`LearnFromOutcome`，并公开
   `GoodsRuntime::verify_and_learn`，它接收此前的 `CareAction` 并通过同一个 `ObservationSource`
   port 再次观测，且不改动现有四个 runtime 方法。
5. 保持 CLI demo 默认内置运行没有待验证事项，维持 Phase 1 平静的参考体验；通过两个构造的观测
   （初始与后续）的测试验证已解决与未解决两条路径。

### 非目标

本提案不实现或授权对 `Urgency` 阈值、`GoodsProfile` 字段、Care 角色路由或任何其他规则的自动调整；
不实现通用的 `Evidence` 类型；不实现持久化；不实现 Phase 6 的多日（“Seven Day Life”）循环；也不涉及
任何前端改动（`apps/goods-garden-web/` 保持不变）。

### 决策：Outcome 是事实比较，而非打分

`Outcome::verify(action, new_state, new_needs)` 收集原始 `CareRequest` 所针对的 `NeedKind`，并检查
其中是否仍有任何一个出现在 `new_needs` 中。`OutcomeStatus::Resolved` 表示一个都不再存在；
`OutcomeStatus::Unresolved` 表示至少一个仍然存在。这完全来自 Phase 2 已经计算的数据（`NeedKind`
身份），因此不引入任何缺乏证据的新业务规则——与 Phase 1 用于 healthy/unhealthy 的比较属于同一类
有边界、可解释的比较。

### 决策：Learning 从不自行调整规则

`Learning { outcome, statement }` 没有其他字段，也没有任何会写回 `GoodsProfile`、`Urgency` 阈值或
其他领域规则的方法。其 `statement` 明确说明“这是一个可 review 的观察，不是规则变更”。这已与仓库负责人
确认为 Phase 5 的正确范围，考虑到 `AGENTS.md` 对学习策略的保护：曾考虑过保留一个供未来人类采纳的
“provisional adjustment”字段的替代方案，但在没有直接需求的情况下会扩大范围和风险，因而被明确不采用。

### 决策：verification 复用既有的 ObservationSource port

`GoodsRuntime::verify_and_learn(goods, action)` 通过 `self.source`（与其他方法相同的
`ObservationSource`）再次观测，并在内部调用 `observe_and_identify_needs`。“后续”观测只是另一个
`ObservationSource` 值（例如测试中第二个 `DemoObservationSource::new(..)`）——不引入新的 port，
也不在此之外建模“时间流逝”的概念。

### 已考虑的替代方案

**让 Learning 提出具体的阈值/参数变更建议**（带有“provisional，尚未应用”标记）：不采用。即使未应用，
一个具体的建议数值也已经足够接近臆造的规则，存在日后在缺乏真实证据的情况下被当作权威依据的风险；
仓库负责人已确认仅采用纯陈述的范围。

**跨多个事件将 Learning 聚合为趋势/评分**：作为过早设计不采用；Phase 6（“Seven Day Life”）才是第一次
让商品经历真正多日循环的阶段，此处的聚合建立在过少的证据之上。

**现在就引入通用的 `Evidence` 类型**：不采用；`Outcome` 自身的字段（哪个 CareAction、哪个后续
State/Need Assessment）已经为本阶段的需求提供了完整的可追溯性，添加抽象包装为时过早。

### 已知、推断与未知

- **KNOWN：** `verify_outcome.rs`、`learn_from_outcome.rs`、`learning/learning.rs` 是为本阶段保留的
  空占位符；Phase 1-4 的类型不受本提案影响（Outcome 只读取 `CareAction` 与 `NeedAssessment`，不扩展
  它们）。
- **KNOWN：** 获得 `Outcome`/`Learning` 值的唯一途径是通过 `GoodsRuntime::verify_and_learn`，它要求
  一个已经记录的 `CareAction`；不存在凭空产生它的路径。
- **INFERRED：** 比较前后 `NeedKind` 的存在与否，是“Need 是否消失”的合理最小代理指标，但不宣称知道
  它消失或未消失背后真实世界的原因。
- **UNKNOWN：** 真实的学习/适应策略、未来阶段是否应让人类根据累积的 Learning 记录明确批准规则变更、
  以及 `docs/phases/phase-6-seven-day-life.md` 中提到的多日场景下 Learning 应如何表现；在后续携带
  证据的 Work Item 处理之前，这些保持为 `UNKNOWN`/不在范围内。
- **UNAVAILABLE：** 本仓库中真实的外部结果/有效性数据。

### Review gate 与验收

本文档状态为 `PROPOSED`。批准应确认：

1. Outcome 是对 NeedKind 存在与否的事实比较，而非对 Caregiver 决定的定性打分。
2. Learning 只是纯粹可 review 的陈述，没有任何调整阈值、profile 值或其他规则的字段或方法。
3. `state::goods_state`、Phase 2 Need 模型、Phase 3 Care 模型、Phase 4 Memory 模型与前端保持不变。
