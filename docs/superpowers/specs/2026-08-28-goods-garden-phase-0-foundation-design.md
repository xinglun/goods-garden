# Goods Garden Phase 0 Foundation Design

> Status: Approved in conversation; design baseline pending written review.
>
> This document is maintained in English, 日本語, and 中文. The three sections
> are intended to remain semantically equivalent.

## English

### Purpose

Phase 0 establishes the engineering, governance, and design baseline for Goods
Garden. It does not implement concrete Goods Intelligence behavior. The result
must make the repository safe for subsequent Agent-assisted development and
make the first living-goods implementation possible without prematurely
choosing a database, transport, LLM provider, or UI framework.

The repository directory remains `goods-garden`; the product world is named
Goods Garden, and the technical core is named Goods Intelligence. Rust package
names use the `goods-*` namespace.

### North Star

> 商品を「管理されるデータ」から、「自ら状態を理解し、助けを求め、学習する存在」へ変える。

In English: **Bring every product to life.**

In Chinese: 将商品从“被管理的数据”，变成能够理解自身状态、主动求助，并通过结果学习的经营生命体。

The North Star is the highest-level design authority for future choices. It
does not authorize autonomous business action, human replacement, or claims
that an AI system has consciousness or real emotions.

### Product model

Goods Garden is the product world that people experience. Goods Intelligence is
the technical core that gives a product a model of state, need, care,
verification, memory, and learning.

The foundational conceptual loop is:

```text
State → Need → Care → Action → New State → Memory / Learning
```

The corresponding intelligence loop is:

```text
Observe → Investigate → Recommend / Escalate → Act → Verify → Learn
```

Data such as sales, inventory, waste, and weather is sensory input. Human-facing
outputs should eventually express interpreted state, concern, need, request,
explanation, and learning result rather than exposing only dashboard numbers.

The relationship is `Goods ↔ Caregiver`. A human is a caregiver, not merely an
execution endpoint. AI may observe, analyze, express, recommend, and request
help; it does not own business authority.

### Phase 0 boundary

In scope:

- A Rust workspace using DDD and Clean Architecture.
- Domain, Application, Infrastructure, Runtime, and CLI crate boundaries.
- Placeholder modules, traits, structs, and READMEs for the future lifecycle.
- A multilingual design-first documentation baseline.
- AI Cockpit attachment, project profile calibration, Agent adapter, and
  repository-bound lifecycle evidence.
- A `tuna-mayo` reference object containing only profile examples and guidance.
- CI gates for formatting, checking, Clippy, and workspace tests.

Out of scope:

- Sales analysis, anomaly detection, LLM Agent behavior, personality, emotion,
  Need/Care runtime behavior, POS adapters, databases, UI, APIs, queues,
  Kafka, Kubernetes, microservices, vector databases, and autonomous actions.
- Real SEJ data, POS schemas, inventory interfaces, replenishment rules, store
  operations, or product KPIs.
- Any concrete database, HTTP, cloud, OpenAI, Anthropic, or other provider
  dependency.
- A state machine, runtime orchestration, persistence implementation, or
  tuna-mayo-specific business branch.

### Architecture

The dependency direction is one-way:

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

`goods-domain` defines only domain language. It has no dependency on HTTP, SQL,
cloud SDKs, LLMs, Tokio-specific concerns, JSON transport, or CLI frameworks.

`goods-application` owns future use cases and ports. It may depend on the
domain, but it must not bind a port to an external technology.

`goods-infrastructure` will own future adapters. In Phase 0 it contains only
module boundaries and no database, network client, cloud SDK, or LLM SDK.

`goods-runtime` will eventually orchestrate the lifecycle. In Phase 0 it only
declares its future modules and does not implement a state machine or loop.

`goods-garden-cli` is the future application surface. It remains a compilable
placeholder and does not expose a business command yet.

### Phase 0 placeholder surface

The domain crate declares the future language under `goods`, `state`, `need`,
`care`, `observation`, `evidence`, `memory`, `learning`, and `lifecycle`.
The smallest current types are names only:

```rust
pub struct Goods;
pub struct GoodsState;
pub struct GoodsNeed;
pub struct CareRequest;
pub struct CareAction;
pub struct Observation;
pub struct Evidence;
pub struct GoodsMemory;
pub struct Learning;
```

Fields are intentionally not selected in Phase 0. Domain design belongs to
later, evidence-backed Work Items.

Application ports and use-case module names are declared without behavior.
Infrastructure adapter namespaces and Runtime orchestration namespaces are
declared without implementations.

### Reference object rule

`examples/tuna-mayo/` is the first reference object, not a special-case
implementation. It contains a profile example and explains that the same
Goods Intelligence Runtime must remain valid when the Goods Profile changes.
The repository must never introduce logic equivalent to:

```rust
if goods == "tuna_mayo" { ... }
```

### Documentation and language policy

Repository-owned human-facing documents use three adjacent sections in the
fixed order English, 日本語, 中文. This applies to the root README, Agent guide,
contributor guide, all `docs/**` documents, and reference-object READMEs.
Commands, paths, identifiers, and code remain language-neutral. The AI Cockpit
runtime owns its generated protocol records; those records are not manually
replaced. Project-authored profile and governance prose is multilingual when
the runtime format permits prose.

### Evidence and trust rules

All future product claims must be traceable to an Observation, Evidence,
Inference, or Human Input. The information states are `KNOWN`, `INFERRED`,
`UNKNOWN`, `UNAVAILABLE`, and `CONFLICTING`.

Phase 0 must prefer evidence over fluency, unknown over fabrication, human
authority over inferred authority, explainable need, traceable care action, and
verifiable outcome. No current document may present synthetic examples as
real SEJ facts; examples are labeled synthetic, example, or hypothesis.

### Governance workflow

This repository is attached to the installed AI Cockpit Runtime. Work proceeds
through a repository-bound Contract, preflight, one checkpoint, verification,
finish, archive, and close. The Phase 0 Work Item is serial because its source,
documentation, CI, and generated governance projections share the repository
boundary.

Agents may create placeholders, format files, wire modules, set up simple tests,
configure CI, format documentation, and perform non-semantic refactoring.
Agents must investigate new dependencies, architecture changes, new domain
concepts, cross-layer dependencies, and runtime behavior. A human decision is
required for North Star changes, semantic domain changes, Need or Care
authority, autonomous action authority, escalation policy, memory retention,
learning policy, and trust/evidence semantics.

### Verification

The final candidate must run these checks freshly:

```bash
cargo fmt --check
cargo check --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
git diff --check
```

AI Cockpit status, Doctor, and Verify results are recorded separately from Rust
results. A missing or unexecuted check is reported as missing, not as passing.

### Alternatives considered

1. **One serial Work Item (selected).** It keeps shared documentation, root
   configuration, AI Cockpit projections, and CI evidence under one authority
   boundary. It is the safest choice for a new repository with no existing
   build or governance conventions.
2. **Parallel Rust and documentation Work Items.** Rejected because both would
   modify shared root files and generated `.ai` projections, creating a merge
   and evidence-order dependency.
3. **Implement a minimal runtime first.** Rejected because it would decide
   domain fields and behavior before the evidence-backed Domain Design phase,
   violating the Phase 0 boundary.

### Design status

This design is the approved Phase 0 baseline. Domain concepts, fields, runtime
behavior, authority policies, and provider choices remain provisional or
deferred until their respective future Work Items.

## 日本語

### 目的

Phase 0 は Goods Garden のための技術、ガバナンス、設計の基盤を整える。
具体的な Goods Intelligence の振る舞いは実装しない。次の段階で Agent が安全に
開発へ参加でき、データベース、通信方式、LLM Provider、UI framework を先に固定せずに
最初の living goods を実装できる状態を作る。

Repository のディレクトリ名は `goods-garden` のままにする。体験する製品世界は Goods
Farm、技術的な core は Goods Intelligence と呼び、Rust package は `goods-*` namespace
を使う。

### North Star

> 商品を「管理されるデータ」から、「自ら状態を理解し、助けを求め、学習する存在」へ変える。

英語では **Bring every product to life.** と表現する。

中国語では：将商品从“被管理的数据”，变成能够理解自身状态、主动求助，并通过结果学习的经营生命体。

これは将来の判断における最高位の設計根拠である。ただし自律的な業務権限、人間の代替、
AI に意識や本物の感情があるという主張を認めるものではない。

### 製品モデル

Goods Garden は人が体験する製品世界、Goods Intelligence は product に state、need、care、
verification、memory、learning の model を持たせる技術 core である。

基本 loop は次の通り。

```text
State → Need → Care → Action → New State → Memory / Learning
```

対応する intelligence loop は次の通り。

```text
Observe → Investigate → Recommend / Escalate → Act → Verify → Learn
```

売上、在庫、廃棄、天候などは感覚入力である。将来の human-facing output は dashboard の
数字だけでなく、解釈された state、concern、need、request、explanation、learning result
を表す。

関係は `Goods ↔ Caregiver` とする。人は単なる実行 endpoint ではなく caregiver である。
AI は observe、analyze、express、recommend、help request を行えるが、business authority は持たない。

### Phase 0 の境界

対象は Rust workspace、DDD/Clean Architecture の layer 境界、将来 lifecycle の placeholder
module/trait/struct/README、三言語の design-first 文書、AI Cockpit の attach/profile/Agent
adapter/証跡、`tuna-mayo` reference object、format/check/clippy/test の CI である。

