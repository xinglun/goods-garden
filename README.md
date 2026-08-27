# Goods Garden

## English

### What is Goods Garden?

Goods Garden is the product world where goods can become understandable,
careable business entities. Its technical core is **Goods Intelligence**. The
world and the kernel are related but must not be confused.

### North Star

> Bring every product to life.

In the original formulation: 商品を「管理されるデータ」から、「自ら状態を理解し、助けを求め、学習する存在」へ変える。

Goods should move from passive data objects toward entities that can understand
state, ask for help and learn from outcomes. This is a domain and product
direction, not a claim that software has consciousness or emotions.

### Core idea

```text
State → Need → Care → Action → New State → Memory / Learning
```

Data is sensory input, not the user interface. Human-facing meaning should be
grounded in state, concern, need, request, explanation and learning result.

### Current status

Phase 0 — Foundation. The repository contains the Rust workspace boundary,
placeholder modules, governed development setup, documentation baseline,
reference object and CI quality gates. Concrete product behavior is out of
scope.

### Architecture

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

### Project structure

- `crates/` contains the layered Rust workspace.
- `apps/goods-garden-cli/` is the future application entry point.
- `docs/` contains the North Star, architecture, roadmap, ADRs and glossary.
- `examples/tuna-mayo/` is a product-independent reference profile.
- `.ai/` contains repository-bound AI Cockpit governance artifacts.

### AI Cockpit

AI Cockpit is an external development-governance Runtime. It is attached to
this repository; its source is not copied here. Contract, preflight, checkpoint,
verification and human-decision evidence must remain inspectable and truthful.

### Documentation

Human-facing documentation is trilingual in the fixed order English, 日本語,
中文. Unknown facts are labelled `UNKNOWN`; examples and hypotheses never
masquerade as operational facts.

### Development

```text
cargo fmt --check
cargo check --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
bash tests/architecture/check-boundaries.sh
```

### Non-goals

Phase 0 does not implement sales analysis, anomaly detection, LLM Agents,
personality, emotion, Need/Care Runtime behavior, POS, databases, UI, API
servers, queues, Kafka, Kubernetes, microservices, vector databases or
autonomous actions.

## 日本語

### Goods Garden とは

Goods Garden は、商品を理解し、care できる business entity へ近づける product world
である。技術的な core は **Goods Intelligence**。二つは関係するが混同しない。

### North Star

> Bring every product to life.

原文は「商品を『管理されるデータ』から、『自ら状態を理解し、助けを求め、学習する存在』へ変える。」

商品を passive data object から、状態を理解し、助けを求め、結果から学ぶ entity へ近づける。
これは domain/product の方向であり、software に意識や本当の感情があるという主張ではない。

### Core idea

```text
State → Need → Care → Action → New State → Memory / Learning
```

Data は sensory input であり user interface ではない。人への意味は state、concern、need、
request、explanation、learning result に基づく。

### 現在の状態

Phase 0 — Foundation。Rust workspace boundary、placeholder module、governed development
setup、documentation、reference object、CI quality gate を用意する。具体的な商品挙動は対象外。

### Architecture

依存方向は `goods-domain`、`goods-application`、`goods-infrastructure`、`goods-runtime`、
`goods-garden-cli` の順で、下位層は上位層だけを知る。

### Project structure

`crates/` は layer、`apps/goods-garden-cli/` は将来の entry point、`docs/` は設計文書、
`examples/tuna-mayo/` は商品非依存の reference profile、`.ai/` は repository-bound AI Cockpit
governance artifact を含む。

### AI Cockpit

AI Cockpit は外部の development-governance Runtime であり、source は repository にコピーしない。
Contract、preflight、checkpoint、verification、human-decision の evidence は inspectable かつ truthfully
保持する。

### Documentation

Human-facing documentation は English、日本語、中文の順で三言語化する。未知の事実は
`UNKNOWN` とし、example と hypothesis は operational fact に見せない。

### Development と Non-goals

README の English section にある Cargo command を実行する。Phase 0 では sales analysis、anomaly
detection、LLM Agent、personality、emotion、Need/Care Runtime、POS、database、UI、API、queue、
Kafka、Kubernetes、microservice、vector database、自律 action を実装しない。

## 中文

### Goods Garden 是什么

Goods Garden 是一个让商品逐步成为可理解、可照料经营实体的产品世界。技术内核是
**Goods Intelligence**。两者相关，但不能混淆。

### North Star

> Bring every product to life.

原文：将商品从“被管理的数据”变成“能够理解自身状态、主动求助，并通过结果学习的存在”。

这是一项领域与产品方向，不宣称软件拥有真正的意识或情感。

### Core idea

```text
State → Need → Care → Action → New State → Memory / Learning
```

数据是感官输入，不是用户界面。面向人的表达应基于状态、担忧、需求、求助、解释和学习结果。

### 当前状态

Phase 0 — Foundation。已规划 Rust workspace 边界、占位模块、受治理开发环境、文档基线、
reference object 和 CI 质量门。具体商品行为不在本阶段范围内。

### Architecture

依赖方向固定为 `goods-domain`、`goods-application`、`goods-infrastructure`、`goods-runtime`、
`goods-garden-cli`；下层只能了解上层。

### Project structure

`crates/` 保存分层 Rust workspace，`apps/goods-garden-cli/` 是未来应用入口，`docs/` 保存设计文档，
`examples/tuna-mayo/` 是与商品实现无关的参考 profile，`.ai/` 保存仓库绑定的 AI Cockpit governance artifact。

### AI Cockpit

AI Cockpit 是外部 development-governance Runtime，不把它的源码复制进本仓库。Contract、preflight、
checkpoint、verification 和 human decision evidence 必须可检查且真实。

### Documentation

所有面向人的文档按英文、日语、中文固定顺序提供三语内容。未知事实标记为 `UNKNOWN`；example 和
hypothesis 不得伪装成运营事实。

### Development 与 Non-goals

执行 English section 中的 Cargo 命令。Phase 0 不实现销售分析、异常检测、LLM Agent、人格、情绪、
Need/Care Runtime、POS、数据库、UI、API Server、消息队列、Kafka、Kubernetes、微服务、Vector DB 或自主行动。
