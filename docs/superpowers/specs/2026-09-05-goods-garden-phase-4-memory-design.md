# Goods Garden Phase 4 — Memory Design Specification

**Status:** PROPOSED — requires human review before implementation

**Date:** 2026-09-05

**Scope:** `goods-domain`, `goods-application`, `goods-runtime` and `apps/goods-garden-cli`
additions for Phase 4 (GoodsMemory, MemoryRecord).

## English

### Context and problem

Phase 3 gave the good an explainable Care model (CareRequest, Caregiver,
HumanFeedback, CareAction). `crates/goods-domain/src/memory/memory.rs` was
left as an empty placeholder. `docs/phases/phase-4-memory.md` names
Relationship Memory and explicitly warns against inventing retention policy.

The repository's own governance (`AGENTS.md`) treats memory retention as a
protected semantic area requiring a human decision. The safest way to honor
that constraint is to not define one at all in this phase: Memory here is a
transient, in-process, append-only log with no persistence, no expiry and no
eviction, so there is no retention question left unanswered — it is simply
out of scope.

### Goals

1. Add `GoodsMemory` and `MemoryRecord` to `goods-domain` without changing
   the Phase 1-3 State/Need/Care model.
2. Make Memory a plain, append-only, in-process value with no persistence:
   `GoodsMemory` is owned by its caller (the CLI in this phase) and threaded
   across repeated observations, not stored inside `GoodsRuntime` or behind
   the still-empty `MemoryStore` port.
3. Implement `RememberCare` in `goods-application` and expose
   `GoodsRuntime::request_care_and_remember`, which calls the unchanged
   `request_care` and then remembers any resulting Care Action, without
   altering `observe_and_assess`, `observe_and_identify_needs` or
   `request_care`.
4. Keep the CLI demo's default, bundled run showing no Need and therefore no
   remembered episode, preserving the calm Phase 1 reference experience;
   exercise the accumulation behavior through tests with crafted
   observations.

### Non-goals

This proposal does not implement or authorize any persistence, database or
file-backed adapter, a `MemoryStore` implementation, a retention or eviction
policy, Outcome/Verification/Learning (Phase 5), autonomous business action,
or any frontend change (`apps/goods-garden-web/` is untouched).

### Decision: Memory is transient and in-process, not a store

`GoodsMemory` is a plain `Vec<MemoryRecord>` wrapper with `remember` and
`records` methods; it implements no trait, has no adapter, and is never
serialized to disk. It is created fresh by the CLI (`GoodsMemory::new()`)
and passed by `&mut` reference into
`GoodsRuntime::request_care_and_remember`. This sidesteps the retention
question entirely rather than answering it with an unreviewed default (e.g.
an arbitrary record cap or TTL), consistent with the Trust Model's "Unknown
over fabrication."

### Decision: a Memory Record does not judge the Care Action

`MemoryRecord { state, action }` only binds the State that prompted a Need
to the Care Action that responded to it. It adds no "resolved"/"unresolved"
judgment, no comparison against a later State, and no score. That comparison
— whether yesterday's Care actually helped — is explicitly reserved for
Phase 5 ("Care Action → Outcome → Verification → Learning" per
`docs/phases/phase-5-learning.md`), so this proposal does not pre-empt it.

### Alternatives considered

**Giving `GoodsMemory` a bounded capacity (e.g., keep only the last N
records):** rejected; any specific number would be an invented retention
policy with no evidence behind it, which is exactly what Phase 4's own goal
tells us not to do.

**Implementing the `MemoryStore` port now (e.g., an in-memory or file-backed
store):** rejected as premature; Phase 4's own phase document scopes Memory
to "Relationship Memory" without a persistence requirement, and adding a
store now would invent retention/lifecycle questions (when is a stored
record loaded, when is it dropped) before any later phase asks for them.

**Recording a "result" field on `MemoryRecord` describing whether the Need
was resolved:** rejected; evaluating an outcome is Phase 5's job, and having
Memory pre-judge it would blur the boundary the phase docs draw between
"remembering" and "verifying/learning."

