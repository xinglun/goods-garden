# Phase 8 — Multiple Goods

## English

### Goal

Add salmon rice ball, coffee, sandwich and bento only to prove that Goods
Intelligence is a class of capability while a concrete good is an object or
instance. Do not add products merely for quantity.

### Implemented boundary

Phase 8 introduces no new domain type. It runs four distinct product
species — salmon rice ball, coffee, sandwich and bento — through the
unchanged Phase 1-7 model via the exact same `request_care_and_remember`
call, exposed as a new `multiple-goods` CLI subcommand alongside the
unchanged `demo`, `seven-day-life` and `multiple-individuals` subcommands.

The four species were chosen to span a genuinely different range of
`GoodsProfile` values, not to pad the count:

- Salmon rice ball: `expected_lifetime_hours = 8`, `minimum_stock_quantity =
  2` — similar shelf-life category to Phase 1's tuna-mayo reference.
- Coffee: `expected_lifetime_hours = 4`, `minimum_stock_quantity = 3` — the
  shortest lifetime, deliberately observed past its expectation so it is
  the one product that raises a `FreshnessConcern` Need, a Care Request and
  a Care Action.
- Sandwich: `expected_lifetime_hours = 6`, `minimum_stock_quantity = 2` — a
  middle case.
- Bento: `expected_lifetime_hours = 10`, `minimum_stock_quantity = 1` — the
  longest lifetime and lowest stock floor.

No product-specific branch exists in `goods-domain`, `goods-application` or
`goods-runtime`: every species is data (a `GoodsIdentity` and a
`GoodsProfile`) passed through the identical code path already established
in Phase 1-7. This is exactly what "Goods Intelligence is a class of
capability, and a concrete good is an object or instance" means in
practice.

### Local demo and data boundary

Run the demo from the repository root:

```bash
cargo run -p goods-garden-cli -- multiple-goods
```

All four observations and Human Feedback values are constructed directly in
the CLI (matching Phase 6/7's precedent) and labelled `synthetic-example`.
The existing `demo`, `seven-day-life` and `multiple-individuals` subcommands
and their output are unchanged.

### Exit criteria

- A local user can run `multiple-goods` and see four distinct product
  species processed by the identical runtime call, with coffee raising a
  Need/Care episode and the others staying healthy, plus a closing
  statement confirming no product-specific code branch was needed.
- The existing `demo`, `seven-day-life` and `multiple-individuals`
  subcommands are unchanged.
- No new domain type, autonomous action or rule adjustment is introduced.

## 日本語

### Goal

salmon rice ball、coffee、sandwich、bento を追加する目的は数を増やすことではなく、Goods
Intelligence が class of capability、concrete good が object/instance であることを証明する
ことである。

### Implemented boundary

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

### Local demo と data boundary

repository root から demo を実行する。

```bash
cargo run -p goods-garden-cli -- multiple-goods
```

4つ全ての observation と Human Feedback 値は（Phase 6/7 の前例に合わせ）CLI 内で直接構築され、
`synthetic-example` と明記される。既存の `demo`、`seven-day-life`、`multiple-individuals`
subcommand とその出力は変更しない。

### Exit criteria

- local user が `multiple-goods` を実行し、同一の runtime call で処理される4つの異なる product
  species を確認でき、coffee が Need/Care episode を発生させ他は healthy のままであること、
  商品固有の code branch が不要だったことを確認する締めの statement を確認できる。
- 既存の `demo`、`seven-day-life`、`multiple-individuals` subcommand は変更しない。
- 新しい domain type、自律 action、rule adjustment は導入しない。

## 中文

### Goal

加入 salmon rice ball、coffee、sandwich 和 bento 的目的不是堆商品，而是证明 Goods Intelligence
是能力 Class，具体商品是 Object 或 Instance。

### Implemented boundary

Phase 8 不引入任何新的领域类型。它将不变的 Phase 1-7 模型运行于四种不同的产品
species——salmon rice ball、coffee、sandwich、bento——通过完全相同的
`request_care_and_remember` 调用处理，作为新的 `multiple-goods` CLI 子命令公开，既有的
`demo`、`seven-day-life`、`multiple-individuals` 子命令保持不变。

选择这四种商品不是为了凑数，而是为了覆盖真正不同的 `GoodsProfile` 数值范围：

- Salmon rice ball：`expected_lifetime_hours = 8`、`minimum_stock_quantity = 2`——与 Phase 1
  的 tuna-mayo 参考对象类似的保鲜期类别。
- Coffee：`expected_lifetime_hours = 4`、`minimum_stock_quantity = 3`——寿命最短，被刻意观测为
  超出 expectation，因此是唯一引发 `FreshnessConcern` Need、Care Request 与 Care Action 的商品。
- Sandwich：`expected_lifetime_hours = 6`、`minimum_stock_quantity = 2`——中间情况。
- Bento：`expected_lifetime_hours = 10`、`minimum_stock_quantity = 1`——寿命最长、库存下限最低。

`goods-domain`、`goods-application`、`goods-runtime` 中不存在任何商品专属分支：每个 species 都
只是数据（一个 `GoodsIdentity` 与一个 `GoodsProfile`），通过 Phase 1-7 已经确立的相同代码路径。
这正是“Goods Intelligence 是能力类，具体商品是对象/实例”在实践中的含义。

### 本地 demo 与数据边界

从仓库根目录运行 demo：

```bash
cargo run -p goods-garden-cli -- multiple-goods
```

全部四个 observation 与 Human Feedback 值都直接在 CLI 中构造（与 Phase 6/7 的先例一致），并标记为
`synthetic-example`。既有的 `demo`、`seven-day-life`、`multiple-individuals` 子命令及其输出保持
不变。

### Exit criteria

- 本地用户运行 `multiple-goods`，能看到四种不同的产品 species 由相同的 runtime 调用处理，
  coffee 引发 Need/Care 事件而其余保持健康，并有结束陈述确认无需任何商品专属代码分支。
- 既有的 `demo`、`seven-day-life`、`multiple-individuals` 子命令保持不变。
- 不引入任何新的领域类型、自主行动或规则调整。
