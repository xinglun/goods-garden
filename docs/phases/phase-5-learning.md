# Phase 5 — Verification & Learning

## English

### Goal

Goods judges whether yesterday's care helped. Establish
`Care Action → Outcome → Verification → Learning` on top of the unchanged
Phase 1-4 State/Need/Care/Memory model, with traceable evidence and explicit
authority: Learning only records a reviewable observation and never adjusts
a rule by itself.

### Implemented boundary

Phase 5 includes only:

- Outcome: compares whether the NeedKind(s) a CareAction addressed are still
  present in a follow-up Need Assessment. `OutcomeStatus` is `Resolved`
  (none of the addressed Need kinds remain) or `Unresolved` (at least one
  remains). This is a factual comparison, not a judgment of whether the
  Caregiver's decision was good or sufficient.
- Learning: a reviewable, plain-language statement derived from an Outcome.
  Learning never adjusts a `Urgency` threshold, `GoodsProfile` field or any
  other rule by itself; whether to act on it remains a separate human
  decision, out of scope here.
- The generic `Evidence` placeholder remains provisional and unimplemented:
  Outcome's own fields (the CareAction, the follow-up State and Need
  Assessment) already carry the traceability this phase needs.

The bounded rule is that verification always compares against a follow-up
observation supplied by the same `ObservationSource` port used elsewhere;
Learning is produced for exactly one Outcome at a time, with no aggregation
across multiple episodes.

### Local demo and data boundary

Run the demo from the repository root:

```bash
cargo run -p goods-garden-cli -- demo
```

The path extends Phase 1-4's:

```text
examples/tuna-mayo/observation.example.txt
examples/tuna-mayo/human_feedback.example.txt
    ↓
DemoObservationSource / DemoHumanFeedbackSource
    ↓
ObservationSource / HumanFeedbackSource ports
    ↓
GoodsRuntime::request_care_and_remember
    ↓
Goods State + Health Assessment (unchanged)
Need Assessment (unchanged)
Care Request + Care Action (unchanged)
Memory (unchanged)
    ↓
GoodsRuntime::verify_and_learn (new, only when a Care Action exists)
    ↓
Outcome + Learning
    ↓
CLI output
```

The bundled fixtures remain tuned so the demo shows no Need, and therefore no
Care Action and nothing to verify, preserving the calm Phase 1 reference
experience. The resolved and unresolved verification paths are exercised
only by the automated test suite with two crafted observations (an initial
one and a follow-up one).

### Exit criteria

- A local user can run the demo and see, alongside the unchanged Phase 1-4
  output, whether a Care episode was pending verification and, if so, the
  Outcome status and Learning statement.
- Both the resolved and unresolved verification cases are tested.
- Learning never mutates a threshold, profile field or other rule; no
  autonomous business action is implemented.

## 日本語

### Goal

Goods が昨日の care に効果があったか判断する。不変の Phase 1-4 State/Need/Care/Memory model の上に
`Care Action → Outcome → Verification → Learning` を、traceable evidence と explicit authority
とともに作る：Learning は reviewable な観測を記録するだけで、自らルールを調整することはない。

### Implemented boundary

Phase 5 に含めるのは次だけである。

- Outcome: CareAction が対象とした NeedKind が follow-up の Need Assessment にまだ存在するかを比較する。
  `OutcomeStatus` は `Resolved`（対象とした Need kind が1つも残っていない）または `Unresolved`（少なくとも
  1つ残っている）。これは事実の比較であり、Caregiver の決定が良かった・十分だったかの判断ではない。
- Learning: Outcome から導かれる、reviewable で plain-language な statement。Learning は `Urgency`
  閾値、`GoodsProfile` field、その他のルールを自ら調整することは決してない。それに基づいて行動するかは
  別の human decision であり、本 phase の対象外である。
- 汎用の `Evidence` placeholder は引き続き provisional・未実装のままとする：Outcome 自身の field
  （CareAction、follow-up の State と Need Assessment）が既に本 phase に必要な traceability を持つ。

bounded rule は、verification は常に他の箇所と同じ `ObservationSource` port から供給される follow-up
observation と比較するというもので、Learning は一度に正確に1つの Outcome に対して生成され、複数 episode
をまたぐ集約は行わない。

