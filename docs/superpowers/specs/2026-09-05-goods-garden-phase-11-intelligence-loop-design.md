# Goods Garden Phase 11 — Intelligence Loop Design Specification

**Status:** PROPOSED — requires human review before implementation

**Date:** 2026-09-05

**Scope:** `goods-runtime` and `apps/goods-garden-cli` additions for
Phase 11 (IntelligenceCycleOutcome, GoodsRuntime::run_cycle).

## 背景と課題

`crates/goods-runtime/src/intelligence_loop/mod.rs` には Phase 0 由来の
空プレースホルダー（`//! Intelligence Loop orchestration boundary
placeholder.` というコメントのみ）が未実装のまま残っていた。North Star の
Fundamental loop（State → Need → Care → Action → New State →
Memory/Learning）は Phase 1-10 を通じて `GoodsRuntime` の5つのメソッドと
して実装済みだが、「1周分のサイクル」という概念自体には一度も名前が
付いていない。`apps/goods-garden-cli` の `run_seven_day_life` は、この
1周分のサイクル（前日の Care Action を検証してから今日の Need を確認する）
を手作業で毎日オーケストレーションしている。

## 目標

1. `IntelligenceCycleOutcome` と `GoodsRuntime::run_cycle` を実装する。
2. `run_seven_day_life` だけを `run_cycle` を使うよう書き換え、CLI 出力を
   完全に維持する。

## 非目標

本提案は `crates/goods-runtime/src/scheduler/mod.rs`（Intelligence Loop
を自動的に駆動する自律実行の話）の実装、`run_multiple_individuals`/
`run_multiple_goods` のオーケストレーション変更、`goods-domain`/
`goods-application`/`goods-infrastructure` の変更、persistence、
frontend の変更のいずれも実装・許可しない。

## 決定：run_cycle は既存2メソッドの組み合わせに過ぎない

`run_cycle` は `verify_and_learn`（`pending_action` がある場合のみ）と
`request_care_and_remember` を順番に呼ぶだけであり、新しい判断や
business rule を一切追加しない。既存の5つの `GoodsRuntime` メソッドは
すべて変更しない。

## 決定：呼び出し元は run_seven_day_life のみ変更する

`run_cycle` を実際に使う呼び出し元は `run_seven_day_life` の1箇所だけで
ある。`run_multiple_individuals`/`run_multiple_goods` は verify ステップを
持たない単発処理であり、`run_cycle` を使っても複雑さは減らないため、
意図的に `request_care_and_remember` の直接呼び出しのままとする。この
work item の主な価値はコード重複の削減ではなく、North Star が名指す
「Intelligence Loop」という概念に、既存の空 placeholder を使って初めて
具体的な型とメソッドを与えることにある。

## 決定：scheduler は対象外

`scheduler` placeholder（Intelligence Loop を「いつ」自動で駆動するかと
いう自律実行の話）は、North Star の「Human authority decides whether
any action may be autonomous」に直接触れる別の human decision が必要な
領域であるため、本 work item では一切変更しない。repository owner にも
設計相談の段階で確認済みである。

## 検討した代替案

**`run_multiple_individuals`/`run_multiple_goods` も `run_cycle` を
使うよう統一する**：不採用。呼び出し元を無理に統一するよりも、実際に
verify ステップを必要とする箇所だけに限定する方が、リポジトリの
minimalism の方針（必要以上の抽象化を避ける）に合う。

**`run_cycle` を `goods-application` の新しい use case として実装する**：
不採用。`run_cycle` は既存の `GoodsRuntime` メソッド2つを呼ぶだけの
オーケストレーションであり、新しい application-layer use case を発明する
必要はない。

## Known、inference、unknown

- **KNOWN：** `intelligence_loop` placeholder は Phase 0 以来未実装のまま
  repository 内に存在していた（本 spec 執筆前に確認済み）。
- **KNOWN：** `run_cycle` を実際に使う呼び出し元は `run_seven_day_life`
  の1箇所だけである。これは率直なトレードオフとして記録する：
  コード重複の削減という観点では正当化しにくいが、North Star の
  概念に型を与えるという観点では価値がある。
- **INFERRED：** 該当なし。
- **UNKNOWN：** `scheduler` が将来実装されるべきかどうか、Intelligence
  Loop の自動駆動が許可される条件。これらは別途の human decision が
  行われるまで `UNKNOWN` のままとする。
- **UNAVAILABLE：** この repository 内の real な自動運用データ。

## Review gate と acceptance

この文書は `PROPOSED` である。承認では次を確認する。

1. `run_cycle` が既存の `verify_and_learn`/`request_care_and_remember`
   の組み合わせに過ぎず、新しい business rule を追加しないこと。
2. `run_seven_day_life` の CLI 出力が refactor 前後で完全に一致すること。
3. `scheduler`、`run_multiple_individuals`/`run_multiple_goods`、
   `goods-domain`/`goods-application`/`goods-infrastructure`、frontend が
   無変更のままであること。
