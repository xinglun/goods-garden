# Domain Model

## English

Phase 1 separates the minimal model needed by the local demo from future
provisional concepts. Phase 2 adds a minimal Need model on top without
changing Phase 1; Phase 3 adds a minimal Care model on top without changing
Phase 1 or Phase 2. All are reviewable, deliberately small and do not claim to
settle the long-term domain schema.

### Phase 1 minimal model

| Type | Minimal role in the demo | Status |
| --- | --- | --- |
| Goods | Aggregate containing an individual identity and profile | PHASE 1 MINIMAL |
| GoodsIdentity | `species` and `individual_id` for the individual good | PHASE 1 MINIMAL |
| GoodsProfile | `display_name` and `expected_lifetime_hours` | PHASE 1 MINIMAL |
| Observation | `source`, `observed_at` and `age_hours` sensory input | PHASE 1 MINIMAL |
| Expectation | Maximum age used for the bounded assessment | PHASE 1 MINIMAL |
| GoodsState | Identity, observation, expectation and health result | PHASE 1 MINIMAL |
| HealthAssessment | Healthy/unhealthy status with a plain-language explanation | PHASE 1 MINIMAL |

The assessment rule is intentionally narrow: an observed age at or below the
profile expectation is healthy; an age above it is unhealthy. The bundled input
is a local `synthetic-example`, not a real POS or SEJ record. This rule and
these fields may be refined by a later Domain Design Work Item.

### Phase 2 minimal model

Phase 2 adds an explainable Need model on top of the unchanged Phase 1
State/Health Assessment. It introduces one additional evaluation dimension,
StockAvailability, alongside the existing Freshness dimension, so that two
Needs can genuinely coexist and conflict.

| Type | Minimal role in Phase 2 | Status |
| --- | --- | --- |
| DeviationDimension | `Freshness` or `StockAvailability`, the axis a Deviation was derived from | PHASE 2 MINIMAL |
| Deviation | Signed gap between an observation and its expectation on one dimension | PHASE 2 MINIMAL |
| Urgency | `Low`/`Medium`/`High`, derived from a Deviation's magnitude by fixed, dimension-specific thresholds | PHASE 2 MINIMAL |
| NeedKind | `FreshnessConcern` or `StockAvailabilityConcern`; names the concern, not a Care response | PHASE 2 MINIMAL |
| GoodsNeed | Kind, Urgency, Deviation and explanation for one concerning Deviation | PHASE 2 MINIMAL |
| NeedConflict | Explainable contradiction when a FreshnessConcern and a StockAvailabilityConcern Need coexist | PHASE 2 MINIMAL |
| NeedAssessment | The Needs and any Need Conflict identified from one observation | PHASE 2 MINIMAL |

`Observation` gained `quantity_on_hand` and `GoodsProfile` gained
`minimum_stock_quantity` to support the StockAvailability dimension. The
Urgency thresholds are fixed example/hypothesis values, not derived from real
POS or SEJ data; see `docs/phases/phase-2-need.md`.

### Phase 3 minimal model

Phase 3 adds an explainable Care model on top of the unchanged Phase 1/2
State and Need model. A CareRequest is raised whenever a Need exists; Human
Feedback is always external input, never computed by the domain; a CareAction
only records what a Caregiver said.

| Type | Minimal role in Phase 3 | Status |
| --- | --- | --- |
| CareRequest | Needs, any Need Conflict, a requested Caregiver role and an explanation | PHASE 3 MINIMAL |
| Caregiver | A `role` and `display_name` identifying who may help | PHASE 3 MINIMAL |
| HumanFeedback | A Caregiver's `decision` and `provided_at`, supplied as external input | PHASE 3 MINIMAL |
| CareAction | Binds a CareRequest to the HumanFeedback that resolved it | PHASE 3 MINIMAL |

The bounded rule is that every identified Need raises a CareRequest for a
single, fixed `"store staff"` role; no role-routing or permission model is
implemented. HumanFeedback is read from a `HumanFeedbackSource` port whose
only implementation is a local synthetic fixture, matching Phase 1/2's
`ObservationSource` pattern; a real interactive or provider-backed adapter is
not implemented. See `docs/phases/phase-3-care.md`.

### Future provisional concepts

The following remain vocabulary for discussion, not implemented behavior or
settled schemas.

| Candidate | Provisional role | Status |
| --- | --- | --- |
| Evidence | Traceability support candidate | PROVISIONAL |
| Outcome | Result candidate | PROVISIONAL |
| Memory | Relationship/history candidate | PROVISIONAL |
| Learning | Learning-result candidate | PROVISIONAL |
| LifecycleState | Lifecycle candidate | PROVISIONAL |

## 日本語

Phase 1 では local demo に必要な最小 model と、将来の provisional concept を分ける。Phase 2 は Phase 1 を変更せず
その上に最小の Need model を追加し、Phase 3 は Phase 1/2 を変更せずその上に最小の Care model を追加する。いずれも review
可能な小さい model であり、長期的な domain schema を確定したとは主張しない。