### Local demo と data boundary

repository root から demo を実行する。

```bash
cargo run -p goods-garden-cli -- demo
```

path は Phase 1-4 を拡張する。

```text
examples/tuna-mayo/observation.example.txt
examples/tuna-mayo/human_feedback.example.txt
    ↓
DemoObservationSource / DemoHumanFeedbackSource
    ↓
ObservationSource / HumanFeedbackSource ports
    ↓
GoodsRuntime::request_care_and_remember
    ↓
Goods State + Health Assessment（不変）
Need Assessment（不変）
Care Request + Care Action（不変）
Memory（不変）
    ↓
GoodsRuntime::verify_and_learn（新規、Care Action がある場合のみ）
    ↓
Outcome + Learning
    ↓
CLI output
```

同梱 fixture は引き続き Need（したがって Care Action も verify 対象も）が発生しない値に調整し、Phase 1 の
落ち着いた reference 体験を保つ。resolved と unresolved の verification 経路は、自動テストが2つの crafted
observation（初回と follow-up）で検証するのみである。

### Exit criteria

- local user が demo を実行し、不変の Phase 1-4 出力に加え、verify 待ちの Care episode の有無と、あれば
  Outcome status と Learning statement を確認できる。
- resolved と unresolved の両方の verification case を test する。
- Learning は閾値、profile field、その他のルールを一切変更せず、自律 business action は実装しない。

## 中文

### Goal

商品判断昨天的照料是否有效。在不变的 Phase 1-4 State/Need/Care/Memory 模型之上，建立
`Care Action → Outcome → Verification → Learning`，并要求可追溯证据和明确权限：Learning 只记录可
review 的观察，从不自行调整规则。

### Implemented boundary

Phase 5 只包含：

- Outcome：比较 CareAction 所针对的 NeedKind 是否仍存在于后续的 Need Assessment 中。`OutcomeStatus`
  为 `Resolved`（所针对的 Need kind 一个都不再存在）或 `Unresolved`（至少一个仍然存在）。这是基于事实的
  比较，不是对 Caregiver 决定是否恰当或充分的评判。
- Learning：从 Outcome 得出的、可 review 的自然语言陈述。Learning 从不自行调整 `Urgency` 阈值、
  `GoodsProfile` 字段或其他规则；是否据此采取行动是另一项人类决策，不在本阶段范围内。
- 通用的 `Evidence` 占位符继续保持 provisional、未实现状态：Outcome 自身的字段（CareAction、后续的
  State 与 Need Assessment）已经携带了本阶段所需的可追溯性。

有边界规则是：verification 始终与其他地方使用的同一个 `ObservationSource` port 提供的后续观测进行比较；
Learning 每次只针对一个 Outcome 生成，不跨多个事件聚合。

### 本地 demo 与数据边界

从仓库根目录运行 demo：

```bash
cargo run -p goods-garden-cli -- demo
```

路径在 Phase 1-4 基础上扩展：

```text
examples/tuna-mayo/observation.example.txt
examples/tuna-mayo/human_feedback.example.txt
    ↓
DemoObservationSource / DemoHumanFeedbackSource
    ↓
ObservationSource / HumanFeedbackSource ports
    ↓
GoodsRuntime::request_care_and_remember
    ↓
Goods State + Health Assessment（不变）
Need Assessment（不变）
Care Request + Care Action（不变）
Memory（不变）
    ↓
GoodsRuntime::verify_and_learn（新增，仅当存在 Care Action 时）
    ↓
Outcome + Learning
    ↓
CLI output
```

内置 fixture 继续调整为不产生 Need（因而不产生 Care Action 与待验证事项），以保持 Phase 1 平静的参考体验。
已解决与未解决的验证路径仅由自动化测试使用两个构造的 observation（初始与后续）验证。

### Exit criteria

- 本地用户运行 demo，能在不变的 Phase 1-4 输出之外，看到是否有待验证的 Care 事件，以及（若有）Outcome
  状态与 Learning 陈述。
- 测试已解决与未解决两种验证情况。
- Learning 从不修改阈值、profile 字段或其他规则；不实现自主经营行动。
