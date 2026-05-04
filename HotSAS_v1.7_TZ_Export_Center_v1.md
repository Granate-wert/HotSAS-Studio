# ТЗ: HotSAS Studio v1.7 — Export Center v1

## 0. Назначение документа

Этот документ — подробное техническое задание для Codex/агента на следующий этап разработки HotSAS Studio.

Этап:

```text
v1.7 — Export Center v1
```

Главная цель этапа — сделать экспорт в программе отдельным пользовательским и backend-driven сценарием, а не набором разрозненных placeholder-кнопок.

После v1.7 пользователь должен открыть экран Export Center и из одного места экспортировать:

```text
- Markdown report;
- HTML report;
- SPICE netlist;
- CSV simulation data;
- BOM CSV;
- BOM JSON;
- Component library JSON;
- SVG schematic;
- Altium workflow package placeholder.
```

Важно: v1.7 не должен превращаться в полноценный PDF/EDA/PCB этап. Это foundation-версия центра экспорта.

---

## 1. Текущий подтверждённый контекст

Проект уже прошёл:

```text
- v1.0 — RC low-pass vertical slice;
- v1.0.1 — Architecture Hardening;
- v1.1.1 — Formatting + Build/Test Infrastructure;
- v1.1.2 — Backend Test Expansion;
- v1.1.3 — FormulaPackLoader + FormulaRegistry;
- v1.1.4-fix — Generic Formula Engine Completion;
- v1.1.5 — Exact E-Series Tables;
- v1.2 — Project Package Storage .circuit;
- v1.3 — Schematic Editor Foundations;
- v1.4 — Engineering Notebook / Calculator Foundations;
- v1.4-fix — Engineering Notebook Integration, Documentation, Verification;
- v1.5 — Component Library Foundation;
- v1.5-fix — Component Library Completion, Verification, Documentation;
- v1.6 — Selected Region Analysis Foundation;
- v1.6-fix — Selected Region Verification, Formatting, Testing Docs & Git Push.
```

Текущий ожидаемый статус перед стартом v1.7:

```text
branch: main
latest accepted stage: v1.6-fix
last known commit: 946b387 — v1.6-fix — Selected Region Verification, Formatting, Testing Docs & Git Push
origin/main: synchronized
Rust tests: 125+ PASS
Frontend tests: 41 PASS
Current roadmap stage in README: v1.7 next
```

Перед началом работы агент обязан проверить реальное состояние Git локально и не полагаться только на этот документ.

---

## 2. Что изменится для пользователя после v1.7

После выполнения v1.7 пользователь должен увидеть в программе новый или полноценно рабочий раздел **Export Center**.

### 2.1. Новые пользовательские возможности

Пользователь сможет:

```text
1. Открыть Export Center из UI.
2. Увидеть группы доступного экспорта:
   - Reports;
   - Netlists;
   - Simulation data;
   - BOM;
   - Component library;
   - Schematic image;
   - EDA workflow package.
3. Выбрать формат экспорта.
4. Запустить экспорт через backend.
5. Получить ExportResult с:
   - статусом;
   - именем файла;
   - относительным/абсолютным путём;
   - MIME/type hint;
   - размером файла, если доступно;
   - warnings/errors.
6. Увидеть список последних export artifacts текущей сессии.
7. Для .circuit project package — сохранять exports в папку проекта `exports/`.
8. Для демо/временного режима — сохранять export artifacts в выбранную/временную папку, не ломая старый JSON save.
```

### 2.2. Практический статус программы после v1.7

До v1.7 экспорт был частично распределён по отдельным сценариям: markdown/html report, netlist, package storage, mock simulation. После v1.7 появляется понятный пользовательский центр:

```text
Создал/открыл проект → рассчитал/проанализировал → открыл Export Center → выгрузил нужные артефакты.
```

Это делает HotSAS Studio ближе к инженерному инструменту, где результаты можно передать дальше:

```text
- в отчёт;
- в таблицу BOM;
- в SPICE workflow;
- в EDA workflow preparation;
- в документацию проекта.
```

### 2.3. Что всё ещё НЕ появится

Пользователь ещё не получит:

```text
- настоящий PDF renderer;
- real KiCad symbol/footprint export;
- real Altium proprietary files;
- Gerber/Excellon/Pick-and-place;
- PCB editor;
- real ngspice simulation;
- полноценный графический SVG renderer схемы уровня EDA.
```

v1.7 — это Export Center foundation, а не финальный EDA exporter.

---

## 3. Главная цель v1.7

Реализовать backend-driven Export Center:

```text
React UI
→ Tauri commands
→ hotsas_api facade
→ hotsas_application ExportCenterService / ExportService
→ hotsas_ports exporters
→ hotsas_adapters concrete exporters
→ files/artifacts
```

Ключевой принцип:

```text
UI не генерирует файлы экспорта.
UI не формирует BOM.
UI не генерирует SPICE netlist.
UI не собирает report body.
UI не сериализует component library.
UI не создаёт Altium workflow package.

UI только отправляет request DTO и показывает result DTO.
```

