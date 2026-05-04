# ТЗ HotSAS Studio v1.9 — SPICE/Touchstone Import Foundation

## 0. Назначение этапа

Выполнить этап:

```text
v1.9 — SPICE/Touchstone Import Foundation
```

Цель этапа — начать работу с реальными моделями компонентов:

- импортировать SPICE-файлы `.lib`, `.mod`, `.subckt`, `.cir`;
- находить внутри них `.model` и `.subckt`;
- показывать пользователю найденные модели;
- дать foundation для pin mapping;
- сохранять simulation model в проекте/библиотеке компонентов;
- прикреплять модель к `ComponentDefinition`;
- импортировать Touchstone `.s1p` / `.s2p` как основу для RF/AC-моделей;
- парсить frequency points и S-parameters;
- хранить импортированные данные в backend;
- подготовить будущий просмотр `S11/S21/S12/S22`.

v1.9 — это **foundation**, а не полноценный vendor-library manager и не полный SPICE parser.

---

## 1. Текущее состояние проекта

Считать, что перед v1.9 уже закрыты:

```text
v1.0 — Initial RC Low-Pass Vertical Slice
v1.0.1 — Architecture Hardening
v1.1.1 — Formatting + Build/Test Infrastructure
v1.1.2 — Backend Test Expansion
v1.1.3 — FormulaPackLoader + FormulaRegistry
v1.1.4 — Generic FormulaEnginePort
v1.1.5 — Exact E-Series Tables
v1.2 — Project Package Storage .circuit
v1.3 — Schematic Editor Foundations
v1.4 — Engineering Notebook / Calculator Foundations
v1.5 — Component Library Foundation
v1.6 — Selected Region Analysis Foundation
v1.7 — Export Center v1
v1.8 — ngspice Adapter v1
```

Ожидаемый статус после v1.8:

```text
- README показывает Current roadmap stage: v1.9 next.
- latest verification log указывает на v1.8.
- Есть NgspiceSimulationAdapter / NgspiceSimulationService.
- Mock simulation сохранён.
- real ngspice integration tests opt-in.
- Export Center уже умеет экспортировать Component Library JSON, SPICE netlist, BOM, SVG schematic и Altium workflow placeholder.
```

---

## 2. Что изменится для пользователя

После v1.9 пользователь должен получить новую возможность:

```text
Import Models / Model Import
```

Пользователь сможет:

1. Открыть экран или вкладку импорта моделей.
2. Импортировать SPICE-модель из текста или файла:
   - `.lib`
   - `.mod`
   - `.subckt`
   - `.cir`
3. Увидеть список обнаруженных моделей:
   - `.model` diode/BJT/MOSFET/etc.;
   - `.subckt` macro-model / op-amp / IC / generic subcircuit.
4. Открыть details найденной модели:
   - model name;
   - model kind;
   - source file;
   - number of pins;
   - parameters;
   - warnings/errors.
5. Для `.subckt` выполнить базовый pin mapping:
   - увидеть pin list из `.subckt`;
   - сопоставить pins с pin names компонента;
   - сохранить mapping.
6. Прикрепить импортированную модель к `ComponentDefinition` из Component Library.
7. Импортировать Touchstone `.s1p` / `.s2p`.
8. Увидеть summary Touchstone-файла:
   - number of ports;
   - number of frequency points;
   - frequency range;
   - reference impedance;
   - parameter format: RI/MA/DB;
   - warnings/errors.
9. Сохранить импортированную модель в backend state.
10. Увидеть, что компонент теперь имеет simulation model.

Пока НЕ нужно:

```text
- строить полноценные S11/S21-графики;
- запускать автоматический validation netlist для любой модели;
- импортировать огромные vendor libraries с полноценной навигацией;
- поддерживать весь SPICE language;
- делать PCB/KiCad/Altium library generation.
```

---

## 3. Главная архитектурная идея

UI не парсит SPICE и Touchstone.

Правильный поток:

```text
React UI
→ Tauri command
→ hotsas_api facade
→ hotsas_application ModelImportService
→ ports
→ hotsas_adapters SpiceModelParser / TouchstoneParser
→ hotsas_core models
```

Запрещено:

```text
- читать/парсить SPICE в React;
- читать/парсить Touchstone в React;
- делать pin mapping logic только во frontend;
- напрямую менять ComponentDefinition в UI;
- сохранять imported model только в Zustand;
- запускать ngspice напрямую из UI;
- ломать Component Library;
- ломать Export Center;
- ломать Project Package Storage.
```

---

## 4. Жёсткие ограничения scope v1.9

### Запрещено

