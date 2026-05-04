# ТЗ: HotSAS Studio v2.0 — Product Beta Integration, Workflow Stabilization & Internal RC Build

## 0. Краткое назначение этапа

Выполнить следующий этап дорожной карты:

```text
v2.0 — Product Beta
```

Это не этап добавления одной новой крупной подсистемы. Это **интеграционный этап**, который должен собрать уже реализованные возможности HotSAS Studio в более цельный пользовательский продуктовый сценарий:

```text
создать/открыть проект
→ увидеть схему
→ рассчитать формулы
→ использовать Engineering Notebook
→ назначить компоненты
→ импортировать SPICE/Touchstone модели
→ запустить mock/ngspice simulation
→ проанализировать выбранный участок
→ экспортировать результаты
→ проверить readiness через Diagnostics
→ собрать внутренний Windows .exe
```

По roadmap v2.0 — первая версия, где программа уже должна быть похожа на самостоятельный инженерный инструмент. В проекте уже накоплены фундаментальные блоки: `.circuit` package storage, schematic editor foundations, formula registry/calculation, exact E-series, calculator notebook, component library foundation, export center, basic ngspice и SPICE/Touchstone import foundation.

---

## 1. Текущий статус перед v2.0

Считать, что проект прошёл:

```text
v1.0 — Initial Vertical Slice
v1.0.1 — Architecture Hardening
v1.1.1 — Formatting + Build/Test Infrastructure
v1.1.2 — Backend Test Expansion
v1.1.3 — FormulaPackLoader + FormulaRegistry
v1.1.4-fix — Generic Formula Engine Completion
v1.1.5 — Exact E-Series Tables
v1.2 — Project Package Storage .circuit
v1.3 — Schematic Editor Foundations
v1.4 — Engineering Notebook / Calculator Foundations
v1.5 — Component Library Foundation
v1.6 — Selected Region Analysis Foundation
v1.7 — Export Center v1
v1.8 — ngspice Adapter v1
v1.9 — SPICE/Touchstone Import Foundation
v1.10 — Internal Alpha EXE Build & v2.0 Readiness Gate
```

Текущий известный HEAD после amend:

```text
e44830b — v1.10 — Internal Alpha EXE Build and v2.0 Readiness Gate
```

### Известные обязательные правки текущего состояния

Перед началом v2.0 нужно закрыть небольшие документационные долги v1.10:

```text
1. В docs/testing/verification_logs/v1.10_internal_alpha_build_and_v2_readiness_gate.md
   заменить устаревший commit c0cdb35 на фактический e44830b.

2. В этом же файле заменить:
   a360e9c..c0cdb35 → a360e9c..e44830b

3. В agent self-check заменить:
   Git commit создан PASS c0cdb35 → Git commit создан PASS e44830b

4. В docs/testing/latest_verification_log.md добавить краткую v1.10-сводку:
   - commit e44830b
   - cargo test 166+ PASS
   - npm test 68 PASS
   - tauri:build PASS
   - EXE path/size/SHA256
   - ZIP path/size/SHA256
   - public GitHub Release: NO

5. В docs/testing/TESTING.md добавить Manual v1.10 Internal Alpha Smoke Check:
   - Diagnostics opens
   - Refresh diagnostics works
   - Run readiness self-check works
   - EXE starts
   - core screens navigable
   - ngspice unavailable is controlled warning
```

Эти правки можно сделать в начале v2.0 отдельным коммитом:

```text
docs(v1.10-fix): update alpha build verification metadata
```

После этого выполнить основной этап:

```text
v2.0 — Product Beta Integration, Workflow Stabilization & Internal RC Build
```

---

## 2. Что изменится для пользователя после v2.0

После v2.0 программа должна восприниматься не как набор отдельных экранов, а как связанный инженерный workflow.

### Пользователь должен получить

