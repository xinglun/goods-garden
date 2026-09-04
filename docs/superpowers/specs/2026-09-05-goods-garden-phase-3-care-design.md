# Goods Garden Phase 3 — Care Design Specification

**Status:** PROPOSED — requires human review before implementation

**Date:** 2026-09-05

**Scope:** `goods-domain`, `goods-application`, `goods-runtime`, `goods-infrastructure` and
`apps/goods-garden-cli` additions for Phase 3 (CareRequest, Caregiver, Human Feedback, CareAction).

## English

### Context and problem

Phase 2 gave the good an explainable Need model (Deviation, Urgency,
GoodsNeed, NeedConflict). `crates/goods-domain/src/care/{care_request.rs,
care_action.rs}` were left as empty placeholders. `docs/phases/phase-3-care.md`
names CareRequest, Caregiver, Human Feedback and CareAction as the first
two-way `Goods ↔ Human` interaction, but did not yet define their fields or
boundary.

The repository's own governance (`AGENTS.md`) treats Care authority as a
protected semantic area: "AI does not own business authority" (North Star),
and a human decision is required before granting any autonomous action
authority. Phase 3 must therefore make the good's role strictly limited to
noticing a Need and asking; it must never decide, infer or synthesize what a
human would say.

### Goals

1. Add `CareRequest`, `Caregiver`, `HumanFeedback` and `CareAction` to
   `goods-domain` without changing the Phase 1/2 State/Need model.
2. Guarantee, by construction, that Human Feedback is always external input:
   introduce a `HumanFeedbackSource` port (mirroring `ObservationSource`) so
   the only way to obtain a `HumanFeedback` value is through an adapter, never
   through domain or application computation.
3. Implement `RequestCare` and `ReceiveCare` in `goods-application` and expose
   `GoodsRuntime::request_care` alongside the unchanged `observe_and_assess`
   and `observe_and_identify_needs`.
4. Keep the CLI demo's default, bundled run showing no Need and therefore no
   Care Request, preserving the calm Phase 1 reference experience; exercise
   the Care path through tests with crafted observations.

### Non-goals

This proposal does not implement or authorize Memory, Learning, autonomous
business action, a real interactive prompt, a provider-backed Caregiver
channel (Slack, email, a real staff app), role-routing or permission models,
or any frontend change (`apps/goods-garden-web/` is untouched; Need and Care
remain explicitly out of scope for the frontend per `AGENTS.md`).

### Decision: Care never decides, it only asks and records

`CareRequest::from_assessment` fires whenever `NeedAssessment.needs` is
non-empty; it carries the Needs, any `NeedConflict`, a single fixed
`requested_role = "store staff"`, and an explanation. It does not recommend
an action beyond "please review." `CareAction::record` binds a `CareRequest`
to a `HumanFeedback` and its explanation is a verbatim restatement of the
Caregiver's decision — the domain adds no independent judgment about whether
that decision was correct or sufficient.

### Decision: Human Feedback is a port-shaped external input, not computed

```rust
pub trait HumanFeedbackSource {
    type Error;
    fn provide_feedback(&self, request: &CareRequest) -> Result<HumanFeedback, Self::Error>;
}
```

The only implementation in this phase is `DemoHumanFeedbackSource`, a local
synthetic fixture adapter mirroring `DemoObservationSource`. This keeps the
architectural door open for a future, explicitly reviewed interactive or
provider-backed adapter without ever letting the domain/application layers
invent a decision themselves.

### Decision: a single fixed Caregiver role, no routing

`Caregiver.role` and `CareRequest.requested_role` are plain `String` values,
matching the existing `GoodsIdentity.species`/`Observation.source` style
rather than a closed enum. Phase 3 only ever produces `"store staff"`; no
role-matching, escalation-routing or authorization model is implemented,
consistent with the single-role minimalism of Phase 1's single product
profile and Phase 2's two fixed dimensions.

### Alternatives considered

**A real interactive stdin prompt for the demo:** rejected for this phase.
While it would make the "first two-way interaction" more literal, it would
make `cargo test`/CI non-deterministic and break the established
fixture-based pattern from Phase 1/2. The port abstraction keeps this option
open as a later, separately reviewed adapter.

**Routing CareRequest by NeedKind (e.g., a different role for
`FreshnessConcern` vs. `StockAvailabilityConcern`):** rejected as premature;
this synthetic single-store context has no evidenced distinction between
roles, and inventing one would be exactly the kind of fabricated business
rule the Trust Model prohibits.