### Phase 1 minimal model

| Type | demo における最小の役割 | Status |
| --- | --- | --- |
| Goods | individual identity と profile を持つ Aggregate | PHASE 1 MINIMAL |
| GoodsIdentity | individual good の `species` と `individual_id` | PHASE 1 MINIMAL |
| GoodsProfile | `display_name` と `expected_lifetime_hours` | PHASE 1 MINIMAL |
| Observation | `source`、`observed_at`、`age_hours` の sensory input | PHASE 1 MINIMAL |
| Expectation | bounded assessment に使う最大 age | PHASE 1 MINIMAL |
| GoodsState | identity、observation、expectation、health result | PHASE 1 MINIMAL |
| HealthAssessment | plain-language explanation 付き healthy/unhealthy status | PHASE 1 MINIMAL |

assessment rule は意図的に狭い。observed age が profile expectation 以下なら healthy、超えれば unhealthy とする。
同梱 input は local `synthetic-example` であり、real POS や SEJ record ではない。この rule と field は後続の Domain Design
Work Item で改訂できる。

### Phase 2 minimal model

Phase 2 は不変の Phase 1 State/Health Assessment の上に、explainable な Need model を追加する。既存の
Freshness 次元に加えて StockAvailability という追加の評価次元を導入し、2つの Need が実際に共存し衝突できるようにする。

| Type | Phase 2 における最小の役割 | Status |
| --- | --- | --- |
| DeviationDimension | Deviation の由来を示す `Freshness` または `StockAvailability` | PHASE 2 MINIMAL |
| Deviation | ある次元での observation と expectation の符号付き gap | PHASE 2 MINIMAL |
| Urgency | Deviation の magnitude から次元別の固定閾値で導く `Low`/`Medium`/`High` | PHASE 2 MINIMAL |
| NeedKind | `FreshnessConcern` または `StockAvailabilityConcern`。Care response ではなく懸念を名付ける | PHASE 2 MINIMAL |
| GoodsNeed | 1つの懸念ある Deviation に対する kind、Urgency、Deviation、explanation | PHASE 2 MINIMAL |
| NeedConflict | FreshnessConcern と StockAvailabilityConcern の Need が共存する場合の explainable な矛盾 | PHASE 2 MINIMAL |
| NeedAssessment | 1つの observation から識別される Need 群と Need Conflict の有無 | PHASE 2 MINIMAL |

StockAvailability 次元のため、`Observation` に `quantity_on_hand`、`GoodsProfile` に
`minimum_stock_quantity` を追加した。Urgency の閾値は固定の example/hypothesis 値であり、real POS
や SEJ data から導いたものではない。詳細は `docs/phases/phase-2-need.md` を参照。

### Phase 3 minimal model

Phase 3 は不変の Phase 1/2 State/Need model の上に、explainable な Care model を追加する。CareRequest は
Need が存在する限り必ず発生し、Human Feedback は常に外部入力であり domain が計算することはなく、CareAction は
Caregiver が実際に言ったことを記録するだけである。

| Type | Phase 3 における最小の役割 | Status |
| --- | --- | --- |
| CareRequest | Need 群、Need Conflict の有無、要求する Caregiver role、explanation | PHASE 3 MINIMAL |
| Caregiver | 誰が助けられるかを示す `role` と `display_name` | PHASE 3 MINIMAL |
| HumanFeedback | Caregiver の `decision` と `provided_at`。外部入力として供給される | PHASE 3 MINIMAL |
| CareAction | CareRequest とそれを解決した HumanFeedback を結びつける | PHASE 3 MINIMAL |

bounded rule は、識別された全ての Need が単一の固定 role `"store staff"` への CareRequest を発生させるというもので、
role-routing や permission model は実装しない。HumanFeedback は `HumanFeedbackSource` port から読み取り、唯一の実装は
Phase 1/2 の `ObservationSource` と同じ local synthetic fixture である。real な対話式または provider-backed adapter は実装しない。
詳細は `docs/phases/phase-3-care.md` を参照。

### Future provisional concepts

以下は議論用 vocabulary であり、実装挙動でも確定 schema でもない。

| Candidate | Provisional role | Status |
| --- | --- | --- |
| Evidence | traceability support の候補 | PROVISIONAL |
| Outcome | result の候補 | PROVISIONAL |
| Memory | relationship/history の候補 | PROVISIONAL |
| Learning | learning-result の候補 | PROVISIONAL |
| LifecycleState | lifecycle の候補 | PROVISIONAL |

## 中文

Phase 1 将本地 demo 所需的最小模型与未来 provisional 概念分开。Phase 2 在不改动 Phase 1 的前提下，于其上新增最小 Need 模型；
Phase 3 在不改动 Phase 1/2 的前提下，于其上新增最小 Care 模型。三者都是可 review 的小模型，并不意味着已经确定长期领域 schema。

