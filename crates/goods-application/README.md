# goods-application

## English

This crate owns use cases and ports. It may depend on `goods-domain`, but it
does not know runtime composition or concrete adapters. Phase 1 implements only
the `ObserveGoods` and `AssessState` boundaries used by the local demo.

## 日本語

この crate は use case と port を所有する。`goods-domain` に依存できるが、runtime の composition や具体的な
adapter は知らない。Phase 1 では local demo が使う `ObserveGoods` と `AssessState` の boundary だけを実装する。

## 中文

此 crate 负责用例和端口。它可以依赖 `goods-domain`，但不感知 runtime 编排或具体 adapter。Phase 1 只实现本地
demo 使用的 `ObserveGoods` 和 `AssessState` 边界。