```text
- не делать full SPICE parser;
- не поддерживать произвольные .control/.param/.func/.measure блоки полностью;
- не делать полноценный SPICE syntax highlighter;
- не делать автоматическую загрузку моделей из интернета;
- не добавлять DigiKey/Mouser/LCSC API;
- не делать real datasheet scraping;
- не делать KiCad/Altium symbol/footprint export;
- не делать PCB editor;
- не делать Gerber/Excellon;
- не делать полный RF analyzer;
- не строить полноценные S-parameter plots как обязательную часть;
- не делать libngspice FFI;
- не делать proprietary Altium files;
- не ломать текущий NgspiceSimulationAdapter;
- не делать Touchstone import обязательным для Simulation screen.
```

### Разрешено

```text
- добавить core-модели imported models;
- добавить SPICE model parser foundation;
- добавить Touchstone parser foundation;
- добавить ModelImportService;
- добавить API DTO/facade methods;
- добавить Tauri commands;
- добавить frontend Import Models screen/tab;
- добавить attach-model-to-component flow;
- добавить tests;
- добавить docs;
- обновить verification logs;
- сделать commit и push.
```

---

## 5. Preflight перед изменениями

Из корня проекта выполнить:

```bash
cd "D:\Документы\vscode\HotSAS Studio"

git rev-parse --show-toplevel
git branch --show-current
git status --short
git log --oneline -10
git diff --stat
git diff --name-only
```

Правила:

```text
- Не выполнять git reset.
- Не выполнять git clean.
- Не удалять пользовательские изменения.
- Не трогать личные untracked-файлы пользователя.
- Если git status не clean — описать это в verification log.
- Если есть неожиданные изменения в коде — зафиксировать их в отчёте перед правками.
```

Проверить базу v1.8:

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

Также проверить:

```text
docs/testing/latest_verification_log.md
docs/testing/verification_logs/v1.8_ngspice_adapter_v1.md
README.md
docs/simulation/NGSPICE_ADAPTER_V1.md
```

Убедиться:

```text
- latest log указывает на v1.8;
- README показывает v1.9 next;
- v1.8 отмечена в Completed;
- ngspice adapter tests не требуют установленный ngspice для обычного cargo test.
```

---

## 6. Core: модели импорта моделей

Создать модуль:

```text
engine/core/src/model_import.rs
```

Подключить в:

```text
engine/core/src/lib.rs
```

Если часть моделей уже есть в `models.rs` или `component_library.rs`, не дублировать. Расширить существующие типы аккуратно.

### 6.1. Общие модели

Добавить:

```rust
pub enum ImportedModelKind {
    SpiceModel,
    SpiceSubcircuit,
    TouchstoneNetwork,
    Unknown,
}

pub enum ModelImportStatus {
    Parsed,
    ParsedWithWarnings,
    Failed,
    Unsupported,
}

pub struct ImportedModelSource {
    pub file_name: Option<String>,
    pub file_path: Option<String>,
    pub source_format: String,
    pub content_hash: Option<String>,
}
```

### 6.2. SPICE-модели

Добавить:

```rust
pub enum SpiceModelKind {
    Diode,
    BjtNpn,
    BjtPnp,
    MosfetN,
    MosfetP,
    JfetN,
    JfetP,
    Resistor,
    Capacitor,
    Inductor,
    Subcircuit,
    OpAmpMacroModel,
    IcMacroModel,
    Unknown,
}

pub struct SpiceModelParameter {
    pub name: String,
    pub value: String,
    pub unit_hint: Option<String>,
}

pub struct SpiceModelDefinition {
    pub id: String,
    pub name: String,
    pub kind: SpiceModelKind,
    pub source: ImportedModelSource,
    pub raw_line: String,
    pub parameters: Vec<SpiceModelParameter>,
    pub warnings: Vec<String>,
}

pub struct SpiceSubcircuitDefinition {
    pub id: String,
    pub name: String,
    pub pins: Vec<String>,
    pub body: Vec<String>,
    pub source: ImportedModelSource,
    pub detected_kind: SpiceModelKind,
    pub parameters: Vec<SpiceModelParameter>,
    pub warnings: Vec<String>,
}
```

### 6.3. Pin mapping

Добавить:

```rust
pub struct SpicePinMapping {
    pub model_id: String,
    pub component_definition_id: String,
    pub mappings: Vec<SpicePinMappingEntry>,
    pub warnings: Vec<String>,
}

pub struct SpicePinMappingEntry {
    pub model_pin: String,
    pub component_pin: String,
    pub role_hint: Option<String>,
}
```

Правила:

```text
- Для .model обычно pin mapping может быть не нужен или минимален.
- Для .subckt pin mapping обязателен перед attach к компоненту.
- Если количество pins не совпадает с выбранным компонентом — вернуть warning/error, но не panic.
- Для op-amp-like subckt можно попытаться предложить mapping по именам: +IN / -IN / V+ / V- / OUT, но это только suggestion.
```

### 6.4. SPICE import report

Добавить:

```rust
pub struct SpiceImportReport {
    pub status: ModelImportStatus,
    pub source: ImportedModelSource,
    pub models: Vec<SpiceModelDefinition>,
    pub subcircuits: Vec<SpiceSubcircuitDefinition>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}
```

### 6.5. Touchstone-модели

Добавить:

```rust
pub enum TouchstoneParameterFormat {
    RI,
    MA,
    DB,
}

pub enum TouchstoneFrequencyUnit {
    Hz,
    KHz,
    MHz,
    GHz,
}

pub struct ComplexValue {
    pub re: f64,
    pub im: f64,
}

pub struct SParameterPoint {
    pub frequency_hz: f64,
    pub values: Vec<ComplexValue>,
}
```

Для `.s1p`:

```text
values length = 1
S11
```

Для `.s2p`:

```text
values length = 4
S11, S21, S12, S22
```

Добавить:

```rust
pub struct TouchstoneNetworkData {
    pub id: String,
    pub name: String,
    pub port_count: usize,
    pub frequency_unit: TouchstoneFrequencyUnit,
    pub parameter_format: TouchstoneParameterFormat,
    pub reference_impedance_ohm: f64,
    pub points: Vec<SParameterPoint>,
    pub source: ImportedModelSource,
    pub warnings: Vec<String>,
}

pub struct TouchstoneImportReport {
    pub status: ModelImportStatus,
    pub network: Option<TouchstoneNetworkData>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}
```

---

## 7. Core: связь с ComponentDefinition

Проверить текущую модель `ComponentDefinition` и `SimulationModel`.

Нужно обеспечить:

```text
- импортированная SPICE-модель может быть прикреплена к ComponentDefinition;
- импортированная Touchstone-модель может быть прикреплена к ComponentDefinition;
- ComponentDefinition сохраняет ссылку на selected/imported simulation model;
- BOM/symbol/footprint данные не ломаются.
```

Если текущая `SimulationModel` уже есть, расширить её без breaking changes.

Минимальная политика:

```rust
pub struct ImportedSimulationModelReference {
    pub id: String,
    pub model_kind: ImportedModelKind,
    pub display_name: String,
    pub source_file_name: Option<String>,
}
```

или использовать существующий `SimulationModel`, добавив поля:

```text
- model_type / kind;
- source;
- pin_mapping;
- raw_model_id.
```

Важно:

```text
- Не ломать сериализацию старых components.
- Новые поля делать Option/Vec с default.
- Если нужна миграция — задокументировать, но не делать большой migration engine в v1.9.
```

---

## 8. Ports

Добавить в `hotsas_ports`:

```rust
pub trait SpiceModelParserPort: Send + Sync {
    fn parse_spice_models_from_str(
        &self,
        source_name: Option<String>,
        content: &str,
    ) -> Result<SpiceImportReport, PortError>;
}

pub trait TouchstoneParserPort: Send + Sync {
    fn parse_touchstone_from_str(
        &self,
        source_name: Option<String>,
        content: &str,
    ) -> Result<TouchstoneImportReport, PortError>;
}
```

Если в проекте уже есть `ModelParserPort`, можно расширить его вместо создания новых trait, но важно сохранить ясную архитектуру.

Не добавлять зависимость `hotsas_ports` на adapters.

---

## 9. Adapters: SPICE parser foundation

Создать:

```text
engine/adapters/src/spice_model_parser.rs
```

или добавить модуль в текущую структуру adapters.

Реализовать:

```rust
pub struct SimpleSpiceModelParser;
```

### 9.1. Поддержать SPICE comments

```text
* comment
; comment, если встречается
```

Комментарии должны игнорироваться, но можно сохранять source line numbers как future TODO.

### 9.2. Поддержать line continuation

SPICE часто использует:

```spice
.model 1N4148 D(IS=2.52n RS=0.568
+ N=1.752 CJO=4p M=0.4)
```

Правила:

```text
- строки, начинающиеся с '+', приклеивать к предыдущей logical line;
- не panic при continuation без предыдущей строки — warning.
```

### 9.3. Поддержать `.model`

Примеры:

```spice
.model 1N4148 D(IS=2.52n RS=0.568 N=1.752 CJO=4p M=0.4)
.model Q2N2222 NPN(IS=1e-14 BF=200 VAF=100)
.model IRLZ44N NMOS(VTO=2.0 KP=50u RD=0.02 RS=0.02)
```