```text
1. Более понятный Start / Project Hub:
   - создать demo project;
   - увидеть текущий статус проекта;
   - перейти к ключевым шагам работы;
   - понять, какие модули ready/limited/unavailable.

2. Единый workflow для RC demo:
   - создать проект;
   - открыть схему;
   - посмотреть компоненты;
   - рассчитать формулу;
   - подобрать E-series;
   - открыть notebook;
   - назначить библиотечный компонент;
   - запустить simulation;
   - открыть selected region analysis;
   - экспортировать report/netlist/BOM/SVG;
   - проверить Diagnostics.

3. Экран Diagnostics должен стать не просто техническим отчётом,
   а понятным readiness dashboard:
   - Formula Library ready/limited;
   - Engineering Notebook ready/limited;
   - Component Library ready/limited;
   - Import Models ready/limited;
   - Export Center ready/limited;
   - ngspice ready/unavailable/limited;
   - .circuit package storage ready;
   - internal EXE build info if available.

4. Внутреннюю Windows-сборку:
   - .exe собран;
   - путь, размер и SHA256 зафиксированы;
   - Windows GUI subsystem подтверждён;
   - внутренний ZIP создан;
   - ZIP/EXE не добавлены в git.

5. Понятный v2.0 quick start:
   - как запустить из исходников;
   - как собрать exe;
   - как пройти demo workflow;
   - какие функции уже работают;
   - какие функции ещё limited.
```

### Статус продукта после v2.0

```text
Это Product Beta / Internal RC, но НЕ публичный релиз.

Запрещено автоматически создавать:
- публичный GitHub Release;
- публичный release tag;
- installer как stable release;
- заявления о production-ready статусе.

Разрешено:
- internal RC build;
- local .exe;
- internal ZIP для проверки на другом ПК;
- smoke test на другом Windows-ПК, если доступен.
```

---

## 3. Жёсткие ограничения scope

### Запрещено

```text
- не делать PCB editor;
- не делать routing;
- не делать Gerber;
- не делать KiCad project export;
- не делать proprietary Altium file generation;
- не делать full symbolic solver;
- не добавлять SymPy/Lcapy/math.js bridge;
- не делать advanced DC-DC calculators;
- не делать Formula Library Expansion v2.1;
- не делать DC-DC Templates v2.2;
- не делать advanced reports v2.3;
- не переписывать весь UI;
- не переносить бизнес-логику во frontend;
- не запускать ngspice из React;
- не парсить SPICE/Touchstone во frontend;
- не коммитить EXE/ZIP/MSI/target/dist/node_modules;
- не создавать публичный GitHub Release без отдельного разрешения пользователя.
```

### Разрешено

```text
- исправить v1.10 verification metadata;
- улучшить Start / Project Hub;
- добавить ProductWorkflowService или ProductBetaReadinessService;
- расширить AppDiagnosticsService;
- добавить unified workflow status DTO;
- добавить API/Tauri команды для workflow/readiness;
- улучшить DiagnosticsScreen;
- добавить Product Beta / Workflow screen или встроить в Start Page;
- добавить интеграционные smoke-тесты вокруг уже существующих сервисов;
- обновить docs/user_manual;
- обновить docs/builds;
- создать v2.0 verification log;
- собрать .exe и internal ZIP;
- commit + push.
```

---

## 4. Preflight перед изменениями

Выполнить из корня проекта:

```bash
cd "D:\Документы\vscode\HotSAS Studio"

git rev-parse --show-toplevel
git branch --show-current
git status --short
git log --oneline -12
git remote -v
git diff --stat
git diff --name-only
```

Правила:

```text
- Не выполнять git reset.
- Не выполнять git clean.
- Не удалять пользовательские untracked-файлы.
- Не трогать личные материалы пользователя:
  - ТЗ-файлы;
  - verification log templates;
  - kimi exports;
  - фотографии;
  - локальные заметки.
- Если есть untracked файлы — записать список в verification log и не добавлять их автоматически.
- Если есть tracked изменения до начала работы — записать это и не затирать без необходимости.
```

Проверить текущую базу:

```bash
cd "D:\Документы\vscode\HotSAS Studio\engine"
cargo fmt --check
cargo test
```

```bash
cd "D:\Документы\vscode\HotSAS Studio\apps\desktop-tauri"
npm.cmd run format:check
npm.cmd run typecheck
npm.cmd run test
npm.cmd run build
```

---

## 5. Обязательный блок A — v1.10 metadata fix

Сначала выполнить документационный fix.

### 5.1. Исправить v1.10 verification log

Файл:

```text
docs/testing/verification_logs/v1.10_internal_alpha_build_and_v2_readiness_gate.md
```

Заменить:

```text
Implementation commit: c0cdb35
Verification log update commit: c0cdb35
Push status: origin/main OK (a360e9c..c0cdb35)
Git commit создан PASS c0cdb35
```

на:

```text
Implementation commit: e44830b
Verification log update commit: e44830b
Push status: origin/main OK (a360e9c..e44830b)
Git commit создан PASS e44830b
```

Если фактический git history показывает другой base range — указать фактический range из `git log`.

