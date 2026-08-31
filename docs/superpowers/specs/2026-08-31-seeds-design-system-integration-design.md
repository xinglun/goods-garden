# SEEDS Web Design System Integration Design

> Status: PROPOSED — chat design approved; written review required before implementation.

## English

### Context

Goods Garden already has a small React/Vite browser presentation for the
bounded Phase 1 Goods State. Its current visual layer is plain CSS with values
written directly in selectors. The external SEEDS design-system workspace
provides the authoritative design-system specification, Figma naming rules and
import JSON. This Work Item projects that external system into the existing web
package without changing the Rust workspace or the Goods State read model.

The implementation source is:

`/Users/sei-rinn/dev/workspace_typescript/seeds_design_system/docs/design-system/SEEDS_figma_plugin_import.json`

The design semantics and naming constraints come from
`SEEDS_design_system_spec.md` and `SEEDS_figma_naming_convention.md` in the
same external directory. The external JSON remains the source of truth; the
web package receives a generated CSS snapshot for independent browser builds.

### Goals

- Generate a web-consumable projection of all 123 primitive, 77 semantic and
  111 material tokens present in the import JSON.
- Validate the six fixed modes: `Light`, `Dark`, `Sakura`, `Momiji`,
  `NatureLaw` and `Disaster`.
- Resolve aliases and fail before writing output on missing modes, missing
  aliases, self references or alias cycles.
- Refactor the existing Goods State screen to use semantic and material CSS
  custom properties for color, spacing, radius, typography and shadow
  treatment.
- Make all six modes selectable as presentation-only state while retaining a
  read-only, synthetic Goods State surface.
- Keep the frontend outside the Cargo member list and independent from Rust,
  POS, SEJ, database, API, provider and Figma runtime code.

### Non-goals

This Work Item does not add a Figma integration, mutate a Figma file, copy the
full design-system documentation into Goods Garden, implement a React
component library for every SEEDS component, add glass to components that do
not have an approved contract, or change Goods domain semantics. It does not
add Need, Care, Memory, Learning, autonomous action, external data, a server or
new runtime dependency.

### Architecture and data flow

The flow is intentionally build-time and one-way:

```text
external SEEDS_figma_plugin_import.json
                 │
                 ▼
generate-seeds-theme.mjs
                 │ validates and resolves aliases
                 ▼
src/design-system/seeds-theme.css
                 │
                 ▼
React presentation styles + data-seeds-mode
                 │
                 ▼
read-only synthetic Goods State screen
```

The generator is invoked explicitly with the external JSON path. The
generated CSS is committed to the web package, so `npm run typecheck`,
`npm test` and `npm run build` do not require the external workspace to be
mounted. The generator is the refresh path when the external import JSON
changes; it is not a runtime loader.

Figma slash paths become stable CSS custom-property names by replacing `/`
with `-` and converting camelCase segments to kebab-case, for example
`semantic/text/primary` becomes `--seeds-semantic-text-primary` and
`material/typography/bodyM/fontSize` becomes
`--seeds-material-typography-body-m-font-size`. The generated file includes
primitive, semantic and material layers, while application selectors use only
semantic and material properties.

### Theme and screen projection

`Light` is the default mode. A presentation-only mode selector updates
`data-seeds-mode` on the document root. Each mode exposes the same CSS
property names, so layout and component structure do not change during a mode
switch. The selector is not a business action and does not modify the
`GoodsStateView` projection.

The existing screen maps its current concerns as follows:

- page canvas and primary surfaces use semantic surface tokens;
- headings, labels, identity text and evidence copy use semantic text tokens;
- card borders and separators use semantic border tokens;
- healthy assessment uses the semantic success state tokens;
- unhealthy assessment uses the semantic error state tokens;
- provenance uses the semantic warning state tokens;
- spacing, radius, typography and shadow values use material tokens.

The screen remains a solid/fallback presentation. Glass styles and the seven
Figma component contracts are not introduced into this screen because the
current browser slice does not expose those approved component contracts.

### Files and responsibilities

- `apps/goods-garden-web/scripts/generate-seeds-theme.mjs`: validates the
  external import JSON, resolves aliases and writes the generated CSS.
- `apps/goods-garden-web/src/design-system/seeds-theme.css`: committed,
  generated CSS projection with source and generation metadata.
- `apps/goods-garden-web/src/design-system/modes.ts`: typed list of the six
  supported modes and the default mode.
- `apps/goods-garden-web/src/App.tsx`: owns presentation-only mode selection
  and applies the document-root mode attribute.
- `apps/goods-garden-web/src/styles.css`: consumes semantic/material
  properties and contains no raw design-system color, spacing, radius,
  typography or shadow values in component selectors.
- `apps/goods-garden-web/tests/App.test.tsx`: verifies mode selection,
  generated-token usage and the unchanged read-only/synthetic boundary.
- `apps/goods-garden-web/package.json`: exposes the explicit token-generation
  command without adding a runtime dependency.

### Verification and acceptance