### Known facts, inferences and unknowns

- **KNOWN:** `GoodsMemory` was an empty placeholder reserved for this phase;
  the Phase 1-3 State/Need/Care types are unaffected by this proposal.
- **KNOWN:** the only way to accumulate `MemoryRecord`s across multiple
  observations is for the caller to keep the same `GoodsMemory` value and
  pass it by `&mut` reference; the current CLI demo still runs once per
  process, so within one run there is at most one record, but the repeated-
  call test proves accumulation without eviction.
- **INFERRED:** a transient, in-process Memory is sufficient to demonstrate
  the Relationship Memory concept without answering unresolved retention
  questions.
- **UNKNOWN:** real retention/expiry policy, real persistence requirements,
  and how Memory should behave across the multi-day scenario named in
  `docs/phases/phase-6-seven-day-life.md`; these remain `UNKNOWN`/out of
  scope until a later, evidence-bearing Work Item addresses them.
- **UNAVAILABLE:** real external Memory/persistence systems in this
  repository.

### Review gate and acceptance

This document is `PROPOSED`. Approval should confirm:

1. Memory in Phase 4 is transient and in-process only, with no persistence,
   retention or eviction policy of any kind.
2. `MemoryRecord` does not judge or score the Care Action it records.
3. `state::goods_state`, the Phase 2 Need model, the Phase 3 Care model and
   the frontend remain untouched.

## 日本語

### 背景と課題

Phase 3 は good に explainable な Care model（CareRequest、Caregiver、HumanFeedback、CareAction）を
与えた。`crates/goods-domain/src/memory/memory.rs` は空プレースホルダーのまま残されていた。
`docs/phases/phase-4-memory.md` は Relationship Memory を挙げ、retention policy を創作しないよう
明示的に警告している。

repository 自身の governance（`AGENTS.md`）は memory retention を human decision を要する protected
semantic area として扱う。この制約を守る最も安全な方法は、本 phase では retention policy を一切定義
しないことである：ここでの Memory は永続化・expiry・eviction のない一時的な in-process append-only log
であり、答えるべき retention の question 自体が残らない——単に対象外とする。

### 目標

1. Phase 1-3 の State/Need/Care model を変更せず、`goods-domain` に `GoodsMemory` と `MemoryRecord`
   を追加する。
2. Memory を永続化のない plain な append-only in-process 値にする：`GoodsMemory` は呼び出し元（本 phase
   では CLI）が所有し、繰り返しの observation をまたいで受け渡す。`GoodsRuntime` の内部や、依然として空の
   `MemoryStore` port の裏には保持しない。
3. `goods-application` に `RememberCare` を実装し、既存の `request_care` を呼び出した上で結果の Care
   Action を記憶する `GoodsRuntime::request_care_and_remember` を公開する。`observe_and_assess`、
   `observe_and_identify_needs`、`request_care` は変更しない。
4. CLI demo の同梱デフォルト実行では Need（したがって記憶される episode）が発生しないままにし、Phase 1
   の落ち着いた reference 体験を保つ。蓄積の挙動は crafted な observation を使う test で検証する。

### 非目標

本提案は persistence、database、file-backed adapter、`MemoryStore` の実装、retention や eviction
policy、Outcome/Verification/Learning（Phase 5）、自律 business action、frontend の変更（
`apps/goods-garden-web/` は無変更）を実装・許可しない。

### 決定：Memory は一時的で in-process、store ではない

`GoodsMemory` は `remember` と `records` method を持つ単純な `Vec<MemoryRecord>` wrapper であり、
trait を実装せず、adapter を持たず、disk に serialize されることもない。CLI が `GoodsMemory::new()` で
新規作成し、`GoodsRuntime::request_care_and_remember` に `&mut` reference で渡す。これにより、
未 review の default（例えば恣意的な record 上限や TTL）で答えるのではなく、retention の question 自体を
完全に回避する。Trust Model の「Unknown over fabrication」に一致する。

### 決定：Memory Record は Care Action を判断しない