### 5.2. Расширить latest verification log

Файл:

```text
docs/testing/latest_verification_log.md
```

Добавить краткую v1.10-сводку:

```text
Version: v1.10 — Internal Alpha EXE Build & v2.0 Readiness Gate
Implementation commit: e44830b
Verification log: docs/testing/verification_logs/v1.10_internal_alpha_build_and_v2_readiness_gate.md

Checks:
- cargo fmt --check — PASS
- cargo test — PASS (166+ Rust tests)
- npm run format:check — PASS
- npm run typecheck — PASS
- npm run test — PASS (68 frontend tests)
- npm run build — PASS
- npm run tauri:build — PASS

Internal build:
- EXE path: apps/desktop-tauri/src-tauri/target/release/hotsas_desktop_tauri.exe
- EXE SHA256: <from v1.10 log>
- ZIP path: apps/desktop-tauri/src-tauri/target/release/HotSAS-Studio-v1.10-internal-alpha-windows-x64.zip
- ZIP SHA256: <from v1.10 log>
- Public GitHub Release: NO
```

### 5.3. Добавить Manual v1.10 smoke в TESTING.md

Файл:

```text
docs/testing/TESTING.md
```

Добавить раздел:

```markdown
## Manual v1.10 Internal Alpha Smoke Check

- [ ] Release EXE starts.
- [ ] No console window appears for release EXE.
- [ ] Start screen opens.
- [ ] Diagnostics screen opens.
- [ ] Refresh diagnostics works.
- [ ] Run readiness self-check works.
- [ ] Formula Library screen opens.
- [ ] Engineering Notebook screen opens.
- [ ] Component Library screen opens.
- [ ] Simulation screen opens.
- [ ] Import Models screen opens.
- [ ] Export Center screen opens.
- [ ] ngspice unavailable is shown as controlled warning/limited status.
```

### 5.4. Commit v1.10-fix

После этого:

```bash
git status --short
git diff --stat
git add docs/testing/latest_verification_log.md docs/testing/TESTING.md docs/testing/verification_logs/v1.10_internal_alpha_build_and_v2_readiness_gate.md
git commit -m "docs(v1.10-fix): update alpha build verification metadata"
```

Если эти изменения решено включить в один v2.0 commit — явно объяснить это в verification log. Рекомендуемый вариант — отдельный commit.

---

## 6. Блок B — Product workflow / readiness domain

### 6.1. Core models

Добавить или расширить core-модели. Рекомендуемый файл:

```text
engine/core/src/product_workflow.rs
```

Подключить в:

```text
engine/core/src/lib.rs
```

Модели:

```rust
pub struct ProductWorkflowStatus {
    pub app_name: String,
    pub app_version: String,
    pub roadmap_stage: String,
    pub current_project: Option<ProjectSummary>,
    pub workflow_steps: Vec<WorkflowStepStatus>,
    pub module_statuses: Vec<WorkflowModuleStatus>,
    pub blockers: Vec<String>,
    pub warnings: Vec<String>,
}

pub struct ProjectSummary {
    pub project_id: String,
    pub project_name: String,
    pub format_version: String,
    pub component_count: usize,
    pub net_count: usize,
    pub simulation_profile_count: usize,
}

pub struct WorkflowStepStatus {
    pub id: String,
    pub title: String,
    pub status: WorkflowStatusKind,
    pub screen_id: String,
    pub description: String,
    pub warnings: Vec<String>,
}

pub struct WorkflowModuleStatus {
    pub id: String,
    pub title: String,
    pub status: WorkflowStatusKind,
    pub details: BTreeMap<String, String>,
}

pub enum WorkflowStatusKind {
    Ready,
    Limited,
    Unavailable,
    NotConfigured,
    Error,
}
```

Если уже есть похожие diagnostics-модели, не дублировать без необходимости. Можно:

```text
- переиспользовать AppDiagnosticsReport;
- добавить ProductWorkflowStatus как thin wrapper;
- добавить conversion helpers.
```

### 6.2. Цель моделей

Модели должны описывать не отдельный низкоуровневый тест, а пользовательский workflow:

```text
project
schematic
formula_library
engineering_notebook
component_library
model_import
simulation
selected_region
export_center
diagnostics
internal_build
```

---

## 7. Блок C — Application layer

Добавить сервис:

```text
engine/application/src/services/product_workflow.rs
```

или расширить `AppDiagnosticsService`, если архитектурно лучше.

Рекомендуемый сервис:

```rust
pub struct ProductWorkflowService;
```

Методы:

```rust
pub fn get_product_workflow_status(
    &self,
    services: &AppServices,
) -> ProductWorkflowStatus;

pub fn run_product_beta_self_check(
    &self,
    services: &AppServices,
) -> ProductWorkflowStatus;

pub fn create_integrated_demo_project(
    &self,
    services: &mut AppServices,
) -> Result<ProjectDto-like-core-model, ApplicationError>;
```

Если нельзя возвращать API DTO из application — вернуть core/application модель, а DTO делать в `hotsas_api`.

### 7.1. Что проверять в `get_product_workflow_status`

Лёгкая status-проверка без тяжёлых операций:

```text
- app version / roadmap stage;
- есть ли current project;
- Formula Registry доступен;
- базовые formula packs загружены;
- Engineering Notebook service доступен;
- Component Library service доступен;
- Import Models service доступен;
- Export Center capabilities count >= 9;
- Simulation service доступен;
- ngspice availability status controlled;
- Selected Region service доступен;
- Diagnostics service доступен.
```

### 7.2. Что проверять в `run_product_beta_self_check`

Более полный self-check, но всё ещё быстрый и deterministic:

```text
1. Create RC low-pass demo project.
2. Validate schematic.
3. Calculate RC cutoff.
4. Evaluate Formula Library rc_low_pass_cutoff.
5. Evaluate Notebook assignment/formula call smoke if API/service available.
6. Load built-in component library.
7. Assign generic resistor to R1 or validate assignment path.
8. Generate SPICE netlist.
9. Run mock simulation.
10. Check ngspice availability returns Ready/Limited/Unavailable without panic.
11. Run selected region preview/analyze for R1+C1 if available.
12. List export capabilities and require 9 formats.
13. Export Markdown/HTML/SPICE/BOM/SVG to string.
14. Parse minimal SPICE `.model` smoke.
15. Parse minimal Touchstone `.s2p` smoke.
16. Return blockers/warnings in structured form.
```

### 7.3. Важное правило

`run_product_beta_self_check` не должен:

```text
- требовать установленный ngspice;
- требовать интернет;
- требовать второго ПК;
- создавать публичный релиз;
- писать в пользовательские папки без явного пути;
- падать panic-ом.
```

Все ошибки должны быть controlled `Result` / warnings / limited status.

---

## 8. Блок D — API DTO / facade

Добавить DTO в:

```text
engine/api/src/dto.rs
```

Минимум:

```rust
pub struct ProductWorkflowStatusDto {
    pub app_name: String,
    pub app_version: String,
    pub roadmap_stage: String,
    pub current_project: Option<ProjectSummaryDto>,
    pub workflow_steps: Vec<WorkflowStepStatusDto>,
    pub module_statuses: Vec<WorkflowModuleStatusDto>,
    pub blockers: Vec<String>,
    pub warnings: Vec<String>,
}

pub struct ProjectSummaryDto {
    pub project_id: String,
    pub project_name: String,
    pub format_version: String,
    pub component_count: usize,
    pub net_count: usize,
    pub simulation_profile_count: usize,
}

pub struct WorkflowStepStatusDto {
    pub id: String,
    pub title: String,
    pub status: String,
    pub screen_id: String,
    pub description: String,
    pub warnings: Vec<String>,
}

pub struct WorkflowModuleStatusDto {
    pub id: String,
    pub title: String,
    pub status: String,
    pub details: Vec<KeyValueDto>,
}
```

Если `KeyValueDto` уже есть — переиспользовать.

В `engine/api/src/facade.rs` добавить методы:

```rust
pub fn get_product_workflow_status(&self) -> Result<ProductWorkflowStatusDto, ApiError>;

pub fn run_product_beta_self_check(&mut self) -> Result<ProductWorkflowStatusDto, ApiError>;

pub fn create_integrated_demo_project(&mut self) -> Result<ProjectDto, ApiError>;
```

Если `HotSasApi` уже имеет mutable/current-project state, использовать существующий паттерн.

---

## 9. Блок E — Tauri commands

В:

```text
apps/desktop-tauri/src-tauri/src/lib.rs
```

добавить commands:

```rust
#[tauri::command]
async fn get_product_workflow_status(
    state: tauri::State<'_, AppState>,
) -> Result<ProductWorkflowStatusDto, String>;

#[tauri::command]
async fn run_product_beta_self_check(
    state: tauri::State<'_, AppState>,
) -> Result<ProductWorkflowStatusDto, String>;

#[tauri::command]
async fn create_integrated_demo_project(
    state: tauri::State<'_, AppState>,
) -> Result<ProjectDto, String>;
```

