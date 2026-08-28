# Goods Garden Frontend Foundation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a same-repository, independently tooled React/TypeScript/Vite browser demo that renders the existing Phase 1 Goods State as a read-only synthetic view.

**Architecture:** The frontend is a sibling application at `apps/goods-garden-web/`, outside the Cargo member list. A local fixture is projected into a frontend-only `GoodsStateView`; React components render that projection without importing Rust, POS, database or provider code. Browser is the first surface; Tauri desktop is a later option and native mobile is a separate decision.

**Tech Stack:** React 19, TypeScript, Vite, Vitest, Testing Library, plain CSS, npm package-local lockfile.

**Spec:** `docs/superpowers/specs/2026-08-28-goods-garden-frontend-foundation-design.md`

## Global Constraints

- `apps/goods-garden-web/` remains outside the Cargo workspace member list.
- The frontend consumes a read-only projection and contains no Goods domain, Need, Care, Memory, Learning, POS, database, API, LLM or autonomous-action behavior.
- Demo data must be labelled `SYNTHETIC EXAMPLE`; unknown external facts remain `UNKNOWN`.
- Human-facing documents must provide semantically equivalent English, Japanese and Chinese sections in that order.
- Browser is the only implemented platform surface; Tauri and native mobile work are not implemented here.
- Run `npm ci`, `npm run typecheck`, `npm test` and `npm run build` from `apps/goods-garden-web/`.

## File Map

- Modify `AGENTS.md`, `README.md`, `docs/README.md` and `docs/phases/phase-1-first-living-goods.md` for the approved frontend boundary.
- Create `docs/architecture/frontend.md` as the implementation-facing frontend architecture document.
- Create `apps/goods-garden-web/` package, source, tests and package-local lockfile.
- Modify `.github/workflows/ci.yml` with a separate frontend quality job.
- Modify only AI Cockpit generated records through AI Cockpit commands.

---

## English

### Task 1: Record the approved implementation boundary

**Files:**
- Modify: `AGENTS.md`
- Modify: `README.md`
- Modify: `docs/README.md`
- Modify: `docs/phases/phase-1-first-living-goods.md`
- Create: `docs/architecture/frontend.md`

**Interfaces:**
- Consumes: the approved design specification and current Phase 1 Rust architecture.
- Produces: three-language guidance naming the web package, projection boundary, runnable commands and platform limits.

- [x] State that the web package is a sibling of `apps/goods-garden-cli/`, outside Cargo, with its own npm lockfile.
- [x] State that the UI renders a read-only `GoodsStateView` from synthetic data and cannot import Rust, POS, database or provider code.
- [x] Add the local web demo command and architecture link to the root and docs indexes.
- [x] Replace any blanket “no Web UI” wording with the precise rule: only the approved read-only State presentation is allowed; production UI and frontend business behavior remain out of scope.
- [x] Run `rg -n '^## (English|日本語|中文)' AGENTS.md README.md docs/README.md docs/architecture/frontend.md docs/phases/phase-1-first-living-goods.md` and `git diff --check`; expect all listed documents to have the three language sections and no whitespace errors.

### Task 2: Bootstrap the package and write the failing test

**Files:**
- Create: `apps/goods-garden-web/package.json`
- Create: `apps/goods-garden-web/package-lock.json`
- Create: `apps/goods-garden-web/tsconfig.json`
- Create: `apps/goods-garden-web/vite.config.ts`
- Create: `apps/goods-garden-web/index.html`
- Create: `apps/goods-garden-web/tests/setup.ts`
- Create: `apps/goods-garden-web/tests/App.test.tsx`

**Interfaces:**
- Consumes: npm registry packages resolved into the package-local lockfile.
- Produces: `dev`, `typecheck`, `test`, `test:watch` and `build` scripts.

- [x] Use React 19, TypeScript, Vite, the Vite React plugin, Vitest, jsdom, Testing Library, jest-dom and the required type packages. Set Node engine `>=22.12.0`.
- [x] Configure Vite React support, jsdom tests with `tests/setup.ts`, strict TypeScript, DOM libraries, JSX transform and bundler module resolution.
- [x] Run `npm install --prefix apps/goods-garden-web`; expect the package-local lockfile and ignored `node_modules` only.
- [x] Write the first failing test before `App` exists:

```tsx
it("renders a synthetic read-only Goods State", () => {
  render(<App />);
  expect(screen.getByRole("heading", { name: "Goods Garden" })).toBeInTheDocument();
  expect(screen.getByText("SYNTHETIC EXAMPLE")).toBeInTheDocument();
  expect(screen.getByText("Healthy")).toBeInTheDocument();
  expect(screen.queryByRole("button")).not.toBeInTheDocument();
});
```

- [x] Run `npm test --prefix apps/goods-garden-web`; expect failure because `src/App.tsx` does not exist, not because of an implemented behavior assertion.

### Task 3: Implement the projection-only State screen

**Files:**
- Create: `apps/goods-garden-web/src/view-models/goods-state-view.ts`
- Create: `apps/goods-garden-web/src/demo/synthetic-goods-state.ts`
- Create: `apps/goods-garden-web/src/components/ProvenanceBanner.tsx`
- Create: `apps/goods-garden-web/src/components/HealthAssessment.tsx`
- Create: `apps/goods-garden-web/src/components/GoodsStateView.tsx`
- Create: `apps/goods-garden-web/src/App.tsx`
- Create: `apps/goods-garden-web/src/main.tsx`
- Create: `apps/goods-garden-web/src/styles.css`

**Interfaces:**
- Consumes: the failing test and a frontend-only `GoodsStateView` projection.
- Produces: `App({ view?: GoodsStateView })`, which renders the default synthetic fixture or an injected generic fixture without calculating domain health.

- [x] Define projection types for identity, profile, observation, expectation, health assessment and provenance; include provenance kind `synthetic` and no domain methods or POS fields.
- [x] Export a tuna-mayo-shaped reference fixture with synthetic provenance, visible `SYNTHETIC EXAMPLE` labeling and a supplied healthy assessment.
- [x] Render semantic headings, definition lists, health text, evidence/provenance and no action controls.
- [x] Keep `HealthAssessment` display-only; it must not derive health from age or implement a new business rule.
- [x] Render `<App />` from `main.tsx`, import the stylesheet, then run `npm test --prefix apps/goods-garden-web` and `npm run typecheck --prefix apps/goods-garden-web`; expect both to pass.

### Task 4: Prove generic rendering and the unhealthy boundary

**Files:**
- Modify: `apps/goods-garden-web/tests/App.test.tsx`

**Interfaces:**
- Consumes: `App({ view })` and `GoodsStateView`.
- Produces: tests proving data-driven rendering, synthetic provenance and a read-only surface.

- [x] Construct a generic `GoodsStateView` with a different product name and an unhealthy supplied assessment; keep provenance synthetic and use no tuna-mayo branch.
- [x] Assert the alternate identity, unhealthy status and supplied explanation are visible.
- [x] Assert `SYNTHETIC EXAMPLE` remains visible and no button, form or action link is rendered.
- [x] Run `npm test --prefix apps/goods-garden-web -- --reporter=verbose`; expect all component tests to pass.

### Task 5: Add CI and complete the directory documentation

**Files:**
- Modify: `.github/workflows/ci.yml`
- Modify: `README.md`
- Modify: `docs/README.md`
- Modify: `docs/architecture/frontend.md`
- Modify: `docs/phases/phase-1-first-living-goods.md`
- Modify: `AGENTS.md`

**Interfaces:**
- Consumes: the working frontend package and its scripts.
- Produces: a separate frontend CI job and first-time developer guidance in three languages.

- [x] Use Node 24 with npm cache keyed by `apps/goods-garden-web/package-lock.json`.
- [x] In `apps/goods-garden-web/`, run `npm ci`, `npm run typecheck`, `npm test` and `npm run build`.
- [x] Document `npm ci`, `npm run dev`, `npm run typecheck`, `npm test` and `npm run build` in English, Japanese and Chinese.
- [x] Run `bash tests/architecture/check-boundaries.sh` and `git diff --check`; expect the existing Rust boundary checks to pass and the web package to remain outside Cargo.

### Task 6: Run complete verification and governed delivery

**Files:**
- Modify: `.ai/work-items/active/frontend-foundation` only through AI Cockpit commands.

