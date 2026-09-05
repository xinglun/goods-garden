# Goods Garden Phase 3 — Care Design Specification

**Status:** PROPOSED — requires human review before implementation

**Date:** 2026-09-05

**Scope:** `goods-domain`, `goods-application`, `goods-runtime`, `goods-infrastructure` and
`apps/goods-garden-cli` additions for Phase 3 (CareRequest, Caregiver, Human Feedback, CareAction).

## 背景と課題

Phase 2 は good に explainable な Need model（Deviation、Urgency、GoodsNeed、NeedConflict）を与えた。
`crates/goods-domain/src/care/{care_request.rs, care_action.rs}` は空プレースホルダーのまま残されていた。
`docs/phases/phase-3-care.md` は CareRequest、Caregiver、Human Feedback、CareAction を初めての
双方向 `Goods ↔ Human` interaction として挙げていたが、field や境界はまだ定義されていなかった。

repository 自身の governance（`AGENTS.md`）は Care authority を protected semantic area として扱う：
「AI does not own business authority」（North Star）であり、自律 action の authority を与える前には
human decision が必要である。したがって Phase 3 では good の役割を「Need に気づき、求める」ことに厳密に限定し、
人間が何を言うかを決定・推論・合成することは決してしてはならない。

## 目標

1. Phase 1/2 の State/Need model を変更せず、`goods-domain` に `CareRequest`、`Caregiver`、
   `HumanFeedback`、`CareAction` を追加する。
2. Human Feedback が常に外部入力であることを構造的に保証する：`ObservationSource` を模した
   `HumanFeedbackSource` port を導入し、`HumanFeedback` 値を得る唯一の方法を adapter 経由に限定し、
   domain や application の計算では得られないようにする。
3. `goods-application` に `RequestCare` と `ReceiveCare` を実装し、既存の `observe_and_assess` と
   `observe_and_identify_needs` を変更せずに `GoodsRuntime::request_care` を追加公開する。
4. CLI demo の同梱デフォルト実行では Need（したがって Care Request）が発生しないままにし、Phase 1
   の落ち着いた reference 体験を保つ。Care の経路は crafted な observation を使う test で検証する。

## 非目標

本提案は Memory、Learning、自律 business action、real な対話式 prompt、provider-backed Caregiver
channel（Slack、email、real staff app）、role-routing や permission model、frontend の変更を実装・許可
しない（`apps/goods-garden-web/` は無変更。AGENTS.md により Need と Care は frontend に対して明示的に対象外）。

## 決定：Care は決して決定せず、求めて記録するだけ

`CareRequest::from_assessment` は `NeedAssessment.needs` が非空なら必ず発火し、Need 群、`NeedConflict`
の有無、単一の固定 `requested_role = "store staff"`、explanation を持つ。「review してください」以上の
具体的な action は推奨しない。`CareAction::record` は `CareRequest` と `HumanFeedback` を結びつけ、その
explanation は Caregiver の決定を逐語的に言い換えるだけで、domain はその決定が正しいか十分かについて
独自の判断を加えない。

## 決定：Human Feedback は port 形の外部入力であり、計算されない

```rust
pub trait HumanFeedbackSource {
    type Error;
    fn provide_feedback(&self, request: &CareRequest) -> Result<HumanFeedback, Self::Error>;
}
```

本 phase での唯一の実装は `DemoHumanFeedbackSource`、`DemoObservationSource` を模した local synthetic
fixture adapter である。これにより、domain/application layer が自ら決定を発明することを決して許さないまま、
将来の明示的に review された対話式または provider-backed adapter への architectural な道を残す。

## 決定：単一の固定 Caregiver role、routing なし

`Caregiver.role` と `CareRequest.requested_role` は、閉じた enum ではなく既存の
`GoodsIdentity.species`/`Observation.source` と同じ流儀の plain `String` 値とする。Phase 3 は常に
`"store staff"` のみを生成し、role-matching、escalation-routing、authorization model は実装しない。
Phase 1 の単一 product profile、Phase 2 の2つの固定次元と同じ単一-role の minimalism に一致する。

## 検討した代替案

**demo での real な interactive stdin prompt**：本 phase では不採用。「最初の双方向 interaction」をより
文字通りにするが、`cargo test`/CI を non-deterministic にし、Phase 1/2 で確立した fixture-based pattern
を破る。port 抽象化により、この選択肢は後続の別途 review された adapter として残る。

**NeedKind による CareRequest の routing**（例：`FreshnessConcern` と `StockAvailabilityConcern` で
異なる role）：時期尚早として不採用。この synthetic な単一店舗 context には role を区別する evidence が
なく、それを発明することは Trust Model が禁じる fabricated business rule そのものである。

**`ReceiveCare` に Human Feedback を評価・採点させる**（例："resolved"/"unresolved" と marking する）：
不採用。domain が human decision を判定することになり、「AI does not own business authority」を侵す。

## Known、inference、unknown

- **KNOWN：** `CareRequest`/`CareAction` は本 phase 用に予約された空プレースホルダーであり、
  Phase 1/2 の State/Need type は本提案の影響を受けない（今回は共有 struct への新規 field 追加なし）。
- **KNOWN：** test 以外で `HumanFeedback` を構築する唯一の方法は `HumanFeedbackSource::provide_feedback`
  経由であり、demo はこれを stdin や real system ではなく synthetic fixture で満たす。
- **INFERRED：** 単一の `"store staff"` Caregiver role で、根拠のない permission model を発明せずに
  Goods↔Human 境界を示すには十分である。
- **UNKNOWN：** real な店舗 staffing role、escalation policy、real な Caregiver 通知 channel。これらは
  Trust Model に従い `UNKNOWN`/対象外のままとする。
- **UNAVAILABLE：** この repository 内の real external Caregiver system や human response data。

## Review gate と acceptance

この文書は `PROPOSED` である。承認では次を確認する。

1. CareRequest が識別された全ての Need に対して発火し（重大度 threshold なし）、特定の action を
   推奨しないこと。
2. Human Feedback が `HumanFeedbackSource` port 経由でのみ得られ、本 phase では synthetic fixture
   adapter のみであること。
3. 単一の固定 `"store staff"` Caregiver role が Phase 3 として妥当であること。
4. `state::goods_state`、Phase 2 Need model、frontend が無変更のままであること。