Добавить их в:

```rust
tauri::generate_handler![
    ...
]
```

И в permissions/capabilities:

```text
apps/desktop-tauri/src-tauri/permissions/hotsas.toml
```

или актуальный permissions-файл проекта.

---

## 10. Блок F — Frontend types/API/store

### 10.1. Types

В:

```text
apps/desktop-tauri/src/types/index.ts
```

добавить:

```ts
export interface ProductWorkflowStatusDto { ... }
export interface ProjectSummaryDto { ... }
export interface WorkflowStepStatusDto { ... }
export interface WorkflowModuleStatusDto { ... }
```

### 10.2. API

В:

```text
apps/desktop-tauri/src/api/index.ts
```

добавить методы:

```ts
getProductWorkflowStatus(): Promise<ProductWorkflowStatusDto>
runProductBetaSelfCheck(): Promise<ProductWorkflowStatusDto>
createIntegratedDemoProject(): Promise<ProjectDto>
```

### 10.3. Store

В:

```text
apps/desktop-tauri/src/store/index.ts
```

добавить поля:

```ts
productWorkflowStatus: ProductWorkflowStatusDto | null
productWorkflowLoading: boolean
productWorkflowError: string | null
```

И actions:

```ts
setProductWorkflowStatus
setProductWorkflowLoading
setProductWorkflowError
```

---

## 11. Блок G — UI: Product Beta / Project Hub

Есть два допустимых варианта.

### Вариант A — улучшить Start Screen

Если Start Screen уже есть и архитектурно удобен, превратить его в Project Hub.

Добавить блоки:

```text
1. Current Project
2. Guided Workflow
3. Module Readiness
4. Internal Build / v2.0 status
5. Quick actions
```

### Вариант B — добавить отдельный экран

Добавить:

```text
apps/desktop-tauri/src/screens/ProductBetaScreen.tsx
```

И пункт навигации:

```text
Product Beta
```

Рекомендуемый вариант: **A + минимальный B**, если это не усложняет UI.

### 11.1. Guided Workflow steps

Показать шаги:

```text
1. Project
2. Schematic
3. Formula Library
4. Engineering Notebook
5. Component Library
6. Model Import
7. Simulation
8. Selected Region
9. Export Center
10. Diagnostics
```

Каждый шаг должен показывать:

```text
- title;
- status badge: Ready / Limited / Unavailable / Error;
- короткое описание;
- кнопку перехода на экран;
- warnings, если есть.
```

### 11.2. Quick actions

Добавить кнопки:

```text
- Create integrated demo project
- Refresh workflow status
- Run product beta self-check
- Open Diagnostics
- Open Export Center
```

### 11.3. Что UI не должен делать

```text
- UI не должен сам рассчитывать formula readiness;
- UI не должен сам парсить SPICE/Touchstone;
- UI не должен сам строить netlist;
- UI не должен сам решать, какие модули ready;
- UI только вызывает backend commands и отображает DTO.
```

---

## 12. Блок H — Diagnostics v2.0 improvements

Расширить существующий Diagnostics screen:

```text
1. Добавить связь с Product Workflow Status.
2. Показать v2.0 readiness blockers.
3. Показать controlled unavailable для ngspice.
4. Показать, что public release не создан.
5. Показать, что internal EXE build должен проверяться отдельно.
```

Если на этом этапе нет доступа к build metadata runtime — не пытаться читать EXE hash из приложения. Достаточно документации и verification log.

---

## 13. Блок I — Internal EXE / RC build

Обязательный пункт: снова собрать Windows `.exe`.

Из:

```bash
cd "D:\Документы\vscode\HotSAS Studio\apps\desktop-tauri"
npm.cmd run tauri:build
```

После сборки зафиксировать:

```powershell
$exe = "apps\desktop-tauri\src-tauri\target\release\hotsas_desktop_tauri.exe"
Get-Item $exe | Select-Object FullName, Length, LastWriteTime
Get-FileHash $exe -Algorithm SHA256
```

Проверить Windows subsystem:

```powershell
$path = Resolve-Path "apps\desktop-tauri\src-tauri\target\release\hotsas_desktop_tauri.exe"
$bytes = [System.IO.File]::ReadAllBytes($path)
$peOffset = [BitConverter]::ToInt32($bytes, 0x3C)
$optionalHeaderOffset = $peOffset + 24
$subsystem = [BitConverter]::ToUInt16($bytes, $optionalHeaderOffset + 0x44)
$kind = switch ($subsystem) {
    2 { "Windows GUI" }
    3 { "Windows CUI console" }
    default { "Unknown" }
}
"Subsystem=$subsystem ($kind)"
```

Создать internal ZIP:

```powershell
$zip = "apps\desktop-tauri\src-tauri\target\release\HotSAS-Studio-v2.0-internal-rc-windows-x64.zip"
Compress-Archive -Path "apps\desktop-tauri\src-tauri\target\release\hotsas_desktop_tauri.exe" -DestinationPath $zip -Force
Get-Item $zip | Select-Object FullName, Length, LastWriteTime
Get-FileHash $zip -Algorithm SHA256
```

Проверить:

```text
- EXE exists: YES
- EXE SHA256 recorded
- ZIP exists: YES
- ZIP SHA256 recorded
- EXE/ZIP committed to git: NO
- Public GitHub Release created: NO
- Public release tag created: NO
```

---

## 14. Блок J — Tests

### 14.1. Rust application tests

Добавить:

```text
engine/application/tests/product_workflow_tests.rs
```

Минимум тесты:

```text
1. workflow_status_contains_expected_steps
2. workflow_status_marks_formula_library_ready
3. workflow_status_marks_component_library_ready_or_limited
4. workflow_status_reports_ngspice_as_controlled_status
5. self_check_creates_rc_demo_without_panic
6. self_check_calculates_rc_cutoff
7. self_check_lists_export_capabilities
8. self_check_reports_import_models_status
9. self_check_collects_warnings_without_failure
10. create_integrated_demo_project_returns_project
```

### 14.2. API tests

Добавить:

```text
engine/api/tests/product_workflow_api_tests.rs
```

Минимум тесты:

```text
1. get_product_workflow_status_returns_steps
2. run_product_beta_self_check_returns_checks
3. create_integrated_demo_project_sets_current_project
4. dto_conversion_preserves_statuses
5. api_does_not_fail_when_ngspice_unavailable
```

### 14.3. Frontend tests

Добавить:

```text
apps/desktop-tauri/src/screens/__tests__/ProductBetaScreen.test.tsx
```

или тесты Start/Diagnostics, если workflow встроен в существующие экраны.

Минимум тесты:

```text
1. renders_product_beta_title
2. renders_guided_workflow_steps
3. refresh_workflow_calls_backend
4. run_self_check_calls_backend
5. create_integrated_demo_project_calls_backend
6. shows_ready_limited_unavailable_badges
7. shows_backend_error_message
8. navigation_buttons_are_rendered
```

### 14.4. Regression tests

Убедиться, что не сломаны:

```text
- Formula Library tests
- Engineering Notebook tests
- Component Library tests
- Selected Region tests
- Export Center tests
- ngspice adapter tests
- SPICE/Touchstone import tests
- Diagnostics tests
```

---

## 15. Блок K — Documentation

### 15.1. Создать Product Beta документацию

Создать:

```text
docs/product/PRODUCT_BETA_V2_0.md
```

Содержание:

```markdown
# HotSAS Studio v2.0 — Product Beta

## Purpose

v2.0 is the first integrated internal product beta.

## What works

- .circuit project package storage
- schematic editor foundations
- formula registry and calculation
- exact E-series
- engineering notebook
- component library foundation
- SPICE/Touchstone import foundation
- selected region analysis foundation
- export center
- basic ngspice adapter
- diagnostics/readiness dashboard

## What is still limited

- no PCB editor
- no routing/Gerber
- no full symbolic solver
- no advanced formula packs v2.1 yet
- no DC-DC calculator pack v2.2 yet
- ngspice may be unavailable unless installed
- Touchstone visualization later
- interactive pin mapper later

## Guided workflow

...

## Internal build

...
```

### 15.2. Создать v2 quick start

Создать:

```text
docs/user_manual/V2_0_PRODUCT_BETA_QUICK_START.md
```

Содержание:

```markdown
# HotSAS Studio v2.0 Product Beta Quick Start

1. Start the app
2. Create integrated demo project
3. Open Schematic
4. Run Formula Library calculation
5. Use Engineering Notebook
6. Assign component from Component Library
7. Import SPICE/Touchstone model
8. Run simulation
9. Analyze selected region
10. Export report/netlist/BOM/SVG
11. Run Diagnostics self-check
```

