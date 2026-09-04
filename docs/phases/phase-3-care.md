# Phase 3 — Care

## English

### Goal

Goods identifies when it cannot resolve a Need alone and who may help. Add
CareRequest, Caregiver, Human Feedback and CareAction on top of the unchanged
Phase 1/2 State and Need model. This is the first two-way interaction:
`Goods ↔ Human`.

### Implemented boundary

Phase 3 includes only:

- CareRequest: raised whenever a Need Assessment identifies at least one
  Need. It carries the identified Needs, any Need Conflict, a requested
  Caregiver role (a single fixed `"store staff"` role; no role-routing or
  permission model), and a plain-language explanation. A CareRequest never
  decides or performs an action; it only asks.
- Caregiver: a minimal `role` and `display_name` identifying who may help.
- Human Feedback: the Caregiver's `decision` and `provided_at`. This is
  always external input read through a `HumanFeedbackSource` port; the
  domain and application layers never invent, infer or synthesize it,
  matching the Trust Model's "evidence over fluency" and "human authority
  over inferred authority" principles.
- CareAction: a traceable record binding a CareRequest to the Human Feedback
  that resolved it. Its explanation only restates what the Caregiver said; it
  adds no independent judgment about whether the decision was correct.

The bounded rule is that every identified Need raises exactly one
CareRequest; when there is no Need, no CareRequest is raised and no Human
Feedback is requested.

### Local demo and data boundary

Run the demo from the repository root:

```bash
cargo run -p goods-garden-cli -- demo
```

The path extends Phase 1/2's:

```text
examples/tuna-mayo/observation.example.txt
examples/tuna-mayo/human_feedback.example.txt
    ↓
DemoObservationSource / DemoHumanFeedbackSource
    ↓
ObservationSource / HumanFeedbackSource ports
    ↓
GoodsRuntime::request_care
    ↓
Goods State + Health Assessment (unchanged)
Need Assessment (unchanged)
Care Request + Care Action (new)
    ↓
CLI output
```

Both fixtures remain labelled `synthetic-example`; the bundled fixtures are
tuned so the demo shows no Need, and therefore no Care Request or Care
Action, preserving the calm Phase 1 reference experience. The Care path is
exercised by the automated test suite with crafted observations. A real
interactive prompt or provider-backed Caregiver channel is not implemented.

### Exit criteria

- A local user can run the demo and see, alongside the unchanged Phase 1/2
  output, whether a Care Request was raised and, when Human Feedback is
  available, the resulting Care Action.
- The no-need (no Care Request), single-Need, and Need-Conflict cases are
  tested.
- Memory, Learning, autonomous action and real external Caregiver systems are
  not implemented.

## 日本語

### Goal

Goods が自分だけでは Need を解決できず、誰が助けられるかを知る。不変の Phase 1/2 State/Need model の上に
CareRequest、Caregiver、Human Feedback、CareAction を追加する。初めて `Goods ↔ Human` の双方向 interaction が生まれる。

### Implemented boundary

Phase 3 に含めるのは次だけである。

- CareRequest: Need Assessment が1つ以上の Need を識別した場合に必ず発生する。識別された Need 群、Need
  Conflict の有無、要求する Caregiver role（単一の固定 role `"store staff"`。role-routing や permission
  model は実装しない）、plain-language な explanation を持つ。CareRequest は決定も行動も行わず、ただ求めるだけである。
- Caregiver: 誰が助けられるかを示す最小の `role` と `display_name`。
- Human Feedback: Caregiver の `decision` と `provided_at`。これは常に `HumanFeedbackSource` port
  経由の外部入力であり、domain と application layer はこれを発明・推論・合成しない。Trust Model の
  「evidence over fluency」「human authority over inferred authority」原則に一致する。
- CareAction: CareRequest とそれを解決した Human Feedback を結びつける traceable な記録。explanation は
  Caregiver が実際に言ったことを言い換えるだけで、その決定が正しいかどうかの独自の判断は加えない。

bounded rule は、識別された全ての Need が正確に1つの CareRequest を発生させるというもので、Need が無い場合は
CareRequest も Human Feedback の要求も発生しない。