**Interfaces:**
- Consumes: all source, tests, docs, lockfile and CI changes.
- Produces: actual verification evidence and a human-readable Outcome.

- [ ] Run `npm ci --prefix apps/goods-garden-web`, `npm run typecheck --prefix apps/goods-garden-web`, `npm test --prefix apps/goods-garden-web` and `npm run build --prefix apps/goods-garden-web`.
- [ ] Run `cargo fmt --check`, `cargo check --workspace`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo test --workspace`, `bash tests/architecture/check-boundaries.sh` and `git diff --check`.
- [ ] Confirm Cargo members do not include `apps/goods-garden-web`, frontend source has no Rust or external-system imports, and no prohibited lifecycle behavior exists.
- [ ] Record actual scenario evidence with AI Cockpit, run verification, finish only after a green/verified state, and present facts, unknowns, evidence, risks, human decision and next action before delivery finalization.

---

## 日本語

### 実装方針

承認済み specification に従い、`apps/goods-garden-web/` に独立した React + TypeScript + Vite browser package を作成する。Cargo workspace の member には追加せず、local synthetic fixture を frontend-only `GoodsStateView` に投影し、read-only React screen として表示する。Tauri desktop と native mobile は実装しない。

### タスクと検証

1. `AGENTS.md`、root README、docs index、architecture、Phase 1 を三言語で更新し、frontend sibling boundary、実行 command、`SYNTHETIC EXAMPLE`、platform 制限を記録する。`docs/architecture/frontend.md` も作成する。
2. package manifest、lockfile、TypeScript/Vite config、HTML、test setup を作成し、`App` が未作成の failing test を先に実行する。その後、projection type、synthetic fixture、read-only components、browser entry point を実装する。
3. generic Goods Profile-shaped fixture と unhealthy assessment の test を追加し、tuna-mayo 専用 branch がないこと、button/form/action surface がないことを確認する。
4. CI に Node 24 の独立 job を追加し、frontend directory で `npm ci`、`npm run typecheck`、`npm test`、`npm run build` を実行する。三言語 documentation と architecture boundary を完成する。
5. frontend、Rust、boundary、git diff、AI Cockpit の実際の verification を実行し、scenario evidence と human Outcome を記録してから delivery finalization を行う。

### 境界

frontend は Rust crate、POS、database、API、LLM/provider を import または call しない。Need、Care、Memory、Learning、自律 action、production Web UI、Tauri shell、native mobile、real SEJ/POS data は対象外であり、不明な事実は `UNKNOWN` と記録する。

---

## 中文

### 实施方针

按照已批准的 specification，在 `apps/goods-garden-web/` 创建独立的 React + TypeScript + Vite 浏览器 package。它不加入 Cargo workspace member，只把 local synthetic fixture 投影为前端专用 `GoodsStateView`，再以只读 React 页面展示。Tauri 桌面端和原生移动端不实现。

### 任务与验证

1. 以三种语言更新 `AGENTS.md`、根 README、docs index、architecture 和 Phase 1，记录前端 sibling 边界、运行命令、`SYNTHETIC EXAMPLE` 和平台限制；新建 `docs/architecture/frontend.md`。
2. 创建 package manifest、lockfile、TypeScript/Vite 配置、HTML 和测试 setup；先执行 `App` 尚不存在的 failing test，再添加 projection type、synthetic fixture、只读组件和浏览器入口。
3. 增加 generic Goods Profile-shaped fixture 与 unhealthy assessment 测试，证明没有 tuna-mayo 专用分支，并且不存在 button/form/action surface。
4. 在 CI 增加独立 Node 24 job，在前端目录执行 `npm ci`、`npm run typecheck`、`npm test`、`npm run build`；完成三语文档和 architecture boundary。
5. 执行前端、Rust、boundary、git diff 与 AI Cockpit 的真实验证，记录 scenario evidence 和 human Outcome，再进行 delivery finalization。

### 边界

前端不得 import 或 call Rust crate、POS、数据库、API 或 LLM/provider。Need、Care、Memory、Learning、自主行动、生产 Web UI、Tauri shell、原生移动端、真实 SEJ/POS 数据均不在范围内；未知事实记录为 `UNKNOWN`。