Парсер должен извлекать:

```text
- model name;
- model type;
- parameters key=value;
- raw_line;
- warnings for unsupported/unknown types.
```

### 9.4. Поддержать `.subckt`

Пример:

```spice
.subckt LM358 IN+ IN- VCC VEE OUT
RIN IN+ IN- 1Meg
EOUT OUT 0 VALUE={V(IN+)-V(IN-)}
.ends LM358
```

Парсер должен извлекать:

```text
- subckt name;
- pins list;
- body lines;
- detected kind heuristic:
  - name contains opamp/lm358/tl072 -> OpAmpMacroModel
  - otherwise IcMacroModel / Subcircuit
```

### 9.5. Поддержать `.lib`, `.mod`, `.cir` как input kinds

Формат файла не важен: parser работает по content. Расширение используется только для source metadata / UI.

### 9.6. Unsupported constructs

Если встречаются:

```text
.include
.lib nested include
.param
.func
.control
.measure
```

Не нужно реализовывать полностью.

Поведение:

```text
- controlled warning;
- продолжить парсинг известных .model/.subckt;
- не panic.
```

---

## 10. Adapters: Touchstone parser foundation

Создать:

```text
engine/adapters/src/touchstone_parser.rs
```

Реализовать:

```rust
pub struct SimpleTouchstoneParser;
```

### 10.1. Поддержать file extension / source name

Определить port count:

```text
.s1p -> 1
.s2p -> 2
```

Дополнительно можно подготовить generic `.sNp`, но обязательны `.s1p` и `.s2p`.

### 10.2. Поддержать option line

Пример:

```touchstone
# GHz S MA R 50
```

Поддержать:

```text
frequency units:
- Hz
- kHz
- MHz
- GHz

parameter:
- S

format:
- RI
- MA
- DB

reference:
- R 50
```

Если option line отсутствует:

```text
- использовать default по Touchstone-like convention: GHz / S / MA / R 50;
- добавить warning.
```

### 10.3. Поддержать comments

```text
! comment
```

Игнорировать.

### 10.4. Поддержать S1P rows

Для `.s1p`:

```text
frequency value1 value2
```

Где:

```text
RI -> re im
MA -> magnitude angle_deg
DB -> db angle_deg
```

Хранить в `ComplexValue`.

### 10.5. Поддержать S2P rows

Для `.s2p`:

```text
frequency s11a s11b s21a s21b s12a s12b s22a s22b
```

Порядок значений:

```text
S11, S21, S12, S22
```

### 10.6. Ошибки

Controlled errors:

```text
- invalid numeric value;
- wrong number of columns;
- unsupported parameter type not S;
- unsupported format;
- empty file;
- cannot determine port count.
```

Не panic.

---

## 11. Application: ModelImportService

Создать:

```text
engine/application/src/services/model_import.rs
```

Подключить к `AppServices`.

Сервис должен иметь методы:

```rust
pub fn import_spice_from_text(
    &mut self,
    source_name: Option<String>,
    content: String,
) -> Result<SpiceImportReport, ApplicationError>;

pub fn import_touchstone_from_text(
    &mut self,
    source_name: Option<String>,
    content: String,
) -> Result<TouchstoneImportReport, ApplicationError>;

pub fn list_imported_models(
    &self,
) -> Result<Vec<ImportedModelSummary>, ApplicationError>;

pub fn get_imported_model(
    &self,
    model_id: String,
) -> Result<ImportedModelDetails, ApplicationError>;

pub fn validate_spice_pin_mapping(
    &self,
    request: SpicePinMappingRequest,
) -> Result<SpicePinMappingValidationReport, ApplicationError>;

pub fn attach_imported_model_to_component(
    &mut self,
    request: AttachImportedModelRequest,
) -> Result<ComponentDefinition, ApplicationError>;
```

Можно адаптировать имена под текущий стиль проекта.

### 11.1. State policy

Для v1.9 допустимо хранить импортированные модели:

```text
- в текущем backend state;
- внутри current project/component library state;
- как part of component definitions after attach.
```

Если есть `.circuit` project package integration:

```text
- подготовить model metadata для папок models/spice и models/touchstone;
- не обязательно реализовывать полноценное копирование исходных файлов, если это требует большой refactor;
- задокументировать limitation.
```

### 11.2. Attach logic

При attach к ComponentDefinition:

```text
1. Проверить, что imported model exists.
2. Проверить, что component definition exists.
3. Если model is .subckt:
   - проверить pin mapping;
   - если mapping missing/incomplete -> error или warning + not attached.
4. Создать/обновить SimulationModel у ComponentDefinition.
5. Вернуть обновлённый ComponentDefinition DTO.
```

---

## 12. API DTO

В `engine/api/src/dto.rs` добавить DTO.

### 12.1. SPICE DTO

```rust
pub struct SpiceImportRequestDto {
    pub source_name: Option<String>,
    pub content: String,
}

pub struct SpiceModelParameterDto {
    pub name: String,
    pub value: String,
    pub unit_hint: Option<String>,
}

pub struct SpiceModelDto {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub parameters: Vec<SpiceModelParameterDto>,
    pub warnings: Vec<String>,
}

pub struct SpiceSubcircuitDto {
    pub id: String,
    pub name: String,
    pub pins: Vec<String>,
    pub detected_kind: String,
    pub parameters: Vec<SpiceModelParameterDto>,
    pub warnings: Vec<String>,
}

pub struct SpiceImportReportDto {
    pub status: String,
    pub models: Vec<SpiceModelDto>,
    pub subcircuits: Vec<SpiceSubcircuitDto>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}
```

### 12.2. Touchstone DTO

```rust
pub struct TouchstoneImportRequestDto {
    pub source_name: Option<String>,
    pub content: String,
}

pub struct TouchstoneSummaryDto {
    pub id: String,
    pub name: String,
    pub port_count: usize,
    pub point_count: usize,
    pub start_frequency_hz: Option<f64>,
    pub stop_frequency_hz: Option<f64>,
    pub parameter_format: String,
    pub reference_impedance_ohm: f64,
}

pub struct TouchstoneImportReportDto {
    pub status: String,
    pub summary: Option<TouchstoneSummaryDto>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}
```

### 12.3. Mapping / attach DTO

```rust
pub struct SpicePinMappingEntryDto {
    pub model_pin: String,
    pub component_pin: String,
    pub role_hint: Option<String>,
}

pub struct SpicePinMappingRequestDto {
    pub model_id: String,
    pub component_definition_id: String,
    pub mappings: Vec<SpicePinMappingEntryDto>,
}

pub struct SpicePinMappingValidationReportDto {
    pub valid: bool,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

pub struct AttachImportedModelRequestDto {
    pub model_id: String,
    pub component_definition_id: String,
    pub pin_mapping: Option<SpicePinMappingRequestDto>,
}
```

---

## 13. API facade

В `engine/api/src/facade.rs` добавить методы:

```rust
pub fn import_spice_model(
    &mut self,
    request: SpiceImportRequestDto,
) -> Result<SpiceImportReportDto, ApiError>;

pub fn import_touchstone_model(
    &mut self,
    request: TouchstoneImportRequestDto,
) -> Result<TouchstoneImportReportDto, ApiError>;

pub fn list_imported_models(
    &self,
) -> Result<Vec<ImportedModelSummaryDto>, ApiError>;

pub fn get_imported_model(
    &self,
    model_id: String,
) -> Result<ImportedModelDetailsDto, ApiError>;

pub fn validate_spice_pin_mapping(
    &self,
    request: SpicePinMappingRequestDto,
) -> Result<SpicePinMappingValidationReportDto, ApiError>;

pub fn attach_imported_model_to_component(
    &mut self,
    request: AttachImportedModelRequestDto,
) -> Result<ComponentDetailsDto, ApiError>;
```

Если есть existing Component Library DTO, использовать его вместо создания несовместимого `ComponentDetailsDto`.

---

## 14. Tauri commands

В:

```text
apps/desktop-tauri/src-tauri/src/lib.rs
```

добавить commands:

```text
import_spice_model
import_touchstone_model
list_imported_models
get_imported_model
validate_spice_pin_mapping
attach_imported_model_to_component
```

Добавить в `tauri::generate_handler!`.

Если проект использует Tauri permissions/capabilities, обновить:

```text
apps/desktop-tauri/src-tauri/capabilities/default.json
```

или актуальный файл permissions.

Правило:

```text
- команды принимают DTO/string content;
- для v1.9 можно не делать file dialog plugin;
- UI может передать content из textarea или путь, если текущая архитектура уже имеет безопасное чтение файла backend-side.
```

---

## 15. Frontend types/API/store

### 15.1. Types

В:

```text
apps/desktop-tauri/src/types/index.ts
```

добавить типы:

```text
SpiceImportRequestDto
SpiceModelParameterDto
SpiceModelDto
SpiceSubcircuitDto
SpiceImportReportDto
TouchstoneImportRequestDto
TouchstoneSummaryDto
TouchstoneImportReportDto
ImportedModelSummaryDto
ImportedModelDetailsDto
SpicePinMappingEntryDto
SpicePinMappingRequestDto
SpicePinMappingValidationReportDto
AttachImportedModelRequestDto
```

### 15.2. API

В:

```text
apps/desktop-tauri/src/api/index.ts
```

добавить методы:

```text
importSpiceModel(request)
importTouchstoneModel(request)
listImportedModels()
getImportedModel(modelId)
validateSpicePinMapping(request)
attachImportedModelToComponent(request)
```

### 15.3. Store

В Zustand store добавить:

```text
importedModels
selectedImportedModel
lastSpiceImportReport
lastTouchstoneImportReport
pinMappingValidation
```

И setters/actions:

```text
setImportedModels
setSelectedImportedModel
setLastSpiceImportReport
setLastTouchstoneImportReport
setPinMappingValidation
clearModelImportState
```

Store не должен содержать parser logic.

---

## 16. Frontend UI: Import Models screen

Создать экран:

```text
apps/desktop-tauri/src/screens/ModelImportScreen.tsx
```

или вкладку в Component Library, если текущая навигация так устроена.

Лучше отдельный экран:

```text
Model Import / Import Models
```

### 16.1. Layout

Экран должен иметь:

```text
- header: Model Import
- tabs:
  - SPICE
  - Touchstone
  - Imported Models
- left/import panel
- right/details panel
```

### 16.2. SPICE tab

Поля:

```text
- source name input, e.g. "1n4148.lib"
- textarea для SPICE content
- button: Import SPICE
```

После импорта:

```text
- status
- warnings/errors
- detected .model list
- detected .subckt list
```

Для выбранной модели:

```text
- name
- kind
- params table
- pins for subckt
- raw/source preview if safe and short
```

### 16.3. Pin mapping UI

Для `.subckt`:

```text
- model pins table
- component pin select/input
- validate mapping button
- attach to component button
```

Минимально:

```text
- выбрать ComponentDefinition из built-in library;
- показать mapping rows;
- отправить validate;
- отправить attach.
```

Если получить component list из текущего API сложно:

```text
- использовать существующий Component Library API;
- не дублировать библиотеку компонентов во frontend.
```

### 16.4. Touchstone tab

Поля:

```text
- source name input, e.g. "filter.s2p"
- textarea для Touchstone content
- button: Import Touchstone
```

После импорта:

```text
- status
- point count
- port count
- frequency range
- parameter format
- reference impedance
- warnings/errors
```

Пока не обязательно рисовать S11/S21 graph. Можно добавить placeholder:

```text
S-parameter plotting planned after v1.9.
```

### 16.5. Imported Models tab

Показать:

```text
- imported models list
- kind
- source
- attached component if any
- status
```

---

## 17. Интеграция в Workbench/navigation

Добавить пункт в UI:

```text
Import Models
```

или:

```text
Component Library → Import Models
```

Требование:

```text
- пользователь должен иметь явный способ открыть новый импорт;
- не прятать функциональность только в tests/API.
```

---

## 18. Project package / storage integration

Использовать foundation из `.circuit`:

```text
models/spice/
models/touchstone/
components.json
```

Для v1.9 минимально:

```text
- импортированная модель сохраняется в runtime state;
- attach к ComponentDefinition отражается в component data;
- документация описывает, что file-copy persistence в .circuit может быть limited/foundation.
```

Лучше, если возможно:

```text
- при save_project_package включить metadata imported models в components.json или models index;
- не удалять пользовательские файлы в models/spice и models/touchstone;
- не ломать старые project packages.
```

Если полная package persistence слишком большая:

```text
- оставить TODO в docs/import/SPICE_TOUCHSTONE_IMPORT_FOUNDATION.md;
- добавить tests только на runtime/service/API attach;
- не притворяться, что persistence полностью готова.
```

---

## 19. Tests: Rust

### 19.1. Core/parser tests

Добавить:

```text
engine/adapters/tests/spice_model_parser_tests.rs
engine/adapters/tests/touchstone_parser_tests.rs
```

SPICE tests:

```text
1. parses_diode_model
2. parses_bjt_model
3. parses_mosfet_model
4. parses_multiple_models_from_lib
5. parses_subckt_name_and_pins
6. parses_subckt_body_until_ends
7. supports_line_continuation
8. ignores_comment_lines
9. unknown_model_type_returns_warning
10. unsupported_directives_return_warnings
11. empty_spice_file_returns_controlled_error_or_empty_report
12. malformed_model_does_not_panic
```

Touchstone tests:

```text
1. parses_s1p_ri
2. parses_s1p_ma
3. parses_s1p_db
4. parses_s2p_ri
5. parses_s2p_ma
6. parses_frequency_units_hz_khz_mhz_ghz
7. parses_reference_impedance
8. ignores_comments
9. missing_option_line_uses_defaults_with_warning
10. wrong_column_count_returns_error
11. unsupported_format_returns_error
12. empty_touchstone_returns_error
```

### 19.2. Application tests

Добавить:

```text
engine/application/tests/model_import_service_tests.rs
```

Тесты:

```text
1. import_spice_from_text_stores_detected_models
2. import_spice_subckt_stores_pins
3. list_imported_models_returns_summaries
4. get_imported_model_returns_details
5. validate_pin_mapping_valid_case
6. validate_pin_mapping_missing_pin_reports_error
7. attach_spice_model_to_component_adds_simulation_model
8. import_touchstone_from_text_stores_network_summary
9. touchstone_invalid_input_returns_controlled_error
```

### 19.3. API tests

Добавить:

```text
engine/api/tests/model_import_api_tests.rs
```

Тесты:

```text
1. api_import_spice_model_returns_report
2. api_import_touchstone_model_returns_summary
3. api_list_imported_models_after_import
4. api_validate_spice_pin_mapping
5. api_attach_imported_model_to_component
6. api_import_invalid_spice_returns_errors_not_panic
7. api_import_invalid_touchstone_returns_errors_not_panic
```

### 19.4. Boundary tests

Проверить, что:

```text
- hotsas_core не зависит от adapters/api/ui;
- hotsas_api tests не зависят напрямую от concrete adapters, если в проекте есть dependency boundary tests;
- если нужны fake parsers, держать их inline в test или в test support module.
```

---

## 20. Tests: Frontend

Добавить:

```text
apps/desktop-tauri/src/screens/__tests__/ModelImportScreen.test.tsx
```

или в актуальную test-структуру.

Тесты:

```text
1. renders_model_import_screen
2. renders_spice_and_touchstone_tabs
3. importing_spice_calls_backend_importSpiceModel
4. displays_detected_spice_models
5. displays_spice_subckt_pins
6. validate_pin_mapping_calls_backend
7. attach_model_calls_backend
8. importing_touchstone_calls_backend_importTouchstoneModel
9. displays_touchstone_summary
10. displays_errors_without_crashing
```

Если используется ECharts или другие browser APIs, мокать только то, что реально нужно.

---

## 21. Manual smoke test v1.9

Агент должен выполнить или подготовить инструкцию и отметить результат в verification log:

```text
[OK/FAIL] App starts
[OK/FAIL] Import Models screen opens
[OK/FAIL] SPICE tab is visible
[OK/FAIL] Paste diode .model and import
[OK/FAIL] Detected model appears
[OK/FAIL] Paste .subckt and import
[OK/FAIL] Subckt pins appear
[OK/FAIL] Pin mapping UI is visible
[OK/FAIL] Validate pin mapping works
[OK/FAIL] Attach model to ComponentDefinition works
[OK/FAIL] Touchstone tab is visible
[OK/FAIL] Paste .s1p and import
[OK/FAIL] Touchstone summary appears
[OK/FAIL] Paste .s2p and import
[OK/FAIL] Port count = 2 and frequency range appears
[OK/FAIL] Component Library still opens
[OK/FAIL] Simulation screen still opens
[OK/FAIL] Export Center still opens
```

---

## 22. Documentation

Создать:

```text
docs/import/SPICE_TOUCHSTONE_IMPORT_FOUNDATION.md
```

Если папки нет — создать:

```text
docs/import/
```

Документ должен содержать:

```markdown
# HotSAS Studio v1.9 — SPICE/Touchstone Import Foundation

## Purpose

## User workflow

## Supported SPICE import

- .model
- .subckt
- .lib/.mod/.cir as source containers

## Unsupported SPICE features in v1.9

## Pin mapping

## Touchstone import

- .s1p
- .s2p
- RI/MA/DB
- frequency units

## Storage model

## Relationship with Component Library

## Relationship with ngspice Adapter

## Limitations

## Future work
```

Обновить:

```text
README.md
docs/testing/TESTING.md
docs/testing/latest_verification_log.md
docs/testing/verification_logs/v1.9_spice_touchstone_import_foundation.md
```

README после завершения должен показывать:

```text
Current roadmap stage: v2.0 next
Completed:
- v1.9 — SPICE/Touchstone Import Foundation
```

---

## 23. Verification log requirement

Агент обязан создать отдельный файл:

```text
docs/testing/verification_logs/v1.9_spice_touchstone_import_foundation.md
```

