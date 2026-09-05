# Goods Garden Phase 12 — Scheduler Design Specification

**Status:** PROPOSED — requires human review before implementation

**Date:** 2026-09-05

**Scope:** `goods-runtime` and `apps/goods-garden-cli` additions for
Phase 12 (ScheduledCycle, StopReason, run_scheduled).

## 背景と課題

`crates/goods-runtime/src/scheduler/mod.rs` は Phase 0 由来の空
プレースホルダー（`//! Scheduler boundary placeholder.` というコメント
のみ）が repository 内で最後に残った未実装の placeholder として残って
いた。これは Phase 11 で実装した `intelligence_loop`（1周分のサイクル）
を「いつ」人間の介在なしに自動で駆動するかという、North Star が
「Human authority decides whether any action may be autonomous」と
明記する自律実行の領域そのものである。

`AGENTS.md` は autonomous action authority を human decision が必要な
protected semantic area として扱う。そのため、設計に入る前に
repository owner との合意形成を行い、以下4つのガードレールを確認した。

## 目標

1. `ScheduledCycle`/`StopReason`/`run_scheduled` を実装する。
2. 新しい CLI subcommand `scheduled-seven-day-life` で、既存
   `SEVEN_DAY_SCRIPT` を再利用した自動実行を実演する。

## 非目標

本提案は real な `ObservationSource`/`HumanFeedbackSource` adapter、
`goods_runtime::scheduler` を汎用化すること、background daemon（detach、
PID file、signal handling）、domain state の persistence
（`MemoryStore` は引き続き空プレースホルダー）、既存の `demo`/
`seven-day-life`/`multiple-individuals`/`multiple-goods` subcommand の
変更、`goods-domain`/`goods-application`/`goods-infrastructure`、
frontend の変更のいずれも実装・許可しない。

## 決定：型レベルで synthetic のみに限定する

`run_scheduled` は `ObservationSource`/`HumanFeedbackSource` に対して
汎用化せず、関数内部で `DemoObservationSource`/`DemoHumanFeedbackSource`
を直接構築する。repository owner との合意形成で、この制限を型レベルで
enforce すること（呼び出し元が real な adapter を差し込む余地を型として
与えないこと）が確認された。real データへの切り替えは、この module
自体を書き換えるという可視的な決定を要求する——設定変更や汎用化された
trait 経由での差し替えでは済まないようにする。

## 決定：回数上限は script 長より小さくし安全機構として実演する

CLI demo では `SCHEDULER_MAX_CYCLES=5`（`SEVEN_DAY_SCRIPT.len()=7` より
小さい値）とする。repository owner との合意で「回数上限に達したら自動
停止し、続けるには人間が改めてコマンドを実行する」ことが確認されており、
上限を script 長と同じ値にすると「たまたま script が終わっただけ」なのか
「独立した安全機構が働いたのか」が実際の出力からは区別できない。
あえて小さい値にすることで、Day 6/7 に到達する前に安全上限で止まる
様子を実際に証明する。

## 決定：ログはファイルに残すが、Phase 4 の persistence 対象外方針と矛盾しない

`goods_runtime::scheduler::run_scheduled` 自体は一切ファイル I/O を
行わない（`on_cycle` コールバックを呼ぶだけ）。ファイルへの書き込みは
CLI 層（`apps/goods-garden-cli`）が行う運用監査ログであり、
`std::env::temp_dir()` 配下に書く（repository の作業木を汚さない）。
Phase 4 が明示した「persistence は対象外」は `GoodsMemory`/
`MemoryStore` という domain の Relationship Memory の retention policy
の話であり、本 phase のログファイルは別の関心事（人間が後から自律実行の
履歴を review できるようにする、repository owner との合意事項）である。
この区別を本文書で明記する。

## 決定：フォアグラウンドのみ、daemon 化しない

repository owner との合意で、`scheduler` は人間が起動した端末/プロセスに
紐づくフォアグラウンド実行のみとし、detach、PID file、signal handling を
伴う真の background daemon は実装しないことが確認された。これにより、
人間が Ctrl-C や端末終了で即座に停止できる。

## 検討した代替案

**`max_cycles` を script 長と同じ値にする**：不採用。安全上限が実際に
機能していることを出力から証明できず、単なる「script が終わった」ことと
区別がつかない。

**`run_scheduled` を `ObservationSource`/`HumanFeedbackSource` に対して
汎用化し、doc comment で「synthetic のみに使うこと」と注意書きする**：
不採用。convention による制約は compile 時点で強制されず、repository
owner との合意（型レベルで enforce する）に反する。

**真の background daemon（detach、PID file）を実装する**：不採用。
repository owner との合意で明示的に対象外とされた。実装・運用の複雑さと
リスクが、本 phase が示したい「Intelligence Loop を人間の介在なしに
進める」という最小限の実演に見合わない。

## Known、inference、unknown

- **KNOWN：** `scheduler` placeholder は Phase 0 以来未実装のまま
  repository 内に存在していた（本 spec 執筆前に確認済み）。
- **KNOWN：** 4つのガードレールは、実装に着手する前に repository owner
  との会話で1つずつ明示的に確認された human decision である。
- **INFERRED：** 該当なし。
- **UNKNOWN：** real な adapter が将来実装された場合に、この scheduler
  パターンをそのまま転用してよいか。これは本 phase の scope 外であり、
  real adapter を実装する別の Work Item が改めて human decision を
  求めるまで `UNKNOWN` のままとする。
- **UNAVAILABLE：** この repository 内の real external な自動運用データ。

## Review gate と acceptance

この文書は `PROPOSED` である。承認では次を確認する。

1. `run_scheduled` が `ObservationSource`/`HumanFeedbackSource` に対して
   汎用化されておらず、synthetic のみに型レベルで限定されていること。
2. `max_cycles=5` が script 長7より小さく、安全上限が独立して機能する
   ことが実際の出力で示されていること。
3. ログファイルへの書き込みが CLI 層の責務であり、`goods_runtime::
   scheduler` 自体はファイル I/O を行わないこと。
4. background daemon 化を実装していないこと。
