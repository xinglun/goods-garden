# Phase 12 — Scheduler

## Goal

Phase 0 由来の空プレースホルダー `crates/goods-runtime/src/scheduler/
mod.rs` を実装し、Phase 11 で実装した Intelligence Loop の1周分の
サイクル（`GoodsRuntime::run_cycle`）を、人間が都度起動しなくても
自動で進める仕組みを与える。これは North Star が「Human authority
decides whether any action may be autonomous」と明記する自律実行の
領域そのものであり、実装前に repository owner との合意形成
（human decision）を経て、以下4つのガードレールを非交渉可能な受け入れ
基準とした。

## Implemented boundary

Phase 12 に含めるのは次だけである。

1. **型レベルで synthetic のみに限定**：`goods_runtime::scheduler::
   run_scheduled` は `ObservationSource`/`HumanFeedbackSource` に対して
   汎用化しない。関数内部で `DemoObservationSource`/
   `DemoHumanFeedbackSource` を直接構築するため、呼び出し元が real な
   adapter を差し込む余地は型として存在しない。real データへの切り替えは
   この module 自体を書き換えるという可視的な決定を要求する。
2. **回数上限**：`max_cycles` に達したら `StopReason::MaxCyclesReached`
   で自動停止し、続けるには人間が改めてコマンドを実行する必要がある。
   CLI demo では `max_cycles=5` を script 長（7）より小さくあえて設定し、
   Day 6/7 に到達する前に安全上限で止まる様子を実際に出力で示す
   （偶然の一致ではなく独立した安全機構であることの実演）。
3. **永続的なログ**：`scheduled-seven-day-life` は各サイクルの内容を
   標準出力に加え、`std::env::temp_dir()` 配下のログファイルにも記録
   する。`goods_runtime::scheduler` 自体は一切ファイル I/O を行わず、
   CLI 層が運用監査ログとして書き出すだけである。これは Phase 4 が
   明示した「persistence は対象外」（domain の Relationship Memory の
   retention policy の話）とは別物であり、矛盾しない。
4. **フォアグラウンドのみ**：detach、PID file、signal handling を伴う
   真の background daemon は実装しない。人間が起動した端末/プロセスに
   紐づき、Ctrl-C や端末終了で即座に停止する。

新しい CLI subcommand `scheduled-seven-day-life` は、Phase 6 で既に
review 済みの `SEVEN_DAY_SCRIPT` をそのまま再利用する（新しい数値は
発明しない）。既存の `demo`/`seven-day-life`/`multiple-individuals`/
`multiple-goods` subcommand とその出力は一切変更しない。

## Local demo と data boundary

repository root から demo を実行する。

```bash
cargo run -p goods-garden-cli -- scheduled-seven-day-life
```

Day 1〜5 が約200msごとに自動的に進み、Day 6 に到達する前に安全上限
（5 cycle）で停止する。停止理由と、進行状況が保存されないため再実行は
Day 1 から始まる旨、ログファイルの絶対パスが表示される。

## Exit criteria

- `goods_runtime::scheduler::run_scheduled` が上記4つのガードレールを
  実装すること。
- `scheduled-seven-day-life` の出力が Day 1〜5 のみを含み、Day 6/7 を
  含まないこと。
- ログファイルの内容が標準出力と一致すること。
- 新規 `phase_12_scheduler.rs` が `StopReason` の両方のケースと CLI の
  挙動を直接検証すること。
- 既存の `demo`/`seven-day-life`/`multiple-individuals`/
  `multiple-goods` subcommand、`goods-domain`/`goods-application`/
  `goods-infrastructure`、real adapter、background daemon、frontend の
  変更は実装・許可しない。
