# Goods Garden Frontend Foundation Design Specification

**Status:** PROPOSED — requires human review before implementation

**Date:** 2026-08-28

**Scope:** Same-repository frontend engineering foundation for the read-only Phase 1 State demo.

## English

### Context and problem

Goods Garden is a Rust workspace whose domain and runtime boundaries are already
defined. The next demo needs a browser surface that can present a clearly
synthetic Goods State to a person. The frontend should be easy for one developer
to run locally, while remaining independent from the Rust build graph and from
unknown external systems.

The product distinction remains firm:

- Goods Garden is the product world presented to people.
- Goods Intelligence is the technical kernel that may later sense, understand,
  express, request care and learn.

This proposal creates a frontend foundation only. It does not change the
North Star, define a new domain aggregate, or implement a real external-data
integration.

### Goals

1. Add a stable, same-repository location for frontend work at
   `apps/goods-garden-web/`.
2. Use an independent Node toolchain so frontend iteration does not require
   Cargo or a Rust-to-browser binding.
3. Make the first supported surface a responsive browser demo that renders a
   read-only, synthetic Goods State.
4. Keep the UI boundary at a view-model projection. The UI does not own Goods
   domain rules and does not call POS, database, LLM or provider code.
5. Preserve a credible future path to a desktop shell without claiming native
   mobile support prematurely.

### Non-goals and phase boundary

This proposal does not implement or authorize sales analysis, anomaly
detection, an LLM Agent, product personality, an emotion system, Need, Care,
Memory, Learning, autonomous business action, a POS adapter, a database, an API
server, a message queue, authentication, deployment, or native mobile code.

The repository remains in Phase 1 — First Living Goods. The first frontend
implementation, after this document is approved, is limited to presenting the
existing bounded State demo. It must not turn a frontend screen into a new
business runtime.

### Decision: repository and toolchain boundary

The frontend will live in the same Git repository but outside the Cargo
workspace member list:

```text
goods-garden/
├── Cargo.toml                 # Rust workspace only
├── crates/
├── apps/
│   ├── goods-garden-cli/      # Rust application
│   └── goods-garden-web/      # Independent Node application
└── docs/
```

`apps/goods-garden-web/` will own its own `package.json` and
`package-lock.json`. The repository will not add a root JavaScript package or
an npm workspace for one frontend package. An npm workspace may be considered
only when a second JavaScript package creates a real need; that would be a
separate decision.

The initial frontend stack is:

- React for the component model.
- TypeScript for explicit UI contracts.
- Vite for local development and production bundling.
- Plain CSS for the initial visual layer; no UI component library, Tailwind,
  Next.js or design-system migration in this foundation.

This keeps the demo understandable and reduces infrastructure decisions before
the domain model has matured.

### Platform strategy

The support promises are deliberately ordered:

1. **Browser first.** The acceptance target is a local responsive browser demo
   that runs on a developer machine.
2. **Desktop later, optional.** A future Work Item may place the same web UI in
   a Tauri 2 shell. The shell, native commands, packaging and permissions are
   not part of this foundation.
3. **Native mobile is a separate decision.** This proposal makes no iOS or
   Android promise. If mobile becomes a first-class product surface, the team
   must compare a React-sharing route such as Expo/React Native with a
   platform-native route such as Flutter and record the decision separately.

Cross-platform means that the read-only presentation model should remain
portable; it does not mean that desktop or mobile implementations already
exist.

### Data boundary and flow

The frontend consumes a read-only projection, not a Rust crate and not an
external system directly:

```text
local synthetic fixture
        │
        ▼
GoodsStateView (frontend read model)
        │
        ▼
React presentation components
        │
        ▼
read-only browser screen
```

The future input boundary is intentionally replaceable:

```text
local fixture / future approved adapter
        ▼
GoodsStateView
        ▼
the same presentation components
```

`GoodsStateView` is a frontend projection. It is not the Goods aggregate, a
database schema, an API contract, or permission to import `goods-domain`.
During the demo it must be visibly marked `SYNTHETIC EXAMPLE`. Any field whose
real source is not known must remain `UNKNOWN`; it must not be filled with
assumed POS, SEJ, inventory or store-operation facts.

