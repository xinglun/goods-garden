# Clean Architecture

## English

### Dependency direction

```text
goods-domain
    ↑
goods-application
    ↑
goods-infrastructure
    ↑
goods-runtime
    ↑
goods-garden-cli
```

An arrow means the lower layer may depend on the layer above it. Domain never
depends on Application, Infrastructure, Runtime or Apps.

### Layer responsibilities

| Layer | Responsibility | Forbidden knowledge |
| --- | --- | --- |
| Domain | Domain language and invariants in a later phase | HTTP, SQL, SDKs, providers, runtime and CLI |
| Application | Use cases and ports | Concrete adapters and app composition |
| Infrastructure | External adapters | Reversing dependency direction or defining domain meaning |
| Runtime | Future lifecycle orchestration | Owning domain semantics or provider identity |
| Apps | Human-facing entry points | Bypassing application ports |

The Domain does not know that an LLM exists. The living-goods model is not an
attachment to an LLM provider.

## 日本語

依存方向は `goods-domain` ↑ `goods-application` ↑ `goods-infrastructure` ↑ `goods-runtime` ↑
`goods-garden-cli`。矢印は下位 layer が上位 layer に依存できることを示す。Domain は Application、
Infrastructure、Runtime、Apps に依存しない。

責務は Domain が domain language、Application が use case/port、Infrastructure が external adapter、
Runtime が将来の lifecycle orchestration、Apps が human-facing entry point。Domain は HTTP、SQL、SDK、
provider、runtime、CLI を知らず、LLM の存在も知らない。

## 中文

依赖方向是 `goods-domain` ↑ `goods-application` ↑ `goods-infrastructure` ↑ `goods-runtime` ↑
`goods-garden-cli`，表示下层可以依赖上层。Domain 不依赖 Application、Infrastructure、Runtime 或 Apps。

职责分别是 Domain 负责领域语言，Application 负责用例与端口，Infrastructure 负责外部 adapter，Runtime 负责未来
生命周期编排，Apps 负责面向人的入口。Domain 不知道 HTTP、SQL、SDK、provider、runtime、CLI 的存在，也不依赖 LLM。