Файл должен содержать не только “PASS”, но и:

```text
- git preflight output summary;
- implementation commit hash;
- verification log update commit hash, если отдельный;
- exact cargo test summary;
- exact frontend test summary;
- focused model import tests summary;
- manual smoke test table with [OK]/[FAIL]/[NOT RUN];
- known limitations;
- final readiness status.
```

Не писать “готово”, если:

```text
- cargo test не запускался;
- npm test не запускался;
- verification log не создан;
- README не обновлён;
- push не выполнен.
```

---

## 24. Обязательные проверки

Выполнить:

```bash
cd "D:\Документы\vscode\HotSAS Studio\engine"
cargo fmt --check
cargo test
cargo test spice
cargo test touchstone
cargo test model_import
```

Выполнить:

```bash
cd "D:\Документы\vscode\HotSAS Studio\apps\desktop-tauri"
npm.cmd run format:check
npm.cmd run typecheck
npm.cmd run test
npm.cmd run build
```

Желательно:

```bash
npm.cmd run tauri:build
```

Если `tauri:build` не выполнялся:

```text
- указать NOT RUN;
- объяснить причину;
- не писать, что release build подтверждён.
```

---

## 25. Self-check агента перед финальным ответом

Перед финальным ответом агент должен сам пройти чек-лист:

```text
[ ] Core model_import models added
[ ] SPICE parser handles .model
[ ] SPICE parser handles .subckt
[ ] SPICE parser handles line continuations
[ ] SPICE parser returns controlled warnings/errors
[ ] Touchstone parser handles .s1p
[ ] Touchstone parser handles .s2p
[ ] Touchstone parser handles RI/MA/DB
[ ] ModelImportService added
[ ] AppServices exposes model import service
[ ] API DTO/facade methods added
[ ] Tauri commands added
[ ] Frontend types/API/store added
[ ] ModelImportScreen or tab added
[ ] Pin mapping UI added
[ ] Attach to ComponentDefinition flow added
[ ] Rust tests added
[ ] Frontend tests added
[ ] Docs created
[ ] TESTING updated
[ ] README updated to v2.0 next
[ ] latest verification log updated
[ ] v1.9 verification log file created
[ ] cargo fmt --check PASS
[ ] cargo test PASS
[ ] npm format:check PASS
[ ] npm typecheck PASS
[ ] npm test PASS
[ ] npm build PASS
[ ] Git commit created
[ ] Git push completed
```

---

## 26. Git requirements

После успешных проверок:

```bash
git status --short
git add ...
git commit -m "v1.9: SPICE and Touchstone import foundation"
git push origin main
```

Если после commit нужно обновить verification log с commit hash:

```bash
git add docs/testing/latest_verification_log.md docs/testing/verification_logs/v1.9_spice_touchstone_import_foundation.md
git commit -m "docs(v1.9): update verification log with commit hash"
git push origin main
```

В verification log указывать:

```text
Implementation commit: <hash>
Verification log update commit: <hash or same commit>
```

---

## 27. Критерии приёмки v1.9

v1.9 считается принятой только если:

```text
1. SPICE .model import работает через backend.
2. SPICE .subckt import работает через backend.
3. Touchstone .s1p import работает через backend.
4. Touchstone .s2p import работает через backend.
5. UI показывает Import Models screen/tab.
6. UI не парсит SPICE/Touchstone самостоятельно.
7. Pin mapping foundation есть.
8. Attach imported model to ComponentDefinition работает.
9. Rust tests PASS.
10. Frontend tests PASS.
11. README обновлён до v2.0 next.
12. docs/import/SPICE_TOUCHSTONE_IMPORT_FOUNDATION.md создан.
13. docs/testing/TESTING.md обновлён.
14. docs/testing/latest_verification_log.md обновлён.
15. docs/testing/verification_logs/v1.9_spice_touchstone_import_foundation.md создан.
16. Коммит и push выполнены.
```

---

## 28. Итоговый ожидаемый статус после v1.9

После этапа программа должна быть в состоянии:

```text
HotSAS Studio уже умеет:
- создавать/редактировать базовую схему;
- считать формулы;
- использовать инженерный notebook;
- работать с component library;
- анализировать selected region;
- экспортировать артефакты через Export Center;
- запускать mock/ngspice simulation flow;
- импортировать SPICE model/subckt foundation;
- импортировать Touchstone S-parameter foundation;
- прикреплять imported simulation models к компонентам.
```

Следующий этап после v1.9:

```text
v2.0 — Product Beta
```

v2.0 должен быть не “ещё одна фича”, а stabilization/productization milestone.
