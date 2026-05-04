# ТЗ: HotSAS Studio v1.6 — Selected Region Analysis Foundation

Дата подготовки ТЗ: 2026-05-03  
Целевой исполнитель: Codex / агент разработки  
Репозиторий: `https://github.com/Granate-wert/HotSAS-Studio`  
Ожидаемая ветка: `main`

---

## 0. Краткий смысл этапа

Выполнить **HotSAS Studio v1.6 — Selected Region Analysis Foundation**.

Цель версии — добавить первую рабочую основу анализа выделенного участка схемы:

1. Пользователь может выбрать несколько компонентов на схеме.
2. Приложение строит backend-модель выбранной области.
3. Backend определяет внутренние узлы, boundary nets и возможные точки входа/выхода.
4. Пользователь задаёт:
   - input+;
   - input- или GND;
   - output+;
   - output- или GND;
   - reference node;
   - направление анализа;
   - режим анализа.
5. Backend выполняет controlled foundation-анализ:
   - структурная проверка выбранного участка;
   - поиск совпадения с известным template;
   - генерация SPICE/netlist fragment;
   - mock/numeric summary, если template не распознан;
   - предупреждения, если symbolic result недоступен.
6. UI отображает результат: summary, detected boundary nodes, warnings, netlist fragment, matched template, доступные графики/measurements placeholders.
7. React остаётся только view adapter. Вся логика анализа — в Rust backend/application layer.

---

## 1. Текущий подтверждённый контекст

Проект уже прошёл:

```text
v1.0 — RC low-pass vertical slice
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
v1.5-fix — Component Library Completion, Verification, Documentation
```

Последний подтверждённый статус по отчёту агента:

```text
HEAD = 2e4fa1c
origin/main синхронизирован
v1.5 закрыта
Current roadmap stage в README: v1.6 next
cargo fmt --check: PASS
cargo test: PASS, 120+ tests, 0 failures
npm run format:check: PASS
npm run typecheck: PASS
npm run test: PASS, 36 tests
npm run build: PASS
npm run tauri:build: PASS
```

Перед началом v1.6 агент обязан перепроверить это локально и зафиксировать фактический HEAD, git status и результаты preflight-команд в verification log.

---

## 2. Что изменится для пользователя после v1.6

После успешной реализации v1.6 пользователь должен увидеть новое поведение в программе.

### 2.1. Новые возможности в UI

Пользователь сможет:

```text
1. Открыть RC demo project или существующую схему.
2. Выделить один или несколько компонентов на Schematic Editor canvas.
3. Нажать Analyze Selection / Analyze Region.
4. Увидеть панель Selected Region Analysis.
5. Увидеть список выбранных компонентов.
6. Увидеть найденные boundary nets.
7. Выбрать input/output/reference nodes.
8. Запустить анализ выбранной области.
9. Получить результат анализа:
   - matched template, если участок похож на известную схему;
   - explanation;
   - warnings;
   - SPICE/netlist fragment;
   - доступные calculated outputs или controlled “unsupported symbolic analysis”;
   - graph placeholders / available graph specs.
10. Добавить результат анализа в отчёт или получить controlled placeholder, если report integration в v1.6 ограничена.
```

### 2.2. Пользовательский статус программы после v1.6

До v1.6 программа уже умеет работать с формулами, notebook, component library и схемным foundation.  
После v1.6 программа начинает превращаться из набора отдельных экранов в инженерную среду, где схема становится объектом анализа:

```text
Текущий статус после v1.6:
- schematic editor: foundation;
- formula library: работает;
- engineering notebook: работает;
- component library: foundation работает;
- selected region analysis: foundation работает;
- arbitrary symbolic solver: нет;
- real ngspice: ещё нет;
- PCB editor: нет.
```

### 2.3. Что НЕ нужно обещать пользователю

Нельзя обещать:

```text
- полноценный symbolic solver для любых схем;
- точный H(s) для произвольной области;
- реальную ngspice-симуляцию, если она ещё mock;
- автоматический анализ сложных нелинейных схем;
- PCB/DRC/Gerber;
- KiCad/Altium export.
```

Если выделенный участок не распознан, пользователь должен получить честное сообщение:

```text
Symbolic analysis is not available for this selected region in v1.6.
A structural summary and SPICE fragment are available.
```

---

## 3. Жёсткие ограничения scope

### 3.1. Запрещено

```text
- не добавлять real ngspice;
- не добавлять PCB editor;
- не делать routing;
- не делать Gerber;
- не делать KiCad export;
- не делать Altium export;
- не делать full symbolic solver;
- не добавлять SymPy/Lcapy/math.js bridge;
- не делать Wolfram/MathCAD-like analysis engine;
- не делать DC-DC calculators;
- не добавлять Touchstone parser;
- не добавлять SPICE model importer;
- не переписывать Schematic Editor полностью;
- не ломать Formula Library;
- не ломать Engineering Notebook;
- не ломать Component Library;
- не ломать .circuit package storage;
- не ломать RC vertical slice;
- не переносить анализ выбранного участка во frontend;
- не хранить source of truth выбранной области только в React Flow nodes/edges.
```

### 3.2. Разрешено