---

## 4. Жёсткие ограничения scope

### 4.1. Запрещено

```text
- не делать PCB editor;
- не делать routing;
- не делать Gerber;
- не делать Excellon drill;
- не делать pick-and-place;
- не делать IPC-2581;
- не делать real proprietary Altium files;
- не делать real Altium DBLib/SchLib/PcbLib;
- не делать полноценный KiCad .kicad_sym / .kicad_mod export;
- не делать real PDF renderer, если для этого нужна крупная зависимость;
- не добавлять ngspice;
- не добавлять SPICE/Touchstone import;
- не делать selected region pro/symbolic analysis;
- не делать full symbolic solver;
- не добавлять SQLite, если его ещё нет в runtime;
- не ломать .circuit package storage;
- не ломать Component Library;
- не ломать Selected Region Analysis;
- не переносить export logic во frontend;
- не хранить source of truth схемы как React Flow nodes/edges.
```

### 4.2. Разрешено

```text
- добавить core-модели export center;
- добавить application service для export center;
- расширить ports exporter contracts;
- добавить/уточнить adapters:
  - MarkdownReportExporter;
  - HtmlReportExporter;
  - SpiceNetlistExporter;
  - CsvSimulationExporter;
  - BomCsvExporter;
  - BomJsonExporter;
  - ComponentLibraryJsonExporter;
  - SvgSchematicExporter v1 placeholder;
  - AltiumWorkflowPackageExporter placeholder;
- добавить API DTO;
- добавить Tauri commands;
- добавить frontend Export Center screen;
- добавить frontend tests;
- добавить Rust tests;
- обновить docs/export;
- обновить docs/testing;
- создать отдельный verification log;
- сделать commit и push.
```

---

## 5. Preflight перед изменениями

Выполнить из корня проекта:

```bash
cd "D:\Документы\vscode\HotSAS Studio"

git rev-parse --show-toplevel
git branch --show-current
git status --short
git log --oneline -10
git remote -v
git diff --stat
git diff --name-only
```

Правила:

```text
- Не выполнять git reset.
- Не выполнять git clean.
- Не удалять пользовательские untracked-файлы.
- Если есть незакоммиченные изменения до начала работы — записать их в verification log.
- Если есть неожиданные изменения в target/node_modules/dist/build — не коммитить их, проверить .gitignore.
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

Если baseline падает — сначала исправить минимально, не начиная новую функциональность.

---

## 6. Preflight-fix перед v1.7

Перед добавлением export center проверить:

```text
README.md
docs/testing/latest_verification_log.md
docs/testing/verification_logs/v1.6_selected_region_analysis_foundation.md
docs/testing/TESTING.md
```

Должно быть:

```text
README.md:
- Current roadmap stage: v1.7 next
- Completed содержит v1.6

latest_verification_log.md:
- указывает на v1.6
- содержит ссылку на v1.6 verification log
- показывает готовность к v1.7

TESTING.md:
- summary: 125+ Rust tests, 41 frontend tests
- есть раздел v1.6
```

Если где-то остался v1.5 или отсутствует v1.6 — исправить в рамках preflight-fix и отразить в v1.7 verification log.

---

## 7. Core: export domain models

Создать или расширить модуль:

```text
engine/core/src/export.rs
```

Подключить в:

```text
engine/core/src/lib.rs
```

Если в проекте уже есть report/export модели, не дублировать несовместимые сущности. Добавлять extension-модели аккуратно.

### 7.1. ExportFormat

Добавить enum:

```rust
pub enum ExportFormat {
    MarkdownReport,
    HtmlReport,
    SpiceNetlist,
    CsvSimulationData,
    BomCsv,
    BomJson,
    ComponentLibraryJson,
    SvgSchematic,
    AltiumWorkflowPackage,
}
```

Если удобнее сериализовать строками, API DTO может использовать `String`, но domain должен иметь typed enum или validated string.

### 7.2. ExportCategory

```rust
pub enum ExportCategory {
    Report,
    Netlist,
    SimulationData,
    Bom,
    ComponentLibrary,
    Schematic,
    EdaWorkflow,
}
```

### 7.3. ExportTarget

```rust
pub enum ExportTarget {
    CurrentProject,
    CurrentSchematic,
    CurrentSimulationResult,
    BuiltInComponentLibrary,
    SelectedRegion,
}
```

Для v1.7 можно поддержать `CurrentProject` как основной target, а остальные — через optional fields / placeholder validation.

### 7.4. ExportRequest

```rust
pub struct ExportRequest {
    pub format: ExportFormat,
    pub target: ExportTarget,
    pub output_dir: Option<String>,
    pub file_name: Option<String>,
    pub include_project_info: bool,
    pub include_schematic: bool,
    pub include_components: bool,
    pub include_formulas: bool,
    pub include_simulation: bool,
    pub include_selected_region: bool,
    pub include_bom: bool,
    pub options: BTreeMap<String, String>,
}
```

### 7.5. ExportArtifact

```rust
pub struct ExportArtifact {
    pub id: String,
    pub format: ExportFormat,
    pub file_name: String,
    pub relative_path: String,
    pub absolute_path: Option<String>,
    pub mime_type: Option<String>,
    pub size_bytes: Option<u64>,
    pub created_at: Option<String>,
    pub warnings: Vec<String>,
}
```

### 7.6. ExportResult

```rust
pub struct ExportResult {
    pub success: bool,
    pub artifacts: Vec<ExportArtifact>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}