The first read model may expose identity, profile label, observation summary,
expectation summary, health assessment and provenance/evidence display data.
The exact field set will be aligned with the existing Phase 1 domain types in
the implementation Work Item; this design does not freeze domain fields.

### UI boundary

The first screen is a calm, read-only Goods Garden State view. It may show:

- the product identity and profile label;
- current Goods State and Health Assessment;
- the observation and expectation used by the demo;
- evidence/provenance and the `SYNTHETIC EXAMPLE` notice.

It must not become a dashboard, chat interface, personality layer, emotion
simulator, Need/Care workflow, or autonomous-action control surface. A button
that implies a business action is outside this foundation. The screen presents
what the bounded demo knows and labels what it does not know.

### Dependency and ownership rules

The dependency direction inside Rust remains:

```text
goods-domain → goods-application → goods-infrastructure → goods-runtime → goods-garden-cli
```

The frontend is a sibling application, not another Rust layer:

```text
goods-garden-web → GoodsStateView boundary
```

The frontend must not import Rust crates, read Cargo internals, connect to a
database, call a POS endpoint, call an LLM/provider, or encode business
decisions in React components. A future API or desktop bridge may be added only
as an explicitly reviewed adapter that produces the same read-only projection.

### Proposed frontend layout

After approval, the bounded implementation may create this minimal layout:

```text
apps/goods-garden-web/
├── package.json
├── package-lock.json
├── index.html
├── src/
│   ├── main.tsx
│   ├── App.tsx
│   ├── styles.css
│   ├── components/
│   ├── demo/
│   └── view-models/
└── tests/
```

The fixture belongs under `src/demo/`; presentation types belong under
`src/view-models/`; components render the projection. No frontend folder may
silently become a home for API, POS or persistence code.

### Testing and CI plan

The frontend package will expose predictable commands:

```text
npm ci
npm run typecheck
npm test
npm run build
```

The implementation will add tests before production UI behavior. Tests should
cover at least:

- rendering a healthy synthetic Goods State;
- rendering the bounded unhealthy State case without inventing an action;
- displaying the synthetic/provenance boundary;
- rendering through a generic Goods Profile-shaped fixture rather than a
  tuna-mayo-specific branch.

Repository CI will run the Rust checks already required by the project and the
frontend commands from `apps/goods-garden-web/` in their own working directory.
The CI change belongs to the implementation Work Item and must not add a
server, container platform or deployment workflow.

### Alternatives considered

**A separate frontend repository:** rejected for the demo foundation because it
would split the design documents, review boundary and local onboarding across
repositories.

**A Rust/WASM or Rust UI frontend:** rejected for now because the immediate
goal is a small browser demo and the current Rust workspace is intentionally
focused on domain and runtime boundaries.

**Tauri first:** rejected because desktop packaging and native permissions are
not needed to validate the first State presentation.

**A mobile-first framework:** rejected because native mobile is not an agreed
product surface and would add an unverified platform commitment.

### Known facts, inferences and unknowns

- **KNOWN:** the repository is a Rust workspace with a Phase 1 bounded State
  demo and a documented Clean Architecture dependency direction.
- **KNOWN:** the requested frontend should be in the same repository while
  remaining independently tooled.
- **INFERRED:** React, TypeScript and Vite provide the smallest practical
  browser-first path for one developer to run the demo locally.
- **UNKNOWN:** the real POS schema, SEJ data contract, inventory API, store
  workflow, authentication model, deployment target and production frontend
  API.
- **UNKNOWN:** whether native mobile will be required and whether Tauri will be
  the eventual desktop shell.
- **UNAVAILABLE:** real external operational data in this repository.

Unknowns are boundaries for later design work, not permission to fabricate
fixtures that look like real SEJ facts.

### Review gate and acceptance

This document is `PROPOSED` and requires human review. No frontend source file,
package manifest, lockfile or external frontend dependency is added before the
owner approves this written specification.

Approval should confirm the following decisions:

1. `apps/goods-garden-web/` is the correct same-repository location.
2. React + TypeScript + Vite is acceptable for browser-first work.
3. The frontend consumes a synthetic/read-only projection and remains outside
   the Cargo member list.
4. Tauri is only a later desktop option, and native mobile remains a separate
   decision.

## 日本語

### 背景と課題

Goods Garden は、domain と runtime の境界が定義された Rust workspace である。次の demo では、明確に synthetic な Goods State を人に見せる browser surface が必要になる。ひとりの開発者が local で簡単に実行でき、同時に Rust の build graph と未知の外部 system から独立している必要がある。

Product の区別は維持する。

- Goods Garden は人が見る product world である。
- Goods Intelligence は、将来、感知、理解、表現、care の依頼、learning を担いうる technical kernel である。

この提案は frontend foundation だけを扱う。North Star を変更せず、新しい domain aggregate を確定せず、実際の external-data integration も実装しない。

### 目標

1. frontend 作業の安定した同一 repository 内の場所として `apps/goods-garden-web/` を追加する。
2. Cargo や Rust-to-browser binding を必要としない独立した Node toolchain を使う。
3. 最初の対応 surface を、read-only で synthetic な Goods State を表示する responsive browser demo とする。
4. UI の境界を view-model projection に置く。UI は Goods の domain rule を所有せず、POS、database、LLM、provider code を直接呼ばない。
5. native mobile を早期に約束せず、将来の desktop shell への現実的な道を残す。

### 非目標と phase の境界

この提案では sales analysis、anomaly detection、LLM Agent、product personality、emotion system、Need、Care、Memory、Learning、autonomous business action、POS adapter、database、API server、message queue、authentication、deployment、native mobile code を実装または許可しない。

Repository は Phase 1 — First Living Goods のままである。この文書が承認された後の最初の frontend implementation も、既存の bounded State demo の表示に限定する。frontend screen を新しい business runtime にしてはならない。

### 決定：repository と toolchain の境界

frontend は同じ Git repository に置くが、Cargo workspace の member list の外に置く。

```text
goods-garden/
├── Cargo.toml                 # Rust workspace only
├── crates/
├── apps/
│   ├── goods-garden-cli/      # Rust application
│   └── goods-garden-web/      # 独立した Node application
└── docs/
```

`apps/goods-garden-web/` は自身の `package.json` と `package-lock.json` を所有する。frontend が一つだけの段階で、root JavaScript package や npm workspace は追加しない。二つ目の JavaScript package に実際の必要性が生じた場合だけ、別の decision として npm workspace を検討する。

初期 frontend stack は React、TypeScript、Vite、plain CSS とする。初期 foundation には UI component library、Tailwind、Next.js、design-system migration を入れない。domain model が成熟する前の infrastructure decision を減らし、demo を理解しやすくするためである。

### Platform 方針

対応の約束は、次の順序で限定する。

1. **Browser first：** acceptance target は developer machine で動く local responsive browser demo とする。
2. **Desktop later, optional：** 将来の Work Item で同じ web UI を Tauri 2 shell に置く可能性はある。ただし shell、native command、packaging、permission はこの foundation の対象外である。
3. **Native mobile は別 decision：** iOS や Android をこの提案で約束しない。mobile が first-class product surface になった場合、Expo/React Native のような React-sharing route と、Flutter のような別の native route を比較し、別途 decision を記録する。

Cross-platform とは read-only presentation model を portable に保つことであり、desktop や mobile implementation が既に存在することを意味しない。

### Data boundary と flow

frontend は Rust crate や external system ではなく、read-only projection を消費する。

```text
local synthetic fixture
        │
        ▼
GoodsStateView (frontend read model)
        │
        ▼
React presentation components
        │
        ▼
read-only browser screen
```

将来の input boundary は交換可能にする。

```text
local fixture / future approved adapter
        ▼
GoodsStateView
        ▼
同じ presentation components
```

`GoodsStateView` は frontend projection であり、Goods aggregate、database schema、API contract、`goods-domain` import の許可ではない。demo では `SYNTHETIC EXAMPLE` と明示する。real source が不明な field は `UNKNOWN` のままとし、推測した POS、SEJ、inventory、store-operation の事実で埋めない。