**Letting `ReceiveCare` evaluate or grade the Human Feedback (e.g., marking
it "resolved"/"unresolved"):** rejected; that would let the domain judge a
human decision, encroaching on "AI does not own business authority."

### Known facts, inferences and unknowns

- **KNOWN:** `CareRequest`/`CareAction` were empty placeholders reserved for
  this phase; the Phase 1/2 State/Need types are unaffected by this proposal
  (no shared struct gains a new field this time).
- **KNOWN:** the only way to construct a `HumanFeedback` outside a test is
  through `HumanFeedbackSource::provide_feedback`, which the demo satisfies
  with a synthetic fixture, not stdin or a real system.
- **INFERRED:** a single `"store staff"` Caregiver role is sufficient to
  demonstrate the Goods↔Human boundary without inventing an unevidenced
  permission model.
- **UNKNOWN:** real store staffing roles, escalation policy and any real
  Caregiver notification channel; these remain `UNKNOWN`/out of scope per the
  Trust Model.
- **UNAVAILABLE:** real external Caregiver systems or human response data in
  this repository.

### Review gate and acceptance

This document is `PROPOSED`. Approval should confirm:

1. CareRequest fires for every identified Need (no severity threshold) and
   never recommends a specific action.
2. Human Feedback is obtained exclusively through the `HumanFeedbackSource`
   port, with only a synthetic fixture adapter in this phase.
3. A single fixed `"store staff"` Caregiver role is acceptable for Phase 3.
4. `state::goods_state`, the Phase 2 Need model and the frontend remain
   untouched.

## 日本語

### 背景と課題

Phase 2 は good に explainable な Need model（Deviation、Urgency、GoodsNeed、NeedConflict）を与えた。
`crates/goods-domain/src/care/{care_request.rs, care_action.rs}` は空プレースホルダーのまま残されていた。
`docs/phases/phase-3-care.md` は CareRequest、Caregiver、Human Feedback、CareAction を初めての
双方向 `Goods ↔ Human` interaction として挙げていたが、field や境界はまだ定義されていなかった。

repository 自身の governance（`AGENTS.md`）は Care authority を protected semantic area として扱う：
「AI does not own business authority」（North Star）であり、自律 action の authority を与える前には
human decision が必要である。したがって Phase 3 では good の役割を「Need に気づき、求める」ことに厳密に限定し、
人間が何を言うかを決定・推論・合成することは決してしてはならない。

### 目標

1. Phase 1/2 の State/Need model を変更せず、`goods-domain` に `CareRequest`、`Caregiver`、
   `HumanFeedback`、`CareAction` を追加する。
2. Human Feedback が常に外部入力であることを構造的に保証する：`ObservationSource` を模した
   `HumanFeedbackSource` port を導入し、`HumanFeedback` 値を得る唯一の方法を adapter 経由に限定し、
   domain や application の計算では得られないようにする。
3. `goods-application` に `RequestCare` と `ReceiveCare` を実装し、既存の `observe_and_assess` と
   `observe_and_identify_needs` を変更せずに `GoodsRuntime::request_care` を追加公開する。
4. CLI demo の同梱デフォルト実行では Need（したがって Care Request）が発生しないままにし、Phase 1
   の落ち着いた reference 体験を保つ。Care の経路は crafted な observation を使う test で検証する。

### 非目標

本提案は Memory、Learning、自律 business action、real な対話式 prompt、provider-backed Caregiver
channel（Slack、email、real staff app）、role-routing や permission model、frontend の変更を実装・許可
しない（`apps/goods-garden-web/` は無変更。AGENTS.md により Need と Care は frontend に対して明示的に対象外）。

### 決定：Care は決して決定せず、求めて記録するだけ

`CareRequest::from_assessment` は `NeedAssessment.needs` が非空なら必ず発火し、Need 群、`NeedConflict`
の有無、単一の固定 `requested_role = "store staff"`、explanation を持つ。「review してください」以上の
具体的な action は推奨しない。`CareAction::record` は `CareRequest` と `HumanFeedback` を結びつけ、その
explanation は Caregiver の決定を逐語的に言い換えるだけで、domain はその決定が正しいか十分かについて
独自の判断を加えない。

### 決定：Human Feedback は port 形の外部入力であり、計算されない

```rust
pub trait HumanFeedbackSource {
    type Error;
    fn provide_feedback(&self, request: &CareRequest) -> Result<HumanFeedback, Self::Error>;
}
```

