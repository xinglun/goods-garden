# Phase 6 — Seven Day Life

## English

### Goal

Product milestone: one rice ball lives for seven days. The demonstration
must include normal, anomaly, investigation, request for care, human
feedback, improvement, verification and memory. An observer should feel that
the product seems alive, without treating the metaphor as consciousness.

### Implemented boundary

Phase 6 introduces no new domain type. It scripts the unchanged Phase 1-5
Observe → Assess → Identify Need → Request/Receive Care → Remember →
Verify → Learn loop across seven synthetic days for one Goods individual,
exposed as a new `seven-day-life` CLI subcommand alongside the unchanged
`demo` subcommand.

The good's identity represents a monitored retail slot rather than a single
physical unit forced to age uninterrupted for a week: a restock resets the
observed age on a later day, which is how a real shelf position behaves.
The seven-day script is:

- Days 1, 2 and 5: normal — within expectation on both dimensions, no Need.
- Day 3: anomaly and investigation — observed age exceeds the profile
  expectation, raising a `FreshnessConcern` Need, a Care Request and a
  synthetic Human Feedback response (a restock).
- Day 4: verification and improvement — the restocked observation no longer
  shows the `FreshnessConcern` Need, so Day 3's Care Action is verified as
  Resolved and a Learning statement is recorded.
- Day 6: a second anomaly — observed quantity falls below the profile's
  minimum, raising a `StockAvailabilityConcern` Need, a Care Request and a
  synthetic Human Feedback response (a restock order).
- Day 7: verification and improvement for Day 6's Care Action, followed by a
  final Memory tally for the week.

Every Need, Care Request, Care Action, Outcome and Learning statement is
produced by the unchanged Phase 2-5 rules; this phase only supplies the
sequence of synthetic observations and Human Feedback that drives them.

### Local demo and data boundary

Run the demo from the repository root:

```bash
cargo run -p goods-garden-cli -- seven-day-life
```

All seven days are clearly synthetic: each observation and Human Feedback
value is constructed in the CLI itself (not read from an external fixture
file, since this is a scripted milestone narrative rather than a single
reusable business fixture), and printed alongside a `(synthetic)` marker.
The existing `cargo run -p goods-garden-cli -- demo` command and its output
are unchanged.

### Exit criteria

- A local user can run `seven-day-life` and see a coherent week: calm days,
  two anomalies each answered by synthetic Human Feedback, two follow-up
  verifications, and a final memory tally.
- The existing `demo` subcommand's behavior is unchanged.
- No new domain type, autonomous action or rule adjustment is introduced.

## 日本語

### Goal

product milestone は一つのおにぎりが7日間 live すること。normal、anomaly、investigation、care
request、human feedback、improvement、verification、memory を含む demo とする。観察者が「本当に商品が
生きているみたい」と感じても、metaphor を consciousness と解釈しない。

### Implemented boundary

Phase 6 は新しい domain type を導入しない。不変の Phase 1-5 の Observe → Assess → Identify Need →
Request/Receive Care → Remember → Verify → Learn loop を、1つの Goods individual に対して7日分の
synthetic day としてスクリプト化し、既存の `demo` subcommand はそのままに、新しい `seven-day-life`
CLI subcommand として公開する。

good の identity は、1週間ぶっ通しで age する単一の物理的な個体ではなく、監視対象の小売スロットを表す：
restock により後の日で observed age がリセットされる、これは実際の棚位置の振る舞いと同じである。7日間の
script は次の通り。

- Day 1、2、5：normal——両次元とも expectation 内で Need 無し。
- Day 3：anomaly と investigation——observed age が profile expectation を超え、`FreshnessConcern`
  Need、Care Request、synthetic Human Feedback response（restock）を発生させる。
- Day 4：verification と improvement——restock 後の observation はもう `FreshnessConcern` Need を
  示さないため、Day 3 の Care Action は Resolved として verify され、Learning statement が記録される。