The generator tests must prove the six modes, token counts, alias resolution,
and fail-closed invalid-input behavior. Frontend tests must prove that the
mode selector changes only presentation state, all existing healthy and
unhealthy projections still render, `SYNTHETIC EXAMPLE` remains visible, and
no business action surface appears.

The required commands are:

```bash
npm run generate:seeds -- --input /Users/sei-rinn/dev/workspace_typescript/seeds_design_system/docs/design-system/SEEDS_figma_plugin_import.json
npm run typecheck --prefix apps/goods-garden-web
npm test --prefix apps/goods-garden-web
npm run build --prefix apps/goods-garden-web
make -f Makefile.ai quality
```

The final Work Item verification also runs the repository-required Rust checks
and `git diff --check`. No claim is made that the external SEEDS source is
available in CI; the committed generated CSS is the build input there.

## 日本語

### 背景

Goods Garden には、Phase 1 の bounded Goods State を表示する小さな
React/Vite browser presentation がすでにある。現在の visual layer は plain
CSS で、selector に値を直接記述している。外部の SEEDS design-system
workspace には、正式な design-system specification、Figma naming rule、
import JSON がある。本 Work Item では、その外部 system を既存 web package
へ投影する。Rust workspace と Goods State read model は変更しない。

実装元は次のファイルである。

`/Users/sei-rinn/dev/workspace_typescript/seeds_design_system/docs/design-system/SEEDS_figma_plugin_import.json`

設計意味と命名制約は同じ外部 directory の
`SEEDS_design_system_spec.md` と `SEEDS_figma_naming_convention.md` を参照する。
外部 JSON を正本とし、web package には browser build を独立させる生成 CSS
snapshot を置く。

### 目的

- import JSON にある primitive 123 個、semantic 77 個、material 111 個を
  web で利用できる形へ生成する。
- `Light`、`Dark`、`Sakura`、`Momiji`、`NatureLaw`、`Disaster` の固定 6 mode
  を検証する。
- alias を解決し、mode 欠落、alias 欠落、自参照、alias cycle があれば出力前に停止する。
- 既存 Goods State screen の color、spacing、radius、typography、shadow treatment
  を semantic/material CSS custom property へ移行する。
- 6 mode を presentation-only state として切り替え可能にし、read-only の synthetic
  Goods State surface を保つ。
- frontend を Cargo member list の外に置き、Rust、POS、SEJ、database、API、provider、
  Figma runtime code から独立させる。

### 対象外

Figma integration、Figma file の変更、design-system document 全体の Goods Garden への
コピー、全 SEEDS component を対象とする React component library、承認済み contract のない
component への glass 導入、Goods domain semantics の変更は行わない。Need、Care、Memory、
Learning、自律 action、external data、server、新しい runtime dependency も追加しない。

### Architecture と data flow

処理は build-time の一方向とする。

```text
外部 SEEDS_figma_plugin_import.json
                 ↓
generate-seeds-theme.mjs
                 ↓ 検証・alias 解決
src/design-system/seeds-theme.css
                 ↓
React presentation style + data-seeds-mode
                 ↓
read-only synthetic Goods State screen
```

generator は外部 JSON path を明示して実行する。生成 CSS は web package に commit するため、
`npm run typecheck`、`npm test`、`npm run build` は外部 workspace の mount を必要としない。
外部 JSON の更新時は generator を再実行する。runtime loader にはしない。

Figma の slash path は `/` を `-` に置き、camelCase segment を kebab-case にする。例えば
`semantic/text/primary` は `--seeds-semantic-text-primary`、
`material/typography/bodyM/fontSize` は `--seeds-material-typography-body-m-font-size` となる。
生成 file は primitive、semantic、material の層を含むが、application selector は semantic と
material の property だけを使う。

### Theme と screen projection

default は `Light` とする。presentation-only mode selector が document root の
`data-seeds-mode` を更新する。各 mode は同じ CSS property 名を公開し、mode switch で layout
と component structure は変えない。selector は business action ではなく、`GoodsStateView`
projection も変更しない。

既存 screen は surface、text、border、success state、error state、warning state を semantic
token へ、spacing、radius、typography、shadow を material token へ対応させる。screen は
solid/fallback presentation のままとし、現 browser slice に approved component contract がない
ため glass style と 7 件の Figma component contract は導入しない。

### File と責務

`apps/goods-garden-web/scripts/generate-seeds-theme.mjs` は JSON の検証、alias 解決、CSS 出力を担当する。
`src/design-system/seeds-theme.css` は commit 済み生成物、`src/design-system/modes.ts` は 6 mode と
default mode の型付き定義とする。`App.tsx` は presentation-only mode selection、`styles.css` は
semantic/material property の消費、`App.test.tsx` は mode selection と read-only/synthetic 境界を検証する。
`package.json` には runtime dependency を増やさず明示的な generation command を追加する。

### 検証と受け入れ条件

generator test は 6 mode、token count、alias 解決、invalid input 時の fail-closed を検証する。
frontend test は mode selector が presentation state だけを変更すること、healthy/unhealthy projection、
`SYNTHETIC EXAMPLE`、business action surface がないことを検証する。generator、frontend typecheck、
test、build、`make -f Makefile.ai quality`、Rust required checks、`git diff --check` を実行する。