`MemoryRecord { state, action }` は、Need を促した State と、それに応答した Care Action を結びつける
だけである。「resolved」/「unresolved」の判断も、後の State との比較も、スコアも加えない。昨日の Care が
実際に効果があったかという比較は明示的に Phase 5（`docs/phases/phase-5-learning.md` の
「Care Action → Outcome → Verification → Learning」）に予約されており、本提案はそれを先取りしない。

### 検討した代替案

**`GoodsMemory` に有限の容量を持たせる**（例：直近 N 件のみ保持）：不採用。特定の数値は根拠のない
retention policy の発明そのものであり、まさに Phase 4 自身の goal が禁じていることである。

**今 `MemoryStore` port を実装する**（in-memory または file-backed store）：時期尚早として不採用。
Phase 4 自身の phase document は Memory を永続化要件なしの「Relationship Memory」に限定しており、
今 store を追加すると、後続 phase が求める前に retention/lifecycle の question（保存された record は
いつ読み込まれ、いつ破棄されるか）を発明することになる。

**`MemoryRecord` に Need が解決したかを示す「result」field を記録する**：不採用。outcome の評価は
Phase 5 の仕事であり、Memory がそれを先取りして判断すると、phase doc が引く「記憶する」ことと
「verify/learn する」ことの境界を曖昧にする。

### Known、inference、unknown

- **KNOWN：** `GoodsMemory` は本 phase 用に予約された空プレースホルダーであり、Phase 1-3 の
  State/Need/Care type は本提案の影響を受けない。
- **KNOWN：** 複数の observation をまたいで `MemoryRecord` を蓄積する唯一の方法は、呼び出し元が同じ
  `GoodsMemory` 値を保持し `&mut` reference で渡すことである。現在の CLI demo は依然として1プロセスに
  つき1回しか実行しないため、1回の実行内では最大1件の記録になるが、繰り返し呼び出す test が eviction
  なしの蓄積を証明する。
- **INFERRED：** 一時的な in-process Memory で、未解決の retention question に答えることなく
  Relationship Memory の concept を示すには十分である。
- **UNKNOWN：** real な retention/expiry policy、real な persistence 要件、`docs/phases/phase-6-seven-day-life.md`
  が挙げる複数日にまたがる scenario で Memory がどう振る舞うべきか。これらは、後続の evidence を伴う
  Work Item が扱うまで `UNKNOWN`/対象外のままとする。
- **UNAVAILABLE：** この repository 内の real external Memory/persistence system。

### Review gate と acceptance

この文書は `PROPOSED` である。承認では次を確認する。

1. Phase 4 の Memory が一時的・in-process のみであり、いかなる persistence、retention、eviction
   policy も持たないこと。
2. `MemoryRecord` が記録する Care Action を判断・採点しないこと。
3. `state::goods_state`、Phase 2 Need model、Phase 3 Care model、frontend が無変更のままであること。

## 中文

### 背景与问题

Phase 3 为商品提供了可解释的 Care 模型（CareRequest、Caregiver、HumanFeedback、CareAction）。
`crates/goods-domain/src/memory/memory.rs` 被保留为空占位符。`docs/phases/phase-4-memory.md`
列出了 Relationship Memory，并明确警告不要编造保留策略。

仓库自身的治理（`AGENTS.md`）将 memory 保留视为需要人类决策的受保护语义区域。遵守该约束最安全的方式
是本阶段完全不定义保留策略：这里的 Memory 是无持久化、无过期、无淘汰的临时进程内仅追加日志，因此没有
遗留任何未回答的保留问题——它只是不在范围内。

### 目标

1. 在不修改 Phase 1-3 State/Need/Care 模型的前提下，为 `goods-domain` 增加 `GoodsMemory` 与
   `MemoryRecord`。
2. 使 Memory 成为无持久化的普通仅追加进程内值：`GoodsMemory` 由调用方（本阶段为 CLI）拥有，跨多次
   observation 传递，而不保存在 `GoodsRuntime` 内部或仍为空的 `MemoryStore` port 之后。