最初の read model は identity、profile label、observation summary、expectation summary、health assessment、provenance/evidence display data を表示しうる。正確な field set は implementation Work Item で既存 Phase 1 domain type と整合させ、この design では domain field を固定しない。

### UI の境界

最初の screen は、落ち着いた read-only Goods Garden State view とする。product identity、profile label、Goods State、Health Assessment、demo に使った observation と expectation、evidence/provenance、`SYNTHETIC EXAMPLE` notice を表示してよい。

dashboard、chat interface、personality layer、emotion simulator、Need/Care workflow、autonomous-action control surface にしてはならない。business action を意味する button はこの foundation の対象外である。screen は bounded demo が知っている内容だけを提示し、知らない内容を表示上も明示する。

### Dependency と ownership の規則

Rust 内の dependency direction は維持する。

```text
goods-domain → goods-application → goods-infrastructure → goods-runtime → goods-garden-cli
```

frontend は sibling application であり、別の Rust layer ではない。

```text
goods-garden-web → GoodsStateView boundary
```

frontend は Rust crate、Cargo internals、database、POS endpoint、LLM/provider を直接 import または call してはならない。将来の API や desktop bridge は、同じ read-only projection を生成する明示的に review された adapter としてのみ追加できる。

### 提案する frontend layout

承認後の bounded implementation は、次の最小 layout を作成できる。

```text
apps/goods-garden-web/
├── package.json
├── package-lock.json
├── index.html
├── src/
│   ├── main.tsx
│   ├── App.tsx
│   ├── styles.css
│   ├── components/
│   ├── demo/
│   └── view-models/
└── tests/
```

fixture は `src/demo/`、presentation type は `src/view-models/` に置く。component は projection を render する。frontend folder を API、POS、persistence code の置き場にしてはならない。

### Testing と CI の計画

frontend package は次の予測可能な command を公開する。

```text
npm ci
npm run typecheck
npm test
npm run build
```

implementation では production UI behavior の前に test を追加する。少なくとも healthy な synthetic Goods State、action を発明しない bounded unhealthy State、synthetic/provenance boundary、tuna-mayo 固有 branch ではない generic Goods Profile-shaped fixture の rendering を検証する。

Repository CI は既存の Rust check に加え、`apps/goods-garden-web/` の working directory で frontend command を実行する。server、container platform、deployment workflow は追加しない。

### 検討した代替案

Separate frontend repository は、design document、review boundary、local onboarding を分割するため demo foundation には採用しない。Rust/WASM または Rust UI frontend は、現在の Rust workspace が domain と runtime boundary に集中しているため採用しない。Tauri first は desktop packaging と native permission が最初の State presentation の検証に不要なので採用しない。Mobile-first framework は合意された product surface ではなく、未検証の platform commitment を追加するため採用しない。

### Known、inference、unknown

- **KNOWN：** repository は Phase 1 の bounded State demo を持つ Rust workspace で、Clean Architecture の dependency direction が記録されている。
- **KNOWN：** 要求された frontend は同じ repository に置きつつ、toolchain は独立させる。
- **INFERRED：** React、TypeScript、Vite は、一人の開発者が local demo を動かす browser-first path として最小である。
- **UNKNOWN：** real POS schema、SEJ data contract、inventory API、store workflow、authentication、deployment target、production frontend API。
- **UNKNOWN：** native mobile が必要になるか、Tauri が最終的な desktop shell になるか。
- **UNAVAILABLE：** この repository 内の real external operational data。

Unknown は後続設計の境界であり、real SEJ fact に見える fixture を創作する許可ではない。

### Review gate と acceptance

この文書の status は `PROPOSED` であり、human review が必要である。owner がこの written specification を承認する前に frontend source、package manifest、lockfile、external frontend dependency を追加しない。

Review では `apps/goods-garden-web/` の場所、React + TypeScript + Vite、synthetic/read-only projection、Cargo member list 外という境界、browser first、Tauri の将来 option、native mobile の別 decision を確認する。

