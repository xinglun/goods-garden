# ADR 0005: Domain Independent from LLM

## English

### Context

The living-goods model must outlast any provider or language model.

### Decision

The Domain has no LLM knowledge. Any future intelligence provider is an
application port with an infrastructure adapter.

### Alternatives

Put prompts in entities, bind a provider in Domain, or make an Agent framework
the domain runtime.

### Consequences

Provider changes do not redefine Goods; intelligence claims remain subject to
evidence and authority.

### Status

Accepted for Phase 0.

## 日本語

### Context

living-goods model は provider や language model より長く成立する必要がある。

### Decision

Domain は LLM を知らない。将来の intelligence provider は application port と infrastructure adapter に分ける。

### Alternatives

entity に prompt を置く、Domain で provider を bind する、Agent framework を domain runtime にする。

### Consequences

provider の変更で Goods の意味は変わらない。intelligence claim は evidence と authority に従う。

### Status

Phase 0 で Accepted。

## 中文

### Context

living-goods model 必须超越任何 provider 或语言模型而成立。

### Decision

Domain 不知道 LLM。未来的 intelligence provider 通过 application port 和 infrastructure adapter 接入。

### Alternatives

把 prompt 放进 entity，在 Domain 绑定 provider，或让 Agent framework 成为领域 runtime。

### Consequences

更换 provider 不会重新定义 Goods；智能主张仍受证据和权限约束。

### Status

Phase 0 Accepted。
