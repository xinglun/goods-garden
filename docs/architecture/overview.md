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

Phase 1 implements only this bounded local slice:

```text
synthetic fixture
    ↓
DemoObservationSource
    ↓
ObservationSource port
    ↓
GoodsRuntime
    ↓
Goods State + Health Assessment
    ↓
local CLI output
```

The fixture is explicitly synthetic and is not a POS or SEJ integration. Need,
Care, Action, Memory and Learning remain documented future capabilities.

## 日本語

Goods Garden は product world、Goods Intelligence Runtime は将来の orchestration layer、Domain Model
は商品の意味、Adapter は application port 経由の外部 input/action 接続を担う。

`Data → Senses → Goods State → Need → Care → Action → New State → Memory` は intended loop である。
Phase 1 ではそのうち local synthetic fixture → `ObservationSource` port → `GoodsRuntime` →
`Goods State` + `Health Assessment` → CLI output の slice だけを実装する。fixture は POS や SEJ integration ではなく、
Need、Care、Action、Memory、Learning は将来 capability として文書化する。

## 中文

Goods Garden 是产品世界，Goods Intelligence Runtime 是未来的编排层，Domain Model 定义商品的含义，
Adapter 只能通过 application port 连接外部输入和行动。

预期信息循环为 `Data → Senses → Goods State → Need → Care → Action → New State → Memory`。Phase 1
只实现 local synthetic fixture → `ObservationSource` port → `GoodsRuntime` → `Goods State` + `Health Assessment` →
CLI output 这一段。fixture 不是 POS 或 SEJ 接入；Need、Care、Action、Memory、Learning 仍是未来能力。
