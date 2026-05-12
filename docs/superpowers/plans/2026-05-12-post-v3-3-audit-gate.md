# Post-v3.3 Audit Gate Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Complete the post-v3.3 HotSAS Studio audit gate by auditing code, UI, CLI, documentation, tests, and specification compliance, fixing safe issues, and producing final audit artifacts.

**Architecture:** The audit follows the existing HotSAS layered structure: React UI through Tauri commands, the `hotsas_api` facade, application services, core domain models and ports, then adapters and CLI. Fixes must preserve this boundary and remain within post-v3.3 audit scope rather than starting v3.4 feature work.

**Tech Stack:** Rust workspace under `engine/`, React/Vite/Tauri frontend under `apps/desktop-tauri/`, Markdown documentation under `docs/`, PowerShell commands on Windows.

---

### Task 1: Preflight And Inventory

**Files:**

- Read: `README.md`
- Read: `docs/testing/latest_verification_log.md`
- Read: `docs/testing/TESTING.md`
- Read: `docs/testing/verification_logs/*.md`
- Read: `docs/testing/acceptance_matrices/*.md`
- Read: root `HotSAS_*TZ*.md`, `HotSAS_*verification*`, `HotSAS_*acceptance*`, `HotSAS_*template*`
- Create later from evidence: `docs/audits/POST_V3_3_DEEP_CODE_PRODUCT_AUDIT.md`

- [ ] **Step 1: Record git preflight**

Run each command from repository root:

```powershell
git rev-parse --show-toplevel
git branch --show-current
git status --short
git log --oneline -20
git remote -v
git diff --stat
git diff --name-only
```

Expected: repository root is `D:/Документы/vscode/HotSAS Studio`, branch is `main`, no tracked diff before audit edits, and any untracked root context/export files are recorded but not staged.

- [ ] **Step 2: Build TZ/spec inventory**

Run:

```powershell
Get-ChildItem -Recurse -File |
  Where-Object {
    $_.Name -match "HotSAS.*(TZ|ТЗ|verification|acceptance|template)" -or
    $_.FullName -match "docs\\testing\\verification_logs" -or
    $_.FullName -match "docs\\testing\\acceptance_matrices"
  } |
  Select-Object -ExpandProperty FullName
```

Expected: all root-level historical TZ/template files plus docs verification logs and acceptance matrices are listed. Root context exports under `qwen/`, `review_vneshnoe/`, `.kilo/`, `kimi-export*`, and `codex_history*` are treated as context only unless already tracked.

- [ ] **Step 3: Read current source of truth**

Run:

```powershell
Get-Content -LiteralPath README.md
Get-Content -LiteralPath docs/testing/latest_verification_log.md
Get-Content -LiteralPath docs/testing/TESTING.md
```

Expected: current accepted version and known limitations are captured for audit report and roadmap recommendation.

### Task 2: Architecture And Backend Audit

**Files:**

- Read: `engine/Cargo.toml`
- Read: `engine/core/**`
- Read: `engine/ports/**`
- Read: `engine/application/**`
- Read: `engine/adapters/**`
- Read: `engine/api/**`
- Read: `engine/cli/**`
- Modify if safe issues are found: focused files in the same directories
- Test if modified: matching Rust unit or integration tests

- [ ] **Step 1: Check dependency boundaries**

Run:

```powershell
cd engine
cargo tree
```

Run from repo root:

```powershell
rg "from .*adapters|hotsas_adapters|invoke\(|generate_spice|ngspice|Touchstone|SParameter|localStorage|fs" apps engine
```

Expected: core has no application/adapters/api/frontend dependencies, UI does not implement backend-only parsing/simulation/netlisting logic, and CLI delegates through API/application rather than duplicating business logic.

- [ ] **Step 2: Inspect critical backend models and services**

Use `rg --files engine` and focused reads for:

```text
EngineeringValue / ValueWithUnit
FormulaDefinition / FormulaPack
ComponentDefinition / ComponentInstance
SimulationModel / model mapping
ProjectPackage
Export models
SimulationWorkflow
TwoPortFilterAnalysis
SParameters / Touchstone models
ProjectSessionService / persistence
```

