# ADR 0006: AI Cockpit Governance

## English

### Context

An AI-native project needs a governed way for Agents to inspect, change and
verify repository work.

### Decision

Attach the external AI Cockpit Runtime to the repository. Use its Contract,
preflight, checkpoint, verification and human-decision records as the
development governance path.

### Alternatives

Copy the Runtime source into the repository, use ungoverned Agent files only,
or defer governance until production.

### Consequences

Governance artifacts are repository-local and inspectable; actual Runtime
states and missing evidence must be reported honestly.

### Status

Accepted for Phase 0.

## 日本語

### Context

AI-native project には Agent の inspect、change、verify を governance する仕組みが必要である。

### Decision

外部 AI Cockpit Runtime を repository に attach し、Contract、preflight、checkpoint、verification、human-decision record を
development governance path とする。

### Alternatives

Runtime source を repository に copy、ungoverned Agent file のみを使う、production まで governance を延期する。

### Consequences

governance artifact は repository-local で inspectable。実際の Runtime state と missing evidence を正直に報告する。

### Status

Phase 0 で Accepted。

## 中文

### Context

AI-native 项目需要一种受治理的方式，让 Agent 检查、修改并验证仓库工作。

### Decision

将外部 AI Cockpit Runtime 接入仓库，以 Contract、preflight、checkpoint、verification 和 human-decision 记录作为开发治理路径。

### Alternatives

把 Runtime 源码复制到仓库，只使用无治理 Agent 文件，或等到生产阶段再治理。

### Consequences

治理 artifact 位于仓库且可检查；必须诚实报告实际 Runtime 状态和缺失证据。

### Status

Phase 0 Accepted。