外部 SEEDS source が CI で利用できるとは主張しない。CI の build input は commit 済み generated CSS である。

## 中文

### 背景

Goods Garden 已有一个用于展示 Phase 1 有边界 Goods State 的小型 React/Vite 浏览器界面。当前
visual layer 是 plain CSS，并在 selector 中直接写入数值。外部 SEEDS design-system workspace
提供正式的 design-system specification、Figma naming rule 和 import JSON。本 Work Item 将外部
系统投影到现有 web package，不改变 Rust workspace 和 Goods State read model。

实现来源是：

`/Users/sei-rinn/dev/workspace_typescript/seeds_design_system/docs/design-system/SEEDS_figma_plugin_import.json`

设计语义和命名约束来自同一外部目录的 `SEEDS_design_system_spec.md` 与
`SEEDS_figma_naming_convention.md`。外部 JSON 继续作为正本；web package 接收一份已生成的 CSS
snapshot，以保持浏览器构建独立。

### 目标

- 生成 import JSON 中的 123 个 primitive、77 个 semantic 和 111 个 material token 的 web 投影。
- 验证固定的 6 个模式：`Light`、`Dark`、`Sakura`、`Momiji`、`NatureLaw`、`Disaster`。
- 解析 alias；遇到缺失模式、缺失 alias、自引用或 alias cycle 时，在写出结果前 fail-closed。
- 将现有 Goods State screen 的颜色、间距、圆角、排版和阴影处理改为 semantic/material CSS custom property。
- 允许将 6 个模式作为仅用于展示的状态切换，同时保持只读 synthetic Goods State surface。
- 保持前端位于 Cargo member list 之外，并独立于 Rust、POS、SEJ、数据库、API、provider 和 Figma runtime code。

### 非目标

本 Work Item 不增加 Figma integration，不修改 Figma file，不把完整 design-system 文档复制到 Goods Garden，
不实现覆盖全部 SEEDS component 的 React component library，不给没有 approved contract 的 component 引入
glass，也不改变 Goods domain semantics。不增加 Need、Care、Memory、Learning、自主 action、external data、
server 或新的 runtime dependency。

### Architecture 与 data flow

流程明确为 build-time 单向流程：外部 `SEEDS_figma_plugin_import.json` 经过
`generate-seeds-theme.mjs` 的验证和 alias 解析，生成 `src/design-system/seeds-theme.css`，再由 React
presentation styles 与 `data-seeds-mode` 消费，最终展示只读 synthetic Goods State screen。

generator 通过显式外部 JSON path 运行。生成 CSS 会提交到 web package，因此
`npm run typecheck`、`npm test` 和 `npm run build` 不需要挂载外部 workspace。外部 JSON 更新时重新运行
generator；它不是 runtime loader。

Figma slash path 通过把 `/` 替换为 `-`，并把 camelCase segment 转为 kebab-case，映射为稳定的 CSS custom
property。例如 `semantic/text/primary` 变为 `--seeds-semantic-text-primary`，
`material/typography/bodyM/fontSize` 变为 `--seeds-material-typography-body-m-font-size`。生成文件包含
primitive、semantic、material 三层，但 application selector 只使用 semantic 和 material property。

### Theme 与 screen projection

默认模式为 `Light`。仅用于展示的 mode selector 更新 document root 上的 `data-seeds-mode`。每个模式提供
相同的 CSS property 名称，因此切换模式不会重建 layout 或 component structure。selector 不是业务 action，
也不会修改 `GoodsStateView` projection。

现有 screen 将 surface、text、border、success state、error state、warning state 映射到 semantic token，
将 spacing、radius、typography、shadow 映射到 material token。该 screen 继续使用 solid/fallback presentation；
由于当前浏览器切片没有对应的 approved component contract，不引入 glass style 和 7 个 Figma component contract。

### 文件与职责

`apps/goods-garden-web/scripts/generate-seeds-theme.mjs` 负责 JSON 验证、alias 解析和 CSS 输出。
`src/design-system/seeds-theme.css` 是提交的生成物，`src/design-system/modes.ts` 定义 6 个模式和默认模式。
`App.tsx` 负责仅用于展示的模式选择，`styles.css` 消费 semantic/material property，`App.test.tsx` 验证模式
选择与只读/synthetic 边界。`package.json` 增加显式生成命令，但不增加 runtime dependency。

### 验证与验收

generator test 必须验证 6 个模式、token 数量、alias 解析，以及非法输入时的 fail-closed 行为。前端 test 必须
验证模式选择只改变展示状态、healthy/unhealthy projection 仍可渲染、`SYNTHETIC EXAMPLE` 仍可见，以及没有业务
action surface。执行 generator、frontend typecheck、test、build、`make -f Makefile.ai quality`、Rust required
checks 和 `git diff --check`。

不声明外部 SEEDS source 在 CI 中一定可用；CI 的 build input 是已提交的 generated CSS。