```text
- добавить core-модели selected region;
- добавить SelectedRegionAnalysisService;
- добавить template matching для простых известных участков;
- добавить structural analysis;
- добавить SPICE/netlist fragment generation;
- добавить controlled unsupported symbolic result;
- добавить API DTO/facade methods;
- добавить Tauri commands;
- добавить frontend API/types/store;
- добавить Selected Region Analysis UI panel;
- добавить tests;
- добавить документацию;
- создать verification log;
- создать отдельный raw log файл с выводом тестов;
- обновить README/TESTING/latest_verification_log;
- сделать commit и push.
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
- Не коммитить личные файлы пользователя, локальные ТЗ, временные заметки, target, dist, node_modules.
- Если git status не clean — записать это в verification log.
- Если есть неожиданные изменения в проектных файлах — описать их в verification log и действовать минимально безопасно.
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
npm.cmd run tauri:build
```

Перед началом v1.6 желательно вручную проверить v1.5 UI smoke, если доступен GUI:

```bash
cd "D:\Документы\vscode\HotSAS Studio\apps\desktop-tauri"
npm.cmd run tauri:dev
```

Проверить:

```text
[OK/FAIL] приложение открывается
[OK/FAIL] RC demo project создаётся
[OK/FAIL] Schematic Editor открывается
[OK/FAIL] Component Library открывается
[OK/FAIL] built-in library показывает 12 компонентов
[OK/FAIL] поиск resistor работает
[OK/FAIL] details открываются
[OK/FAIL] assign Generic Resistor to R1 работает
[OK/FAIL] Formula Library не сломана
[OK/FAIL] Engineering Notebook не сломан
```

Если GUI smoke test не запускался, честно указать `NOT RUN`.

---

## 5. Архитектурное правило v1.6

Источник истины:

```text
Rust CircuitModel / CircuitDto / backend project state
```

Frontend может хранить только:

```text
- временный UI selection state;
- ids выбранных components;
- selected input/output/reference choices;
- DTO result, полученный от backend.
```

Frontend НЕ должен:

```text
- определять boundary nets самостоятельно;
- строить subcircuit model самостоятельно;
- генерировать SPICE fragment;
- распознавать templates;
- рассчитывать transfer function;
- решать, что region valid/invalid по электрическим правилам.
```

---

## 6. Core: модели selected region

Добавить новый модуль:

```text
engine/core/src/selected_region.rs
```

Подключить:

```text
engine/core/src/lib.rs
```

Все модели должны поддерживать:

```text
Debug
Clone
PartialEq where useful
Serialize
Deserialize
```

### 6.1. `SelectedCircuitRegion`

```rust
pub struct SelectedCircuitRegion {
    pub id: String,
    pub title: String,
    pub component_ids: Vec<String>,
    pub internal_nets: Vec<String>,
    pub boundary_nets: Vec<String>,
    pub input_port: Option<RegionPort>,
    pub output_port: Option<RegionPort>,
    pub reference_node: Option<String>,
    pub analysis_direction: RegionAnalysisDirection,
    pub analysis_mode: RegionAnalysisMode,
    pub metadata: BTreeMap<String, String>,
}
```

### 6.2. `RegionPort`

```rust
pub struct RegionPort {
    pub positive_net: String,
    pub negative_net: Option<String>,
    pub label: Option<String>,
}
```

Назначение:

```text
Используется для input/output пары:
- input+: positive_net
- input-: negative_net или reference/GND
- output+: positive_net
- output-: negative_net или reference/GND
```

### 6.3. `RegionAnalysisDirection`

```rust
pub enum RegionAnalysisDirection {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
    Custom,
}
```

### 6.4. `RegionAnalysisMode`

```rust
pub enum RegionAnalysisMode {
    Structural,
    TemplateBased,
    NumericMock,
    AllAvailable,
}
```

Важно: `Symbolic` как отдельный режим в v1.6 не добавлять, если он будет обещать больше, чем реализовано. Можно добавить controlled value:

```rust
pub enum SymbolicAvailability {
    AvailableForMatchedTemplate,
    UnavailableForArbitraryRegion,
}
```

### 6.5. `SelectedRegionPreview`

```rust
pub struct SelectedRegionPreview {
    pub region: SelectedCircuitRegion,
    pub selected_components: Vec<RegionComponentSummary>,
    pub detected_internal_nets: Vec<RegionNetSummary>,
    pub detected_boundary_nets: Vec<RegionNetSummary>,
    pub suggested_input_nets: Vec<String>,
    pub suggested_output_nets: Vec<String>,
    pub suggested_reference_nodes: Vec<String>,
    pub warnings: Vec<SelectedRegionIssue>,
    pub errors: Vec<SelectedRegionIssue>,
}
```

### 6.6. `RegionComponentSummary`

```rust
pub struct RegionComponentSummary {
    pub instance_id: String,
    pub definition_id: Option<String>,
    pub component_kind: String,
    pub display_label: String,
    pub connected_nets: Vec<String>,
}
```

### 6.7. `RegionNetSummary`

```rust
pub struct RegionNetSummary {
    pub net_id: String,
    pub net_name: String,
    pub connected_selected_components: Vec<String>,
    pub connected_external_components: Vec<String>,
    pub is_ground: bool,
    pub role_hint: Option<String>,
}
```

### 6.8. `SelectedRegionIssue`

```rust
pub struct SelectedRegionIssue {
    pub code: String,
    pub severity: SelectedRegionIssueSeverity,
    pub message: String,
    pub component_id: Option<String>,
    pub net_id: Option<String>,
}
```

```rust
pub enum SelectedRegionIssueSeverity {
    Info,
    Warning,
    Error,
}
```

### 6.9. `SelectedRegionAnalysisRequest`