Expected: serialization is stable, user input paths avoid panics, diagnostics carry enough UI context, and adapters avoid silent data loss.

- [ ] **Step 3: Fix small backend defects with tests**

For each backend defect that is safe to fix, first add a failing Rust test in the closest existing test module, run the specific test to confirm failure, implement the minimum fix, then rerun the specific test.

Expected: any fixed backend issue receives an issue-register ID and regression evidence.

### Task 3: Frontend, Tauri, UI, And Layout Audit

**Files:**

- Read: `apps/desktop-tauri/src/**`
- Read: `apps/desktop-tauri/src-tauri/**`
- Modify if safe issues are found: focused UI, Tauri command, or test files
- Test if modified: matching Vitest test files under `apps/desktop-tauri/src`

- [ ] **Step 1: Inspect app routes and screens**

Run:

```powershell
rg "Workbench|Schematic|Formula|Component|Selected|Export|Simulation|DC-DC|Filter|S-Parameter|Project|Recent|Settings|Help|About" apps/desktop-tauri/src
```

Expected: every navigation target opens a defined screen/component, and no blank-screen imports are found.

- [ ] **Step 2: Audit UI reliability patterns**

Run:

```powershell
rg "useEffect|echarts|ResizeObserver|console\\.error|console\\.warn|key=|aria-|disabled|localStorage|invoke\\(" apps/desktop-tauri/src
```

Expected: chart lifecycles dispose instances, async backend calls expose errors, impossible actions are disabled, and forms have accessible labels or obvious aria attributes.

- [ ] **Step 3: Fix small UI defects with tests**

For each UI defect that is safe to fix, add or update a focused Vitest test first, run the specific test to confirm failure, implement the minimum fix, then rerun the specific test.

Expected: any fixed UI/layout issue receives an issue-register ID and regression evidence.

### Task 4: CLI, Documentation, And Repository Hygiene Audit

**Files:**

- Read: `engine/cli/**`
- Read: `docs/**`
- Read: `.gitignore`
- Modify if safe issues are found: docs or focused CLI tests

- [ ] **Step 1: Audit CLI command surface**

Run help and representative command checks after building the CLI or by using `cargo run -p hotsas_cli --`:

```powershell
cargo run -p hotsas_cli -- --help
cargo run -p hotsas_cli -- validate --help
cargo run -p hotsas_cli -- formula --help
cargo run -p hotsas_cli -- netlist --help
cargo run -p hotsas_cli -- export --help
cargo run -p hotsas_cli -- simulate --help
cargo run -p hotsas_cli -- simulate-diagnostics --help
cargo run -p hotsas_cli -- simulation-history --help
cargo run -p hotsas_cli -- model-check --help
cargo run -p hotsas_cli -- filter-analyze --help
cargo run -p hotsas_cli -- sparams --help
```

Expected: command names match the current docs, unsupported or missing input states report clear errors, and JSON flags are documented where supported.

- [ ] **Step 2: Audit docs and logs**

Read latest verification docs and acceptance matrices from v1.2 through v3.3. Check for broken tables, stale "next" labels, missing known limitations, and mismatches against README.

Expected: accepted v3.1, v3.2, and v3.3 limitations remain visible: deferred imported model persistence, user-editable parameter binding persistence, foundation-level v3.2 filter behavior, ngspice impedance extraction foundation-only, no VNA-grade claims, no calibration/de-embedding, no Smith chart unless separately implemented, and only 1-port/2-port Touchstone support.

- [ ] **Step 3: Audit repository hygiene**

Run:

```powershell
git status --short
git ls-files | rg "(kimi|codex|export|\.exe|\.zip|node_modules|target|secret|key|token)"
```

Expected: suspicious tracked files are reported; untracked root context and temporary fix scripts are not staged.

### Task 5: Audit Artifacts

**Files:**

