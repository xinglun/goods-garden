# Phase 9 — Evidence

## Goal

Phase 0 から未実装のまま残っていた `Evidence` placeholder を実装し、
`docs/architecture/trust-model.md` が定義する information state の語彙
（`KNOWN`/`INFERRED`/`UNKNOWN`/`UNAVAILABLE`/`CONFLICTING`）を、既存7箇所の
素の `explanation: String` に代わって実際に使う。これにより、これまで
untyped な文字列でしかなかった「なぜそう判定したか」の説明が、それが
測定・計算された事実（Known）なのか、hypothesis 閾値のような解釈ルールに
基づく判断（Inferred）なのかを区別できるようになる。

## Implemented boundary

Phase 9 に含めるのは次だけである。

- `evidence::evidence::Evidence`：`InformationState`（5値の enum）と
  plain-language な `statement: String` を持つ struct。`Display` により
  `"{statement} [{STATE}]"` の形式で出力される。
- 既存7箇所の `explanation: String` field を `evidence: Evidence` に置き換える：
  `HealthAssessment`、`Deviation`、`NeedConflict`、`GoodsNeed`、`CareRequest`、
  `CareAction`、`Outcome`。文言自体は変更せず、既存の文字列組み立てロジックを
  `Evidence::known(..)`/`Evidence::inferred(..)` でラップするだけである。
- `GoodsNeed` のみ `Evidence::inferred(..)` を使う。`Urgency` の閾値は
  `example`/`hypothesis` 値であり実データ由来ではないと Phase 2 から明記
  されているため、Urgency 分類に依存する Need の evidence は Known ではなく
  Inferred が正しい。他の6箇所は決定的な計算・記録に基づくため Known である。
- `HealthAssessment`（`state::goods_state`）も対象に含める。これまでの
  Phase 2-8 設計書はすべて `state::goods_state` を不変と明記してきたが、
  本 phase はこれを意図的な例外として崩す。`phase_1_demo.rs` の
  `.explanation.contains("exceeds")` assertion は `.evidence.statement.contains("exceeds")`
  に書き換える。
- CLI（`demo`/`seven-day-life`/`multiple-individuals`/`multiple-goods` の
  4 subcommand）は `explanation` ではなく `evidence` を出力するよう変更し、
  `Display` により `[KNOWN]`/`[INFERRED]` タグが自動的に見えるようにする。

`learning::Learning.statement` はこの phase の対象外である。Learning は
Outcome から導かれる reviewable な statement であり、`explanation: String`
を持つ7箇所とは別カテゴリとして扱い、意図的に変更しない。

## Local demo と data boundary

repository root から4つの subcommand を実行する。

```bash
cargo run -p goods-garden-cli -- demo
cargo run -p goods-garden-cli -- seven-day-life
cargo run -p goods-garden-cli -- multiple-individuals
cargo run -p goods-garden-cli -- multiple-goods
```

いずれの出力も、health/need/care request/care action/outcome の各行末尾に
`[KNOWN]` または `[INFERRED]` タグが付く。`multiple-goods` の coffee は
確実に `FreshnessConcern` Need を発生させるため、`[INFERRED]` タグを
確認できる唯一の同梱 fixture である。既存の4 subcommand の入力データや
Need/Care/Memory/Outcome の判定ロジック自体は変更しない。

## Exit criteria

- 7箇所すべてで `explanation: String` が `evidence: Evidence` に置き換わり、
  `GoodsNeed` だけが `Inferred`、他の6箇所は `Known` であること。
- 4つの CLI subcommand の出力に `[KNOWN]`/`[INFERRED]` タグが表示されること。
- `phase_1_demo.rs`/`phase_3_care.rs` の既存 assertion が
  `.evidence.statement` を参照するよう更新され、パスすること。
- 新規 `phase_9_evidence.rs` が Known/Inferred の区別と CLI 出力のタグを
  直接検証し、パスすること。
- `Learning.statement`、新しい domain type（Evidence/InformationState 以外）、
  persistence、自律 action、frontend の変更は実装・許可しない。
