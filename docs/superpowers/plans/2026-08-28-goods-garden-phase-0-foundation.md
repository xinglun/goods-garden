# Goods Garden Phase 0 Foundation Implementation Plan

## English

### Goal

Establish a compiling Rust workspace and an agent-readable, trilingual
documentation baseline for Goods Garden. The implementation must preserve
Clean Architecture dependency direction and create names-only placeholders;
it must not implement product intelligence behavior.

### Work sequence

1. Create the workspace manifests, toolchain and quality configuration.
   Add `goods-domain`, `goods-application`, `goods-infrastructure`,
   `goods-runtime`, and `goods-garden-cli` with only the declared dependency
   edges. Generate `Cargo.lock` and keep dependencies limited to the Rust
   standard library unless compilation requires otherwise.
2. Create the domain, application, infrastructure, runtime and CLI module
   surfaces. Use minimal public structs, empty traits and module declarations;
   do not add fields, algorithms, state transitions, adapters or provider
   clients.
3. Add the trilingual root guidance and design documents. Every human-facing
   file uses equivalent English, Japanese and Chinese sections in that order,
   including the README, agent guide, contributing guide, architecture,
   roadmap, ADRs, glossary and reference object.
4. Add the tuna-mayo reference profile as declarative example data only. Keep
   the runtime product-independent and explicitly prohibit tuna-mayo branches.
5. Add the CI workflow and a small architecture test that checks the declared
   Cargo graph and required documentation paths without implementing business
   behavior.
6. Calibrate the attached AI Cockpit profile using the real Cargo quality
   command, inspect the agent adapter state without overwriting provider files,
   and record doctor/verify evidence through the Runtime.
7. Run formatting, check, Clippy, tests, diff checks, documentation scans and
   AI Cockpit verification. Reconcile any generated evidence through commands,
   never by editing generated summaries.

### Acceptance mapping

- Workspace and placeholder compilation: steps 1–2.
- Dependency direction and architecture boundaries: steps 1, 2 and 5.
- Trilingual documentation and design completeness: step 3.
- Reference object independence: step 4.
- CI quality gates: step 5.
- Governed Agent development and evidence: step 6.
- Final executable evidence: step 7.

### Verification commands

```text
cargo fmt --check
cargo check --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
git diff --check
ai-cockpit inspect --repo .
ai-cockpit doctor --repo .
ai-cockpit agent doctor --repo .
ai-cockpit verify --repo . --work-item phase-0-foundation
```

### Stop conditions

Stop and record the actual state if a requested design would require a new
domain semantic, a real external-system contract, an LLM/provider dependency,
autonomous authority, or an unverified SEJ fact. Those decisions belong to a
later human-reviewed Work Item.

## 日本語

### 目的

Goods Garden のコンパイル可能な Rust Workspace と、Agent が読める三言語
ドキュメント基盤を構築する。Clean Architecture の依存方向を守り、名前だけの
placeholder を作る。商品 Intelligence の実際の挙動は実装しない。

### 実施順序

1. Workspace manifest、toolchain、品質設定を作成する。
   `goods-domain`、`goods-application`、`goods-infrastructure`、
   `goods-runtime`、`goods-garden-cli` と宣言された依存辺を追加し、
   `Cargo.lock` を生成する。コンパイルに必要な場合を除き、依存を標準ライブラリに
   限定する。
2. Domain、Application、Infrastructure、Runtime、CLI の module surface を作る。
   最小の public struct、空の trait、module 宣言だけを置き、field、algorithm、
   state transition、adapter、provider client は追加しない。
3. 三言語の root guidance と設計文書を追加する。README、Agent guide、contributing
   guide、architecture、roadmap、ADR、glossary、reference object を含む全ての
   human-facing file は、英語、日本語、中国語の同等な section をこの順で持つ。
4. tuna-mayo reference profile は宣言的な example data としてのみ追加する。
   Runtime は商品非依存とし、tuna-mayo 専用分岐を明示的に禁止する。
5. CI workflow と、宣言された Cargo graph と必須文書 path を確認する小さな
   architecture test を追加する。業務挙動は実装しない。
6. 接続済み AI Cockpit profile を実際の Cargo quality command で calibrate し、
   provider file を上書きせず Agent adapter state を確認する。doctor/verify の
   evidence は Runtime の command で記録する。
7. format、check、Clippy、test、diff、documentation scan、AI Cockpit verification
   を実行する。生成 Summary は直接編集せず、必ず command で evidence を更新する。

### 受入れ対応

Workspace と placeholder の compile は 1–2、依存方向と architecture boundary は
1・2・5、三言語文書は 3、reference object の独立性は 4、CI は 5、Agent governance
と evidence は 6、最終実行証拠は 7 で確認する。

### 停止条件

新しい domain semantic、実際の外部 system contract、LLM/provider dependency、
autonomous authority、未検証の SEJ 事実が必要になった場合は、実際の状態を記録して
停止する。それらは後続の human-reviewed Work Item の決定事項である。

## 中文

### 目标

建立可编译的 Goods Garden Rust Workspace，以及 Agent 可直接读取的三语文档基线。
保持 Clean Architecture 依赖方向，只创建名称级 placeholder，不实现 Goods
Intelligence 的真实行为。

### 实施顺序

1. 创建 Workspace manifest、toolchain 与质量配置，加入 `goods-domain`、
   `goods-application`、`goods-infrastructure`、`goods-runtime`、
   `goods-garden-cli` 以及声明的依赖边，生成 `Cargo.lock`。除非编译需要，依赖限制为
   Rust 标准库。
2. 创建 Domain、Application、Infrastructure、Runtime、CLI 的 module surface。只放
   最小 public struct、空 trait 与 module 声明，不增加字段、算法、状态转移、adapter
   或 provider client。
3. 添加三语根目录指引与设计文档。README、Agent 指南、贡献指南、架构、路线图、ADR、
   glossary、reference object 等所有面向人的文件，都按英文、日语、中文顺序提供语义
   等价的 section。
4. 仅以声明式 example data 添加 tuna-mayo reference profile。Runtime 必须与具体商品
   无关，并明确禁止 tuna-mayo 专用分支。
5. 添加 CI workflow 与小型 architecture test，用于检查 Cargo 图和必需文档路径，不实现
   业务行为。
6. 使用真实 Cargo quality command 校准已接入的 AI Cockpit profile；不覆盖 provider 文件，
   检查 Agent adapter 状态，并通过 Runtime 记录 doctor/verify evidence。
7. 执行 format、check、Clippy、test、diff、文档扫描和 AI Cockpit verification。生成的
   Summary 只能通过 command 更新，禁止直接编辑。

### 验收对应

Workspace 和 placeholder 编译由步骤 1–2 覆盖；依赖方向和架构边界由步骤 1、2、5
覆盖；三语文档由步骤 3 覆盖；reference object 独立性由步骤 4 覆盖；CI 由步骤 5
覆盖；Agent governance 与 evidence 由步骤 6 覆盖；最终执行证据由步骤 7 覆盖。

### 停止条件

如果需要新的领域语义、真实外部系统契约、LLM/provider 依赖、自主权限或未经验证的
SEJ 事实，应记录实际状态并停止。它们属于后续经过 Human Review 的 Work Item 决策。
