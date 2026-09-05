# Phase 7 — Multiple Individuals

## Goal

`TunaMayo@StoreA` から複数 store の individual へ拡張し、Goods Species と Individual Memory の
分離を検証する。

## Implemented boundary

Phase 7 は新しい domain type を導入しない。不変の Phase 1-6 model を、同一 species の2つの Goods
individual（`TunaMayo@StoreA` と `TunaMayo@StoreB`）に対して実行し、既存の `demo` と
`seven-day-life` subcommand はそのままに、新しい `multiple-individuals` CLI subcommand として
公開する。

両 individual は1つの clone された `GoodsProfile` 値（Species レベルのデータ：display name、
expected lifetime、minimum stock）を共有する一方、それぞれ異なる `GoodsIdentity.individual_id` と、
決定的に、独立した `GoodsMemory` instance を持つ。Store A の synthetic observation は
`FreshnessConcern` Need、Care Request、Care Action を発生させ、それは Store A の memory にのみ
記憶される。Store B の observation は expectation 内にとどまり、何も記憶しない。demo の最後の
summary は、どちらの individual の memory にも相手の記録が含まれないことを明示する。

これは、型を追加するのではなく直接の構築によって、「Species」（共有される `GoodsProfile`/`species`
label）と「Individual Memory」（1つの `Goods` instance が所有する `GoodsMemory` 値）が、Phase 1-6
の architecture において既に分離された関心事であることを検証する。

## Local demo と data boundary

repository root から demo を実行する。

```bash
cargo run -p goods-garden-cli -- multiple-individuals
```

両 individual の observation と Human Feedback は（Phase 6 の前例に合わせ）CLI 内で直接構築され、
`synthetic-example` と明記される。既存の `demo` と `seven-day-life` subcommand とその出力は変更しない。

## Exit criteria

- local user が `multiple-individuals` を実行し、同一 species の2つの individual が独立した
  Need、Care episode、Memory を生成すること、そしてそれらの memory が交差しないことを明示する
  statement を確認できる。
- 既存の `demo` と `seven-day-life` subcommand は変更しない。
- 新しい domain type、自律 action、rule adjustment は導入しない。