```rust
pub struct SelectedRegionAnalysisRequest {
    pub component_ids: Vec<String>,
    pub input_port: Option<RegionPort>,
    pub output_port: Option<RegionPort>,
    pub reference_node: Option<String>,
    pub analysis_direction: RegionAnalysisDirection,
    pub analysis_mode: RegionAnalysisMode,
}
```

### 6.10. `SelectedRegionAnalysisResult`

```rust
pub struct SelectedRegionAnalysisResult {
    pub region: SelectedCircuitRegion,
    pub status: SelectedRegionAnalysisStatus,
    pub summary: String,
    pub matched_template: Option<MatchedRegionTemplate>,
    pub equivalent_circuit: Option<EquivalentCircuitSummary>,
    pub transfer_function: Option<RegionTransferFunction>,
    pub measurements: Vec<RegionMeasurement>,
    pub graph_specs: Vec<RegionGraphSpec>,
    pub netlist_fragment: Option<RegionNetlistFragment>,
    pub warnings: Vec<SelectedRegionIssue>,
    pub errors: Vec<SelectedRegionIssue>,
    pub report_section_markdown: Option<String>,
}
```

### 6.11. `SelectedRegionAnalysisStatus`

```rust
pub enum SelectedRegionAnalysisStatus {
    Success,
    Partial,
    Unsupported,
    Error,
}
```

### 6.12. `MatchedRegionTemplate`

```rust
pub struct MatchedRegionTemplate {
    pub template_id: String,
    pub title: String,
    pub confidence: f64,
    pub formula_ids: Vec<String>,
    pub explanation: String,
}
```

### 6.13. `EquivalentCircuitSummary`

```rust
pub struct EquivalentCircuitSummary {
    pub title: String,
    pub description: String,
    pub assumptions: Vec<String>,
    pub limitations: Vec<String>,
}
```

### 6.14. `RegionTransferFunction`

```rust
pub struct RegionTransferFunction {
    pub expression: String,
    pub latex: Option<String>,
    pub output_name: String,
    pub unit: Option<Unit>,
    pub availability_note: Option<String>,
}
```

Важно:

```text
В v1.6 transfer_function можно возвращать только для known template.
Для arbitrary region возвращать None + warning.
```

### 6.15. `RegionMeasurement`

```rust
pub struct RegionMeasurement {
    pub name: String,
    pub value: Option<ValueWithUnit>,
    pub description: String,
    pub source: String,
}
```

### 6.16. `RegionGraphSpec`

```rust
pub struct RegionGraphSpec {
    pub id: String,
    pub title: String,
    pub x_unit: Option<Unit>,
    pub y_unit: Option<Unit>,
    pub description: String,
    pub available: bool,
    pub unavailable_reason: Option<String>,
}
```

### 6.17. `RegionNetlistFragment`

```rust
pub struct RegionNetlistFragment {
    pub title: String,
    pub format: String,
    pub content: String,
    pub warnings: Vec<String>,
}
```

---

## 7. Core helpers

Добавить pure helper-функции в `selected_region.rs` или отдельный модуль:

```rust
pub fn normalize_component_ids(component_ids: &[String]) -> Vec<String>;

pub fn region_has_component(region: &SelectedCircuitRegion, component_id: &str) -> bool;

pub fn is_region_configured(region: &SelectedCircuitRegion) -> bool;

pub fn selected_region_summary(region: &SelectedCircuitRegion) -> String;
```

Требования:

```text
- удалить дубликаты component_ids;
- сохранить стабильный порядок;
- не panic-овать на empty input;
- возвращать controlled errors через service layer.
```

---

## 8. Application: SelectedRegionAnalysisService

Создать:

```text
engine/application/src/services/selected_region_analysis.rs
```

Подключить в:

```text
engine/application/src/services/mod.rs
engine/application/src/services/app_services.rs
engine/application/src/lib.rs
```

Если в проекте другая структура services, использовать существующий паттерн.

### 8.1. Основной service API

```rust
pub struct SelectedRegionAnalysisService {
    // использовать уже существующие services/ports where needed
}
```

Методы:

```rust
pub fn preview_selected_region(
    &self,
    circuit: &CircuitModel,
    component_ids: Vec<String>,
) -> Result<SelectedRegionPreview, ApplicationError>;

pub fn analyze_selected_region(
    &self,
    circuit: &CircuitModel,
    request: SelectedRegionAnalysisRequest,
) -> Result<SelectedRegionAnalysisResult, ApplicationError>;

pub fn validate_selected_region(
    &self,
    circuit: &CircuitModel,
    request: &SelectedRegionAnalysisRequest,
) -> Vec<SelectedRegionIssue>;

pub fn build_subcircuit_view(
    &self,
    circuit: &CircuitModel,
    component_ids: &[String],
) -> Result<SubcircuitView, ApplicationError>;

pub fn detect_boundary_nets(
    &self,
    circuit: &CircuitModel,
    component_ids: &[String],
) -> Result<Vec<RegionNetSummary>, ApplicationError>;

pub fn match_known_region_template(
    &self,
    subcircuit: &SubcircuitView,
    request: &SelectedRegionAnalysisRequest,
) -> Option<MatchedRegionTemplate>;

pub fn generate_region_netlist_fragment(
    &self,
    circuit: &CircuitModel,
    request: &SelectedRegionAnalysisRequest,
) -> Result<RegionNetlistFragment, ApplicationError>;
```

Если текущие модели circuit называются иначе (`Circuit`, `CircuitDto`, `Project.schematic`), адаптировать названия к реальному коду без создания дублирующих моделей.

