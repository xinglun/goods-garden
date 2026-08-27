# Phase 0 — Foundation

## English

### Goal

Create the Rust Workspace, DDD and Clean Architecture boundaries, AI Cockpit
attachment and profile calibration, North Star and architecture documents,
trilingual guidance, reference profile and CI.

### Exit criteria

- `cargo fmt --check` passes.
- `cargo check --workspace` passes.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` passes.
- `cargo test --workspace` passes.
- AI Cockpit `attach`, profile, agent adapter, `doctor` and `verify` states are
  inspected and reported as actually observed.
- Architecture, trust, living-goods model, roadmap, ADRs and glossary are
  documented in English, Japanese and Chinese.
- All placeholder modules compile and no real Goods Intelligence behavior is
  present.

### Explicit boundary

Do not implement sales analysis, anomaly detection, LLM Agents, personality,
emotion, Need/Care Runtime, POS, database, UI, API, queue or autonomous action.

## 日本語

### Goal

Rust Workspace、DDD/Clean Architecture boundary、AI Cockpit attachment/profile calibration、
North Star、architecture document、三言語 guidance、reference profile、CI を作る。

### Exit criteria

`cargo fmt --check`、`cargo check --workspace`、`cargo clippy --workspace --all-targets --all-features -- -D warnings`、
`cargo test --workspace` が pass。AI Cockpit の attach、profile、Agent adapter、doctor、verify は実際の状態を確認して報告する。
architecture、trust、living-goods、roadmap、ADR、glossary は English、日本語、中文で文書化する。placeholder は compile し、
real Goods Intelligence 挙動はない。

### Explicit boundary

sales analysis、anomaly detection、LLM Agent、personality、emotion、Need/Care Runtime、POS、database、UI、API、queue、
autonomous action を実装しない。

## 中文

### Goal

建立 Rust Workspace、DDD 与 Clean Architecture 边界、AI Cockpit 接入与 profile 校准、North Star 与架构文档、三语指引、
reference profile 和 CI。

### Exit criteria

`cargo fmt --check`、`cargo check --workspace`、`cargo clippy --workspace --all-targets --all-features -- -D warnings`、
`cargo test --workspace` 全部通过；真实检查并报告 AI Cockpit attach、profile、Agent adapter、doctor、verify 状态；架构、信任、
living-goods、路线图、ADR、glossary 以英文、日语、中文记录；所有占位模块可编译且没有真实 Goods Intelligence 行为。

### Explicit boundary

不实现销售分析、异常检测、LLM Agent、商品人格、情绪、Need/Care Runtime、POS、数据库、UI、API、消息队列或自主行动。
