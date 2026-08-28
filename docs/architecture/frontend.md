# Frontend Architecture

## English

### Purpose

The frontend is a sibling application that presents the bounded Phase 1 Goods
State in the Goods Garden product world. It is not a second domain layer and it
does not make Goods Intelligence decisions.

### Repository boundary

The package lives at `apps/goods-garden-web/` and is outside the Cargo
workspace member list. It owns its own `package.json` and
`package-lock.json`. It uses React, TypeScript, Vite, Vitest, Testing Library
and plain CSS.

The Rust dependency direction remains:

```text
goods-domain → goods-application → goods-infrastructure → goods-runtime → goods-garden-cli
```

The frontend is adjacent to this graph:

```text
local synthetic fixture / approved future adapter
                    ↓
             GoodsStateView
                    ↓
          React presentation layer
                    ↓
             browser screen
```

The frontend must not import Rust crates, Cargo internals, POS systems,
databases, API servers, LLM providers or persistence adapters. A future
integration must produce the same read-only projection through an explicitly
reviewed boundary.

### Current surface

Browser is the first and only implemented platform surface. The local demo is
read-only and uses synthetic data marked `SYNTHETIC EXAMPLE`. Tauri 2 is a
possible later desktop shell; its commands, permissions and packaging require a
separate Work Item. Native mobile is a separate product and architecture
decision; this repository makes no iOS or Android promise here.

### View-model boundary

`GoodsStateView` is a frontend read model. It can contain display-ready
identity, profile, observation, expectation, health assessment and
provenance/evidence fields. It is not the Goods aggregate, a database schema, or
an API contract.

Components render supplied values. They do not calculate health, infer Need,
request Care, invoke actions, or store Memory/Learning. Unknown facts remain
`UNKNOWN`; fixture values are examples and must not look like real SEJ or POS
records.

### Package commands

From the repository root:

```bash
npm ci --prefix apps/goods-garden-web
npm run dev --prefix apps/goods-garden-web
npm run typecheck --prefix apps/goods-garden-web
npm test --prefix apps/goods-garden-web
npm run build --prefix apps/goods-garden-web
```

The package is intentionally small. A root npm workspace is not needed for one
frontend package; adding another JavaScript package would require a separate
decision.

### Testing boundary

Tests verify that a healthy and unhealthy supplied State can be rendered, that
the synthetic provenance is visible, that a generic profile-shaped projection
uses the same path, and that no button or autonomous-action surface appears.
They do not validate POS behavior, domain rules or external data.

### CI boundary

CI runs the frontend commands in a separate job and working directory. The job
does not start a server, database, queue, container platform or deployment
environment.

## 日本語

### 目的

frontend は Phase 1 の bounded Goods State を Goods Garden product world に表示する sibling application である。二つ目の domain layer ではなく、Goods Intelligence の decision を行わない。

### Repository の境界

package は `apps/goods-garden-web/` に置き、Cargo workspace member list の外に置く。自身の `package.json` と `package-lock.json` を持ち、React、TypeScript、Vite、Vitest、Testing Library、plain CSS を使う。

Rust の dependency direction は維持する。

```text
goods-domain → goods-application → goods-infrastructure → goods-runtime → goods-garden-cli
```

frontend はこの graph に隣接し、local synthetic fixture または承認済みの将来 adapter から `GoodsStateView` を受けて React presentation layer で browser screen を表示する。Rust crate、Cargo internals、POS、database、API server、LLM provider、persistence adapter を直接 import しない。将来 integration は review 済み boundary から同じ read-only projection を生成する。

### 現在の surface

Browser が最初で、実装済みの唯一の platform surface である。local demo は read-only で、synthetic data を `SYNTHETIC EXAMPLE` と明記する。Tauri 2 は将来の desktop shell の option であり、command、permission、packaging は別 Work Item とする。native mobile は別の product/architecture decision であり、この repository は iOS/Android を約束しない。

### View-model の境界

`GoodsStateView` は frontend read model であり、display 用の identity、profile、observation、expectation、health assessment、provenance/evidence を含みうる。Goods aggregate、database schema、API contract ではない。

component は渡された value を表示するだけで、health を計算せず、Need を推論せず、Care や action を呼ばず、Memory/Learning を保存しない。不明な事実は `UNKNOWN` とし、fixture は example として扱い、real SEJ/POS record に見せない。

### Package command

repository root から次を実行する。

```bash
npm ci --prefix apps/goods-garden-web
npm run dev --prefix apps/goods-garden-web
npm run typecheck --prefix apps/goods-garden-web
npm test --prefix apps/goods-garden-web
npm run build --prefix apps/goods-garden-web
```

frontend が一つだけなので root npm workspace は不要である。二つ目の JavaScript package を追加する場合は別 decision とする。

### Testing と CI の境界

test は healthy/unhealthy の supplied State、synthetic provenance、generic profile-shaped projection、button や autonomous-action surface が無いことだけを検証する。POS、domain rule、external data は検証しない。CI は別 job と working directory で frontend command を実行し、server、database、queue、container platform、deployment environment を起動しない。

## 中文

### 目的

前端是把 Phase 1 有边界的 Goods State 展示在 Goods Garden 产品世界中的 sibling application。它不是第二个领域层，也不负责做 Goods Intelligence 决策。

### 仓库边界

package 位于 `apps/goods-garden-web/`，在 Cargo workspace member list 之外，自有 `package.json` 和 `package-lock.json`，使用 React、TypeScript、Vite、Vitest、Testing Library 和 plain CSS。

Rust 依赖方向保持不变：

```text
goods-domain → goods-application → goods-infrastructure → goods-runtime → goods-garden-cli
```

前端位于该依赖图旁边，从 local synthetic fixture 或获批的未来 adapter 接收 `GoodsStateView`，再通过 React presentation layer 展示浏览器页面。前端不得直接 import Rust crate、Cargo internals、POS、数据库、API server、LLM provider 或 persistence adapter。未来接入必须经过明确 review 的边界，并产生同一只读 projection。

### 当前界面

Browser 是第一个也是当前唯一实现的平台界面。本地 demo 只读，并把 synthetic data 标记为 `SYNTHETIC EXAMPLE`。Tauri 2 只是未来可选的桌面壳层，其 command、权限和打包需要单独 Work Item。原生移动端属于单独的产品与架构决策；本仓库当前不承诺 iOS/Android。

### View-model 边界

`GoodsStateView` 是前端 read model，可以包含用于展示的 identity、profile、observation、expectation、health assessment 和 provenance/evidence 字段。它不是 Goods aggregate、数据库 schema 或 API contract。

组件只展示传入的值，不计算 health、不推断 Need、不请求 Care、不调用 action，也不保存 Memory/Learning。未知事实保持为 `UNKNOWN`；fixture 只能作为 example，不得伪装成真实 SEJ/POS 记录。

### Package 命令

从仓库根目录运行：

```bash
npm ci --prefix apps/goods-garden-web
npm run dev --prefix apps/goods-garden-web
npm run typecheck --prefix apps/goods-garden-web
npm test --prefix apps/goods-garden-web
npm run build --prefix apps/goods-garden-web
```

当前只有一个前端 package，不需要 root npm workspace；增加第二个 JavaScript package 时必须单独决策。

### 测试与 CI 边界

测试只验证 healthy/unhealthy supplied State、synthetic provenance、generic profile-shaped projection，以及页面没有 button 或 autonomous-action surface。它不验证 POS 行为、领域规则或外部数据。CI 在独立 job 和工作目录运行前端命令，不启动 server、数据库、队列、容器平台或部署环境。