---

## 9. SubcircuitView

Добавить application/internal model:

```rust
pub struct SubcircuitView {
    pub component_ids: Vec<String>,
    pub internal_nets: Vec<String>,
    pub boundary_nets: Vec<String>,
    pub external_nets: Vec<String>,
}
```

Назначение:

```text
SubcircuitView — это внутреннее representation для анализа, не обязательно сохранять его в .circuit package.
```

Логика определения:

```text
internal_net:
- net подключён только к выбранным компонентам.

boundary_net:
- net подключён хотя бы к одному выбранному компоненту и хотя бы к одному внешнему компоненту/источнику/ground/probe.

external_net:
- net не принадлежит выбранной области, но связан через boundary.
```

Особый случай:

```text
gnd/reference net всегда должен быть виден как reference candidate,
даже если он формально internal.
```

---

## 10. Validation rules v1.6

`validate_selected_region` должен возвращать issues.

### 10.1. Empty selection

```text
code: empty_selection
severity: Error
message: Select at least one component before running region analysis.
```

### 10.2. Unknown component id

```text
code: unknown_component
severity: Error
message: Selected component does not exist in current circuit.
```

### 10.3. Missing input/output configuration

```text
code: missing_input_port
severity: Error
code: missing_output_port
severity: Error
```

### 10.4. Missing reference node

```text
code: missing_reference_node
severity: Warning или Error
```

Для v1.6 можно разрешить default GND, если в схеме есть ground net.

### 10.5. No boundary nets

```text
code: no_boundary_nets
severity: Warning
message: Selected region has no detected boundary nets; only structural analysis is available.
```

### 10.6. Unsupported symbolic analysis

```text
code: symbolic_unavailable
severity: Warning
message: Symbolic derivation is available only for matched templates in v1.6.
```

### 10.7. Too complex / nonlinear

```text
code: unsupported_region_complexity
severity: Warning
message: This selected region is not recognized as a supported v1.6 template.
```

---

## 11. Template matching v1.6

Не делать общий solver. Только ограниченный deterministic matching.

### 11.1. Минимально поддержать RC low-pass region

Если выбранные компоненты:

```text
R + C
```

и соединение соответствует:

```text
Vin -- R -- Vout
          |
          C
          |
         GND
```

Тогда вернуть:

```text
template_id: rc_low_pass_template
title: RC Low-Pass Filter
formula_ids: [rc_low_pass_cutoff]
confidence: 0.9+
transfer_function:
H(s) = 1 / (1 + sRC)
```

Если текущая circuit model не даёт достаточно информации для надёжного H(s), вернуть template match + formula reference, но не выдумывать точные параметры.

### 11.2. Поддержать voltage divider region, если template/формула уже есть

Если выбранные компоненты:

```text
R1 + R2
```

и топология соответствует:

```text
Vin -- R1 -- Vout -- R2 -- GND
```

Тогда вернуть:

```text
template_id: voltage_divider
formula_ids: [voltage_divider]
transfer_function/expression:
Vout = Vin * R2 / (R1 + R2)
```

### 11.3. Остальные области

Для любых других selections:

```text
status: Partial или Unsupported
matched_template: None
summary: Structural analysis completed. No supported v1.6 template matched.
netlist_fragment: Some(...)
warnings: symbolic_unavailable / unsupported_region_complexity
```

---

## 12. Netlist fragment generation

Не добавлять real ngspice.

Использовать существующий netlist/exporter pattern, если он есть.

Требования:

```text
- fragment должен включать только selected components;
- boundary nets должны быть сохранены;
- reference node/GND должен быть сохранён;
- если component не поддерживается netlist exporter, добавить warning;
- не ломать full project netlist generation.
```

Пример результата:

```spice
* Selected region fragment: RC Low-Pass
R1 net_in net_out 10k
C1 net_out gnd 100n
* Boundary nets: net_in, net_out, gnd
```

---

## 13. Report integration v1.6

Добавить foundation:

```text
report_section_markdown: Option<String>
```

Для результата selected region analysis backend должен уметь сформировать Markdown-фрагмент:

```markdown
## Selected Region Analysis

Components:
- R1
- C1

Boundary nets:
- net_in
- net_out
- gnd

Matched template:
RC Low-Pass Filter

Warnings:
- Symbolic analysis is limited to supported templates in v1.6.

SPICE fragment:
...
```

Если существующий report service уже поддерживает добавление sections, добавить command/facade method:

```rust
pub fn add_selected_region_analysis_to_report(...)
```

Если report integration потребует слишком большой refactor, оставить controlled placeholder:

```text
report_section_markdown returned by analysis result, but not persisted into project report in v1.6.
```

Это обязательно задокументировать.

---

## 14. API DTO

В:

```text
engine/api/src/dto.rs
```

добавить DTO:

```rust
pub struct RegionPortDto {
    pub positive_net: String,
    pub negative_net: Option<String>,
    pub label: Option<String>,
}

pub struct SelectedRegionAnalysisRequestDto {
    pub component_ids: Vec<String>,
    pub input_port: Option<RegionPortDto>,
    pub output_port: Option<RegionPortDto>,
    pub reference_node: Option<String>,
    pub analysis_direction: String,
    pub analysis_mode: String,
}

pub struct SelectedRegionPreviewDto {
    pub region: SelectedCircuitRegionDto,
    pub selected_components: Vec<RegionComponentSummaryDto>,
    pub detected_internal_nets: Vec<RegionNetSummaryDto>,
    pub detected_boundary_nets: Vec<RegionNetSummaryDto>,
    pub suggested_input_nets: Vec<String>,
    pub suggested_output_nets: Vec<String>,
    pub suggested_reference_nodes: Vec<String>,
    pub warnings: Vec<SelectedRegionIssueDto>,
    pub errors: Vec<SelectedRegionIssueDto>,
}

pub struct SelectedRegionAnalysisResultDto {
    pub region: SelectedCircuitRegionDto,
    pub status: String,
    pub summary: String,
    pub matched_template: Option<MatchedRegionTemplateDto>,
    pub equivalent_circuit: Option<EquivalentCircuitSummaryDto>,
    pub transfer_function: Option<RegionTransferFunctionDto>,
    pub measurements: Vec<RegionMeasurementDto>,
    pub graph_specs: Vec<RegionGraphSpecDto>,
    pub netlist_fragment: Option<RegionNetlistFragmentDto>,
    pub warnings: Vec<SelectedRegionIssueDto>,
    pub errors: Vec<SelectedRegionIssueDto>,
    pub report_section_markdown: Option<String>,
}
```

Также добавить DTO для:

```text
SelectedCircuitRegionDto
RegionComponentSummaryDto
RegionNetSummaryDto
SelectedRegionIssueDto
MatchedRegionTemplateDto
EquivalentCircuitSummaryDto
RegionTransferFunctionDto
RegionMeasurementDto
RegionGraphSpecDto
RegionNetlistFragmentDto
```

Требования:

```text
- conversion helpers из core/application models в DTO;
- строковые enum values должны быть стабильными;
- unknown enum input должен возвращать ApiError, а не panic;
- frontend должен получать serializable DTO без Rust-specific типов.
```

---

## 15. API facade

В:

```text
engine/api/src/facade.rs
```

добавить методы:

```rust
pub fn preview_selected_region(
    &self,
    component_ids: Vec<String>,
) -> Result<SelectedRegionPreviewDto, ApiError>;

pub fn analyze_selected_region(
    &mut self,
    request: SelectedRegionAnalysisRequestDto,
) -> Result<SelectedRegionAnalysisResultDto, ApiError>;

pub fn clear_selected_region(
    &mut self,
) -> Result<ProjectDto, ApiError>;
```

Опционально, если вписывается без большого refactor:

```rust
pub fn add_selected_region_analysis_to_report(
    &mut self,
    analysis_id: String,
) -> Result<ProjectDto, ApiError>;
```

Правила:

```text
- методы требуют current_project;
- если current_project отсутствует — controlled ApiError;
- component_ids проверяются backend-ом;
- region analysis не должен менять проект, кроме clear/add-to-report;
- preview/analyze должны быть deterministic.
```

---

## 16. Tauri commands

В:

```text
apps/desktop-tauri/src-tauri/src/lib.rs
```

добавить команды:

```rust
#[tauri::command]
async fn preview_selected_region(
    state: tauri::State<'_, AppState>,
    component_ids: Vec<String>,
) -> Result<SelectedRegionPreviewDto, String>;

#[tauri::command]
async fn analyze_selected_region(
    state: tauri::State<'_, AppState>,
    request: SelectedRegionAnalysisRequestDto,
) -> Result<SelectedRegionAnalysisResultDto, String>;

#[tauri::command]
async fn clear_selected_region(
    state: tauri::State<'_, AppState>,
) -> Result<ProjectDto, String>;
```

Добавить в `generate_handler!`.

Проверить capabilities/permissions Tauri v2:

```text
apps/desktop-tauri/src-tauri/capabilities/default.json
```

Команды должны быть разрешены, если проект использует command allowlist.

---

## 17. Frontend types/API

### 17.1. Types

В:

```text
apps/desktop-tauri/src/types/index.ts
```

или актуальном файле типов добавить:

```ts
export interface RegionPortDto {
  positive_net: string;
  negative_net?: string | null;
  label?: string | null;
}

export interface SelectedRegionAnalysisRequestDto {
  component_ids: string[];
  input_port?: RegionPortDto | null;
  output_port?: RegionPortDto | null;
  reference_node?: string | null;
  analysis_direction: string;
  analysis_mode: string;
}

export interface SelectedRegionAnalysisResultDto {
  region: SelectedCircuitRegionDto;
  status: string;
  summary: string;
  matched_template?: MatchedRegionTemplateDto | null;
  equivalent_circuit?: EquivalentCircuitSummaryDto | null;
  transfer_function?: RegionTransferFunctionDto | null;
  measurements: RegionMeasurementDto[];
  graph_specs: RegionGraphSpecDto[];
  netlist_fragment?: RegionNetlistFragmentDto | null;
  warnings: SelectedRegionIssueDto[];
  errors: SelectedRegionIssueDto[];
  report_section_markdown?: string | null;
}
```

Добавить остальные DTO по backend shape.

### 17.2. API methods

В:

```text
apps/desktop-tauri/src/api/index.ts
```

добавить:

```ts
previewSelectedRegion(componentIds: string[]): Promise<SelectedRegionPreviewDto>;

analyzeSelectedRegion(
  request: SelectedRegionAnalysisRequestDto,
): Promise<SelectedRegionAnalysisResultDto>;

clearSelectedRegion(): Promise<ProjectDto>;
```

Правило:

```text
API methods только вызывают Tauri invoke.
Никакой domain logic в TypeScript.
```

---

## 18. Frontend store

Если используется Zustand store, добавить состояние:

```ts
selectedRegionComponentIds: string[];
selectedRegionPreview: SelectedRegionPreviewDto | null;
selectedRegionRequest: SelectedRegionAnalysisRequestDto | null;
selectedRegionResult: SelectedRegionAnalysisResultDto | null;
selectedRegionLoading: boolean;
selectedRegionError: string | null;
```

Actions:

```ts
setSelectedRegionComponentIds(ids: string[]): void;
previewSelectedRegion(ids: string[]): Promise<void>;
analyzeSelectedRegion(request: SelectedRegionAnalysisRequestDto): Promise<void>;
clearSelectedRegion(): Promise<void>;
```

Важно:

```text
store не определяет boundary nets сам.
store только хранит DTO от backend.
```

---

## 19. Frontend UI

Создать папку:

```text
apps/desktop-tauri/src/components/selected-region/
```

Компоненты:

```text
SelectedRegionPanel.tsx
SelectedRegionToolbar.tsx
SelectedRegionPreviewCard.tsx
RegionPortSelector.tsx
RegionAnalysisModeSelector.tsx
SelectedRegionResultCard.tsx
RegionWarningsList.tsx
RegionNetlistFragment.tsx
RegionGraphSpecsList.tsx
RegionTemplateMatchCard.tsx
```

Если структура проекта использует другие пути — адаптировать.

### 19.1. `SelectedRegionToolbar`

Задачи:

```text
- показывает число выбранных компонентов;
- кнопка Preview Region;
- кнопка Analyze Selection;
- кнопка Clear Selection;
```

### 19.2. `SelectedRegionPreviewCard`

Показывает:

```text
- selected components;
- internal nets;
- boundary nets;
- suggested input nets;
- suggested output nets;
- suggested reference nodes;
- warnings/errors.
```

### 19.3. `RegionPortSelector`

Позволяет выбрать:

```text
- input+;
- input- / GND;
- output+;
- output- / GND;
- reference node.
```

Источник вариантов — только `SelectedRegionPreviewDto`.

### 19.4. `RegionAnalysisModeSelector`

Варианты:

```text
Structural
TemplateBased
NumericMock
AllAvailable
```

Пояснение в UI:

```text
TemplateBased — returns formulas only for known supported templates.
NumericMock — mock/foundation summary; real ngspice is planned later.
```

### 19.5. `SelectedRegionResultCard`

Показывает:

```text
- status;
- summary;
- matched template;
- equivalent circuit;
- transfer function, если есть;
- measurements;
- warnings;
- errors;
- report_section_markdown preview.
```

### 19.6. `RegionNetlistFragment`

Показывает:

```text
- SPICE/netlist fragment;
- copy button optional;
- warnings.
```

### 19.7. `RegionGraphSpecsList`

Показывает только specs/placeholders:

```text
- gain;
- phase;
- Zin;
- Zout;
- Vin/Vout;
```

Если данных нет:

```text
Graph data is not generated in v1.6. This version exposes available graph specs for future simulation integration.
```

---

## 20. SchematicCanvas integration

Нужно добавить/проверить multi-select foundation.

Требования:

```text
- пользователь может выбрать несколько компонентов;
- выбранные компоненты визуально подсвечены;
- selection ids передаются в SelectedRegionPanel;
- React Flow selection остаётся UI adapter;
- backend получает только component_ids;
- backend заново валидирует component_ids.
```

Если полноценный multi-select через React Flow усложняет задачу, допустимый v1.6 fallback:

```text
- добавить checkbox/list selection в боковой панели компонентов;
- пользователь выбирает компоненты в списке;
- analysis работает по выбранным ids;
- в docs указать, что canvas multi-select будет улучшен позже.
```

Но предпочтительно использовать существующее selection событие React Flow, если оно уже подключено.

---

## 21. Интеграция с существующими экранами

В Schematic Editor / Workbench добавить:

```text
- вкладку/панель “Selected Region”
- кнопку “Analyze Selection”
- результат анализа в right/bottom panel
```

Не делать отдельный большой экран, если это ломает навигацию. Лучше встроить foundation в текущий schematic workflow.

---

## 22. Rust tests

Добавить tests, не складывая всё в один файл.

Рекомендуемая структура:

```text
engine/core/tests/selected_region_model_tests.rs
engine/application/tests/selected_region_analysis_service_tests.rs
engine/api/tests/selected_region_api_tests.rs
```

### 22.1. Core tests

Проверить:

```text
- normalize_component_ids удаляет дубликаты;
- normalize_component_ids сохраняет стабильный порядок;
- empty region не panic-ует;
- is_region_configured false без input/output;
- selected_region_summary возвращает понятный текст.
```

### 22.2. Service tests

Проверить:

```text
- empty_selection -> error issue;
- unknown component id -> error issue;
- RC low-pass R1+C1 region даёт boundary/internal nets;
- RC low-pass R1+C1 region распознаётся как rc_low_pass_template, если topology доступна;
- voltage divider region распознаётся, если template/topology доступна;
- unsupported region возвращает Partial/Unsupported + warning, но не panic;
- generated netlist fragment содержит selected components;
- no_boundary_nets даёт warning;
- missing input/output даёт controlled errors.
```

### 22.3. API tests

Проверить:

```text
- preview_selected_region требует current_project;
- analyze_selected_region требует current_project;
- analyze_selected_region с пустым selection возвращает controlled error/result;
- invalid analysis_mode string -> ApiError;
- valid RC region -> result DTO serializable;
- no panic on unknown component id.
```

