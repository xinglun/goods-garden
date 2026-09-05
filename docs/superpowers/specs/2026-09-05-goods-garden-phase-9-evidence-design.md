# Goods Garden Phase 9 — Evidence Design Specification

**Status:** PROPOSED — requires human review before implementation

**Date:** 2026-09-05

**Scope:** `goods-domain` and `apps/goods-garden-cli` additions for Phase 9
(Evidence, InformationState).

## 背景と課題

`crates/goods-domain/src/evidence/evidence.rs` には Phase 0 由来の空
プレースホルダー `pub struct Evidence;` が未実装のまま残っていた。
`docs/architecture/trust-model.md` はすでに information state の語彙
（`KNOWN | INFERRED | UNKNOWN | UNAVAILABLE | CONFLICTING`）を定義して
いるが、実際に使われたことは一度もない。一方で、Phase 1-5 が追加した7つの
struct（`HealthAssessment`、`Deviation`、`NeedConflict`、`GoodsNeed`、
`CareRequest`、`CareAction`、`Outcome`）はすべて素の `explanation: String`
を持ち、その文言が測定された事実なのか、閾値ルールに基づく解釈なのかを
型として区別する手段がなかった。

`AGENTS.md` は Trust/Evidence semantics の変更を human decision が必要な
protected semantic area として扱う。そのため本 work item は、実装に入る前に
repository owner と2つの決定を確認済みである（下記 Decision 参照）。

## 目標

1. `goods-domain` に `Evidence`/`InformationState` を実装する
   （`crates/goods-domain/src/evidence/evidence.rs` のプレースホルダーを
   置換）。
2. 既存7箇所の `explanation: String` を `evidence: Evidence` に置き換える。
   文言は変えず、tag だけを付与する。
3. `GoodsNeed` の evidence だけを `Inferred` にする。他の6箇所は `Known`
   とする。
4. CLI（`demo`/`seven-day-life`/`multiple-individuals`/`multiple-goods`）の
   出力が `evidence` の `Display` 経由で `[KNOWN]`/`[INFERRED]` タグを
   自動的に表示するようにする。

## 非目標

本提案は `Evidence::unknown()`/`unavailable()`/`conflicting()` の便利
コンストラクタ（現状どの呼び出し元も必要としない）、`learning::Learning.statement`
への同様の変更、`goods-application`/`goods-runtime`/`goods-infrastructure`
の変更、persistence、自律的な rule 調整、frontend の変更のいずれも実装・
許可しない。

## 決定：置換であり追加ではない

7箇所すべてで `explanation: String` field 自体を `evidence: Evidence` に
置き換える。追加 field にはしない。追加 field にすると、素の String が
「本当の」field であり続け、`Evidence` が単なる飾りになってしまう。
repository owner にこの2択（置換 vs 追加）を確認し、置換が選ばれた。

## 決定：Phase 1 の凍結契約をあえて破る

`HealthAssessment`（`state::goods_state`）もこの置換対象に含める。
Phase 2-8 の設計書はすべて「`state::goods_state` は変更しない」と明記して
きたが、本 work item はこれを意図的な例外として崩す。これにより
`apps/goods-garden-cli/tests/phase_1_demo.rs` の
`state.health.explanation.contains("exceeds")` という assertion は
`state.health.evidence.statement.contains("exceeds")` に書き換わる。
repository owner にはこの結果（Phase 1 の frozen contract が崩れること、
既存 assertion の書き換えが必要なこと）を明示した上で確認し、それでも
`HealthAssessment` を含めることが選ばれた。

## 決定：GoodsNeed だけが Inferred

`Urgency::from_freshness_magnitude`/`from_stock_magnitude` の doc comment は
すでに「Fixed, example-only thresholds... Not derived from real POS/SEJ
data」と明記している。`GoodsNeed` の evidence はこの hypothesis 閾値に
基づく Urgency 分類に依存するため、`Deviation`（決定的な算術）とは異なり
`Known` ではなく `Inferred` が正しい。他の6箇所（`HealthAssessment`、
`Deviation`、`NeedConflict`、`CareRequest`、`CareAction`、`Outcome`）は
いずれも決定的な計算または記録済みの事実の言い換えであり、根拠のない
不確実性を作り出さないために `Known` のままとする。

## 検討した代替案

**7箇所すべてに追加 field として `evidence: Option<Evidence>` を持たせる**：
不採用。既存の `explanation: String` が実質的な情報源であり続け、Evidence
が実装されたことの意味が薄れる。

**`Learning.statement` も Evidence に置き換える**：不採用。Learning は
Outcome から導かれる reviewable な statement であり、Phase 5 の non-goal
（閾値やルールを自ら調整しない）とは別の関心事である。本 work item の
scope を7箇所の `explanation` field に限定し、scope creep を避ける。

**`GoodsNeed` 以外にも Inferred を割り当てる（例：`NeedConflict`）**：
不採用。`NeedConflict` は既に固定された `NeedKind` の組み合わせに対する
構造的・決定的な事実の提示であり、曖昧なデータからの推測ではない。
根拠のない不確実性を作らないという Trust Model の原則に従い、実際に
hypothesis 閾値ルールに依存する `GoodsNeed` のみを Inferred とする。

## Known、inference、unknown

- **KNOWN：** `Evidence` placeholder は Phase 0 以来未実装のまま
  repository 内に存在していた（本 spec 執筆前に grep で確認済み）。
  `trust-model.md` の information state 語彙は Phase 0 から変更されて
  いない。
- **KNOWN：** 7箇所の `explanation` field 以外に `.explanation` を参照する
  箇所は `apps/goods-garden-cli/src/main.rs` の CLI 出力と、
  `phase_1_demo.rs`/`phase_3_care.rs` の3つの assertion のみである
  （grep で確認済み）。
- **INFERRED：** `GoodsNeed` の evidence を `Inferred` とする判断は、
  `Urgency` の閾値がすでに `example`/`hypothesis` と文書化されている
  ことからの合理的な帰結であり、恣意的な区別ではない。
- **UNKNOWN：** 将来 `Learning.statement` や `unknown()`/`unavailable()`/
  `conflicting()` の各状態を実際に使う具体的なユースケースがいつ生まれる
  かは、後続の evidence を伴う Work Item が扱うまで `UNKNOWN` のままと
  する。
- **UNAVAILABLE：** この repository 内の real external な evidence
  provenance データ（本当に POS/SEJ から来た値かどうかを示す real
  metadata）。

## Review gate と acceptance

この文書は `PROPOSED` である。承認では次を確認する。

1. 7箇所すべてで `explanation: String` が `evidence: Evidence` に
   置き換わり、追加 field にはなっていないこと。
2. `HealthAssessment` を含め、Phase 1 の frozen contract を崩す例外が
   明示的に記録されていること。
3. `GoodsNeed` だけが `Inferred` であり、他の6箇所が `Known` であること、
   その理由が `Urgency` の hypothesis 閾値に基づいていること。
4. `Learning.statement`、新しい domain type（Evidence/InformationState
   以外）、frontend が無変更のままであること。