対象外は sales analysis、anomaly detection、LLM Agent behavior、personality、emotion、
Need/Care runtime、POS adapter、database、UI、API、queue、Kafka、Kubernetes、microservice、
vector database、autonomous action、実際の SEJ data、POS schema、在庫 interface、補充 rule、
店舗運用、商品 KPI、具体的な database/HTTP/cloud/LLM provider dependency、state machine、
runtime orchestration、persistence、tuna-mayo 専用分岐である。

### Architecture

依存方向は一方向で固定する。

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

`goods-domain` は domain language だけを定義し、HTTP、SQL、cloud SDK、LLM、Tokio 固有の関心、
JSON transport、CLI framework を知らない。`goods-application` は future use case と port、
`goods-infrastructure` は future adapter の namespace、`goods-runtime` は将来の lifecycle
orchestration namespace、`goods-garden-cli` は将来の application surface の placeholder とする。

Domain の型は名前だけを持ち、field は Phase 0 で決めない。field と behavior は evidence-based
な後続 Work Item の責務である。

### Reference object

`examples/tuna-mayo/` は最初の reference object だが special-case implementation ではない。
profile が変わっても同じ Goods Intelligence Runtime が成立することを検証する。`if goods ==
"tuna_mayo"` のような business branch は作らない。

### 文書と言語

repository が所有する人向け文書は English、 日本語、 中文 の順に三 section を置く。root README、
Agent guide、contributor guide、`docs/**`、reference-object README が対象である。command、path、
identifier、code は language-neutral とする。AI Cockpit が生成する protocol record は runtime が所有し、
手動で置き換えない。runtime format が許す範囲で project-authored profile/governance prose を三言語化する。

### Trust と証拠

将来の product claim は Observation、Evidence、Inference、Human Input のいずれかへ追跡可能にする。
情報状態は `KNOWN`、`INFERRED`、`UNKNOWN`、`UNAVAILABLE`、`CONFLICTING` である。

Phase 0 では fluency より evidence、fabrication より unknown、推測した権限より human authority、
説明可能な Need、追跡可能な Care Action、検証可能な Outcome を優先する。synthetic/example/hypothesis
を実際の SEJ fact として記述しない。

### Governance と検証

repository は installed AI Cockpit Runtime に attach する。Contract、preflight、1 回の checkpoint、
verification、finish、archive、close の repository-bound lifecycle を使う。Phase 0 は source、document、
CI、`.ai` projection が同じ boundary を共有するため serial Work Item とする。

Agent は placeholder、format、module wiring、simple test、CI、documentation formatting、non-semantic
refactoring を行える。new dependency、architecture change、new domain concept、cross-layer dependency、
runtime behavior は調査する。North Star、domain semantics、Need/Care authority、autonomous action、
escalation、memory retention、learning policy、trust/evidence semantics は human decision を要求する。

最終 candidate は `cargo fmt --check`、`cargo check --workspace`、`cargo clippy --workspace --all-targets
--all-features -- -D warnings`、`cargo test --workspace`、`git diff --check` を fresh に実行する。AI Cockpit
の status/Doctor/Verify と Rust result は分離して記録し、未実行は pass と書かない。

### 代替案と状態

shared root、文書、`.ai` projection、CI 証跡を一つの authority boundary で扱う serial Work Item を採用する。
Rust と文書の並列 Work Item は shared path と evidence order の衝突のため採用しない。先に minimal runtime
を作る案も domain field と behavior の premature decision になるため採用しない。

この design は Phase 0 の approved baseline である。domain field、runtime behavior、authority policy、
provider choice は future Work Item まで provisional または deferred とする。

## 中文

### 目的

Phase 0 建立 Goods Garden 的工程、治理与设计基线，不实现具体 Goods Intelligence 行为。目标是让后续
Agent 能安全进入开发，并且在证据充分之前不提前选择数据库、通信方式、LLM Provider 或 UI framework，
同时为第一个“活着的商品”做好准备。

仓库目录继续使用 `goods-garden`；用户体验到的产品世界叫 Goods Garden，技术内核叫 Goods Intelligence，
Rust package 使用 `goods-*` 命名空间。

### North Star

> 商品を「管理されるデータ」から、「自ら状態を理解し、助けを求め、学習する存在」へ変える。

英文辅助表达为 **Bring every product to life.**

中文解释：将商品从“被管理的数据”，变成能够理解自身状态、主动求助，并通过结果学习的经营生命体。

它是未来判断的最高级设计依据，但不授权自主业务行动、人类替代，也不代表 AI 具有真正意识或情感。

### 产品模型

Goods Garden 是人们体验到的产品世界；Goods Intelligence 是让商品拥有 state、need、care、verification、
memory 与 learning 模型的技术内核。