本 phase での唯一の実装は `DemoHumanFeedbackSource`、`DemoObservationSource` を模した local synthetic
fixture adapter である。これにより、domain/application layer が自ら決定を発明することを決して許さないまま、
将来の明示的に review された対話式または provider-backed adapter への architectural な道を残す。

### 決定：単一の固定 Caregiver role、routing なし

`Caregiver.role` と `CareRequest.requested_role` は、閉じた enum ではなく既存の
`GoodsIdentity.species`/`Observation.source` と同じ流儀の plain `String` 値とする。Phase 3 は常に
`"store staff"` のみを生成し、role-matching、escalation-routing、authorization model は実装しない。
Phase 1 の単一 product profile、Phase 2 の2つの固定次元と同じ単一-role の minimalism に一致する。

### 検討した代替案

**demo での real な interactive stdin prompt**：本 phase では不採用。「最初の双方向 interaction」をより
文字通りにするが、`cargo test`/CI を non-deterministic にし、Phase 1/2 で確立した fixture-based pattern
を破る。port 抽象化により、この選択肢は後続の別途 review された adapter として残る。

**NeedKind による CareRequest の routing**（例：`FreshnessConcern` と `StockAvailabilityConcern` で
異なる role）：時期尚早として不採用。この synthetic な単一店舗 context には role を区別する evidence が
なく、それを発明することは Trust Model が禁じる fabricated business rule そのものである。

**`ReceiveCare` に Human Feedback を評価・採点させる**（例："resolved"/"unresolved" と marking する）：
不採用。domain が human decision を判定することになり、「AI does not own business authority」を侵す。

### Known、inference、unknown

- **KNOWN：** `CareRequest`/`CareAction` は本 phase 用に予約された空プレースホルダーであり、
  Phase 1/2 の State/Need type は本提案の影響を受けない（今回は共有 struct への新規 field 追加なし）。
- **KNOWN：** test 以外で `HumanFeedback` を構築する唯一の方法は `HumanFeedbackSource::provide_feedback`
  経由であり、demo はこれを stdin や real system ではなく synthetic fixture で満たす。
- **INFERRED：** 単一の `"store staff"` Caregiver role で、根拠のない permission model を発明せずに
  Goods↔Human 境界を示すには十分である。
- **UNKNOWN：** real な店舗 staffing role、escalation policy、real な Caregiver 通知 channel。これらは
  Trust Model に従い `UNKNOWN`/対象外のままとする。
- **UNAVAILABLE：** この repository 内の real external Caregiver system や human response data。

### Review gate と acceptance

この文書は `PROPOSED` である。承認では次を確認する。

1. CareRequest が識別された全ての Need に対して発火し（重大度 threshold なし）、特定の action を
   推奨しないこと。
2. Human Feedback が `HumanFeedbackSource` port 経由でのみ得られ、本 phase では synthetic fixture
   adapter のみであること。
3. 単一の固定 `"store staff"` Caregiver role が Phase 3 として妥当であること。
4. `state::goods_state`、Phase 2 Need model、frontend が無変更のままであること。

## 中文

### 背景与问题

Phase 2 为商品提供了可解释的 Need 模型（Deviation、Urgency、GoodsNeed、NeedConflict）。
`crates/goods-domain/src/care/{care_request.rs, care_action.rs}` 被保留为空占位符。
`docs/phases/phase-3-care.md` 将 CareRequest、Caregiver、Human Feedback、CareAction 列为第一次
双向 `Goods ↔ Human` 交互，但尚未定义其字段与边界。

仓库自身的治理（`AGENTS.md`）将 Care 权限视为受保护的语义区域：“AI does not own business authority”
（North Star），在授予任何自主行动权限之前都需要人类决策。因此 Phase 3 必须严格限定商品的角色仅限于
“注意到 Need 并请求”，绝不能由其决定、推断或合成人类会说什么。

### 目标

1. 在不修改 Phase 1/2 State/Need 模型的前提下，为 `goods-domain` 增加 `CareRequest`、`Caregiver`、
   `HumanFeedback`、`CareAction`。
2. 从结构上保证 Human Feedback 始终是外部输入：引入模仿 `ObservationSource` 的 `HumanFeedbackSource`
   port，使获得 `HumanFeedback` 值的唯一途径是通过 adapter，而非领域或应用层的计算。
3. 在 `goods-application` 中实现 `RequestCare` 与 `ReceiveCare`，并在不改动现有 `observe_and_assess`
   与 `observe_and_identify_needs` 的前提下新增公开 `GoodsRuntime::request_care`。
