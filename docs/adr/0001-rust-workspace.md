# ADR 0001: Rust Workspace

## English

### Context

Goods Garden needs stable package ownership before product behavior is added.

### Decision

Use a Rust Workspace with Domain, Application, Infrastructure, Runtime and
CLI packages.

### Alternatives

Use one crate, a service-per-crate layout, or postpone package boundaries.

### Consequences

Boundaries are visible early; behavior must wait for later Domain Design.

### Status

Accepted for Phase 0.

## 日本語

### Context

Goods Garden は商品挙動を追加する前に package ownership を安定させる必要がある。

### Decision

Domain、Application、Infrastructure、Runtime、CLI package を持つ Rust Workspace を使う。

### Alternatives

一つの crate、service-per-crate、または package boundary の先送り。

### Consequences

boundary を早期に可視化できるが、挙動は後続の Domain Design を待つ。

### Status

Phase 0 で Accepted。

## 中文

### Context

Goods Garden 需要在增加商品行为前稳定 package 归属。

### Decision

使用包含 Domain、Application、Infrastructure、Runtime 和 CLI package 的 Rust Workspace。

### Alternatives

单 crate、每服务一个 crate，或推迟 package 边界。

### Consequences

可以尽早看见边界，但行为必须等待后续 Domain Design。

### Status

Phase 0 Accepted。
