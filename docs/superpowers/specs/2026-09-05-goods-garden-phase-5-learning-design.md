# Goods Garden Phase 5 — Verification & Learning Design Specification

**Status:** PROPOSED — requires human review before implementation

**Date:** 2026-09-05

**Scope:** `goods-domain`, `goods-application`, `goods-runtime` and `apps/goods-garden-cli`
additions for Phase 5 (Outcome, OutcomeStatus, Learning).

## 背景と課題

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

## 目標

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

## 非目標

本提案は `Urgency` 閾値、`GoodsProfile` field、Care role routing、その他ルールのいかなる自動調整も、
汎用 `Evidence` type も、persistence も、Phase 6 の複数日（「Seven Day Life」）loop も、frontend の
変更（`apps/goods-garden-web/` は無変更）も実装・許可しない。

## 決定：Outcome は事実比較であり、採点ではない

`Outcome::verify(action, new_state, new_needs)` は、元の `CareRequest` が対象とした `NeedKind` を
集め、それらのいずれかが `new_needs` にまだ現れるかを確認する。`OutcomeStatus::Resolved` は1つも
残っていないこと、`OutcomeStatus::Unresolved` は少なくとも1つ残っていることを意味する。これは Phase 2
が既に計算しているデータ（`NeedKind` の identity）から完全に導出されるため、新しい根拠のない business
rule は導入しない——Phase 1 が healthy/unhealthy に使ったのと同種の、bounded で explainable な比較である。

## 決定：Learning は自らルールを調整しない

`Learning { outcome, statement }` は他の field を持たず、`GoodsProfile`、`Urgency` 閾値、その他の
domain rule に書き戻す method も持たない。その `statement` は明示的に「これは reviewable な観測であり、
ルール変更ではない」と述べる。これは `AGENTS.md` の learning-policy 保護を踏まえ、Phase 5 の正しい
scope として repository owner と確認済みである：将来の human adoption 向けに「provisional adjustment」
field を残す代替案も検討したが、直近の必要性なしに scope とリスクを広げるものとして明示的に不採用とした。

## 決定：verification は既存の ObservationSource port を再利用する

`GoodsRuntime::verify_and_learn(goods, action)` は `self.source`（他の method と同じ
`ObservationSource`）を通じて再度 observe し、内部で `observe_and_identify_needs` を呼ぶ。
「follow-up」observation とは単に別の `ObservationSource` 値（例えば test での2つ目の
`DemoObservationSource::new(..)`）であり、新しい port は導入せず、「時間の経過」という概念もそれ以上には
モデル化しない。

## 検討した代替案

**Learning に具体的な閾値/パラメータ変更を提案させる**（「provisional、未適用」マーカー付き）：不採用。
未適用であっても、具体的な提案値は根拠のない rule の発明に近く、後で本当の evidence なしに authoritative
なものとして扱われるリスクがある。repository owner は plain-statement のみの scope を確認済みである。

**複数 episode をまたいで Learning を trend/score に集約する**：時期尚早として不採用。Phase 6
（「Seven Day Life」）が good を本当の複数日 loop に通す最初の phase であり、ここでの集約は
evidence が少なすぎる土台の上に構築されることになる。

**今 汎用の `Evidence` type を導入する**：不採用。`Outcome` 自身の field（どの CareAction、どの
follow-up State/Need Assessment）が本 phase の必要とする traceability を既に完全に提供しており、
抽象的な wrapper の追加は時期尚早である。

## Known、inference、unknown

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

## Review gate と acceptance

この文書は `PROPOSED` である。承認では次を確認する。

1. Outcome が Caregiver の決定を定性的に採点するものではなく、NeedKind の有無を比較する事実判定である
   こと。
2. Learning が単なる reviewable な statement であり、閾値、profile 値、その他ルールを調整する field
   や method を持たないこと。
3. `state::goods_state`、Phase 2 Need model、Phase 3 Care model、Phase 4 Memory model、frontend が
   無変更のままであること。
