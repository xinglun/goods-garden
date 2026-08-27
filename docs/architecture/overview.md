# Architecture Overview

## English

Goods Garden is the product world. Goods Intelligence Runtime is the future
orchestration layer. The Domain Model defines the meaning of a good. Adapters
connect outside observations and actions only through application ports.

```text
Goods Garden
    ↓
Goods Intelligence Runtime
    ↓
Domain Model
    ↓
Adapters
```

The intended information loop is:

```text
Data → Senses → Goods State → Need → Care → Action → New State → Memory
```

In Phase 0 these are documented boundaries, not implemented behavior.

## 日本語

Goods Garden は product world、Goods Intelligence Runtime は将来の orchestration layer、Domain Model
は商品の意味、Adapter は application port 経由の外部 input/action 接続を担う。

`Data → Senses → Goods State → Need → Care → Action → New State → Memory` は intended loop だが、
Phase 0 では boundary として文書化するだけで挙動を実装しない。

## 中文

Goods Garden 是产品世界，Goods Intelligence Runtime 是未来的编排层，Domain Model 定义商品的含义，
Adapter 只能通过 application port 连接外部输入和行动。

预期信息循环为 `Data → Senses → Goods State → Need → Care → Action → New State → Memory`。Phase 0
只记录边界，不实现这些行为。