### Phase 1 minimal model

| 类型 | demo 中的最小作用 | Status |
| --- | --- | --- |
| Goods | 包含单个商品身份和 profile 的 Aggregate | PHASE 1 MINIMAL |
| GoodsIdentity | 单个商品的 `species` 和 `individual_id` | PHASE 1 MINIMAL |
| GoodsProfile | `display_name` 和 `expected_lifetime_hours` | PHASE 1 MINIMAL |
| Observation | `source`、`observed_at`、`age_hours` 感官输入 | PHASE 1 MINIMAL |
| Expectation | 有边界评估所使用的最大 age | PHASE 1 MINIMAL |
| GoodsState | identity、observation、expectation 和 health result | PHASE 1 MINIMAL |
| HealthAssessment | 带有自然语言解释的 healthy/unhealthy 状态 | PHASE 1 MINIMAL |

评估规则刻意保持狭窄：observed age 小于等于 profile expectation 时为 healthy，超过时为 unhealthy。仓库内置输入是本地
`synthetic-example`，不是 POS 或 SEJ 实际记录。这些字段和规则可以由后续 Domain Design Work Item 调整。

### Phase 2 minimal model

Phase 2 在不变的 Phase 1 State/Health Assessment 之上，增加了可解释的 Need 模型。除已有的 Freshness
维度外，新增 StockAvailability 评估维度，使两个 Need 能够真实共存并发生冲突。

| 类型 | 在 Phase 2 中的最小作用 | Status |
| --- | --- | --- |
| DeviationDimension | 标明 Deviation 来源的 `Freshness` 或 `StockAvailability` | PHASE 2 MINIMAL |
| Deviation | 某维度上 observation 与 expectation 之间带符号的 gap | PHASE 2 MINIMAL |
| Urgency | 根据 Deviation 的 magnitude、按维度专属固定阈值得出的 `Low`/`Medium`/`High` | PHASE 2 MINIMAL |
| NeedKind | `FreshnessConcern` 或 `StockAvailabilityConcern`；命名的是关注点而非 Care 响应 | PHASE 2 MINIMAL |
| GoodsNeed | 针对一个令人担忧的 Deviation 的 kind、Urgency、Deviation 与 explanation | PHASE 2 MINIMAL |
| NeedConflict | FreshnessConcern 与 StockAvailabilityConcern 的 Need 共存时的可解释矛盾 | PHASE 2 MINIMAL |
| NeedAssessment | 从一次 observation 中识别出的 Need 集合与是否存在 Need Conflict | PHASE 2 MINIMAL |

为支持 StockAvailability 维度，`Observation` 新增 `quantity_on_hand`，`GoodsProfile` 新增
`minimum_stock_quantity`。Urgency 阈值是固定的 example/hypothesis 数值，并非源自真实 POS 或 SEJ
数据；详见 `docs/phases/phase-2-need.md`。

### Phase 3 minimal model

Phase 3 在不变的 Phase 1/2 State/Need 模型之上，增加了可解释的 Care 模型。只要存在 Need 就必定产生 CareRequest；
Human Feedback 始终是外部输入，领域从不计算它；CareAction 只记录 Caregiver 实际说过的话。

| 类型 | 在 Phase 3 中的最小作用 | Status |
| --- | --- | --- |
| CareRequest | Need 集合、是否存在 Need Conflict、请求的 Caregiver role 与 explanation | PHASE 3 MINIMAL |
| Caregiver | 标明谁可以提供帮助的 `role` 与 `display_name` | PHASE 3 MINIMAL |
| HumanFeedback | Caregiver 的 `decision` 与 `provided_at`，作为外部输入提供 | PHASE 3 MINIMAL |
| CareAction | 将 CareRequest 与解决它的 HumanFeedback 绑定 | PHASE 3 MINIMAL |

有边界规则是：每一个被识别的 Need 都会向单一固定角色 `"store staff"` 发出 CareRequest，不实现角色路由或权限模型。
HumanFeedback 从 `HumanFeedbackSource` port 读取，唯一实现是与 Phase 1/2 的 `ObservationSource` 相同风格的本地
synthetic fixture；不实现真实的交互式或 provider-backed adapter。详见 `docs/phases/phase-3-care.md`。

### Future provisional concepts

以下仍只是讨论用 vocabulary，不是已实现行为，也不是已经确定的 schema。

| 候选 | Provisional 作用 | Status |
| --- | --- | --- |
| Evidence | 可追溯性支持的候选 | PROVISIONAL |
| Outcome | 结果的候选 | PROVISIONAL |
| Memory | 关系/历史的候选 | PROVISIONAL |
| Learning | 学习结果的候选 | PROVISIONAL |
| LifecycleState | 生命周期的候选 | PROVISIONAL |