```

### 7.7. ExportCapability

```rust
pub struct ExportCapability {
    pub format: ExportFormat,
    pub category: ExportCategory,
    pub title: String,
    pub description: String,
    pub file_extensions: Vec<String>,
    pub available: bool,
    pub requires_project: bool,
    pub requires_simulation_result: bool,
    pub placeholder: bool,
}
```

`AltiumWorkflowPackage` должен иметь `placeholder: true`, но быть рабочим как папка с README/import hints.

---

## 8. Ports: export contracts

Проверить существующий `ExporterPort` в:

```text
engine/ports/src/lib.rs
```

Если он слишком общий или привязан только к report, добавить новый port без ломки старого:

```rust
pub trait ExportCenterPort: Send + Sync {
    fn list_export_capabilities(&self) -> Result<Vec<ExportCapability>, PortError>;

    fn export_project(
        &self,
        project: &CircuitProject,
        request: &ExportRequest,
    ) -> Result<ExportResult, PortError>;
}
```

Либо использовать несколько специализированных ports:

```rust
pub trait BomExporterPort: Send + Sync { ... }
pub trait SimulationCsvExporterPort: Send + Sync { ... }
pub trait SchematicSvgExporterPort: Send + Sync { ... }
pub trait ComponentLibraryExporterPort: Send + Sync { ... }
pub trait EdaWorkflowExporterPort: Send + Sync { ... }
```

Рекомендация для v1.7:

```text
Не усложнять ports чрезмерно.
Лучше сделать ExportCenterService, который использует существующие exporters/adapters и возвращает единый ExportResult.
```

---

## 9. Adapters: конкретные exporters v1.7

Добавить/расширить:

```text
engine/adapters/src/export_center.rs
```

или отдельные modules, если структура уже разнесена.

### 9.1. MarkdownReportExporter

Если уже есть — подключить к Export Center.

Требования:

```text
- генерирует `.md` файл;
- включает project info;
- включает component list;
- включает formulas/calculations, если доступны;
- включает SPICE netlist fragment или full netlist, если доступен;
- включает selected region summary, если include_selected_region = true и данные есть;
- не panic при отсутствии optional sections;
- возвращает warning, если section requested but data missing.
```

### 9.2. HtmlReportExporter

Требования:

```text
- генерирует `.html` файл;
- использует безопасный HTML escaping;
- не должен пропускать `<script>` как активный HTML;
- может быть простым HTML wrapper без полноценного CSS;
- должен иметь title и body;
- должен включать те же основные sections, что Markdown report.
```

Обязательный тест:

```text
Input section: <script>alert(1)</script>
HTML output НЕ содержит активный <script>alert(1)</script>
HTML output содержит &lt;script&gt;alert(1)&lt;/script&gt;
```

### 9.3. SPICE Netlist Export

Если `SpiceNetlistExporter` уже существует — Export Center должен вызывать его, а не дублировать генерацию.

Требования:

```text
- экспорт `.cir` или `.spice`;
- для RC demo содержит V1/R1/C1/gnd/.ac/.end;
- для unsupported project возвращает controlled error/warning, а не panic;
- output filename по умолчанию: project_name.cir или netlist.cir.
```

### 9.4. CSV Simulation Data Export

Добавить exporter:

```text
CsvSimulationExporter
```

Требования:

```text
- генерирует `.csv`;
- использует mock simulation result / current simulation result, если он есть;
- колонки минимум:
  - series_name;
  - x;
  - x_unit;
  - y;
  - y_unit;
