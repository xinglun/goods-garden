# Goods Garden Agent Guide

## English

### Project purpose

Goods Garden is a design-first, evidence-first Rust workspace. Goods
Intelligence is its technical kernel; Goods Garden is its product world.

### North Star

Bring every product to life: move goods from managed data to entities that can
understand state, ask for help and learn from outcomes.

### Current phase

Phase 10 — Lifecycle. Phases 1-10 are implemented: the bounded State demo
(Goods Identity, Goods Profile, Observation, Expectation, Goods State, Health
Assessment); Need (Deviation, Urgency, GoodsNeed, Need Conflict); Care
(CareRequest, Caregiver, Human Feedback, CareAction); Memory (GoodsMemory,
MemoryRecord); Verification & Learning (Outcome, Learning); the Seven Day
Life, Multiple Individuals and Multiple Goods CLI milestones; Evidence
(Evidence, InformationState); and Lifecycle (LifecycleState). All of this is
demonstrated only through `goods-garden-cli`'s synthetic-example fixtures.
The approved read-only browser presentation at `apps/goods-garden-web/`
still only consumes the Phase 1 synthetic `GoodsStateView` projection and
owns no domain behavior; it does not reflect Need, Care, Memory, Evidence or
Lifecycle. Autonomous orchestration of the Intelligence Loop
(`goods-runtime`'s `intelligence_loop` and `scheduler` placeholders) remains
prohibited without a dedicated human decision on autonomous action
authority.

### Architecture rules

The dependency direction is `goods-domain` → `goods-application` →
`goods-infrastructure` → `goods-runtime` → `goods-garden-cli`, where each arrow
means the downstream layer may depend on the upstream layer. Domain owns
domain language and has no HTTP, SQL, cloud SDK, LLM provider, Tokio-specific
runtime, JSON transport or CLI dependency. Application owns use cases and
ports. Infrastructure owns adapters and currently contains the local
synthetic `DemoObservationSource` and `DemoHumanFeedbackSource`. Runtime owns
the Observe → Assess → Identify Needs → Request Care → Remember →
Verify/Learn composition (`GoodsRuntime`); its `intelligence_loop` and
`scheduler` modules remain unimplemented placeholders for future
orchestration. The frontend is a sibling application outside the Cargo
member list; it renders a read-only projection and must not import Rust
crates or external-system code.

### Human decision boundaries

Investigate carefully before adding a dependency, domain concept, cross-layer
dependency or runtime behavior. Request a human decision before changing the
North Star, domain semantics, Need definition, Care authority, autonomous
action authority, human escalation policy, memory retention, learning policy or
Trust/Evidence semantics. `crates/goods-domain/**`, `docs/north-star.md`,
`docs/architecture/**` and `docs/phases/**` are protected semantic areas.

### AI Cockpit usage

AI Cockpit is an external Runtime. Use the repository-bound Contract and its
fail-closed lifecycle. Never hand-edit generated `.ai` Summary, evidence,
Outcome, archive or decision records. Do not overwrite pre-existing provider
files. Record actual command output and actual Runtime states.

### Evidence rules

Evidence is stronger than fluency. Every material claim must be `KNOWN`,
`INFERRED`, `UNKNOWN`, `UNAVAILABLE` or `CONFLICTING`. If the agent does not
know a fact, write `UNKNOWN`; do not complete it by invention. No real SEJ data
is available in this repository. POS schemas, inventory APIs, replenishment
rules, product KPIs and store operations are only `example`, `synthetic` or
`hypothesis` when explicitly labelled.

### Documentation rules

Human-facing documents must contain semantically equivalent English, Japanese
and Chinese sections in that order. Keep Goods Garden and Goods Intelligence
distinct. Do not introduce a new synonym for an existing glossary term.

### Out of scope

No sales analysis, anomaly detection, LLM Agent, product personality, emotion
system, Need/Care Runtime, POS Adapter, database, production Web UI, frontend
business behavior, API Server, message queue, Kafka, Kubernetes, microservice,
vector database, fabricated SEJ fact or autonomous business action. Only the
approved read-only synthetic State presentation is in scope for the browser
package.

## 日本語

### Project purpose / North Star

Goods Garden は design-first、evidence-first の Rust workspace である。Goods Intelligence
は technical kernel、Goods Garden は product world である。North Star は商品を管理される
data から、状態を理解し、助けを求め、結果から学ぶ entity へ近づけること。

### Current phase と Architecture

現在は Phase 10 — Lifecycle まで実装済みである。State（Goods Identity、Goods Profile、Observation、
Expectation、Goods State、Health Assessment）、Need（Deviation、Urgency、GoodsNeed、Need Conflict）、
Care（CareRequest、Caregiver、Human Feedback、CareAction）、Memory（GoodsMemory、MemoryRecord）、
Verification & Learning（Outcome、Learning）、Seven Day Life／Multiple Individuals／Multiple Goods の
CLI milestone、Evidence（Evidence、InformationState）、Lifecycle（LifecycleState）を実装済みで、いずれも
`goods-garden-cli` の synthetic-example fixture でのみ実演する。承認済みの `apps/goods-garden-web/`
read-only browser presentation は依然として Phase 1 の synthetic `GoodsStateView` projection だけを
消費し、domain behavior を所有しない——Need、Care、Memory、Evidence、Lifecycle は反映しない。
Intelligence Loop の自動駆動（`goods-runtime` の `intelligence_loop`／`scheduler` placeholder）は、
autonomous action authority についての別途の human decision なしには禁止のままとする。
依存は `goods-domain` → `goods-application` → `goods-infrastructure` → `goods-runtime` →
`goods-garden-cli`。Domain は HTTP、SQL、cloud SDK、LLM provider、Tokio 固有 runtime、JSON transport、
CLI を知らない。Infrastructure は local synthetic `DemoObservationSource` と `DemoHumanFeedbackSource`
を持つ。Runtime は Observe → Assess → Identify Needs → Request Care → Remember → Verify/Learn の
composition（`GoodsRuntime`）を持ち、`intelligence_loop`／`scheduler` module は未実装の placeholder の
ままである。frontend は Cargo member list の外にある sibling application で、Rust crate や
external-system code を import しない。

### Human decision / AI Cockpit

dependency、domain concept、cross-layer dependency、runtime behavior は慎重に調査する。North Star、
domain semantics、Need、Care authority、自律 action、escalation、memory retention、learning、
Trust/Evidence を変える時は human decision を求める。AI Cockpit の生成 record は手で編集せず、
provider file を上書きしない。

### Evidence / Documentation / Out of scope

主張は `KNOWN`、`INFERRED`、`UNKNOWN`、`UNAVAILABLE`、`CONFLICTING` を区別する。不明なら
`UNKNOWN` と書き、SEJ、POS、inventory、KPI、store operation を創作しない。Human-facing document は
English、日本語、中文の同等 section を持つ。sales、LLM Agent、emotion、Need/Care Runtime、POS、
database、production Web UI、frontend business behavior、API、queue、Kafka、Kubernetes、microservice、
vector database、自律 business action は対象外。承認済みの read-only synthetic State presentation だけが
browser package の対象である。

## 中文

### Project purpose / North Star

Goods Garden 是 design-first、evidence-first Rust workspace。Goods Intelligence 是技术内核，
Goods Garden 是产品世界。North Star 是把商品从被管理的数据变成可以理解状态、主动求助并通过结果学习的实体。

### Current phase 与 Architecture

当前已实现至 Phase 10 — Lifecycle。已实现 State（Goods Identity、Goods Profile、Observation、
Expectation、Goods State、Health Assessment）、Need（Deviation、Urgency、GoodsNeed、Need Conflict）、
Care（CareRequest、Caregiver、Human Feedback、CareAction）、Memory（GoodsMemory、MemoryRecord）、
Verification & Learning（Outcome、Learning）、Seven Day Life／Multiple Individuals／Multiple Goods
CLI 里程碑、Evidence（Evidence、InformationState）与 Lifecycle（LifecycleState），均只通过
`goods-garden-cli` 的 synthetic-example fixture 演示。获批的 `apps/goods-garden-web/` 只读浏览器展示
仍然只消费 Phase 1 的 synthetic `GoodsStateView` projection，不拥有领域行为——不反映 Need、Care、
Memory、Evidence、Lifecycle。在没有针对 autonomous action authority 的单独 human decision 之前，
自动驱动 Intelligence Loop（`goods-runtime` 的 `intelligence_loop`／`scheduler` 占位符）仍然禁止。
依赖方向为 `goods-domain` → `goods-application` →
`goods-infrastructure` → `goods-runtime` → `goods-garden-cli`。Domain 不得依赖 HTTP、SQL、云 SDK、LLM provider、
Tokio-specific runtime、JSON transport 或 CLI；Infrastructure 现在拥有本地 synthetic
`DemoObservationSource` 与 `DemoHumanFeedbackSource`。Runtime 拥有 Observe → Assess → Identify
Needs → Request Care → Remember → Verify/Learn 的编排（`GoodsRuntime`）；`intelligence_loop`／
`scheduler` 模块仍是未实现的占位符。frontend 是
Cargo member list 之外的 sibling application，不得 import Rust crate 或 external-system code。

### Human decision / AI Cockpit

新增依赖、领域概念、跨层依赖、runtime 行为必须谨慎调查。修改 North Star、领域语义、Need、Care authority、
自主行动权限、人工升级策略、记忆保留、学习策略或 Trust/Evidence 语义时，必须请求 Human Decision。
AI Cockpit 生成的 Summary、evidence、Outcome、archive、decision 记录禁止手改，也不得覆盖既有 provider 文件。

### Evidence / Documentation / Out of scope

所有主张区分 `KNOWN`、`INFERRED`、`UNKNOWN`、`UNAVAILABLE`、`CONFLICTING`；不知道就写 `UNKNOWN`。仓库没有真实
SEJ 数据；POS schema、库存 API、补货规则、商品 KPI、门店流程只能在明确标记为 `example`、`synthetic` 或
`hypothesis` 时出现。面向人的文档必须按英文、日语、中文提供等价内容。销售分析、LLM Agent、情绪、Need/Care Runtime、
POS、数据库、生产 Web UI、前端业务行为、API、消息队列、Kafka、Kubernetes、微服务、Vector DB、虚构 SEJ 事实和自主经营行动
均不在范围内。只有获批的只读 synthetic State 浏览器展示属于当前 frontend package 范围。

<!-- AI_COCKPIT_ADAPTER_BEGIN provider=codex adapterVersion=1 repositoryId=sha256:0aec331bc68e94391249429b76176094d170caa7875fa92dab27316c7927213f -->

This repository is attached to AI Cockpit.

Canonical interface: .ai/agent-interface.json
Read .ai/README.md before acting; read .ai/glossary.md for the repository-local Agent route and vocabulary.

Use the installed shared Rust Runtime as the repository-governance interface.
Every repository-bound command must include an explicit --repo <path>.
Prefer MCP when available; CLI remains the fallback. Do not infer AI Cockpit state from this file. Query the Runtime for current governance state.

Before editing, query inspect, status, doctor, and agent doctor. Use one bounded Work Item, branch, and worktree. Keep all edits inside the Contract scope; amend and re-run preflight before expanding it.

Contract first: intent, scope, outOfScope, sources, unknowns, acceptance criteria, verification, and authority are human-owned. For code mode, unresolved unknowns or notCodable conditions stop implementation. Do not invent intent, approval, evidence, or completion.

A preflight result of not_ready or needs_human_confirmation is a mandatory human pause. Show the humanDecisionRequest and resume condition; a successful command or yellow result is not authorization.

For authorized changes use: start or work-item new → preflight → checkpoint → verify → finish → archive → close. Keep the Summary current with changed paths and reasons, sources, verification commands/results, guideline compliance, unknowns, risk, generated/destructive changes, and observed issues.

Before archive, present a visible human Outcome with 🟢/🟡/🔴, facts, unknowns, evidence, human decision, and next action. A raw MCP record or folded-only output is not a human handoff. Close only after the merged PR, archive, decision, default-branch synchronization, clean worktrees, and exact branch removal are verified.

Canonical delivery order is latest remote default base → dedicated branch/worktree → implement → finish/archive → push → reviewed PR → merge → close → synchronize and clean. Never merge a feature branch into local main before PR review, delete its branch before merge, or let a provider auto-delete it to bypass finalization. If a remote step fails, preserve the retry checkout and identity until recovery is complete.

A terminal green Outcome is the Rust equivalent of status=completed plus humanStatusColor=green: it requires state=Verified, decisionState=green, current Contract/Summary/evidence bindings, and direct human-visible delivery. Include issue count, blockers/stopping reason, resolved issues, risks, unknowns, verification, impact, human decision, and next action; every factual claim needs evidence, and unproven benefit is an inference.

When a defect is found in the current Work Item, repair it there by amending and revalidating its Contract before opening another Work Item or Issue. A successor is allowed only for a genuinely different scope, authority, or base, an independent compatible change, an unsafe in-scope repair, immutable failed delivery, or explicit human direction.

Never edit global Agent or MCP configuration, secrets, or credentials. Do not copy V1 runtime code, Python modules, Make commands, installers, or schemas into this repository.

<!-- AI_COCKPIT_ADAPTER_END -->
