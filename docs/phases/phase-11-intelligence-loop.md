# Phase 11 — Intelligence Loop

## Goal

North Star が名指す Intelligence Loop（State → Need → Care → Action →
New State → Memory/Learning）の「1周分のサイクル」に、初めて具体的な型と
メソッドを与える。Phase 0 由来の空プレースホルダー
`crates/goods-runtime/src/intelligence_loop/mod.rs` を実装し、
`apps/goods-garden-cli` の Seven Day Life milestone が手作業で毎日
繰り返していたオーケストレーション（前日の Care Action を検証してから
今日の Need を確認する、という2ステップ）を `GoodsRuntime::run_cycle` と
して形式化する。

## Implemented boundary

Phase 11 に含めるのは次だけである。

- `IntelligenceCycleOutcome`：1周分のサイクルの結果（`state`、`needs`、
  `request`、`action`、`verification`）をまとめた struct。
- `GoodsRuntime::run_cycle`：`pending_action` が `Some` なら
  `verify_and_learn` を呼び、続けて `request_care_and_remember` を呼ぶ。
  これは既存の2つのメソッドの組み合わせに過ぎず、新しい business rule は
  一切追加しない。
- `apps/goods-garden-cli` の `run_seven_day_life` だけを `run_cycle` を
  使うよう書き換える。出力文字列・順序は一切変えない。

`run_multiple_individuals`/`run_multiple_goods` は変更しない——これらは
verify ステップを持たない単発処理であり、`run_cycle` を使っても複雑さは
減らないため、意図的に `request_care_and_remember` の直接呼び出しの
ままとする。

`crates/goods-runtime/src/scheduler/mod.rs`（Intelligence Loop を
「いつ」自動で駆動するかという自律実行の話）は対象外のままである。これは
North Star の「Human authority decides whether any action may be
autonomous」に直接触れる別の human decision が必要な領域であり、
repository owner にも確認済みである。

## Local demo と data boundary

repository root から demo を実行する。

```bash
cargo run -p goods-garden-cli -- seven-day-life
```

出力は本 phase の前後で完全に同一である（`run_cycle` への置き換えは
内部のオーケストレーションのみを変更し、印字される文字列・順序は
変更しない）。既存の `demo`/`multiple-individuals`/`multiple-goods`
subcommand は無変更のままである。

## Exit criteria

- `GoodsRuntime::run_cycle` が既存の `verify_and_learn`/
  `request_care_and_remember` の組み合わせに過ぎないこと。
- `seven-day-life` の CLI 出力が本 refactor の前後で完全に一致すること
  （既存の `phase_6_seven_day_life.rs` が無変更のままパスすることで
  確認する）。
- 新規 `phase_11_intelligence_loop.rs` が `run_cycle` の verification
  有無の両方のケースを直接検証すること。
- `run_multiple_individuals`/`run_multiple_goods`、`goods-domain`、
  `goods-application`、`goods-infrastructure`、`scheduler` placeholder、
  persistence、frontend の変更は実装・許可しない。