### 22.4. Regression tests

Убедиться, что не сломано:

```text
- RC vertical slice;
- Formula Library calculation;
- Engineering Notebook;
- Component Library search/details/assign;
- .circuit save/load;
- existing schematic validation.
```

---

## 23. Frontend tests

Добавить/обновить Vitest/React Testing Library tests:

```text
apps/desktop-tauri/src/components/selected-region/__tests__/
```

Минимум:

```text
SelectedRegionPanel.test.tsx
SelectedRegionPreviewCard.test.tsx
SelectedRegionResultCard.test.tsx
RegionPortSelector.test.tsx
RegionNetlistFragment.test.tsx
```

Проверить:

```text
- panel renders empty selection state;
- preview card renders components/boundary nets;
- port selector uses backend-provided nets;
- result card renders matched template;
- warning list renders symbolic_unavailable;
- netlist fragment renders content;
- UI не рассчитывает formulas/netlist в тестах.
```

Если test setup требует mocks:

```text
- добавить минимальный mock Tauri invoke;
- не ломать существующие 36 frontend tests.
```

---

## 24. Документация

Создать:

```text
docs/selected_region/SELECTED_REGION_ANALYSIS_FOUNDATION.md
```

Документ должен содержать:

```text
1. Что такое Selected Region Analysis.
2. Что пользователь может делать в v1.6.
3. Что считается selected region.
4. Что такое boundary nets.
5. Как задаются input/output/reference.
6. Какие analysis modes есть в v1.6.
7. Почему arbitrary symbolic solver не поддерживается.
8. Какие templates поддерживаются:
   - RC low-pass;
   - voltage divider, если реализован.
9. Что возвращает unsupported region.
10. Как будет развиваться v1.7/v1.8+.
```

Обновить:

```text
README.md
docs/testing/TESTING.md
docs/testing/latest_verification_log.md
```

README после завершения:

```text
Current roadmap stage: v1.7 next

Completed:
- v1.6 — Selected Region Analysis Foundation
```

---

## 25. Обязательное требование: файл с логами проверок/тестов

Агент обязан создать отдельный файл с логами прохождения проверок/тестов и предоставить этот файл пользователю для проверки здесь.

Создать:

```text
docs/testing/verification_logs/v1.6_selected_region_analysis.md
```

Дополнительно создать raw-log файл:

```text
docs/testing/raw_logs/v1.6_selected_region_analysis_commands.txt
```

Если папки `docs/testing/raw_logs/` нет — создать.

### 25.1. Что должно быть в verification log

```text
- дата и время;
- branch;
- commit before changes;
- commit after changes;
- git status before;
- git status after;
- список изменённых файлов;
- summary of implementation;
- user-facing changes;
- rust checks;
- frontend checks;
- tauri build;
- UI smoke test;
- agent self-audit;
- known limitations;
- final verdict.
```

### 25.2. Что должно быть в raw-log файле

Raw файл должен содержать фактический вывод команд:

```text
git rev-parse --show-toplevel
git branch --show-current
git status --short
git log --oneline -12
cargo fmt --check
cargo test
npm.cmd run format:check
npm.cmd run typecheck
npm.cmd run test
npm.cmd run build
npm.cmd run tauri:build
```

Если какая-то команда не запускалась, записать:

```text
COMMAND NOT RUN
Reason: ...
```

Не писать PASS без фактического запуска.

---

## 26. Agent self-check / перепроверка выполненности ТЗ

После реализации агент обязан перечитать это ТЗ и пройти self-audit.

Добавить в verification log раздел:

```markdown
## Agent self-audit against v1.6 TZ

### Scope
- No real ngspice added: PASS / FAIL
- No PCB/routing/Gerber added: PASS / FAIL
- No full symbolic solver added: PASS / FAIL
- React does not calculate selected region analysis: PASS / FAIL

### Core
- selected_region.rs exists: PASS / FAIL
- SelectedCircuitRegion model exists: PASS / FAIL
- SelectedRegionAnalysisRequest model exists: PASS / FAIL
- SelectedRegionAnalysisResult model exists: PASS / FAIL
- Region issues/warnings model exists: PASS / FAIL

### Application
- SelectedRegionAnalysisService exists: PASS / FAIL
- preview_selected_region implemented: PASS / FAIL
- analyze_selected_region implemented: PASS / FAIL
- validate_selected_region implemented: PASS / FAIL
- boundary nets detection implemented: PASS / FAIL
- netlist fragment generation implemented: PASS / FAIL
- unsupported arbitrary region handled without panic: PASS / FAIL

### API/Tauri
- DTOs added: PASS / FAIL
- facade methods added: PASS / FAIL
- Tauri commands added: PASS / FAIL
- Tauri commands registered: PASS / FAIL
- capabilities/permissions checked: PASS / FAIL

### Frontend
- frontend types added: PASS / FAIL
- frontend API methods added: PASS / FAIL
- selected-region UI components added: PASS / FAIL
- selection flow integrated: PASS / FAIL
- UI uses backend DTO for boundary nets: PASS / FAIL

### Tests/docs/git
- Rust tests added/updated: PASS / FAIL
- Frontend tests added/updated: PASS / FAIL
- SELECTED_REGION_ANALYSIS_FOUNDATION.md created: PASS / FAIL
- README updated: PASS / FAIL
- TESTING.md updated: PASS / FAIL
- latest_verification_log.md updated: PASS / FAIL
- raw log file created: PASS / FAIL
- changes committed: PASS / FAIL
- changes pushed to origin/main: PASS / FAIL
```

