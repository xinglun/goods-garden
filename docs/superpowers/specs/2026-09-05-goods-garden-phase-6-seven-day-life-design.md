# Goods Garden Phase 6 — Seven Day Life Design Specification

**Status:** PROPOSED — requires human review before implementation

**Date:** 2026-09-05

**Scope:** `apps/goods-garden-cli` only. No `goods-domain`/`goods-application`/`goods-runtime`
changes.

## 背景と課題

Phase 2-5 は `State → Need → Care → Memory → Verification → Learning` の
model を一通り構築したが、いずれも単一 observation の demo と test でしか検証されていなかった。
`docs/phases/phase-6-seven-day-life.md` は「一つのおにぎりが7日間 live する」という product
milestone を挙げ、normal、anomaly、investigation、request for care、human feedback、
improvement、verification、memory を1つの一貫した narrative で示すことを求めている。Phase 2-5 と
異なり、この milestone は新しい domain semantics を必要としない：既存の、既に review された rule を
CLI level で orchestration するだけである。

## 目標

1. 既存の `goods-domain`/`goods-application`/`goods-runtime` の型だけを使い、1つの Goods
   individual に対して7日分の synthetic day をスクリプト化し、新しい `seven-day-life` CLI
   subcommand として公開する。
2. 少なくとも2つの anomaly day を含め、それぞれが Need、synthetic Human Feedback で応答される
   Care Request、続く verification day（Care Action が Need を解決したことを示す）を発生させる。
3. 既存の `demo` subcommand の挙動と出力は完全に変更しない。
4. Phase 1 が確立した labelling 規約に合わせ、すべての値を明確に synthetic かつ explainable に保つ。

## 非目標

本提案は新しい domain type、自動的な rule/threshold 調整、Multiple Individuals（Phase 7）、
Multiple Goods（Phase 8）、frontend の変更のいずれも実装・許可しない。

## 決定：「1日」とは同じ監視対象スロットの restock 後 observation である

文字通り単一のおにぎりは、Phase 1 自身の freshness rule の下では1週間安全に age できない。代わりに
good の `GoodsIdentity` は監視対象の小売スロットを表す：各日の `age_hours`/`quantity_on_hand` は
その日に店員が observe するであろう値を反映し、restock（Human-Feedback 駆動の Care Action として
記録される）が後の日で `age_hours` をリセットする。これは Phase 1 が既に定義した「Individual」概念と
同じであり、新しい type を必要としない。

## 決定：独立した2つの anomaly、Need 次元ごとに1つ

Day 3 は Freshness 次元（age が expectation を超過）、Day 6 は StockAvailability 次元
（quantity が minimum を下回る）を扱う。Phase 2 の両次元をそれぞれ1回ずつ使うことで、3つ目の次元を
発明することなく既存 Need model の全幅を示し、各 anomaly の原因と解決を独立して追いやすく保つ。

## 決定：`seven-day-life` は `demo` への変更ではなく独立した subcommand とする

`apps/goods-garden-cli/tests/phase_1_demo.rs` は既存の `demo` subcommand の出力を assert している。
この frozen contract に触れる代わりに、本 phase は新しい additive な subcommand と、それ専用の test
file を追加する。

## 検討した代替案

**7日分を外部の複数日 fixture file から駆動する**：不採用。一度限りの milestone script のためだけに
新しい fixture text format と parser を発明することになる一方、Phase 2/3 の test は既に、
スクリプト化された scenario のために `Observation`/`HumanFeedback` 値を Rust 内で直接構築するという
前例を確立している。

**`demo` subcommand を単一日の story ではなく7日間の story を示すものに作り替える**：不採用。
必要のない理由で Phase 1 の frozen test contract を壊すことになる。

## Known、inference、unknown

- **KNOWN：** 7日間で生成される全ての Need/Care/Outcome/Learning 値は不変の Phase 2-5 rule に
  由来する。本 phase は新しい比較や決定 logic を一切追加しない。
- **INFERRED：** 2つの anomaly（既存 Need 次元ごとに1つ）は、scope を広げることなく phase document
  の checklist を満たすのに十分である。
- **UNKNOWN/UNAVAILABLE：** real な日々の店舗データ；script の具体的な数値は `example`/`synthetic`
  のみである。

## Review gate と acceptance

この文書は `PROPOSED` である。承認では次を確認する。

1. 新しい domain type を導入せず、7日間の script は Phase 2-5 の既存 rule への input を供給する
   だけであること。
2. 既存の `demo` subcommand が変更されないままであること。
3. 2つの anomaly（Freshness、StockAvailability）とその verification day が、本 milestone の
   妥当な最小 script であること。