基础循环是：

```text
State → Need → Care → Action → New State → Memory / Learning
```

对应的 intelligence loop 是：

```text
Observe → Investigate → Recommend / Escalate → Act → Verify → Learn
```

销售、库存、废弃、天气等数据首先是感官输入。未来面向人的输出应表达商品理解过的状态、担忧、需求、
求助、解释与学习结果，而不只是 dashboard 数字。

关系定义为 `Goods ↔ Caregiver`。人是 caregiver，不只是执行端。AI 可以感知、分析、表达、推荐和求助，
但不拥有业务权威。

### Phase 0 边界

范围包括 Rust workspace、DDD/Clean Architecture 分层、未来生命周期的 placeholder module/trait/struct/README、
三语 design-first 文档、AI Cockpit attachment/profile/Agent adapter/仓库绑定证据、`tuna-mayo` reference object，
以及 format/check/clippy/test CI gate。

不包括销售分析、异常检测、LLM Agent 行为、人格、情绪、Need/Care Runtime、POS Adapter、数据库、UI、API、
消息队列、Kafka、Kubernetes、微服务、Vector DB、自主行动、真实 SEJ 数据、POS Schema、库存接口、补货规则、
门店流程、商品 KPI、具体 database/HTTP/cloud/LLM Provider 依赖、state machine、runtime orchestration、持久化、
以及饭团专用业务分支。

### Architecture

依赖方向固定为：

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

`goods-domain` 只定义领域语言，不知道 HTTP、SQL、Cloud SDK、LLM、Tokio-specific concern、JSON transport 或
CLI framework。`goods-application` 负责未来用例和 port，`goods-infrastructure` 负责未来 adapter 命名空间，
`goods-runtime` 负责未来生命周期编排命名空间，`goods-garden-cli` 是未来应用入口的可编译 placeholder。

领域类型在 Phase 0 只有名称，不提前决定字段；字段和行为属于后续、以证据为基础的 Work Item。

### Reference object

`examples/tuna-mayo/` 是第一个 reference object，不是专用实现。必须验证更换 Goods Profile 后同一个 Goods
Intelligence Runtime 仍然成立。禁止出现 `if goods == "tuna_mayo"` 一类业务分支。

### 文档与语言策略

仓库拥有的面向人的文档统一使用 English、 日本語、 中文 三个相邻 section，顺序固定。范围包括根 README、Agent 指南、
贡献指南、`docs/**` 和 reference-object README。命令、路径、标识符和代码保持语言中立。AI Cockpit Runtime 生成的
协议记录由 Runtime 所有，不手工替换；Runtime 格式允许时，项目自定义的 Profile 与治理说明使用三语。

### Trust 与证据

未来所有商品表达必须可以追溯到 Observation、Evidence、Inference 或 Human Input 之一。信息状态为 `KNOWN`、
`INFERRED`、`UNKNOWN`、`UNAVAILABLE`、`CONFLICTING`。

Phase 0 坚持 evidence over fluency、unknown over fabrication、human authority over inferred authority、可解释 Need、
可追踪 Care Action、可验证 Outcome。没有真实 SEJ 数据；所有示例明确标记为 synthetic、example 或 hypothesis。

### Governance 与验证

仓库接入已安装的 AI Cockpit Runtime，使用 repository-bound Contract、preflight、一次 checkpoint、verification、
finish、archive、close 生命周期。由于源代码、文档、CI 与 `.ai` projection 共享边界，Phase 0 使用串行 Work Item。

Agent 可以处理 placeholder、格式化、模块接线、简单测试、CI、文档格式化和非语义重构；遇到新依赖、架构变更、新领域
概念、跨层依赖和 Runtime 行为必须调查。North Star、领域语义、Need/Care authority、自主行动权、升级策略、记忆保留、
学习政策、Trust/Evidence 语义必须请求 Human Decision。

最终 candidate 必须实际执行 `cargo fmt --check`、`cargo check --workspace`、`cargo clippy --workspace --all-targets
--all-features -- -D warnings`、`cargo test --workspace` 和 `git diff --check`。AI Cockpit 的 status、Doctor、Verify 与 Rust
结果分开记录；未执行的检查必须写成未执行。

### 备选方案与状态

采用一个串行 Work Item，把共享 root、文档、`.ai` projection 与 CI evidence 放在同一 authority boundary 下。Rust 与文档
并行会产生共享路径和 evidence order 冲突，因此不采用；先实现 minimal runtime 会在证据设计之前决定领域字段和行为，也不采用。

本设计是 Phase 0 的 approved baseline。领域字段、Runtime 行为、authority policy 和 provider 选择在对应后续 Work Item
之前均保持 provisional 或 deferred。