- если graph series пустые — controlled warning/error;
- не генерировать fake data внутри UI;
- не запускать simulation внутри exporter без явного request.
```

Если текущий проект хранит simulation result только transient/session state, ExportCenterService может взять последний result из backend state. Если last result отсутствует — вернуть ошибку:

```text
No simulation result available for CSV export
```

### 9.5. BOM CSV Export

Добавить exporter:

```text
BomCsvExporter
```

Требования к колонкам:

```text
Designator,ComponentName,Category,Value,Unit,Quantity,Manufacturer,PartNumber,Footprint,Datasheet,Notes
```

Для текущего уровня:

```text
- grouped BOM допустим;
- ungrouped BOM допустим;
- если manufacturer/part_number нет — пустая строка;
- если component definition назначен через Component Library — использовать данные definition;
- если нет library assignment — использовать instance/component kind;
- не падать на missing optional fields.
```

### 9.6. BOM JSON Export

Добавить exporter:

```text
BomJsonExporter
```

Пример структуры:

```json
{
  "project_id": "...",
  "project_name": "...",
  "items": [
    {
      "designators": ["R1"],
      "quantity": 1,
      "component_name": "Generic Resistor",
      "category": "resistor",
      "value": "10k",
      "unit": "Ohm",
      "manufacturer": null,
      "part_number": null,
      "footprint": "axial_resistor_placeholder",
      "datasheet": null,
      "notes": null
    }
  ]
}
```

### 9.7. Component Library JSON Export

Добавить exporter:

```text
ComponentLibraryJsonExporter
```

Требования:

```text
- экспорт built-in component library или project-linked library;
- формат `.json`;
- включает components, symbols, footprints, simulation_models, metadata;
- сохраняет совместимость с v1.5 Component Library domain;
- не пытается грузить компоненты из интернета.
```

### 9.8. SVG Schematic Export v1

Добавить exporter:

```text
SvgSchematicExporter
```

v1 допустимо сделать простым, структурным, не EDA-идеальным.

Требования:

```text
- генерирует валидный `.svg`;
- отображает базовые компоненты как простые блоки/лейблы/линии;
- не использует React Flow DOM snapshot;
- строит SVG из backend CircuitModel / DTO;
- содержит component labels: R1, C1, V1, GND;
- содержит nets/wires, если модель даёт координаты/соединения;
- если координат недостаточно — применяет deterministic layout placeholder и warning.
```

Запрещено:

```text
- делать screenshot UI;
- читать DOM;
- использовать frontend как источник схемы.
```

### 9.9. Altium Workflow Package Placeholder

Добавить exporter:

```text
AltiumWorkflowPackageExporter
```

Это не proprietary Altium export. Это подготовленная папка:

```text
altium_workflow/
├── bom.csv
├── components.json
├── symbols/
├── footprints/
├── spice_models/
├── datasheets/
└── README_IMPORT.md
```

Минимальные требования:

```text
- создать папку altium_workflow/;
- положить bom.csv;
- положить components.json;
- создать пустые папки symbols/, footprints/, spice_models/, datasheets/;
- создать README_IMPORT.md;
- README_IMPORT.md должен честно объяснять:
  - это не Altium SchLib/PcbLib;
  - это workflow package;
  - proprietary Altium files не генерируются;
  - будущий путь: internal model → KiCad-compatible export / DB workflow → Altium Import Wizard.
```

---

## 10. Application layer

Создать сервис:

```text
engine/application/src/services/export_center.rs
```

Подключить в:

```text
engine/application/src/services/mod.rs
engine/application/src/services/app_services.rs
```

Если структура services другая — следовать существующему паттерну.

### 10.1. ExportCenterService

Минимальные методы:

```rust
pub fn list_export_capabilities(&self) -> Vec<ExportCapability>;

pub fn export_current_project(
    &self,
    request: ExportRequest,
) -> Result<ExportResult, ApplicationError>;

pub fn preview_export_plan(
    &self,
    request: ExportRequest,
) -> Result<ExportPlan, ApplicationError>;
```

### 10.2. ExportPlan

```rust
pub struct ExportPlan {
    pub format: ExportFormat,
    pub target: ExportTarget,
    pub expected_artifacts: Vec<String>,
    pub warnings: Vec<String>,
    pub blocking_errors: Vec<String>,
}
```

`preview_export_plan` нужен, чтобы UI мог показать, что будет создано, до фактического экспорта.

### 10.3. Поведение ExportCenterService

Сервис должен:

```text
- проверять наличие current_project;
- выбирать exporter по ExportFormat;
- формировать safe file name;
- определять output directory;
- для .circuit package использовать exports/;
- для временного проекта использовать временную/заданную папку;
- возвращать ExportResult;
- сохранять список последних artifacts в backend state, если в архитектуре есть session state;
- не panic;
- возвращать controlled ApplicationError/ApiError.
```

---

## 11. API DTO

В:

```text
engine/api/src/dto.rs
```

добавить DTO:

```rust
pub struct ExportCapabilityDto {
    pub format: String,
    pub category: String,
    pub title: String,
    pub description: String,
    pub file_extensions: Vec<String>,
    pub available: bool,
    pub requires_project: bool,
    pub requires_simulation_result: bool,
    pub placeholder: bool,
}

pub struct ExportRequestDto {
    pub format: String,
    pub target: Option<String>,
    pub output_dir: Option<String>,
    pub file_name: Option<String>,
    pub include_project_info: bool,
    pub include_schematic: bool,
    pub include_components: bool,
    pub include_formulas: bool,
    pub include_simulation: bool,
    pub include_selected_region: bool,
    pub include_bom: bool,
    pub options: BTreeMap<String, String>,
}

