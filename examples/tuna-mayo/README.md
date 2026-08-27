# Tuna-mayo reference object

## English

ツナマヨおにぎり is the first Reference Object. It is a declarative example
profile, not a permission to write tuna-mayo-specific business logic.

The project must not contain branches such as `if goods == "tuna_mayo"`. Every
future implementation must remain valid when this Goods Profile is replaced;
the same Goods Intelligence Runtime should still apply.

The example contains no real SEJ data. Any operational value would need to be
labelled `synthetic`, `example` or `hypothesis`.

## 日本語

ツナマヨおにぎりは最初の Reference Object である。これは declarative な example profile であり、
tuna-mayo 専用 business logic を許可するものではない。

`if goods == "tuna_mayo"` のような分岐は禁止する。Goods Profile を置き換えても同じ Goods Intelligence Runtime が
成立することを、将来の実装ごとに確認する。

real SEJ data は含まない。運用値を使う場合は `synthetic`、`example`、`hypothesis` のいずれかを明記する。

## 中文

ツナマヨおにぎり 是第一个 Reference Object。它只是声明式 example profile，不允许编写饭团专用业务逻辑。

禁止出现 `if goods == "tuna_mayo"` 一类分支。未来每个实现都必须验证：替换 Goods Profile 后，同一个 Goods Intelligence Runtime
仍然成立。

不包含真实 SEJ 数据。任何运营数值都必须标记为 `synthetic`、`example` 或 `hypothesis`。
