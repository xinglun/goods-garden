# Goods Garden Phase 2 — Need Design Specification

**Status:** PROPOSED — requires human review before implementation

**Date:** 2026-09-05

**Scope:** `goods-domain`, `goods-application`, `goods-runtime`, `goods-infrastructure` and
`apps/goods-garden-cli` additions for Phase 2 (Deviation, Urgency, GoodsNeed, Need Conflict).

## 背景と課題

Phase 1 は1つの synthetic reference good に bounded な healthy/unhealthy State を与えた。
`crates/goods-domain/src/need/goods_need.rs` と
`crates/goods-application/src/use_cases/identify_need.rs` は次 phase 用の空プレースホルダーとして
残されていた。`docs/phases/phase-2-need.md` は Deviation、Need、Urgency、Need Conflict という語を
挙げていたが、field や境界はまだ定義されていなかった。

単一の評価次元（Freshness/age）だけでは、Freshness の Need が矛盾する相手が存在しないため、実際の
Need Conflict を発生させられない。本提案は StockAvailability という追加の評価次元を導入し、2つの
Need が共存し相反する方向を示唆できるようにする。

## 目標

1. `state::goods_state`（Phase 1 の archived contract）を変更せず、`goods-domain` に `Deviation`、
   `Urgency`、`GoodsNeed`、`NeedConflict` を追加する。
2. 抽象的なモデルにとどまらず実際に Need Conflict が発生するよう、独立した第2の評価次元
   （StockAvailability）を追加する。
3. `goods-application` に `IdentifyNeed` を実装し、既存の `observe_and_assess` を変更せずに
   `GoodsRuntime` から公開する。
4. CLI demo の同梱デフォルト実行では Need が発生しないままにし、Phase 1 の落ち着いた reference
   体験を保つ。Need/Conflict の経路は crafted な observation を使う test で検証する。
5. Need の語彙を Care の動詞ではなく懸念/状態の記述として命名し、Phase 2 が Phase 3 の Care 境界を
   先取りしないようにする。

## 非目標

本提案は Care、CareRequest、CareAction、Memory、Learning、自律 business action、real POS/SEJ
data、API server、database、frontend の変更を実装・許可しない
（`apps/goods-garden-web/` は無変更。AGENTS.md により Need は frontend に対して明示的に対象外）。

## 決定：次元は2つ、1つではない

`Observation` に `quantity_on_hand: u32`、`GoodsProfile` に `minimum_stock_quantity: u32` を追加する。
`Deviation::freshness` は既存の `Expectation` を再利用し（`age_hours - max_age_hours`）、
`Deviation::stock_availability` は新設する（`minimum_stock_quantity - quantity_on_hand`）。
どちらの次元でも magnitude が正なら懸念ありとし、懸念ある Deviation はそれぞれ `NeedKind` を持つ
1つの `GoodsNeed` になる。

```rust
pub enum NeedKind { FreshnessConcern, StockAvailabilityConcern }
```

`NeedConflict::detect` は同じ `NeedAssessment` の中に両方の kind が存在する場合にのみ発火し、
矛盾（棚から下げる vs 棚に置き続ける）を説明する。解決はせず、どちらの行動も推奨しない。

## 決定：Urgency の閾値は固定の example 値

```text
Freshness（max_age_hours超過の時間）:       1-2=Low, 3-5=Medium, 6以上=High
StockAvailability（minimum不足の個数）:     1=Low, 2-3=Medium, 4以上=High
```

これらは Trust Model（`docs/architecture/trust-model.md`）の意味での `example`/`hypothesis` 値であり、
real POS、SEJ、inventory data から導いたものではない。後続の evidence を伴う Work Item が置き換えてよい。

## 決定：NeedKind は懸念を名付け、Care action を名付けない

`FreshnessConcern`/`StockAvailabilityConcern` は、動詞形の代替案（例:
`ReduceForFreshness`/`ReplenishForAvailability`）よりも採用された。Need の名前が推奨対応ではなく
懸念状態を記述するようにし、`AGENTS.md` の Need/Care 境界を明示的に保つため。

## 検討した代替案

**Need Conflict を後続 phase に先送りする**案は、`docs/phases/phase-2-need.md` が Phase 2 の scope として
明示的に Need Conflict を挙げており、単一次元モデルでは意味のある検証ができないため不採用。

**動的に次元を登録できる汎用 `Vec<Deviation>`**案は、過剰な抽象化として不採用。具体的で名前の付いた
2つの次元があれば、投機的な拡張性なしに実際の conflict を実証・検証するには十分である。

**動詞形の `NeedKind` variant** は上記の命名決定により不採用。

## Known、inference、unknown

- **KNOWN：** `GoodsNeed` と `IdentifyNeed` は本 phase 用に予約された空プレースホルダーであり、
  `state::goods_state` は Phase 1 の archived かつ frozen な contract である。
- **KNOWN：** `Observation`/`GoodsProfile` の struct literal 構築箇所は repository 内にちょうど5箇所
  （simulator adapter、CLI、`phase_1_demo.rs` 内の3箇所）のみであり、必須 field の追加は compiler が
  網羅性を保証する、完全に監査可能な変更である。
- **INFERRED：** stock-availability 次元は、コンビニ商品的な good にとって妥当な第2の Need 源であり、
  POS 固有の挙動を創作せずに freshness と availability の現実的な緊張関係を示せる。
- **UNKNOWN：** 実際の店舗が Urgency に使う閾値、実際の minimum stock policy。どちらも Trust Model に
  従い example/hypothesis のままとする。
- **UNAVAILABLE：** この repository 内の real external operational data。

## Review gate と acceptance

この文書は `PROPOSED` である。承認では次を確認する。

1. `quantity_on_hand`/`minimum_stock_quantity` を第2次元として追加し、Phase 2 で Need Conflict を
   具体化する方法が妥当であること。
2. 固定の Urgency 閾値が example/hypothesis 値として妥当であること。
3. `NeedKind::FreshnessConcern`/`StockAvailabilityConcern`（懸念形の命名）が Care 動詞形の代替案より
   妥当であること。
4. `state::goods_state` と frontend が無変更のままであること。