3. 在 `goods-application` 中实现 `RememberCare`，并公开 `GoodsRuntime::request_care_and_remember`，
   它调用不变的 `request_care` 并记住其产生的 Care Action，且不改动 `observe_and_assess`、
   `observe_and_identify_needs` 或 `request_care`。
4. 保持 CLI demo 默认内置运行不产生 Need、因而不记住任何事件，维持 Phase 1 平静的参考体验；通过带有
   构造 observation 的测试验证累积行为。

### 非目标

本提案不实现或授权任何持久化、数据库或文件支持的 adapter、`MemoryStore` 实现、保留或淘汰策略、
Outcome/Verification/Learning（Phase 5）、自主经营行动，也不涉及任何前端改动（`apps/goods-garden-web/`
保持不变）。

### 决策：Memory 是临时且进程内的，而非存储

`GoodsMemory` 是一个带有 `remember` 与 `records` 方法的简单 `Vec<MemoryRecord>` 包装器；不实现任何
trait，没有 adapter，也从不序列化到磁盘。由 CLI 通过 `GoodsMemory::new()` 新建，并以 `&mut` 引用传入
`GoodsRuntime::request_care_and_remember`。这完全绕开了保留问题，而不是用一个未经评审的默认值（例如
任意的记录上限或 TTL）来回答它，符合 Trust Model 的“未知优于臆造”。

### 决策：Memory Record 不评判 Care Action

`MemoryRecord { state, action }` 只是将促成 Need 的 State 与响应它的 Care Action 绑定在一起，不附加
“已解决”/“未解决”的判断，不与后续 State 比较，也不打分。昨天的 Care 是否真的有效这一比较，明确保留给
Phase 5（`docs/phases/phase-5-learning.md` 中的“Care Action → Outcome → Verification → Learning”），
本提案不会提前实现它。

### 已考虑的替代方案

**给 `GoodsMemory` 设置有限容量**（例如只保留最近 N 条记录）：不采用；任何具体数字都是缺乏证据的臆造
保留策略，而这正是 Phase 4 自身目标所禁止的。

**现在就实现 `MemoryStore` port**（内存或文件支持的存储）：作为过早设计不采用；Phase 4 自身的阶段文档
将 Memory 限定为无持久化要求的“Relationship Memory”，现在添加存储会在后续阶段真正需要之前就臆造
保留/生命周期问题（存储的记录何时加载、何时丢弃）。

**在 `MemoryRecord` 上记录说明 Need 是否已解决的“result”字段**：不采用；评估结果是 Phase 5 的工作，
让 Memory 提前判断会模糊阶段文档在“记忆”与“验证/学习”之间划定的边界。

### 已知、推断与未知

- **KNOWN：** `GoodsMemory` 是为本阶段保留的空占位符；Phase 1-3 的 State/Need/Care 类型不受本提案影响。
- **KNOWN：** 跨多次 observation 累积 `MemoryRecord` 的唯一方式是调用方保持同一个 `GoodsMemory` 值并
  以 `&mut` 引用传递；当前 CLI demo 每进程仍只运行一次，因此单次运行内最多产生一条记录，但重复调用的
  测试证明了无淘汰的累积。
- **INFERRED：** 临时的进程内 Memory 足以展示 Relationship Memory 概念，而无需回答尚未解决的保留问题。
- **UNKNOWN：** 真实的保留/过期策略、真实的持久化需求，以及 `docs/phases/phase-6-seven-day-life.md`
  中提到的多日场景下 Memory 应如何表现；在后续携带证据的 Work Item 处理之前，这些保持为
  `UNKNOWN`/不在范围内。
- **UNAVAILABLE：** 本仓库中真实的外部 Memory/持久化系统。

### Review gate 与验收

本文档状态为 `PROPOSED`。批准应确认：

1. Phase 4 中的 Memory 仅为临时且进程内，不带有任何持久化、保留或淘汰策略。
2. `MemoryRecord` 不评判或评分其记录的 Care Action。
3. `state::goods_state`、Phase 2 Need 模型、Phase 3 Care 模型与前端保持不变。