### 15.3. Обновить build docs

Обновить:

```text
docs/builds/INTERNAL_ALPHA_BUILD.md
```

Добавить v2.0 internal RC section:

```text
HotSAS-Studio-v2.0-internal-rc-windows-x64.zip
```

Не заменять полностью v1.10, а добавить новую секцию.

### 15.4. Обновить README

После выполнения v2.0:

```text
Current roadmap stage: v2.1 next

Completed:
- v2.0 — Product Beta
```

Добавить краткий раздел:

```markdown
## v2.0 — Product Beta

v2.0 integrates the previously implemented modules into a guided engineering workflow...
```

### 15.5. Обновить TESTING.md

Добавить:

```markdown
## v2.0 — Product Beta Integration

Rust tests:
- product workflow service tests
- product workflow API tests
- diagnostics regression
- integration smoke self-check

Frontend tests:
- Product Beta / Project Hub screen
- guided workflow cards
- self-check actions
- error states

Manual v2.0 Product Beta Smoke Check:
...
```

### 15.6. Создать verification log

Создать:

```text
docs/testing/verification_logs/v2.0_product_beta_integration.md
```

Обновить:

```text
docs/testing/latest_verification_log.md
```

---

## 16. Manual v2.0 Product Beta Smoke Check

В verification log обязательно заполнить:

```text
[OK/FAIL] Release EXE starts
[OK/FAIL] No console window appears
[OK/FAIL] Start / Project Hub opens
[OK/FAIL] Create integrated demo project works
[OK/FAIL] Schematic screen opens and shows RC demo
[OK/FAIL] Formula Library opens
[OK/FAIL] Formula calculation works
[OK/FAIL] Engineering Notebook opens
[OK/FAIL] Notebook assignment/formula command works or limited status shown
[OK/FAIL] Component Library opens
[OK/FAIL] Component details open
[OK/FAIL] Import Models screen opens
[OK/FAIL] SPICE text import smoke works
[OK/FAIL] Touchstone text import smoke works
[OK/FAIL] Simulation screen opens
[OK/FAIL] Mock simulation works
[OK/FAIL] ngspice unavailable is controlled warning if ngspice absent
[OK/FAIL] Selected Region screen/panel opens
[OK/FAIL] Region preview/analyze works for RC demo or limited status shown
[OK/FAIL] Export Center opens
[OK/FAIL] Markdown export works
[OK/FAIL] SPICE netlist export works
[OK/FAIL] BOM export works
[OK/FAIL] SVG schematic export works
[OK/FAIL] Diagnostics opens
[OK/FAIL] Run readiness self-check works
```

Если ручной UI smoke не выполнен, указать:

```text
Manual UI smoke test: NOT RUN
Reason: ...
```

Но для v2.0 желательно выполнить хотя бы локальный EXE smoke.

---

## 17. Full verification commands

Обязательно выполнить:

```bash
cd "D:\Документы\vscode\HotSAS Studio\engine"
cargo fmt --check
cargo test
```

```bash
cd "D:\Документы\vscode\HotSAS Studio\apps\desktop-tauri"
npm.cmd run format:check
npm.cmd run typecheck
npm.cmd run test
npm.cmd run build
npm.cmd run tauri:build
```

Дополнительно, если есть focused tests:

```bash
cd "D:\Документы\vscode\HotSAS Studio\engine"
cargo test product_workflow
cargo test app_diagnostics
cargo test export_center
cargo test ngspice
cargo test spice
cargo test touchstone
```

```bash
cd "D:\Документы\vscode\HotSAS Studio\apps\desktop-tauri"
npm.cmd test -- ProductBeta
npm.cmd test -- Diagnostics
```

---

## 18. Git commit и push

Перед commit:

```bash
cd "D:\Документы\vscode\HotSAS Studio"
git status --short
git diff --stat
git diff --name-only
```

Проверить, что в commit не попадут:

```text
target/
src-tauri/target/
node_modules/
dist/
build/
*.exe
*.zip
*.msi
*.log
личные ТЗ-файлы
локальные отчёты
фото
```

Коммиты:

```bash
git add docs/testing/latest_verification_log.md docs/testing/TESTING.md docs/testing/verification_logs/v1.10_internal_alpha_build_and_v2_readiness_gate.md
git commit -m "docs(v1.10-fix): update alpha build verification metadata"
```

Затем основной v2.0 commit:

```bash
git add README.md docs engine apps
git commit -m "v2.0: Product Beta integration and internal RC build"
```

Если всё сделано одним commit — объяснить в verification log.

Push:

```bash
git log --oneline -5
git push origin main
```

После push:

```bash
git status --short
git log --oneline -5
```

---

## 19. Агент обязан создать отдельный файл логов

Обязательное требование:

```text
Агент обязан создать отдельный файл с логами прохождения всех проверок/тестов:
docs/testing/verification_logs/v2.0_product_beta_integration.md
```

Файл должен содержать:

```text
- дату;
- branch;
- implementation commit;
- verification log update commit, если отдельный;
- git status before;
- git status after;
- список untracked файлов, которые не трогались;
- все команды проверок;
- полный PASS/FAIL summary;
- EXE path/size/SHA256;
- ZIP path/size/SHA256;
- public release/tag status;
- manual smoke check;
- second PC smoke status;
- self-check агента по пунктам ТЗ.
```

После выполнения агент должен предоставить этот файл пользователю для проверки здесь.

---

## 20. Acceptance criteria

`v2.0 — Product Beta` считается завершённой только если:

```text
1. v1.10 verification metadata исправлена.
2. latest_verification_log.md содержит актуальную v1.10 и v2.0 информацию.
3. TESTING.md содержит Manual v1.10 smoke и v2.0 testing section.
4. Product workflow/readiness модели добавлены или корректно расширены существующие diagnostics-модели.
5. Application service для workflow/readiness добавлен или AppDiagnosticsService расширен.
6. API facade methods добавлены.
7. Tauri commands добавлены.
8. Frontend API/types/store добавлены.
9. Start / Project Hub или Product Beta screen показывает guided workflow.
10. Diagnostics показывает v2.0 readiness status.
11. Create integrated demo project работает.
12. Product beta self-check работает.
13. UI не содержит бизнес-логики.
14. React не парсит SPICE/Touchstone.
15. React не запускает ngspice.
16. React не считает формулы/E-series.
17. Rust tests добавлены.
18. Frontend tests добавлены.
19. docs/product/PRODUCT_BETA_V2_0.md создан.
20. docs/user_manual/V2_0_PRODUCT_BETA_QUICK_START.md создан.
21. docs/builds/INTERNAL_ALPHA_BUILD.md обновлён.
22. README обновлён на Current roadmap stage: v2.1 next.
23. v2.0 добавлена в Completed.
24. docs/testing/verification_logs/v2.0_product_beta_integration.md создан.
25. cargo fmt --check PASS.
26. cargo test PASS.
27. npm run format:check PASS.
28. npm run typecheck PASS.
29. npm run test PASS.
30. npm run build PASS.
31. npm run tauri:build PASS.
32. EXE path/size/SHA256 записаны.
33. ZIP path/size/SHA256 записаны.
34. EXE/ZIP не закоммичены.
35. Public GitHub Release не создан.
36. Public release tag не создан.
37. Git commit создан.
38. Git push в origin/main выполнен.
39. Второй ПК smoke либо выполнен, либо честно NOT RUN с причиной.
40. Scope не расширен в v2.1/v2.2/v2.3 задачи.
```

---

## 21. Что НЕ считать ошибкой

```text
- Если ngspice не установлен: это не failure, если статус controlled Unavailable/Limited.
- Если second PC недоступен: это не failure, если в verification log указано NOT RUN + reason.
- Если raw GitHub/парсер показывает Rust/TS/Markdown как 1–4 строки: не считать это проблемой само по себе.
  Опираться на cargo fmt --check, npm run format:check и локальный вид файлов.
```

---

## 22. Итоговый отчёт агента

После выполнения агент должен ответить кратко:

```text
v2.0 — Product Beta Integration выполнена.

Commits:
- <hash> docs(v1.10-fix): ...
- <hash> v2.0: ...

Checks:
- cargo fmt --check — PASS
- cargo test — PASS (<exact test count>)
- npm run format:check — PASS
- npm run typecheck — PASS
- npm run test — PASS (<exact test count>)
- npm run build — PASS
- npm run tauri:build — PASS

Internal build:
- EXE: <path>, <size>, <SHA256>
- ZIP: <path>, <size>, <SHA256>
- Public GitHub Release: NO
- Public tag: NO

Docs:
- PRODUCT_BETA_V2_0.md
- V2_0_PRODUCT_BETA_QUICK_START.md
- v2.0 verification log

Ready for:
v2.1 — Formula Library Expansion
```