pub struct ExportArtifactDto {
    pub id: String,
    pub format: String,
    pub file_name: String,
    pub relative_path: String,
    pub absolute_path: Option<String>,
    pub mime_type: Option<String>,
    pub size_bytes: Option<u64>,
    pub created_at: Option<String>,
    pub warnings: Vec<String>,
}

pub struct ExportResultDto {
    pub success: bool,
    pub artifacts: Vec<ExportArtifactDto>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

pub struct ExportPlanDto {
    pub format: String,
    pub target: String,
    pub expected_artifacts: Vec<String>,
    pub warnings: Vec<String>,
    pub blocking_errors: Vec<String>,
}
```

Добавить conversion helpers:

```text
ExportCapability -> ExportCapabilityDto
ExportRequestDto -> ExportRequest
ExportArtifact -> ExportArtifactDto
ExportResult -> ExportResultDto
ExportPlan -> ExportPlanDto
```

---

## 12. HotSasApi facade

В:

```text
engine/api/src/facade.rs
```

добавить методы:

```rust
pub fn list_export_capabilities(&self) -> Result<Vec<ExportCapabilityDto>, ApiError>;

pub fn preview_export_plan(
    &self,
    request: ExportRequestDto,
) -> Result<ExportPlanDto, ApiError>;

pub fn export_current_project(
    &mut self,
    request: ExportRequestDto,
) -> Result<ExportResultDto, ApiError>;

pub fn get_recent_exports(&self) -> Result<Vec<ExportArtifactDto>, ApiError>;
```

Если backend state immutable/mutable pattern отличается, использовать существующий стиль проекта.

---

## 13. Tauri commands

В:

```text
apps/desktop-tauri/src-tauri/src/lib.rs
```

добавить команды:

```rust
#[tauri::command]
async fn list_export_capabilities(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<ExportCapabilityDto>, String>;

#[tauri::command]
async fn preview_export_plan(
    state: tauri::State<'_, AppState>,
    request: ExportRequestDto,
) -> Result<ExportPlanDto, String>;

#[tauri::command]
async fn export_current_project(
    state: tauri::State<'_, AppState>,
    request: ExportRequestDto,
) -> Result<ExportResultDto, String>;

#[tauri::command]
async fn get_recent_exports(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<ExportArtifactDto>, String>;
```

Добавить в `generate_handler!`.

Проверить Tauri permissions/capabilities:

```text
apps/desktop-tauri/src-tauri/capabilities/default.json
```

Если используется allowlist — добавить команды export center.

---

## 14. Frontend types

В:

```text
apps/desktop-tauri/src/types/index.ts
```

добавить:

```ts
export type ExportFormat =
  | "markdown_report"
  | "html_report"
  | "spice_netlist"
  | "csv_simulation_data"
  | "bom_csv"
  | "bom_json"
  | "component_library_json"
  | "svg_schematic"
  | "altium_workflow_package";

export type ExportCategory =
  | "report"
  | "netlist"
  | "simulation_data"
  | "bom"
  | "component_library"
  | "schematic"
  | "eda_workflow";

export interface ExportCapabilityDto { ... }
export interface ExportRequestDto { ... }
export interface ExportArtifactDto { ... }
export interface ExportResultDto { ... }
export interface ExportPlanDto { ... }
```

Названия строк должны совпадать с Rust DTO mappings.

---

## 15. Frontend API

В:

```text
apps/desktop-tauri/src/api/index.ts
```

добавить методы:

```ts
export async function listExportCapabilities(): Promise<ExportCapabilityDto[]>;
export async function previewExportPlan(request: ExportRequestDto): Promise<ExportPlanDto>;
export async function exportCurrentProject(request: ExportRequestDto): Promise<ExportResultDto>;
export async function getRecentExports(): Promise<ExportArtifactDto[]>;
```

Использовать `invoke`, как в существующих API methods.

---

## 16. Frontend store

В:

```text
apps/desktop-tauri/src/store/index.ts
```

или текущем store-файле добавить state:

```ts
exportCapabilities: ExportCapabilityDto[];
selectedExportFormat: ExportFormat | null;
exportPlan: ExportPlanDto | null;
lastExportResult: ExportResultDto | null;
recentExports: ExportArtifactDto[];
exportLoading: boolean;
exportError: string | null;
```

Actions/setters:

```ts
setExportCapabilities(...)
setSelectedExportFormat(...)
setExportPlan(...)
setLastExportResult(...)
setRecentExports(...)
setExportLoading(...)
setExportError(...)
clearExportState()
```

Не хранить generated file contents во frontend store, только DTO metadata.

---

## 17. UI: Export Center Screen

Создать:

```text
apps/desktop-tauri/src/screens/ExportCenterScreen.tsx
```

Если экран уже есть placeholder — заменить на рабочий foundation.

### 17.1. Layout

Минимальная структура:

```text
Export Center
├── Header / explanation
├── Capability list grouped by category
│   ├── Reports
│   ├── Netlists
│   ├── Simulation Data
│   ├── BOM
│   ├── Component Library
│   ├── Schematic
│   └── EDA Workflow
├── Export options panel
│   ├── file name
│   ├── output dir optional
│   ├── include checkboxes
│   └── selected format details
├── Export plan preview
├── Run Export button
├── Export result card
└── Recent exports list
```

### 17.2. Required UI behavior

```text
- On screen mount: load list_export_capabilities.
- Click capability: set selected format.
- Preview button: call preview_export_plan.
- Export button: call export_current_project.
- Show loading state.
- Show errors/warnings.
- Show artifacts after export.
- Show placeholder label for Altium workflow package.
```

### 17.3. UX text

For Altium workflow package, show clearly:

```text
This creates a workflow folder for future Altium/KiCad handoff. It does not create proprietary Altium library files.
```

Russian UI text is acceptable if project UI uses Russian; otherwise follow current UI language style.

---

## 18. UI components

Create folder:

```text
apps/desktop-tauri/src/components/export-center/
```

Components:

```text
ExportCapabilityList.tsx
ExportOptionsPanel.tsx
ExportPlanCard.tsx
ExportResultCard.tsx
RecentExportsList.tsx
```

Tests:

```text
apps/desktop-tauri/src/components/export-center/__tests__/ExportCenterScreen.test.tsx
apps/desktop-tauri/src/components/export-center/__tests__/ExportResultCard.test.tsx
```

If project test style prefers fewer files, at minimum add one focused screen test.

---

## 19. Navigation integration

Add Export Center to app navigation if not present:

```text
Start Page / sidebar / top nav:
- Export Center
```

Do not break existing screens:

```text
- Schematic;
- Formula Library;
- Engineering Notebook;
- Component Library;
- Selected Region tab.
```

---

## 20. File output strategy

### 20.1. Default output directory

If current project is saved as `.circuit` package:

```text
<project>.circuit/exports/
```

If project is not saved as package:

```text
project-local temp/export directory
```

or return controlled error requiring output_dir:

```text
Output directory is required for unsaved projects
```

Choose the approach that best matches current storage implementation, but document it.

### 20.2. Safe filenames

Implement safe filename helper:

```text
- trim whitespace;
- replace path separators;
- avoid `..` traversal;
- ensure extension matches format;
- default names:
  - report.md
  - report.html
  - netlist.cir
  - simulation.csv
  - bom.csv
  - bom.json
  - component_library.json
  - schematic.svg
  - altium_workflow/
```

### 20.3. No path traversal

Mandatory tests:

```text
file_name = "../../evil.txt" must not write outside output_dir
output_dir traversal must be rejected or normalized safely
```

---

## 21. .circuit package integration

If current project package has:

```text
exports/
reports/
results/
```

then v1.7 should:

```text
- write exports to exports/ by default;
- optionally write reports to reports/ if existing package structure expects it;
- not break v1.2 project package validation;
- update export indexes only if such index model exists;
- if no index exists, do not invent a large migration in v1.7; document future index update.
```

If project package validation expects specific directories, ensure Export Center-created files do not invalidate package structure.

---

## 22. BOM generation rules

### 22.1. Source of data

BOM should be built from backend current project state:

```text
CircuitProject
→ schematic components / component instances
→ linked ComponentDefinition if available
→ overridden parameters
→ selected footprint/simulation model metadata
```

### 22.2. Grouping

For v1.7 support simple grouping:

```text
group key = component_definition_id + value + footprint + part_number
```

If grouping is too much for current model, ungrouped BOM is acceptable, but document it.

### 22.3. Required BOM fields

```text
Designator
ComponentName
Category
Value
Unit
Quantity
Manufacturer
PartNumber
Footprint
Datasheet
Notes
```

---

## 23. Report content rules

Markdown/HTML report exported from Export Center should include, when available:

```text
- Project info;
- Schematic summary;
- Component list;
- Formula calculations;
- E-series selection;
- Component library assignments;
- SPICE netlist;
- Mock simulation result summary;
- Selected region analysis summary;
- Warnings;
- BOM summary.
```

If some sections are not available, do not fail the whole export unless requested as mandatory. Add warning instead:

```text
Selected region analysis was requested but no selected region result is available.
```

---

## 24. SVG schematic v1 details

SVG export does not need to be beautiful, but must be useful.

Minimum:

```text
<svg ...>
  <title>Project schematic</title>
  components with labels
  wires/lines if available
  nets labels if available
</svg>
```

Required properties:

```text
- deterministic output for same project;
- valid XML-ish SVG string;
- component labels visible;
- no React/DOM dependency;
- no screenshot.
```

---

## 25. Altium workflow README_IMPORT.md content

The generated README must include:

```markdown
# Altium Workflow Package

This folder is generated by HotSAS Studio Export Center.

It is NOT a proprietary Altium library export.

Included:
- bom.csv
- components.json
- symbols/ placeholder directory
- footprints/ placeholder directory
- spice_models/ placeholder directory
- datasheets/ placeholder directory

Recommended future workflow:
1. Use BOM and component JSON as reference data.
2. Use future KiCad-compatible symbol/footprint export when available.
3. Import through Altium supported workflows manually or through a future DBLib workflow.

Limitations:
- No .SchLib generated.
- No .PcbLib generated.
- No PCB layout generated.
- No Gerber/production data generated.
```

---

## 26. Tests: Rust

Add tests under appropriate crates, for example:

```text
engine/application/tests/export_center_tests.rs
engine/adapters/tests/export_adapters_tests.rs
engine/api/tests/export_center_api_tests.rs
```

Required tests:

### 26.1. Capabilities

```text
list_export_capabilities_returns_required_formats
```

Must include:

```text
MarkdownReport
HtmlReport
SpiceNetlist
CsvSimulationData
BomCsv
BomJson
ComponentLibraryJson
SvgSchematic
AltiumWorkflowPackage
```

### 26.2. Markdown export

```text
export_markdown_report_creates_artifact
```

Check:

```text
- success = true;
- one artifact .md;
- file exists;
- contains project name;
- contains component list or expected header.
```

### 26.3. HTML escaping

```text
html_report_escapes_script_tags
```

### 26.4. SPICE netlist export

```text
export_spice_netlist_contains_expected_rc_fragments
```

### 26.5. BOM CSV

```text
export_bom_csv_contains_designators_and_headers
```

### 26.6. BOM JSON

```text
export_bom_json_contains_items
```

### 26.7. Component library JSON

```text
export_component_library_json_contains_builtin_components
```

Check at least:

```text
Generic Resistor
Generic Capacitor
Generic Op-Amp
```

### 26.8. SVG schematic

```text
export_svg_schematic_contains_svg_and_component_labels
```

### 26.9. Altium workflow package

```text
export_altium_workflow_package_creates_expected_folder_structure
```

Check:

```text
altium_workflow/bom.csv
altium_workflow/components.json
altium_workflow/symbols/
altium_workflow/footprints/
altium_workflow/spice_models/
altium_workflow/datasheets/
altium_workflow/README_IMPORT.md
```

### 26.10. Path safety

```text
export_rejects_or_sanitizes_path_traversal_file_names
```

### 26.11. No current project

```text
export_without_current_project_returns_controlled_error
```

No panic.

---

## 27. Tests: Frontend

Add tests for Export Center UI.

Required tests:

```text
- renders export capabilities grouped by category;
- selecting a format enables preview/export panel;
- preview button calls backend API and renders export plan;
- export button calls backend API and renders artifacts;
- error state is displayed if backend rejects export;
- Altium workflow capability displays placeholder warning.
```

Use existing test setup/mocks pattern. Do not call real Tauri in tests.

Expected frontend test count after v1.7:

```text
41 existing + new Export Center tests
```

Do not hardcode a final number in code; record actual number in verification log.

---

## 28. Documentation

Create:

```text
docs/export/EXPORT_CENTER_V1.md
```

The document must include:

```text
- purpose of Export Center;
- supported formats in v1.7;
- user-facing workflow;
- backend architecture;
- DTO/API overview;
- file output strategy;
- .circuit exports/ integration;
- Altium workflow package limitations;
- what is not implemented yet;
- future roadmap: real PDF, KiCad export, Altium workflow improvements, ngspice results.
```

Update:

```text
README.md
```

After completion:

```text
Current roadmap stage: v1.8 next
Completed:
- v1.7 — Export Center v1
```

Update:

```text
docs/testing/TESTING.md
```

Add section:

```text
## v1.7 — Export Center v1
```

Include:

```text
- Rust export center tests;
- adapter tests;
- API tests;
- frontend Export Center tests;
- manual smoke check.
```

Update:

```text
docs/testing/latest_verification_log.md
```

It must point to v1.7 after completion.

Create:

```text
docs/testing/verification_logs/v1.7_export_center_v1.md
```

This file is mandatory.

---

## 29. Mandatory verification log file

Agent must create a separate file with logs:

```text
docs/testing/verification_logs/v1.7_export_center_v1.md
```

It must include:

```text
1. Date/time.
2. Branch.
3. Commit before changes.
4. Commit after changes.
5. Git status before changes.
6. Git status after changes.
7. Files changed summary.
8. Full list of commands run.
9. Output summary for each command.
10. Rust tests added.
11. Frontend tests added.
12. Manual UI smoke test table with [OK]/[FAIL].
13. Known limitations.
14. Final readiness verdict.
```

Important:

```text
Do not write only "PASS".
Include enough command output to verify the claim.
```

The agent must provide this file to the user here for independent review.

---

## 30. Required final checks

Run from `engine`:

```bash
cd "D:\Документы\vscode\HotSAS Studio\engine"
cargo fmt --check
cargo test
```

Run from frontend:

```bash
cd "D:\Документы\vscode\HotSAS Studio\apps\desktop-tauri"
npm.cmd run format:check
npm.cmd run typecheck
npm.cmd run test
npm.cmd run build
npm.cmd run tauri:build
```

If `npm.cmd run tauri:build` cannot run due environment, record exact reason. But if it ran in v1.5/v1.6, it should be attempted again.

---

## 31. Manual UI smoke test

Agent must perform or request manual check if GUI automation is unavailable.

Minimum smoke checklist:

```text
[OK/FAIL] App starts
[OK/FAIL] RC demo project can be created
[OK/FAIL] Schematic screen opens
[OK/FAIL] Formula Library still works
[OK/FAIL] Engineering Notebook still works
[OK/FAIL] Component Library still opens
[OK/FAIL] Selected Region tab still opens
[OK/FAIL] Export Center opens
[OK/FAIL] Export capabilities load
[OK/FAIL] Markdown report export works
[OK/FAIL] HTML report export works
[OK/FAIL] SPICE netlist export works
[OK/FAIL] BOM CSV export works
[OK/FAIL] BOM JSON export works
[OK/FAIL] Component library JSON export works
[OK/FAIL] SVG schematic export works
[OK/FAIL] Altium workflow package placeholder creates expected folder
[OK/FAIL] Export result artifacts are shown in UI
[OK/FAIL] Recent exports list updates
```

If some manual checks are not possible, mark `NOT RUN` and explain why. Do not mark as PASS without running.

---

## 32. Agent self-check against this ТЗ

After implementation, agent must add a self-check section to the final report and verification log.

Checklist:

```text
[ ] Preflight git status recorded
[ ] Baseline checks run
[ ] Core export models added/updated
[ ] ExportCenterService added
[ ] Export DTOs added
[ ] HotSasApi facade methods added
[ ] Tauri commands added and registered
[ ] Frontend types added
[ ] Frontend API methods added
[ ] Store state/actions added
[ ] Export Center UI implemented
[ ] Markdown export available from Export Center
[ ] HTML export available and escaped safely
[ ] SPICE netlist export available
[ ] CSV simulation data export available or controlled missing-result error implemented
[ ] BOM CSV export available
[ ] BOM JSON export available
[ ] Component library JSON export available
[ ] SVG schematic export available
[ ] Altium workflow package placeholder available
[ ] Path traversal test added
[ ] No frontend export business logic added
[ ] Rust tests added and passing
[ ] Frontend tests added and passing
[ ] docs/export/EXPORT_CENTER_V1.md created
[ ] README updated to v1.8 next
[ ] TESTING.md updated
[ ] latest_verification_log.md updated to v1.7
[ ] v1.7 verification log created
[ ] Manual smoke check recorded
[ ] Commit created
[ ] Push to origin/main completed
```

---

## 33. Git commit and push

After all checks pass:

```bash
git status --short
git add README.md docs engine apps shared
git commit -m "v1.7: Export Center v1"
git push origin main
```

If verification log is updated after obtaining commit hash, create second commit if needed:

```bash
git add docs/testing/latest_verification_log.md docs/testing/verification_logs/v1.7_export_center_v1.md
git commit -m "v1.7: update verification logs"
git push origin main
```

Expected final state:

```text
branch: main
git status: clean
origin/main synchronized
README: Current roadmap stage: v1.8 next
latest_verification_log: v1.7
v1.7 verification log exists
```

---

## 34. Acceptance criteria

v1.7 is accepted only if:

```text
1. Export Center screen exists and is reachable.
2. Export capabilities are loaded from backend.
3. Export action goes through Tauri/API/application/backend.
4. Markdown report export works.
5. HTML report export works and escapes dangerous HTML.
6. SPICE netlist export works.
7. CSV simulation data export works or returns controlled missing-result error.
8. BOM CSV export works.
9. BOM JSON export works.
10. Component library JSON export works.
11. SVG schematic export works as backend-generated placeholder.
12. Altium workflow package placeholder creates documented folder structure.
13. Export result artifacts are visible in UI.
14. No export business logic is moved into React.
15. Rust tests pass.
16. Frontend tests pass.
17. npm build passes.
18. tauri build is attempted and result recorded.
19. docs/export/EXPORT_CENTER_V1.md exists.
20. docs/testing/verification_logs/v1.7_export_center_v1.md exists.
21. docs/testing/latest_verification_log.md points to v1.7.
22. README marks v1.8 next.
23. Git commit and push completed.
```

If any acceptance criterion fails, do not claim v1.7 is complete. Report what is missing.

---

## 35. Expected final report from agent

Agent final response must include:

```text
1. Short summary of what was implemented.
2. What changed for the user.
3. Files changed.
4. Tests added.
5. Commands run and result.
6. Manual smoke test result.
7. Link/path to verification log file.
8. Git commit hash/hashs.
9. Push status.
10. Readiness verdict:
   - v1.7 accepted / not accepted
   - ready for v1.8 / not ready
```

Important:

```text
Agent must provide the verification log file here for review.
```