- Create: `docs/audits/POST_V3_3_DEEP_CODE_PRODUCT_AUDIT.md`
- Create: `docs/audits/POST_V3_3_ISSUE_REGISTER.md`
- Create: `docs/audits/POST_V3_3_REQUIREMENT_TRACEABILITY_MATRIX.md`
- Create: `docs/audits/POST_V3_3_UI_UX_AUDIT.md`
- Create: `docs/audits/POST_V3_3_ROADMAP_RECOMMENDATION.md`
- Create: `docs/testing/verification_logs/post_v3_3_audit_gate.md`
- Create: `docs/testing/acceptance_matrices/post_v3_3_audit_gate_acceptance_matrix.md`
- Modify: `docs/testing/latest_verification_log.md`
- Modify: `docs/testing/TESTING.md`
- Create if audit must pause: `docs/audits/POST_V3_3_AUDIT_RESUME_STATE.md`

- [ ] **Step 1: Write audit report**

The report must include executive summary, git preflight, methodology, TZ/spec inventory, architecture audit, backend audit, frontend/UI audit, CLI audit, docs audit, test coverage audit, repository hygiene audit, issue summary, fixes applied, remaining risks, and recommendation before v3.4.

Expected: the report uses only evidence observed in this audit and explicitly says when manual GUI verification was unavailable.

- [ ] **Step 2: Write issue register**

Each issue row includes ID, severity, area, status, found-in evidence, impact, fix or decision, tests, and commit placeholder text `Pending final commit` until a commit exists.

Expected: fixed and deferred issues are distinct, and no invented issue is marked fixed.

- [ ] **Step 3: Write traceability matrix**

Map each required version/stage from v1.0, v1.2, v1.5, v1.6, v1.7, v1.8, v1.9, v2.0, v2.1, v2.2, v2.6, v2.7, v2.8, v2.9, v3.0, v3.1, v3.2, and v3.3 to implementation, tests, docs, status, issues, and fix/issue ID.

Expected: missing historical evidence is marked `NOT FOUND IN CURRENT REPO / NEEDS HISTORICAL CONFIRMATION` instead of invented.

- [ ] **Step 4: Write UI/UX audit and roadmap recommendation**

UI/UX audit includes screen-by-screen review, layout/debugging findings, manual smoke status, fixed UI issues, remaining limitations, and recommendations. Roadmap recommendation states whether v3.4 can start and whether persistence hardening should precede new RF features.

Expected: recommended next scope is conservative and respects documented v3.1-v3.3 limitations.

### Task 6: Verification And Final State

**Files:**

- Modify with final results: `docs/testing/verification_logs/post_v3_3_audit_gate.md`
- Modify with final results: `docs/testing/latest_verification_log.md`
- Modify with final results: `docs/testing/TESTING.md`
- Modify with final results: `docs/testing/acceptance_matrices/post_v3_3_audit_gate_acceptance_matrix.md`

- [ ] **Step 1: Run Rust verification**

Run from `engine/`:

```powershell
cargo fmt --check
cargo test
cargo build -p hotsas_cli --release
```

Expected: PASS, or failures are recorded with exact failing command and cause.

- [ ] **Step 2: Run frontend verification**

Run from `apps/desktop-tauri/`:

```powershell
npm.cmd run format:check
npm.cmd run typecheck
npm.cmd run test
npm.cmd run build
npm.cmd run tauri:build
```

Expected: PASS, or failures are recorded with exact failing command and cause.

- [ ] **Step 3: Run repository verification**

Run from repository root:

```powershell
git diff --check
git status --short
git diff --stat
git diff --name-only
```

Expected: whitespace check passes; changed files are scoped to audit outputs, safe fixes, and docs updates; untracked root context files remain uncommitted.

- [ ] **Step 4: Pause honestly or finish**

If all audit artifacts and verification are complete, final response reports PASS/PARTIAL/FAIL with commands and evidence. If context, time, tooling, or environment prevents completion, write `docs/audits/POST_V3_3_AUDIT_RESUME_STATE.md` with exact completed phases, checked files, PASS/FAIL/NOT RUN commands, remaining tasks, and next command.

Expected: no final answer claims completion unless the evidence supports it.