## 中文

### 背景与问题

Goods Garden 是一个已经定义了领域与 runtime 边界的 Rust workspace。下一个 demo 需要一个浏览器界面，把明确为 synthetic 的 Goods State 展示给人。它要让一个开发者可以在本地轻松运行，同时与 Rust 构建图以及未知外部系统保持独立。

产品区分保持不变：

- Goods Garden 是用户看到的产品世界。
- Goods Intelligence 是未来可能负责感知、理解、表达、请求照料和学习的技术内核。

本提案只处理前端基础工程，不改变 North Star，不确定新的领域聚合，也不实现真实外部数据接入。

### 目标

1. 在同一仓库增加稳定的前端工作位置 `apps/goods-garden-web/`。
2. 使用独立 Node 工具链，使前端迭代不依赖 Cargo 或 Rust-to-browser binding。
3. 第一支持界面是 responsive browser demo，只读展示 synthetic Goods State。
4. 把 UI 边界放在 view-model projection。UI 不拥有 Goods 领域规则，也不直接调用 POS、数据库、LLM 或 provider 代码。
5. 保留通往桌面壳层的可信未来路径，但不提前承诺原生移动端。

### 非目标与阶段边界

本提案不实现或授权销售分析、异常检测、LLM Agent、商品人格、情绪系统、Need、Care、Memory、Learning、自主经营行动、POS adapter、数据库、API Server、消息队列、认证、部署或原生移动端代码。

仓库仍处于 Phase 1 — First Living Goods。本文档获批后的第一版前端实现，也只限于展示现有有边界的 State demo。不得把前端页面变成新的业务 runtime。

### 决策：仓库与工具链边界

前端放在同一 Git 仓库，但位于 Cargo workspace member list 之外。

```text
goods-garden/
├── Cargo.toml                 # 仅 Rust workspace
├── crates/
├── apps/
│   ├── goods-garden-cli/      # Rust application
│   └── goods-garden-web/      # 独立 Node application
└── docs/
```

`apps/goods-garden-web/` 自己拥有 `package.json` 和 `package-lock.json`。在只有一个前端 package 时，不增加 root JavaScript package 或 npm workspace。只有当第二个 JavaScript package 产生真实需求时，才通过单独决策评估 npm workspace。

初始前端技术栈为 React、TypeScript、Vite 和 plain CSS。这个基础工程暂不加入 UI component library、Tailwind、Next.js 或 design-system migration，以便在领域模型成熟前减少基础设施决策，让 demo 保持易懂。

### 跨平台策略

支持承诺明确按以下顺序限定：

1. **Browser first：** 验收目标是在开发者机器上运行的本地 responsive browser demo。
2. **Desktop later, optional：** 未来 Work Item 可以把同一 web UI 放进 Tauri 2 shell，但 shell、native command、打包和权限不属于本基础工程。
3. **Native mobile 是单独决策：** 本提案不承诺 iOS 或 Android。如果移动端成为一等产品界面，团队必须比较 Expo/React Native 这类 React-sharing 路线与 Flutter 这类独立 native 路线，并单独记录决策。

跨平台表示只读展示模型应保持可移植，并不表示桌面或移动端实现已经存在。

### 数据边界与流转

前端消费的是只读 projection，而不是 Rust crate 或外部系统：

```text
local synthetic fixture
        │
        ▼
GoodsStateView (frontend read model)
        │
        ▼
React presentation components
        │
        ▼
read-only browser screen
```

未来输入边界保持可替换：

```text
local fixture / future approved adapter
        ▼
GoodsStateView
        ▼
同一套 presentation components
```

`GoodsStateView` 是前端 projection，不是 Goods aggregate、数据库 schema、API contract，也不是导入 `goods-domain` 的许可。demo 必须清楚标记 `SYNTHETIC EXAMPLE`。真实来源未知的字段保持 `UNKNOWN`，不得用臆测的 POS、SEJ、inventory 或门店运营事实填充。

