# Domain Model

## English

Phase 1 separates the minimal model needed by the local demo from future
provisional concepts. The Phase 1 model is reviewable, deliberately small and
does not claim to settle the long-term domain schema.

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

### Future provisional concepts

The following remain vocabulary for discussion, not implemented Phase 1
behavior or settled schemas.

| Candidate | Provisional role | Status |
| --- | --- | --- |
| GoodsNeed | Need representation candidate | PROVISIONAL |
| Evidence | Traceability support candidate | PROVISIONAL |
| CareRequest | Request-for-care candidate | PROVISIONAL |
| CareAction | Care record candidate | PROVISIONAL |
| Outcome | Result candidate | PROVISIONAL |
| Memory | Relationship/history candidate | PROVISIONAL |
| Learning | Learning-result candidate | PROVISIONAL |
| LifecycleState | Lifecycle candidate | PROVISIONAL |

## 日本語

Phase 1 では local demo に必要な最小 model と、将来の provisional concept を分ける。Phase 1 model は review
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

### Future provisional concepts

以下は議論用 vocabulary であり、Phase 1 の実装挙動でも確定 schema でもない。

| Candidate | Provisional role | Status |
| --- | --- | --- |
| GoodsNeed | Need representation の候補 | PROVISIONAL |
| Evidence | traceability support の候補 | PROVISIONAL |
| CareRequest | request-for-care の候補 | PROVISIONAL |
| CareAction | care record の候補 | PROVISIONAL |
| Outcome | result の候補 | PROVISIONAL |
| Memory | relationship/history の候補 | PROVISIONAL |
| Learning | learning-result の候補 | PROVISIONAL |
| LifecycleState | lifecycle の候補 | PROVISIONAL |

## 中文

Phase 1 将本地 demo 所需的最小模型与未来 provisional 概念分开。Phase 1 模型是可 review 的小模型，并不意味着已经确定长期领域 schema。

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

### Future provisional concepts

以下仍只是讨论用 vocabulary，不是 Phase 1 的实现行为，也不是已经确定的 schema。

| 候选 | Provisional 作用 | Status |
| --- | --- | --- |
| GoodsNeed | Need 表示的候选 | PROVISIONAL |
| Evidence | 可追溯性支持的候选 | PROVISIONAL |
| CareRequest | 求助请求的候选 | PROVISIONAL |
| CareAction | 照料记录的候选 | PROVISIONAL |
| Outcome | 结果的候选 | PROVISIONAL |
| Memory | 关系/历史的候选 | PROVISIONAL |
| Learning | 学习结果的候选 | PROVISIONAL |
| LifecycleState | 生命周期的候选 | PROVISIONAL |
