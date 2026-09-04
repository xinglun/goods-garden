# Phase 4 — Memory

## English

### Goal

Goods remembers what happened, who helped, what action was taken and what
the result was. Establish Relationship Memory on top of the unchanged
Phase 1-3 State/Need/Care model, without inventing retention policy.

### Implemented boundary

Phase 4 includes only:

- MemoryRecord: binds the State that prompted a Need to the Care Action that
  responded to it (which itself already carries the Caregiver and the Human
  Feedback). A Memory Record never judges whether the Care Action worked;
  comparing a later State against this record is Phase 5's Verification and
  Learning.
- GoodsMemory: an append-only, in-process collection of Memory Records for
  one good. It has no persistence of its own — it is a value the caller
  (the CLI in this phase) owns and threads across repeated observations.
- No retention or eviction policy is defined: nothing is ever removed or
  expired. This is a deliberate choice, not an oversight — Phase 4's own
  goal explicitly says "without inventing retention policy," and the
  `MemoryStore` port remains an empty placeholder with no database or
  file-backed adapter.

The bounded rule is that a Care episode is remembered exactly when a Care
Action was recorded; when there is no Need (and therefore no Care Request or
Care Action), nothing is remembered.

### Local demo and data boundary

Run the demo from the repository root:

```bash
cargo run -p goods-garden-cli -- demo
```

The path extends Phase 1-3's:

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
Memory (new: how many episodes remembered in this run)
    ↓
CLI output
```

The bundled fixtures remain tuned so the demo shows no Need, and therefore no
Care Action and nothing remembered, preserving the calm Phase 1 reference
experience. The Memory path — including that repeated episodes accumulate
without evicting a prior record — is exercised only by the automated test
suite with crafted observations.

### Exit criteria

- A local user can run the demo and see how many Care episodes have been
  remembered in this run, alongside the unchanged Phase 1-3 output.
- The no-episode, single-episode, and repeated-episode (append-only, nothing
  evicted) cases are tested.
- Learning, Verification, Outcome, persistence and autonomous action are not
  implemented.

## 日本語

### Goal

Goods が何が起き、誰が助け、どの action を行い、結果が何だったかを記憶する。不変の Phase 1-3 State/Need/Care model
の上に、retention policy を創作せず Relationship Memory を作る。

### Implemented boundary

Phase 4 に含めるのは次だけである。

- MemoryRecord: Need を促した State と、それに応答した Care Action（それ自体が既に Caregiver と Human
  Feedback を持つ）を結びつける。Memory Record は Care Action が効果を発揮したかを決して判断しない。後の
  State をこの記録と比較することは Phase 5 の Verification と Learning の仕事である。
- GoodsMemory: 1つの good に対する Memory Record の append-only な in-process collection。それ自体の
  永続化は持たず、呼び出し元（本 phase では CLI）が所有し、繰り返しの observation をまたいで受け渡す値である。
- retention や eviction policy は定義しない：何も削除・expire されない。これは見落としではなく意図的な選択
  である——Phase 4 自身の goal が明示的に「retention policy を創作しない」と述べており、`MemoryStore` port
  は空プレースホルダーのままで、database や file-backed adapter は実装しない。

bounded rule は、Care Action が記録された時に限りその Care episode を記憶するというもので、Need が無い場合
（したがって Care Request も Care Action も無い場合）は何も記憶されない。

### Local demo と data boundary

repository root から demo を実行する。

```bash
cargo run -p goods-garden-cli -- demo
```

path は Phase 1-3 を拡張する。

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
Memory（新規：この実行内で記憶された episode 数）
    ↓
CLI output
```

同梱 fixture は引き続き Need（したがって Care Action も記憶も）が発生しない値に調整し、Phase 1 の落ち着いた
reference 体験を保つ。Memory の経路——繰り返しの episode が既存記録を eviction せず蓄積することを含む——は
自動テストが crafted な observation で検証するのみである。

### Exit criteria

- local user が demo を実行し、不変の Phase 1-3 出力に加え、この実行内で記憶された Care episode の件数を
  確認できる。
- episode無し・単一episode・繰り返しepisode（append-onlyでeviction無し）の case を test する。
- Learning、Verification、Outcome、永続化、自律 action は実装しない。

## 中文

### Goal

商品记住发生了什么、谁帮助过、采取了什么行动以及结果是什么。在不变的 Phase 1-3 State/Need/Care 模型之上，
不编造保留策略的前提下建立 Relationship Memory。

### Implemented boundary

Phase 4 只包含：

- MemoryRecord：将促成 Need 的 State 与响应它的 Care Action（其自身已经携带 Caregiver 与 Human
  Feedback）绑定。Memory Record 从不判断 Care Action 是否奏效；将后续 State 与该记录比较是 Phase 5
  Verification 与 Learning 的工作。
- GoodsMemory：针对一个商品的 Memory Record 仅追加进程内集合。它自身没有持久化——是由调用方（本阶段为
  CLI）拥有、跨多次 observation 传递的值。
- 不定义保留或淘汰策略：任何内容都不会被删除或过期。这是刻意的选择而非疏漏——Phase 4 自身的目标明确指出
  “不编造保留策略”，`MemoryStore` port 仍为空占位符，不实现数据库或文件支持的 adapter。

有边界规则是：恰好在记录了 Care Action 时才会记住该 Care 事件；没有 Need（因而没有 Care Request 或 Care
Action）时，不记住任何内容。

### 本地 demo 与数据边界

从仓库根目录运行 demo：

```bash
cargo run -p goods-garden-cli -- demo
```

路径在 Phase 1-3 基础上扩展：

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
Memory（新增：本次运行中记住的事件数）
    ↓
CLI output
```

内置 fixture 继续调整为不产生 Need（因而不产生 Care Action 与记忆），以保持 Phase 1 平静的参考体验。Memory
路径——包括重复事件会累积而不淘汰既有记录——仅由自动化测试使用构造的 observation 验证。

### Exit criteria

- 本地用户运行 demo，能在不变的 Phase 1-3 输出之外，看到本次运行中记住了多少个 Care 事件。
- 测试无事件、单一事件与重复事件（仅追加、不淘汰）三种情况。
- 不实现 Learning、Verification、Outcome、持久化和自主行动。
