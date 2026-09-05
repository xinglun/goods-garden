# Goods Garden Phase 10 — Lifecycle Design Specification

**Status:** PROPOSED — requires human review before implementation

**Date:** 2026-09-05

**Scope:** `goods-domain` and `apps/goods-garden-cli` additions for Phase 10
(LifecycleState).

## 背景と課題

`crates/goods-domain/src/lifecycle/lifecycle_state.rs` には Phase 0 由来の
空プレースホルダー `pub struct LifecycleState;` が未実装のまま残っていた。
Phase 9（Evidence）の完了により、これが `docs/architecture/domain-model.md`
の Future provisional concepts 表に残る最後の候補となった。一方、
`crates/goods-domain/**` は `AGENTS.md` が domain semantics の変更に human
decision を要求する protected semantic area であり、`LifecycleState` には
`Evidence` の trust-model.md のような既存の確立された語彙が一切なかった
ため、実装前に repository owner と意味論そのものを確認する必要があった。

## 目標

1. `LifecycleState`（`Active`/`Retired` の2値）を実装する。
2. `Goods` に `lifecycle: LifecycleState` field を追加し、`Goods::new` は
   引数を変えず内部で `Active` を既定値とする。
3. `Goods::retire(&self) -> Self` を実装し、これを唯一の遷移手段とする。
4. CLI 4 subcommand が `lifecycle: active`/`lifecycle: retired` を出力する
   ようにする。

## 非目標

本提案は `goods-runtime` の `intelligence_loop`/`scheduler` placeholder の
実装、`GoodsRuntime` の既存メソッドに Retired な Goods への挙動制約を
課すこと、`goods-application`/`goods-infrastructure` の変更、persistence、
frontend の変更のいずれも実装・許可しない。

## 決定：Active/Retired の2値のみ

`LifecycleState` は「Goods Garden で監視されているか（Active）、既に
監視対象外になったか（Retired）」を示す最小の2値とする。repository owner
に、より多段階のライフサイクル（例：入荷→陳列→販売→廃棄）を導入する
代替案も提示した上で確認し、2値が選ばれた。多段階の遷移は、real な
店舗オペレーションの evidence なしに発明することになり、Trust Model の
「Unknown over fabrication」に反する。

## 決定：遷移は明示的な呼び出しのみ

`Goods::retire(&self) -> Self` は `identity`/`profile` を変えず
`lifecycle` だけを変更した新しい値を返す非破壊的な method である。
`CareAction`/`HumanFeedback` の自由テキスト（例：「Reviewed and pulled the
item from the shelf.」）から domain がこの遷移を推論することは一切ない。
これは Phase 3 が確立した「domain は Human Feedback の内容を発明・推論・
合成しない」という既存原則をそのまま踏襲したものである。

## 決定：Retired な good への runtime 制約は課さない

`GoodsRuntime` の既存メソッド（`observe_and_assess` 等）は、Retired な
Goods に対しても現状どおり動作させる。「Retired の good を観測しようと
したらエラーにすべきか」は real な店舗オペレーションの evidence が
必要な業務ルールであり、本 phase では意図的に `UNKNOWN` のままとする。

## 検討した代替案

**多段階のライフサイクル（入荷/陳列/販売/廃棄等）を導入する**：不採用。
real な店舗オペレーションのライフサイクル定義の evidence がなく、発明する
ことになる。

**`CareAction` の内容（例：「pulled from the shelf」という文言）から
自動的に `retire()` を呼ぶ**：不採用。domain が human decision の自由
テキストを解釈・推論することになり、「AI does not own business authority」
および Phase 3 の既存原則に反する。

**Retired な Goods に対して `GoodsRuntime` のメソッドをエラーにする**：
不採用。real な業務要件の evidence なしに新しい制約を発明することになる。
`UNKNOWN` のままとし、後続の evidence を伴う Work Item に委ねる。

## Known、inference、unknown

- **KNOWN：** `LifecycleState` placeholder は Phase 0 以来未実装のまま
  repository 内に存在していた（本 spec 執筆前に grep で確認済み）。
- **KNOWN：** `Goods {` struct literal を直接使う箇所は `entity.rs` 自身
  のみであり、他のすべての呼び出しは `Goods::new(..)` 経由である
  （grep で確認済み）。そのため `lifecycle` field の追加は非破壊的である。
- **INFERRED：** 該当なし（本 phase は決定的なデータ field の追加のみで
  あり、推論を伴う判断は存在しない）。
- **UNKNOWN：** real な店舗が実際に商品のライフサイクルをどう管理するか、
  Retired な good を `GoodsRuntime` がどう扱うべきか。これらは後続の
  evidence を伴う Work Item が扱うまで `UNKNOWN` のままとする。
- **UNAVAILABLE：** この repository 内の real external なライフサイクル
  管理データ。

## Review gate と acceptance

この文書は `PROPOSED` である。承認では次を確認する。

1. `LifecycleState` が `Active`/`Retired` の2値のみであること。
2. `Goods::retire()` が非破壊的であり、`CareAction`/`HumanFeedback` の
   内容から遷移を推論しないこと。
3. `GoodsRuntime` の既存メソッド、`intelligence_loop`/`scheduler`
   placeholder、frontend が無変更のままであること。
