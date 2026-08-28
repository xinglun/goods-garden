# Phase 1 — First Living Goods

## English

### Goal

Give one synthetic tuna-mayo reference object its first reviewable State. The
endpoint is the answer “Am I okay?”; this phase does not implement asking for
care.

### Implemented boundary

Phase 1 includes only:

- Goods Identity: `species` and `individual_id`.
- Goods Profile: display name and an expected lifetime in hours.
- Observation: source label, observation time and observed age.
- Expectation: the profile-derived maximum age.
- Goods State: identity, observation, expectation and health assessment.
- Health Assessment: healthy/unhealthy status with an explanation.

The bounded rule is that an observed age at or below the profile expectation is
healthy; an age above it is unhealthy. This is a minimal, reviewable domain
rule, not a complete freshness, inventory or sales model.

### Local demo and data boundary

Run the demo from the repository root:

```bash
cargo run -p goods-garden-cli -- demo
```

The path is:

```text
examples/tuna-mayo/observation.example.txt
    ↓
DemoObservationSource
    ↓
ObservationSource port
    ↓
GoodsRuntime
    ↓
Goods State + Health Assessment
    ↓
CLI output
```

The fixture is labelled `synthetic-example`. It is not real POS data, SEJ
data, an external contract or an integration adapter. The runtime accepts a
Goods value and its profile; it must remain valid when the profile is replaced.

### Exit criteria

- A local user can run the demo and see identity, observation, expectation and
  current health.
- The within-expectation and beyond-expectation cases are tested.
- A different Goods Profile uses the same runtime path.
- Need, Care, Memory, Learning, autonomous action and real external data are
  not implemented.

## 日本語

### Goal

synthetic な tuna-mayo reference object に最初の reviewable な State を与える。終点は「今、自分は大丈夫か」に答えることであり、
この phase では care を求める挙動を実装しない。

### Implemented boundary

Phase 1 に含めるのは次だけである。

- Goods Identity: `species` と `individual_id`。
- Goods Profile: display name と hours 単位の expected lifetime。
- Observation: source label、observation time、observed age。
- Expectation: profile から得る maximum age。
- Goods State: identity、observation、expectation、health assessment。
- Health Assessment: explanation 付き healthy/unhealthy status。

bounded rule は observed age が profile expectation 以下なら healthy、超えれば unhealthy とする。これは minimal で reviewable な
domain rule であり、完全な freshness、inventory、sales model ではない。

### Local demo と data boundary

repository root から demo を実行する。

```bash
cargo run -p goods-garden-cli -- demo
```

path は次の通りである。

```text
examples/tuna-mayo/observation.example.txt
    ↓
DemoObservationSource
    ↓
ObservationSource port
    ↓
GoodsRuntime
    ↓
Goods State + Health Assessment
    ↓
CLI output
```

fixture は `synthetic-example` と明記する。real POS data、SEJ data、external contract、integration adapter ではない。runtime は Goods
value と profile を受け取り、profile を置き換えても同じ path が成立しなければならない。

### Exit criteria

- local user が demo を実行し、identity、observation、expectation、current health を確認できる。
- expectation 内と expectation 超過の case を test する。
- 別の Goods Profile でも同じ runtime path を使う。
- Need、Care、Memory、Learning、自律 action、real external data は実装しない。

## 中文

### Goal

让一个 synthetic 金枪鱼蛋黄酱饭团 reference object 第一次拥有可 review 的 State。终点是回答“我现在好不好”；本阶段不实现求助。

### Implemented boundary

Phase 1 只包含：

- Goods Identity：`species` 和 `individual_id`。
- Goods Profile：display name 和以小时计的 expected lifetime。
- Observation：source label、观察时间和 observed age。
- Expectation：从 profile 得到的 maximum age。
- Goods State：identity、observation、expectation 和 health assessment。
- Health Assessment：带解释的 healthy/unhealthy 状态。

有边界规则是：observed age 小于等于 profile expectation 时为 healthy，超过时为 unhealthy。这是最小的、可 review 的领域规则，不是完整的
保鲜、库存或销售模型。

### 本地 demo 与数据边界

从仓库根目录运行 demo：

```bash
cargo run -p goods-garden-cli -- demo
```

路径如下：

```text
examples/tuna-mayo/observation.example.txt
    ↓
DemoObservationSource
    ↓
ObservationSource port
    ↓
GoodsRuntime
    ↓
Goods State + Health Assessment
    ↓
CLI output
```

fixture 明确标记为 `synthetic-example`。它不是真实 POS 数据、SEJ 数据、外部 contract 或集成 adapter。runtime 接收 Goods value 和其
profile；替换 profile 后必须仍能使用相同的 runtime 路径。

### Exit criteria

- 本地用户可以运行 demo，并看到 identity、observation、expectation 和当前健康状态。
- 测试 expectation 内和 expectation 超出的情况。
- 使用不同 Goods Profile 时仍走同一 runtime 路径。
- 不实现 Need、Care、Memory、Learning、自主行动和真实外部数据接入。