### Local demo と data boundary

repository root から demo を実行する。

```bash
cargo run -p goods-garden-cli -- demo
```

path は Phase 1/2 を拡張する。

```text
examples/tuna-mayo/observation.example.txt
examples/tuna-mayo/human_feedback.example.txt
    ↓
DemoObservationSource / DemoHumanFeedbackSource
    ↓
ObservationSource / HumanFeedbackSource ports
    ↓
GoodsRuntime::request_care
    ↓
Goods State + Health Assessment（不変）
Need Assessment（不変）
Care Request + Care Action（新規）
    ↓
CLI output
```

両 fixture は引き続き `synthetic-example` と明記し、同梱 fixture は Need（したがって Care Request・Care
Action も）が発生しない値に調整する。Care の経路は自動テストが crafted な observation で検証する。real な
対話式 prompt や provider-backed Caregiver channel は実装しない。

### Exit criteria

- local user が demo を実行し、不変の Phase 1/2 出力に加え、Care Request の有無と、Human Feedback が
  利用可能な場合の Care Action を確認できる。
- need無し（Care Request無し）・単一Need・Need Conflict の case を test する。
- Memory、Learning、自律 action、real external Caregiver system は実装しない。

## 中文

### Goal

商品识别自己无法解决 Need 的情况以及谁可以帮助。在不变的 Phase 1/2 State/Need 模型之上，增加 CareRequest、
Caregiver、Human Feedback、CareAction，第一次形成 `Goods ↔ Human` 双向交互。

### Implemented boundary

Phase 3 只包含：

- CareRequest：只要 Need Assessment 识别出至少一个 Need 就必定产生。携带已识别的 Need 集合、是否存在
  Need Conflict、请求的 Caregiver role（单一固定角色 `"store staff"`，不实现角色路由或权限模型）与
  自然语言 explanation。CareRequest 从不做决定或执行行动，只是提出请求。
- Caregiver：标明谁可以提供帮助的最小 `role` 与 `display_name`。
- Human Feedback：Caregiver 的 `decision` 与 `provided_at`。这始终是通过 `HumanFeedbackSource` port
  获取的外部输入；领域与应用层从不臆造、推断或合成它，符合 Trust Model 的“证据优先于流畅表达”与
  “人类权限优先于推断权限”原则。
- CareAction：将 CareRequest 与解决它的 Human Feedback 绑定的可追溯记录。其 explanation 只是复述
  Caregiver 实际说过的话，不附加关于该决定是否正确的独立判断。

有边界规则是：每一个被识别的 Need 都会精确产生一个 CareRequest；没有 Need 时，既不产生 CareRequest 也不请求
Human Feedback。

### 本地 demo 与数据边界

从仓库根目录运行 demo：

```bash
cargo run -p goods-garden-cli -- demo
```

路径在 Phase 1/2 基础上扩展：

```text
examples/tuna-mayo/observation.example.txt
examples/tuna-mayo/human_feedback.example.txt
    ↓
DemoObservationSource / DemoHumanFeedbackSource
    ↓
ObservationSource / HumanFeedbackSource ports
    ↓
GoodsRuntime::request_care
    ↓
Goods State + Health Assessment（不变）
Need Assessment（不变）
Care Request + Care Action（新增）
    ↓
CLI output
```

两个 fixture 继续明确标记为 `synthetic-example`；内置 fixture 调整为不产生 Need（因此也不产生 Care
Request 或 Care Action），以保持 Phase 1 平静的参考体验。Care 路径由自动化测试使用构造的 observation 验证。
不实现真实的交互式提示或 provider-backed Caregiver 渠道。

### Exit criteria

- 本地用户运行 demo，能在不变的 Phase 1/2 输出之外，看到是否产生了 Care Request，以及在 Human Feedback
  可用时产生的 Care Action。
- 测试无 need（无 Care Request）、单一 Need 与 Need Conflict 三种情况。
- 不实现 Memory、Learning、自主行动和真实外部 Caregiver 系统。
