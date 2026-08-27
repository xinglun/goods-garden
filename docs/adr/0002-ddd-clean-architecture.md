# ADR 0002: DDD and Clean Architecture

## English

### Context

The product depends on domain meaning rather than infrastructure or transport.

### Decision

Keep domain language inward and enforce `Domain ← Application ← Infrastructure
← Runtime ← Apps` dependency direction.

### Alternatives

Allow shared infrastructure types everywhere or use transport-first design.

### Consequences

Adapters can change without changing domain language; boundary checks are required.

### Status

Accepted for Phase 0.

## 日本語

### Context

product は infrastructure や transport ではなく domain meaning に依存する。

### Decision

domain language を内側に置き、`Domain ← Application ← Infrastructure ← Runtime ← Apps` の依存方向を守る。

### Alternatives

shared infrastructure type を全層で使う、または transport-first design。

### Consequences

adapter を変更しても domain language を変更せずに済む。boundary check が必要。

### Status

Phase 0 で Accepted。

## 中文

### Context

产品依赖领域含义，而不是基础设施或传输层。

### Decision

让领域语言向内，固定 `Domain ← Application ← Infrastructure ← Runtime ← Apps` 依赖方向。

### Alternatives

在各层共享基础设施类型，或采用 transport-first design。

### Consequences

Adapter 可以变化而不改变领域语言；需要持续检查边界。

### Status

Phase 0 Accepted。
