# Phase 6 — Seven Day Life

## Goal

product milestone は一つのおにぎりが7日間 live すること。normal、anomaly、investigation、care
request、human feedback、improvement、verification、memory を含む demo とする。観察者が「本当に商品が
生きているみたい」と感じても、metaphor を consciousness と解釈しない。

## Implemented boundary

Phase 6 は新しい domain type を導入しない。不変の Phase 1-5 の Observe → Assess → Identify Need →
Request/Receive Care → Remember → Verify → Learn loop を、1つの Goods individual に対して7日分の
synthetic day としてスクリプト化し、既存の `demo` subcommand はそのままに、新しい `seven-day-life`
CLI subcommand として公開する。

good の identity は、1週間ぶっ通しで age する単一の物理的な個体ではなく、監視対象の小売スロットを表す：
restock により後の日で observed age がリセットされる、これは実際の棚位置の振る舞いと同じである。7日間の
script は次の通り。

- Day 1、2、5：normal——両次元とも expectation 内で Need 無し。
- Day 3：anomaly と investigation——observed age が profile expectation を超え、`FreshnessConcern`
  Need、Care Request、synthetic Human Feedback response（restock）を発生させる。
- Day 4：verification と improvement——restock 後の observation はもう `FreshnessConcern` Need を
  示さないため、Day 3 の Care Action は Resolved として verify され、Learning statement が記録される。
- Day 6：2つ目の anomaly——observed quantity が profile の minimum を下回り、
  `StockAvailabilityConcern` Need、Care Request、synthetic Human Feedback response（restock order）
  を発生させる。
- Day 7：Day 6 の Care Action の verification と improvement、続いて週全体の最終 Memory 集計。

全ての Need、Care Request、Care Action、Outcome、Learning statement は不変の Phase 2-5 の rule に
よって生成される。本 phase が供給するのはそれらを駆動する synthetic observation と Human Feedback の
系列だけである。

## Local demo と data boundary

repository root から demo を実行する。

```bash
cargo run -p goods-garden-cli -- seven-day-life
```

7日分は全て明確に synthetic である：各 observation と Human Feedback 値は CLI 自身の中で構築され
（外部 fixture file からではない。これは再利用可能な単一の business fixture ではなく、スクリプト化された
milestone narrative だからである）、`(synthetic)` マーカーとともに表示される。既存の
`cargo run -p goods-garden-cli -- demo` command とその出力は変更しない。

## Exit criteria

- local user が `seven-day-life` を実行し、一貫した1週間——平穏な日、synthetic Human Feedback で
  応答される2つの anomaly、2つの follow-up verification、最終的な memory 集計——を確認できる。
- 既存の `demo` subcommand の挙動は変更しない。
- 新しい domain type、自律 action、rule adjustment は導入しない。