首个 read model 可以展示 identity、profile label、observation summary、expectation summary、health assessment 与 provenance/evidence display data。准确字段集合将在 implementation Work Item 中与现有 Phase 1 领域类型对齐；本设计不冻结领域字段。

### UI 边界

第一屏是克制的只读 Goods Garden State view，可以展示商品 identity 与 profile label、当前 Goods State 与 Health Assessment、demo 使用的 observation 与 expectation、evidence/provenance 以及 `SYNTHETIC EXAMPLE` 提示。

它不得变成 dashboard、chat interface、personality layer、emotion simulator、Need/Care workflow 或 autonomous-action control surface。表示业务动作的按钮不在本基础工程范围内。页面只展示 bounded demo 已知的内容，并在视觉上标明未知内容。

### 依赖与所有权规则

Rust 内部依赖方向保持不变：

```text
goods-domain → goods-application → goods-infrastructure → goods-runtime → goods-garden-cli
```

前端是 sibling application，不是另一个 Rust layer：

```text
goods-garden-web → GoodsStateView boundary
```

前端不得直接 import Rust crate、读取 Cargo internals、连接数据库、调用 POS endpoint 或调用 LLM/provider。未来 API 或 desktop bridge 只能作为经过明确 review、并生成同一只读 projection 的 adapter 加入。

### 建议的前端目录

批准后，有边界的实现可以创建以下最小目录：

```text
apps/goods-garden-web/
├── package.json
├── package-lock.json
├── index.html
├── src/
│   ├── main.tsx
│   ├── App.tsx
│   ├── styles.css
│   ├── components/
│   ├── demo/
│   └── view-models/
└── tests/
```

fixture 放在 `src/demo/`，presentation type 放在 `src/view-models/`，components 负责 render projection。前端目录不得悄悄变成 API、POS 或 persistence code 的容器。

### 测试与 CI 计划

前端 package 提供以下可预测命令：

```text
npm ci
npm run typecheck
npm test
npm run build
```

实现时先增加测试，再增加 production UI behavior。至少验证 healthy synthetic Goods State、不会凭空发明 action 的 bounded unhealthy State、synthetic/provenance 边界，以及通过 generic Goods Profile-shaped fixture 而不是 tuna-mayo 专用分支进行渲染。

仓库 CI 在已有 Rust 检查之外，在 `apps/goods-garden-web/` 工作目录执行前端命令。不增加 server、container platform 或 deployment workflow。

### 已考虑的替代方案

Separate frontend repository 会拆分设计文档、review boundary 和本地 onboarding，因此不采用。Rust/WASM 或 Rust UI frontend 暂不采用，因为当前 Rust workspace 专注于 domain 与 runtime boundary。Tauri first 不采用，因为 desktop packaging 和 native permission 并不是验证第一版 State presentation 所必需。Mobile-first framework 不采用，因为它不是已经同意的产品界面，会增加未经验证的平台承诺。

### 已知、推断与未知

- **KNOWN：** 仓库是带有 Phase 1 bounded State demo 的 Rust workspace，并记录了 Clean Architecture 依赖方向。
- **KNOWN：** 已提出的前端放在同一仓库，同时保持工具链独立。
- **INFERRED：** React、TypeScript、Vite 是单个开发者本地运行 browser-first demo 的最小可行路径。
- **UNKNOWN：** 真实 POS schema、SEJ data contract、inventory API、门店流程、认证、部署目标和生产前端 API。
- **UNKNOWN：** 是否需要原生移动端，以及最终桌面壳层是否采用 Tauri。
- **UNAVAILABLE：** 本仓库中的真实外部运营数据。

Unknown 是后续设计的边界，不是编造看起来像真实 SEJ 事实的 fixture 的许可。

### Review gate 与验收

本文档状态为 `PROPOSED`，需要 human review。在项目负责人批准书面规格前，不增加前端源码、package manifest、lockfile 或外部前端依赖。

Review 应确认：`apps/goods-garden-web/` 的位置；React + TypeScript + Vite；synthetic/read-only projection；前端位于 Cargo member list 之外；browser first；Tauri 只是未来选项；native mobile 仍是单独决策。