4. 保持 CLI demo 默认内置运行不产生 Need、因而不产生 Care Request，维持 Phase 1 平静的参考体验；
   通过带有构造 observation 的测试验证 Care 路径。

### 非目标

本提案不实现或授权 Memory、Learning、自主经营行动、真实的交互式提示、provider-backed Caregiver 渠道
（Slack、邮件、真实员工应用）、角色路由或权限模型，也不涉及任何前端改动（`apps/goods-garden-web/`
保持不变；根据 AGENTS.md，Need 与 Care 对前端明确不在范围内）。

### 决策：Care 从不做决定，只请求与记录

只要 `NeedAssessment.needs` 非空，`CareRequest::from_assessment` 就必定触发，携带 Need 集合、是否存在
`NeedConflict`、单一固定的 `requested_role = "store staff"` 与 explanation。它不推荐超出“请审核”之外的
具体行动。`CareAction::record` 将 `CareRequest` 与 `HumanFeedback` 绑定，其 explanation 只是逐字复述
Caregiver 的决定——领域不对该决定是否正确或充分附加任何独立判断。

### 决策：Human Feedback 是 port 形式的外部输入，而非计算得出

```rust
pub trait HumanFeedbackSource {
    type Error;
    fn provide_feedback(&self, request: &CareRequest) -> Result<HumanFeedback, Self::Error>;
}
```

本阶段唯一的实现是 `DemoHumanFeedbackSource`，一个模仿 `DemoObservationSource` 的本地 synthetic
fixture adapter。这样既不让领域/应用层自行编造决定，又为未来经过明确评审的交互式或
provider-backed adapter 保留了架构上的空间。

### 决策：单一固定 Caregiver 角色，不做路由

`Caregiver.role` 与 `CareRequest.requested_role` 是普通 `String` 值，与既有的
`GoodsIdentity.species`/`Observation.source` 风格一致，而非封闭的 enum。Phase 3 始终只产生
`"store staff"`；不实现角色匹配、升级路由或授权模型，与 Phase 1 单一商品 profile、Phase 2 两个固定维度
的单一角色极简主义保持一致。

### 已考虑的替代方案

**为 demo 提供真实的交互式 stdin 提示**：本阶段不采用。虽然能让“第一次双向交互”更加字面化，但会使
`cargo test`/CI 变得不确定，并打破 Phase 1/2 建立的基于 fixture 的模式。port 抽象保留了该选项，作为
后续单独评审的 adapter。

**按 NeedKind 对 CareRequest 进行路由**（例如 `FreshnessConcern` 与 `StockAvailabilityConcern`
对应不同角色）：作为过早设计不采用；这个 synthetic 的单店铺场景没有区分角色的证据，凭空发明会正是
Trust Model 所禁止的臆造业务规则。

**让 `ReceiveCare` 对 Human Feedback 进行评估或打分**（例如标记为“已解决”/“未解决”）：不采用；这会让
领域评判人类决策，侵犯“AI does not own business authority”。

### 已知、推断与未知

- **KNOWN：** `CareRequest`/`CareAction` 是为本阶段保留的空占位符；Phase 1/2 的 State/Need 类型不受
  本提案影响（这次没有为共享 struct 新增字段）。
- **KNOWN：** 在测试之外构造 `HumanFeedback` 的唯一途径是通过 `HumanFeedbackSource::provide_feedback`，
  demo 用 synthetic fixture 而非 stdin 或真实系统来满足它。
- **INFERRED：** 单一的 `"store staff"` Caregiver 角色足以展示 Goods↔Human 边界，而无需臆造缺乏证据的
  权限模型。
- **UNKNOWN：** 真实的门店排班角色、升级策略以及任何真实的 Caregiver 通知渠道；按照 Trust Model，
  这些保持为 `UNKNOWN`/不在范围内。
- **UNAVAILABLE：** 本仓库中真实的外部 Caregiver 系统或人类响应数据。

### Review gate 与验收

本文档状态为 `PROPOSED`。批准应确认：

1. CareRequest 对每一个被识别的 Need 都会触发（无严重度阈值），且从不推荐具体行动。
2. Human Feedback 仅通过 `HumanFeedbackSource` port 获得，本阶段仅有 synthetic fixture adapter。
3. 单一固定的 `"store staff"` Caregiver 角色对 Phase 3 是可接受的。
4. `state::goods_state`、Phase 2 Need 模型与前端保持不变。
