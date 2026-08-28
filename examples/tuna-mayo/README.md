# Tuna-mayo reference object

## English

ツナマヨおにぎり is the first Reference Object. It is a declarative example
profile, not a permission to write tuna-mayo-specific business logic.

The project must not contain branches such as `if goods == "tuna_mayo"`. Every
future implementation must remain valid when this Goods Profile is replaced;
the same Goods Intelligence Runtime should still apply.

Phase 1 includes `observation.example.txt`, a tiny local fixture used by the
CLI. It is labelled `synthetic-example` and is deliberately not a POS schema,
POS export or real SEJ data. Any operational value would need to be labelled
`synthetic`, `example` or `hypothesis`.

Run it through the generic runtime with:

```bash
cargo run -p goods-garden-cli -- demo
```

## 日本語

ツナマヨおにぎりは最初の Reference Object である。これは declarative な example profile であり、
tuna-mayo 専用 business logic を許可するものではない。

`if goods == "tuna_mayo"` のような分岐は禁止する。Goods Profile を置き換えても同じ Goods Intelligence Runtime が
成立することを、将来の実装ごとに確認する。

Phase 1 では CLI が使う小さな local fixture `observation.example.txt` を含む。これは `synthetic-example` と明記したもので、POS schema、
POS export、real SEJ data ではない。運用値を使う場合は `synthetic`、`example`、`hypothesis` のいずれかを明記する。

generic runtime は次で実行する。

```bash
cargo run -p goods-garden-cli -- demo
```

## 中文

ツナマヨおにぎり 是第一个 Reference Object。它只是声明式 example profile，不允许编写饭团专用业务逻辑。

禁止出现 `if goods == "tuna_mayo"` 一类分支。未来每个实现都必须验证：替换 Goods Profile 后，同一个 Goods Intelligence Runtime
仍然成立。

Phase 1 包含 CLI 使用的微型本地 fixture `observation.example.txt`，并明确标记为 `synthetic-example`。它不是 POS schema、POS export 或真实
SEJ 数据。任何运营数值都必须标记为 `synthetic`、`example` 或 `hypothesis`。

通过通用 runtime 运行：

```bash
cargo run -p goods-garden-cli -- demo
```