Если любой пункт FAIL — агент должен:

```text
1. Исправить, если это входит в scope.
2. Если не исправляет — явно объяснить почему.
3. Не писать “готово”, если критический пункт не выполнен.
```

---

## 27. Финальные проверки

Выполнить:

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

Manual smoke test, если доступен UI:

```bash
cd "D:\Документы\vscode\HotSAS Studio\apps\desktop-tauri"
npm.cmd run tauri:dev
```

Проверить:

```text
[OK/FAIL] приложение запускается
[OK/FAIL] RC demo project создаётся
[OK/FAIL] Schematic Editor открывается
[OK/FAIL] можно выбрать R1/C1
[OK/FAIL] Selected Region panel открывается
[OK/FAIL] Preview Region показывает selected components
[OK/FAIL] Preview Region показывает boundary nets
[OK/FAIL] можно выбрать input/output/reference
[OK/FAIL] Analyze Selection возвращает result
[OK/FAIL] RC low-pass region распознаётся или controlled partial result
[OK/FAIL] netlist fragment отображается
[OK/FAIL] unsupported selection не ломает UI
[OK/FAIL] Formula Library всё ещё работает
[OK/FAIL] Engineering Notebook всё ещё работает
[OK/FAIL] Component Library всё ещё работает
```

---

## 28. Git commit / push

После успешных проверок:

```bash
git status --short
git diff --stat
git diff --name-only
```

Проверить, что в staged changes нет:

```text
target/
node_modules/
dist/
build/
*.exe
личных файлов пользователя
локальных черновиков вне docs/testing/docs/selected_region
```

Стадировать только проектные файлы.

Рекомендуемые коммиты:

```bash
git add engine apps docs README.md
git commit -m "v1.6: add selected region analysis foundation"
git push origin main
```

Если после первого коммита нужно обновить verification log с фактическим hash:

```bash
git add docs/testing/verification_logs/v1.6_selected_region_analysis.md docs/testing/latest_verification_log.md docs/testing/raw_logs/v1.6_selected_region_analysis_commands.txt README.md docs/testing/TESTING.md
git commit -m "v1.6: update selected region verification logs"
git push origin main
```

В verification log указать оба коммита:

```text
Implementation commit:
Verification log update commit:
Final HEAD:
```

---

## 29. Итоговый отчёт агента пользователю

После push агент должен вернуть краткий отчёт:

```text
v1.6 — Selected Region Analysis Foundation: завершено / частично завершено / не завершено

Git:
- branch:
- final HEAD:
- pushed to origin/main: yes/no

Что изменилось для пользователя:
- ...
- ...

Проверки:
- cargo fmt --check: PASS/FAIL
- cargo test: PASS/FAIL
- npm run format:check: PASS/FAIL
- npm run typecheck: PASS/FAIL
- npm run test: PASS/FAIL
- npm run build: PASS/FAIL
- npm run tauri:build: PASS/FAIL
- UI smoke test: PASS/FAIL/NOT RUN

Документация:
- docs/selected_region/SELECTED_REGION_ANALYSIS_FOUNDATION.md
- docs/testing/verification_logs/v1.6_selected_region_analysis.md
- docs/testing/raw_logs/v1.6_selected_region_analysis_commands.txt
- docs/testing/latest_verification_log.md
- README.md

Ограничения:
- arbitrary symbolic solver не добавлен;
- real ngspice не добавлен;
- PCB не добавлен.

Готовность:
- Ready for v1.7: YES/NO
```

Отдельно предоставить файл логов тестов пользователю здесь.

---

## 30. Критерии приёмки v1.6

v1.6 можно считать принятой только если:

```text
1. selected_region core models добавлены.
2. SelectedRegionAnalysisService добавлен.
3. preview selected region работает через backend.
4. analyze selected region работает через backend.
5. boundary nets определяются backend-ом.
6. unsupported arbitrary region возвращает controlled result, а не panic.
7. RC low-pass region распознаётся или честно documented почему only structural in v1.6.
8. SPICE/netlist fragment генерируется.
9. API DTO/facade добавлены.
10. Tauri commands добавлены и зарегистрированы.
11. Frontend panel добавлен.
12. React не считает analysis.
13. Tests добавлены/обновлены.
14. docs/selected_region/SELECTED_REGION_ANALYSIS_FOUNDATION.md создан.
15. docs/testing/verification_logs/v1.6_selected_region_analysis.md создан.
16. docs/testing/raw_logs/v1.6_selected_region_analysis_commands.txt создан.
17. latest_verification_log.md обновлён.
18. README обновлён.
19. cargo fmt --check проходит.
20. cargo test проходит.
21. npm run format:check проходит.
22. npm run typecheck проходит.
23. npm run test проходит.
24. npm run build проходит.
25. npm run tauri:build проходит.
26. Git commit создан.
27. Git push выполнен.
28. Агент провёл self-audit по этому ТЗ.
```

Если пункты 19–25 не проходят, v1.6 не считается принятой.

---

## 31. Следующий этап после v1.6

После успешной v1.6 следующий этап дорожной карты:

```text
v1.7 — Export Center v1
```

Ориентировочный смысл v1.7:

```text
- объединить report export;
- schematic export placeholder;
- simulation export placeholder;
- BOM export foundation;
- component library export;
- selected region analysis report inclusion;
- подготовка к KiCad/Altium workflow.
```

Но v1.7 начинать только после полного закрытия v1.6 и проверки GitHub.
