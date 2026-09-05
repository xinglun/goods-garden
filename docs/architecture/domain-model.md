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

Phase 1 では local demo に必要な最小 model と、将来の provisional concept を分ける。Phase 2 は Phase 1 を変更せず
その上に最小の Need model を追加し、Phase 3 は Phase 1/2 を変更せずその上に最小の Care model を追加し、Phase 4 は
Phase 1-3 を変更せずその上に最小の Memory model を追加し、Phase 5 は Phase 1-4 を変更せずその上に最小の
Outcome/Learning model を追加する。いずれも review 可能な小さい model であり、長期的な domain schema を確定したとは
主張しない。

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

### Phase 4 minimal model

Phase 4 は不変の Phase 1-3 State/Need/Care model の上に、retention や eviction policy を発明せず、
append-only な Relationship Memory を追加する。

| Type | Phase 4 における最小の役割 | Status |
| --- | --- | --- |
| MemoryRecord | Need を促した State と、それに応答した Care Action | PHASE 4 MINIMAL |
| GoodsMemory | 1つの good に対する Memory Record の append-only な in-process collection | PHASE 4 MINIMAL |

`GoodsMemory` はそれ自体の永続化を持たない。呼び出し元（本 phase では CLI）が所有する値であり、繰り返しの
observation をまたいで受け渡される。何も eviction・expire されず、`MemoryStore` port は空プレースホルダーの
ままで、database や file-backed adapter は実装しない。詳細は `docs/phases/phase-4-memory.md` を参照。

### Phase 5 minimal model

Phase 5 は不変の Phase 1-4 State/Need/Care/Memory model の上に、CareAction を follow-up observation
と比較する Outcome と Learning を追加する。

| Type | Phase 5 における最小の役割 | Status |
| --- | --- | --- |
| OutcomeStatus | `Resolved` または `Unresolved`。NeedKind の有無を比較する事実判定であり、Caregiver の決定を評価しない | PHASE 5 MINIMAL |
| Outcome | CareAction、follow-up の State/Need Assessment、OutcomeStatus、explanation | PHASE 5 MINIMAL |
| Learning | Outcome から導かれる reviewable な statement | PHASE 5 MINIMAL |

Learning は閾値、profile field、その他のルールを自ら調整することは決してなく、plain-language で
human-reviewable な観測を記録するだけである。それに基づいて行動するかどうかは別の human decision であり、
本 phase の対象外である。汎用の `Evidence` type は引き続き provisional placeholder のままとする——Outcome
自身の field（CareAction、follow-up の State と Need Assessment）が本 phase に必要な traceability を
既に提供している。詳細は `docs/phases/phase-5-learning.md` を参照。

### Phase 6 — Seven Day Life

Phase 6 は新しい domain type を導入しない。不変の Phase 1-5 model を、1つの Goods individual に対して
7日分の synthetic day としてスクリプト化する CLI level のマイルストーンであり、normal な日、2つの
anomaly とその Care Request・Human Feedback、それらの follow-up Verification/Learning、最終的な
Memory 集計を示す。詳細は `docs/phases/phase-6-seven-day-life.md` を参照。

### Phase 7 — Multiple Individuals

Phase 7 は新しい domain type を導入しない。不変の Phase 1-6 model を、同一 species の2つの Goods
individual に対して実行し、1つの clone された GoodsProfile（Species レベルのデータ）を共有しつつ、
それぞれ異なる `GoodsIdentity.individual_id` と独立した GoodsMemory を持たせることで、Species
レベルのデータは共有され Individual Memory は共有されないことを検証する。詳細は
`docs/phases/phase-7-multiple-individuals.md` を参照。

### Phase 8 — Multiple Goods

Phase 8 は新しい domain type を導入しない。不変の Phase 1-7 model を、意味のある異なる
GoodsProfile 値を持つ4つの product species（salmon rice ball、coffee、sandwich、bento）に対して
同一の code path で実行し、goods-domain、goods-application、goods-runtime のどこにも商品固有の
分岐が無いまま、Goods Intelligence が capability class であり、具体的な good は単なる data——
その capability の object/instance——であることを証明する。詳細は
`docs/phases/phase-8-multiple-goods.md` を参照。

### Future provisional concepts

以下は議論用 vocabulary であり、実装挙動でも確定 schema でもない。

| Candidate | Provisional role | Status |
| --- | --- | --- |
| Evidence | traceability support の候補 | PROVISIONAL |
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