- Day 6：2つ目の anomaly——observed quantity が profile の minimum を下回り、
  `StockAvailabilityConcern` Need、Care Request、synthetic Human Feedback response（restock order）
  を発生させる。
- Day 7：Day 6 の Care Action の verification と improvement、続いて週全体の最終 Memory 集計。

全ての Need、Care Request、Care Action、Outcome、Learning statement は不変の Phase 2-5 の rule に
よって生成される。本 phase が供給するのはそれらを駆動する synthetic observation と Human Feedback の
系列だけである。

### Local demo と data boundary

repository root から demo を実行する。

```bash
cargo run -p goods-garden-cli -- seven-day-life
```

7日分は全て明確に synthetic である：各 observation と Human Feedback 値は CLI 自身の中で構築され
（外部 fixture file からではない。これは再利用可能な単一の business fixture ではなく、スクリプト化された
milestone narrative だからである）、`(synthetic)` マーカーとともに表示される。既存の
`cargo run -p goods-garden-cli -- demo` command とその出力は変更しない。

### Exit criteria

- local user が `seven-day-life` を実行し、一貫した1週間——平穏な日、synthetic Human Feedback で
  応答される2つの anomaly、2つの follow-up verification、最終的な memory 集計——を確認できる。
- 既存の `demo` subcommand の挙動は変更しない。
- 新しい domain type、自律 action、rule adjustment は導入しない。

## 中文

### Goal

产品里程碑是一个饭团连续活7天。演示必须经历正常、异常、调查、求助、人工反馈、改善、验证和记忆；观察者应
产生“商品真的像活着一样”的感受，但不能把隐喻当作意识。

### Implemented boundary

Phase 6 不引入任何新的领域类型。它将不变的 Phase 1-5 Observe → Assess → Identify Need →
Request/Receive Care → Remember → Verify → Learn 循环，为一个 Goods individual 编排为七个 synthetic
day 的脚本，作为新的 `seven-day-life` CLI 子命令公开，既有的 `demo` 子命令保持不变。

商品的 identity 代表的是被监控的零售货位，而不是被迫连续一周老化的单一实体：后续某天的 restock 会重置
observed age，这正是真实货架位置的行为方式。七天脚本如下：

- 第1、2、5天：正常——两个维度都在 expectation 之内，没有 Need。
- 第3天：异常与调查——observed age 超出 profile expectation，引发 `FreshnessConcern` Need、Care
  Request 与 synthetic Human Feedback 响应（补货）。
- 第4天：验证与改善——补货后的 observation 不再显示 `FreshnessConcern` Need，因此第3天的 Care
  Action 被验证为 Resolved，并记录 Learning 陈述。
- 第6天：第二次异常——observed quantity 低于 profile 的最低值，引发 `StockAvailabilityConcern`
  Need、Care Request 与 synthetic Human Feedback 响应（下补货订单）。
- 第7天：对第6天 Care Action 的验证与改善，随后是本周的最终 Memory 汇总。

所有 Need、Care Request、Care Action、Outcome 与 Learning 陈述均由不变的 Phase 2-5 规则产生；本阶段
只提供驱动它们的 synthetic observation 与 Human Feedback 序列。

### 本地 demo 与数据边界

从仓库根目录运行 demo：

```bash
cargo run -p goods-garden-cli -- seven-day-life
```

七天全部明确标记为 synthetic：每个 observation 与 Human Feedback 值都在 CLI 内部构造（而非来自外部
fixture 文件，因为这是脚本化的里程碑叙事，而非可复用的单一业务 fixture），并附带 `(synthetic)`
标记打印。既有的 `cargo run -p goods-garden-cli -- demo` 命令及其输出保持不变。

### Exit criteria

- 本地用户运行 `seven-day-life`，能看到连贯的一周：平静的日子、由 synthetic Human Feedback 回应的
  两次异常、两次后续验证，以及最终的 memory 汇总。
- 既有 `demo` 子命令的行为保持不变。
- 不引入任何新的领域类型、自主行动或规则调整。
