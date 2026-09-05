# Phase 8 — Multiple Goods

## Goal

salmon rice ball、coffee、sandwich、bento を追加する目的は数を増やすことではなく、Goods
Intelligence が class of capability、concrete good が object/instance であることを証明する
ことである。

## Implemented boundary

Phase 8 は新しい domain type を導入しない。不変の Phase 1-7 model を、4つの異なる product
species——salmon rice ball、coffee、sandwich、bento——に対して全く同じ `request_care_and_remember`
call で実行し、既存の `demo`、`seven-day-life`、`multiple-individuals` subcommand はそのままに、
新しい `multiple-goods` CLI subcommand として公開する。

この4種は数合わせではなく、`GoodsProfile` の値が本当に異なる範囲をカバーするよう選ばれている。

- Salmon rice ball: `expected_lifetime_hours = 8`、`minimum_stock_quantity = 2`——Phase 1 の
  tuna-mayo reference と同様の shelf-life category。
- Coffee: `expected_lifetime_hours = 4`、`minimum_stock_quantity = 3`——最も寿命が短く、意図的に
  expectation を超えて observe されるため、`FreshnessConcern` Need、Care Request、Care Action
  を発生させる唯一の商品となる。
- Sandwich: `expected_lifetime_hours = 6`、`minimum_stock_quantity = 2`——中間的な case。
- Bento: `expected_lifetime_hours = 10`、`minimum_stock_quantity = 1`——最も寿命が長く、在庫下限も
  最も低い。

`goods-domain`、`goods-application`、`goods-runtime` のどこにも商品固有の分岐は存在しない：
全ての species は data（`GoodsIdentity` と `GoodsProfile`）であり、Phase 1-7 で既に確立された
同一の code path を通過する。これがまさに「Goods Intelligence は capability の class であり、
concrete good は object/instance である」ことの実践的な意味である。

## Local demo と data boundary

repository root から demo を実行する。

```bash
cargo run -p goods-garden-cli -- multiple-goods
```

4つ全ての observation と Human Feedback 値は（Phase 6/7 の前例に合わせ）CLI 内で直接構築され、
`synthetic-example` と明記される。既存の `demo`、`seven-day-life`、`multiple-individuals`
subcommand とその出力は変更しない。

## Exit criteria

- local user が `multiple-goods` を実行し、同一の runtime call で処理される4つの異なる product
  species を確認でき、coffee が Need/Care episode を発生させ他は healthy のままであること、
  商品固有の code branch が不要だったことを確認する締めの statement を確認できる。
- 既存の `demo`、`seven-day-life`、`multiple-individuals` subcommand は変更しない。
- 新しい domain type、自律 action、rule adjustment は導入しない。
