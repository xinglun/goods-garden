# Domain Model

## English

The following candidates are provisional. They are vocabulary for discussion,
not settled schemas or behavior.

| Candidate | Provisional role | Status |
| --- | --- | --- |
| Goods | Aggregate candidate for an individual product | PROVISIONAL |
| GoodsIdentity | Identity value candidate | PROVISIONAL |
| GoodsProfile | Stable descriptive profile candidate | PROVISIONAL |
| GoodsState | Current-state representation candidate | PROVISIONAL |
| GoodsNeed | Need representation candidate | PROVISIONAL |
| Observation | Sensory input candidate | PROVISIONAL |
| Evidence | Traceability support candidate | PROVISIONAL |
| CareRequest | Request-for-care candidate | PROVISIONAL |
| CareAction | Care record candidate | PROVISIONAL |
| Outcome | Result candidate; no Phase 0 type yet | PROVISIONAL |
| Memory | Relationship/history candidate | PROVISIONAL |
| Learning | Learning-result candidate | PROVISIONAL |
| LifecycleState | Lifecycle candidate | PROVISIONAL |

No candidate receives fields in Phase 0. A later Domain Design Work Item must
confirm aggregate boundaries, invariants, identity, evidence lineage and
authority semantics.

## 日本語

以下は議論用の候補であり、確定した schema や挙動ではない。全ての Status は
`PROVISIONAL`。Phase 0 では field を与えない。

候補は `Goods`（individual product の aggregate 候補）、`GoodsIdentity`、`GoodsProfile`、
`GoodsState`、`GoodsNeed`、`Observation`、`Evidence`、`CareRequest`、`CareAction`、`Outcome`、
`Memory`、`Learning`、`LifecycleState`。後続の Domain Design Work Item で aggregate boundary、
invariant、identity、evidence lineage、authority semantics を確認する。

## 中文

以下是讨论用候选，不是已确定的 schema 或行为，全部标记为 `PROVISIONAL`。Phase 0 不为候选增加字段。

候选包括 `Goods`（单个商品的 Aggregate 候选）、`GoodsIdentity`、`GoodsProfile`、`GoodsState`、
`GoodsNeed`、`Observation`、`Evidence`、`CareRequest`、`CareAction`、`Outcome`、`Memory`、
`Learning`、`LifecycleState`。后续 Domain Design Work Item 必须确认 Aggregate 边界、不变量、身份、证据链和权限语义。
