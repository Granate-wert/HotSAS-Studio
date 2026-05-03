# HotSAS Studio v1.8 — ngspice Adapter v1

## 0. Назначение

Это техническое задание для этапа:

```text
v1.8 — ngspice Adapter v1
```

Цель этапа — добавить первую реальную интеграцию с **ngspice** через backend-архитектуру HotSAS Studio, не ломая существующий mock simulation flow, Export Center, Selected Region Analysis, Engineering Notebook, Component Library и `.circuit` storage.

Репозиторий:

```text
https://github.com/Granate-wert/HotSAS-Studio
```

---

## 1. Текущий статус перед v1.8

Проект уже прошёл:

```text
v1.0   — Initial RC low-pass vertical slice
v1.0.1 — Architecture Hardening
v1.1.1 — Formatting + Build/Test Infrastructure
v1.1.2 — Backend Test Expansion
v1.1.3 — FormulaPackLoader + FormulaRegistry
v1.1.4 — Generic Formula Engine Completion
v1.1.5 — Exact E-Series Tables
v1.2   — Project Package Storage .circuit
v1.3   — Schematic Editor Foundations
v1.4   — Engineering Notebook / Calculator Foundations
v1.5   — Component Library Foundation
v1.6   — Selected Region Analysis Foundation
v1.7   — Export Center v1
```

Текущий принятый stage:

```text
v1.7 — Export Center v1
```

Следующий stage:

```text
v1.8 — ngspice Adapter v1
```

Перед началом работ агент обязан проверить, что README, `docs/testing/latest_verification_log.md` и `docs/testing/verification_logs/v1.7_export_center_v1.md` подтверждают закрытую v1.7.

---

## 2. Что изменится для пользователя

После v1.8 пользователь должен получить первый рабочий SPICE-backed simulation workflow.

Пользователь сможет:

```text
1. Открыть Simulation Results / Simulation screen.
2. Увидеть статус simulation engine:
   - Mock engine available;
   - ngspice available / not found / failed.
3. Нажать Check ngspice.
4. Если ngspice установлен:
   - увидеть найденный путь к бинарнику;
   - увидеть версию или краткий output проверки;
   - увидеть статус Available.
5. Если ngspice не установлен:
   - увидеть понятное сообщение;
   - продолжить пользоваться mock simulation;
   - приложение не должно падать.
6. Запустить real operating point simulation для простой схемы.
7. Запустить real AC sweep для RC low-pass demo.
8. Запустить basic transient simulation для простой RC-схемы.
9. Увидеть результат:
   - status;
   - graph series;
   - measurements, если доступны;
   - stdout/stderr summary;
   - path к raw/output-файлам, если они записаны.
10. Сравнить mock/real simulation на уровне UI.
11. Экспортировать результаты через существующий Export Center, если результат доступен в unified SimulationResult.
```

Пользователь НЕ должен ожидать в v1.8:

```text
- полноценного SPICE model import;
- полноценного .lib/.subckt/.mod manager;
- полноценного parser для всех SPICE dialects;
- advanced convergence settings UI;
- waveform cursors;
- parameter sweep UI;
- symbolic analysis для произвольных схем;
- PCB/EDA workflow сверх уже существующих placeholders;
- libngspice FFI.
```

---

## 3. Главная цель v1.8

Добавить `NgspiceSimulationAdapter v1`, который реализует `SimulationEnginePort` и умеет запускать ngspice как внешний процесс для простых схем:

```text
- operating point;
- AC sweep;
- basic transient.
```

Правильный flow:

```text
React UI
→ Tauri command
→ hotsas_api facade
→ SimulationService / NgspiceSimulationService
→ SimulationEnginePort
→ NgspiceSimulationAdapter
→ external ngspice process
→ parser
→ SimulationResultDto
→ UI graph/result cards
```

Жёсткое правило:

```text
UI не запускает ngspice напрямую.
React не формирует SPICE netlist.
React не парсит raw output.
React только вызывает Tauri commands и отображает DTO.
```

---

## 4. Scope

### Запрещено

```text
- не делать libngspice FFI;
- не подключать C API ngspice;
- не писать custom SPICE solver;
- не делать полноценный parser бинарного .raw в v1.8;
- не делать полноценный parser всех вариантов ASCII RAW;
- не делать SPICE model import .lib/.subckt/.mod;
- не делать Touchstone import;
- не делать DC-DC calculators;
- не делать PCB editor;
- не делать KiCad/Altium export сверх уже существующих placeholders;
- не делать param sweep UI;
- не делать waveform cursors;
- не делать Monte Carlo;
- не делать optimization engine;
- не ломать mock simulation;
- не ломать Export Center;
- не ломать Selected Region Analysis;
- не ломать Engineering Notebook;
- не ломать Component Library;
- не переносить расчёты/парсинг во frontend.
```

### Разрешено

```text
- добавить core-модели для ngspice execution status;
- расширить SimulationProfile/SimulationResult, если нужно;
- расширить SimulationEnginePort;
- добавить NgspiceSimulationAdapter;
- добавить process runner abstraction для тестируемости;
- добавить parser для ограниченного ASCII/CSV-like output;
- добавить API DTO и facade methods;
- добавить Tauri commands;
- обновить Simulation Results UI;
- добавить frontend tests;
- добавить Rust tests;
- добавить docs/simulation/NGSPICE_ADAPTER_V1.md;
- обновить README/TESTING/latest verification log;
- создать отдельный verification log файл;
- commit + push.
```

---

## 5. Preflight

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
- Не удалять пользовательские untracked-файлы.
- Если git status не clean — зафиксировать это в verification log.
- Если есть неожиданные изменения — не затирать их.
```

Проверить базу перед v1.8:

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
README.md
docs/export/EXPORT_CENTER_V1.md
docs/testing/latest_verification_log.md
docs/testing/verification_logs/v1.7_export_center_v1.md
```

---

## 6. Dependency policy для ngspice

Обычный `cargo test` НЕ должен падать, если ngspice не установлен.

Правила:

```text
- Unit tests parser/service/adapter fake runner должны проходить без ngspice.
- Real ngspice integration tests должны быть opt-in.
- Если ngspice отсутствует, runtime должен возвращать Unavailable, а не panic.
```

Real integration tests запускать только при:

```text
HOTSAS_RUN_NGSPICE_INTEGRATION=1
```

Если env var не задана, такие тесты должны быть skipped/ignored и общий `cargo test` должен проходить.

---

## 7. Core models

Проверить существующие simulation-модели. Если отдельного модуля нет, создать:

```text
engine/core/src/ngspice.rs
```

и подключить в:

```text
engine/core/src/lib.rs
```

### 7.1. NgspiceAvailability

```rust
pub struct NgspiceAvailability {
    pub available: bool,
    pub executable_path: Option<String>,
    pub version: Option<String>,
    pub message: Option<String>,
    pub warnings: Vec<String>,
}
```

### 7.2. NgspiceRunStatus

```rust
pub enum NgspiceRunStatus {
    Success,
    Failed,
    TimedOut,
    Unavailable,
}
```

### 7.3. NgspiceRunMetadata

```rust
pub struct NgspiceRunMetadata {
    pub run_id: String,
    pub engine: String,
    pub command: Vec<String>,
    pub working_directory: String,
    pub netlist_path: String,
    pub stdout_path: Option<String>,
    pub stderr_path: Option<String>,
    pub raw_output_path: Option<String>,
    pub parsed_output_path: Option<String>,
    pub exit_code: Option<i32>,
    pub elapsed_ms: Option<u64>,
}
```

### 7.4. NgspiceSimulationRequest

```rust
pub struct NgspiceSimulationRequest {
    pub project_id: String,
    pub profile_id: String,
    pub netlist: String,
    pub analysis_kind: SimulationAnalysisKind,
    pub output_variables: Vec<String>,
    pub timeout_ms: u64,
}
```

### 7.5. SimulationAnalysisKind

Если такого enum ещё нет, добавить:

```rust
pub enum SimulationAnalysisKind {
    OperatingPoint,
    DcSweep,
    AcSweep,
    Transient,
}
```

### 7.6. SimulationResult extension

Расширить существующий `SimulationResult`, если нужно:

```text
- engine: mock/ngspice;
- status;
- graph_series;
- measurements;
- warnings;
- errors;
- raw_data_path;
- metadata.
```

Не ломать существующие DTO и Export Center.

---

## 8. Ports: SimulationEnginePort

Проверить существующий `SimulationEnginePort` в:

```text
engine/ports/src/lib.rs
```

Он должен поддерживать или быть расширен до методов:

```rust
pub trait SimulationEnginePort: Send + Sync {
    fn engine_name(&self) -> &str;

    fn check_availability(&self) -> Result<NgspiceAvailability, PortError>;

    fn run_operating_point(
        &self,
        request: SimulationRunRequest,
    ) -> Result<SimulationResult, PortError>;

    fn run_ac_sweep(
        &self,
        request: SimulationRunRequest,
    ) -> Result<SimulationResult, PortError>;

    fn run_transient(
        &self,
        request: SimulationRunRequest,
    ) -> Result<SimulationResult, PortError>;

    fn stop_simulation(&self, run_id: String) -> Result<(), PortError>;

    fn get_result(&self, run_id: String) -> Result<Option<SimulationResult>, PortError>;
}
```

Если текущий trait отличается, не ломать архитектуру массово. Допустимо:

```text
- добавить default methods;
- добавить отдельный NgspiceSimulationPort;
- добавить newtype adapter;
- сохранить старый MockSimulationAdapter.
```

Главное: Application работает через port. UI/Tauri не знает деталей ngspice process.

---

## 9. Adapters: NgspiceSimulationAdapter

Создать модуль:

```text
engine/adapters/src/ngspice.rs
```

или структуру:

```text
engine/adapters/src/ngspice/
├── mod.rs
├── adapter.rs
├── runner.rs
├── parser.rs
└── netlist_control.rs
```

Рекомендуемые элементы:

```text
NgspiceSimulationAdapter
NgspiceBinaryResolver
NgspiceProcessRunner
NgspiceOutputParser
NgspiceControlBlockBuilder
NgspiceTempWorkspace
```

### 9.1. NgspiceBinaryResolver

Задача:

```text
1. Проверить env HOTSAS_NGSPICE_PATH.
2. Если задан — проверить файл.
3. Если не задан — искать ngspice в PATH.
4. На Windows также допустимо искать ngspice.exe.
5. Не падать при отсутствии.
6. Вернуть NgspiceAvailability.
```

Проверка версии:

```text
ngspice --version
```

Если команда не поддержала `--version` или вернула нестандартный вывод — не падать, а записать warning.

### 9.2. NgspiceProcessRunner

Требования:

```text
- запускать через std::process::Command;
- не использовать shell string;
- arguments передавать отдельно;
- capture stdout/stderr;
- timeout;
- working directory в temp/results folder;
- сохранять stdout/stderr в файлы, если включено;
- возвращать exit code и elapsed_ms.
```

Безопасность:

```text
- не подставлять пользовательский путь в shell;
- не выполнять arbitrary user commands;
- netlist file path должен быть создан backend-ом;
- output paths должны быть внутри controlled workspace.
```

### 9.3. Batch mode

Для v1.8 использовать batch mode:

```text
ngspice -b -o stdout.log circuit.cir
```

Допустимый вариант для raw:

```text
ngspice -b -r output.raw circuit.cir
```

Но в v1.8 лучше опираться на backend-generated `.control` + text/CSV-like output, чтобы не писать полноценный binary raw parser.

### 9.4. Control block generation

#### Operating point

```spice
.control
op
print all
.endc
.end
```

или более ограниченно:

```spice
.control
op
print v(net_in) v(net_out)
.endc
.end
```

#### AC sweep

```spice
.ac dec 100 10 1Meg
.control
set filetype=ascii
run
wrdata ac_output.csv frequency v(net_out)
.endc
.end
```

#### Transient

```spice
.tran 10u 10m
.control
set filetype=ascii
run
wrdata tran_output.csv time v(net_in) v(net_out)
.endc
.end
```

### 9.5. Parser v1

Не делать full raw parser.

Поддержать:

```text
- parsing stdout/stderr status;
- parsing simple numeric rows from wrdata-like files;
- parsing columns:
  - frequency;
  - time;
  - v(net);
  - mag/phase later if available.
```

Требования:

```text
- parser не должен panic на пустом файле;
- parser должен возвращать controlled error;
- NaN/Infinity/invalid number → warning or error;
- unknown columns сохранять в metadata/warnings.
```

### 9.6. Result mapping

Ngspice output должен быть преобразован в существующий `SimulationResult`:

```text
SimulationResult:
- id/run_id;
- profile_id;
- status;
- graph_series;
- measurements;
- warnings;
- errors;
- raw_data_path;
- metadata.
```

Для AC sweep:

```text
x = frequency Hz
series names:
- V(out)
- V(in), if available
```

Для transient:

```text
x = time s
series names:
- V(in)
- V(out)
```

Operating point:

```text
measurements:
- node voltage;
- source current if available;
- warnings if not parsed.
```

---

## 10. Netlist generation requirements

Проверить существующий SPICE netlist exporter.

В v1.8 он должен создавать ngspice-compatible netlist для простых схем:

```text
- resistors;
- capacitors;
- inductors if already supported;
- voltage source;
- ground;
- RC low-pass demo.
```

### 10.1. RC low-pass AC smoke netlist

```spice
* HotSAS RC Low-Pass
V1 net_in 0 AC 1
R1 net_in net_out 10k
C1 net_out 0 100n
.ac dec 100 10 1Meg
.control
set filetype=ascii
run
wrdata ac_output.csv frequency v(net_out)
.endc
.end
```

### 10.2. Transient smoke netlist

```spice
* HotSAS RC Low-Pass transient
V1 net_in 0 PULSE(0 1 0 1u 1u 1m 2m)
R1 net_in net_out 10k
C1 net_out 0 100n
.tran 10u 10m
.control
set filetype=ascii
run
wrdata tran_output.csv time v(net_in) v(net_out)
.endc
.end
```

Netlist generation for simulation must not accidentally mutate current project state unless explicitly required.

---

## 11. Application layer

Добавить или расширить service:

```text
engine/application/src/services/simulation.rs
```

или создать:

```text
engine/application/src/services/ngspice_simulation.rs
```

Methods:

```rust
pub fn check_ngspice_availability(&self) -> Result<NgspiceAvailability, ApplicationError>;

pub fn run_operating_point_for_current_project(
    &mut self,
    request: SimulationRunRequest,
) -> Result<SimulationResult, ApplicationError>;

pub fn run_ac_sweep_for_current_project(
    &mut self,
    request: SimulationRunRequest,
) -> Result<SimulationResult, ApplicationError>;

pub fn run_transient_for_current_project(
    &mut self,
    request: SimulationRunRequest,
) -> Result<SimulationResult, ApplicationError>;

pub fn list_simulation_runs(&self) -> Vec<SimulationResult>;
```

### 11.1. Engine selection policy

Добавить:

```text
SimulationEngineChoice:
- Mock
- Ngspice
- Auto
```

Правила:

```text
Mock:
- всегда работает через текущий MockSimulationAdapter.

Ngspice:
- если ngspice unavailable → controlled error.

Auto:
- если ngspice available → NgspiceSimulationAdapter;
- иначе fallback to MockSimulationAdapter с warning.
```

### 11.2. State/history

Сохранять последние simulation results:

```text
- last_simulation_result;
- simulation_history.
```

Если `.circuit` package уже умеет `results/index.json`, можно подготовить output paths, но не делать большой storage refactor.

---

## 12. API DTO

В `engine/api/src/dto.rs` добавить или расширить DTO:

```rust
pub struct NgspiceAvailabilityDto {
    pub available: bool,
    pub executable_path: Option<String>,
    pub version: Option<String>,
    pub message: Option<String>,
    pub warnings: Vec<String>,
}

pub struct SimulationRunRequestDto {
    pub engine: String,          // "mock" | "ngspice" | "auto"
    pub analysis_kind: String,   // "operating_point" | "ac_sweep" | "transient"
    pub profile_id: Option<String>,
    pub output_variables: Vec<String>,
    pub timeout_ms: Option<u64>,
}

pub struct SimulationRunMetadataDto {
    pub run_id: String,
    pub engine: String,
    pub status: String,
    pub netlist_path: Option<String>,
    pub stdout_path: Option<String>,
    pub stderr_path: Option<String>,
    pub raw_output_path: Option<String>,
    pub parsed_output_path: Option<String>,
    pub exit_code: Option<i32>,
    pub elapsed_ms: Option<u64>,
}
```

Если `SimulationResultDto` уже существует, не дублировать. Расширить существующий тип аккуратно.

---

## 13. API facade

В `engine/api/src/facade.rs` добавить:

```rust
pub fn check_ngspice_availability(
    &self,
) -> Result<NgspiceAvailabilityDto, ApiError>;

pub fn run_simulation(
    &mut self,
    request: SimulationRunRequestDto,
) -> Result<SimulationResultDto, ApiError>;

pub fn simulation_history(
    &self,
) -> Result<Vec<SimulationResultDto>, ApiError>;
```

Допустимо добавить convenience methods:

```rust
pub fn run_operating_point(...)
pub fn run_ac_sweep(...)
pub fn run_transient(...)
```

Но UI лучше использовать unified `run_simulation`.

---

## 14. Tauri commands

В:

```text
apps/desktop-tauri/src-tauri/src/lib.rs
```

добавить:

```rust
#[tauri::command]
async fn check_ngspice_availability(
    state: tauri::State<'_, AppState>,
) -> Result<NgspiceAvailabilityDto, String>;

#[tauri::command]
async fn run_simulation(
    state: tauri::State<'_, AppState>,
    request: SimulationRunRequestDto,
) -> Result<SimulationResultDto, String>;

#[tauri::command]
async fn simulation_history(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<SimulationResultDto>, String>;
```

Добавить их в `generate_handler!`.

Проверить Tauri permissions/capabilities, если проект использует Tauri v2 capabilities.

---

## 15. Frontend types/API/store

### Types

В `apps/desktop-tauri/src/types/index.ts` или актуальном файле типов добавить:

```ts
export interface NgspiceAvailabilityDto {
  available: boolean;
  executablePath?: string | null;
  version?: string | null;
  message?: string | null;
  warnings: string[];
}

export interface SimulationRunRequestDto {
  engine: "mock" | "ngspice" | "auto";
  analysisKind: "operating_point" | "ac_sweep" | "transient";
  profileId?: string | null;
  outputVariables: string[];
  timeoutMs?: number | null;
}
```

Подстроить casing под текущий проект.

### API methods

В `apps/desktop-tauri/src/api/index.ts` добавить:

```ts
checkNgspiceAvailability(): Promise<NgspiceAvailabilityDto>;
runSimulation(request: SimulationRunRequestDto): Promise<SimulationResultDto>;
simulationHistory(): Promise<SimulationResultDto[]>;
```

### Store

В Zustand store добавить:

```text
ngspiceAvailability
selectedSimulationEngine
lastSimulationResult
simulationHistory
isSimulationRunning
simulationError
```

Setters:

```text
setNgspiceAvailability
setSelectedSimulationEngine
setLastSimulationResult
setSimulationHistory
setIsSimulationRunning
setSimulationError
```

---

## 16. Frontend UI: Simulation Results screen

Обновить:

```text
apps/desktop-tauri/src/screens/SimulationResultsScreen.tsx
```

или существующий экран результатов симуляции.

### 16.1. Engine Status Card

Добавить блок:

```text
Simulation Engine

[Check ngspice]

Status:
- ngspice: Available / Not found / Error
- path
- version
- warnings
```

### 16.2. Engine selector

```text
Engine:
[Auto] [Mock] [ngspice]
```

Правила:

```text
- Mock доступен всегда.
- ngspice disabled или warning, если unavailable.
- Auto показывает fallback warning, если ngspice unavailable.
```

### 16.3. Run buttons

```text
[Run Operating Point]
[Run AC Sweep]
[Run Transient]
```

Для v1.8 можно ограничить:

```text
- AC sweep: RC low-pass demo / current simple schematic.
- Transient: RC low-pass demo / current simple schematic.
- Operating point: current simple schematic.
```

### 16.4. Results

Показать:

```text
- status;
- engine;
- warnings/errors;
- graph series table/list;
- chart, если уже есть chart component;
- metadata:
  - run id;
  - elapsed time;
  - exit code;
  - output paths.
```

### 16.5. Logs

Показать collapsed panel:

```text
Ngspice output summary
- stdout path
- stderr path
- first N lines of errors/warnings
```

Не выводить огромный raw output целиком без ограничения.

---

## 17. Integration with Export Center

v1.7 Export Center уже умеет CSV simulation data export.

После v1.8:

```text
- Export Center должен работать с real ngspice SimulationResult так же, как с mock result.
- Если lastSimulationResult получен от ngspice, CSV export должен брать graph_series из него.
- Если результата нет — Export Center должен показывать friendly error, а не crash.
```

Не переписывать весь Export Center.

---

## 18. Integration with Selected Region Analysis

Не делать полноценный numeric SPICE selected-region analysis в v1.8.

Но подготовить совместимость:

```text
- SelectedRegionAnalysisResult уже содержит SPICE netlist fragment.
- v1.8 может добавить helper: convert region netlist fragment to SimulationRunRequest.
- UI может пока не иметь отдельной кнопки Run region in ngspice.
```

Если делать кнопку — только как minimal optional feature, без расширения scope.

---

## 19. Error handling

### ngspice отсутствует

```text
- check_ngspice_availability returns available=false.
- UI shows ngspice not found.
- Auto engine can fallback to mock with warning.
- Direct ngspice run returns controlled error.
- No panic.
```

### ngspice exit code != 0

```text
- SimulationResult.status = Failed.
- errors includes stderr summary.
- stdout/stderr paths saved if available.
- UI shows error card.
```

### timeout

```text
- process killed or marked timed out.
- SimulationResult.status = TimedOut.
- error explains timeout_ms.
```

### parser failure

```text
- process success but parser failed → controlled error/warning.
- preserve raw output paths.
- UI says ngspice ran but output could not be parsed.
```

---

## 20. Tests

### Rust tests

Добавить/адаптировать:

```text
engine/adapters/tests/ngspice_binary_resolver_tests.rs
engine/adapters/tests/ngspice_parser_tests.rs
engine/application/tests/ngspice_simulation_service_tests.rs
engine/api/tests/ngspice_simulation_api_tests.rs
```

#### Binary resolver tests

```text
- returns unavailable when path missing;
- uses HOTSAS_NGSPICE_PATH when set;
- detects invalid path without panic;
- path lookup can be mocked.
```

#### Process runner tests

Использовать fake runner, не настоящий ngspice:

```text
- success exit code maps to success;
- non-zero exit code maps to failed;
- timeout maps to TimedOut;
- stdout/stderr captured.
```

#### Parser tests

Сделать fixtures:

```text
engine/adapters/tests/fixtures/ngspice/ac_output.csv
engine/adapters/tests/fixtures/ngspice/tran_output.csv
engine/adapters/tests/fixtures/ngspice/op_stdout.txt
engine/adapters/tests/fixtures/ngspice/error_stderr.txt
```

Проверить:

```text
- AC output parses frequency series;
- transient output parses time series;
- empty file returns error;
- invalid numeric rows return warning/error;
- parser does not panic.
```

#### Service tests

```text
- Auto falls back to Mock when ngspice unavailable.
- Direct Ngspice returns controlled unavailable error.
- Mock simulation still returns graph series.
- Successful fake ngspice run produces SimulationResult.
```

#### API tests

```text
- check_ngspice_availability returns DTO.
- run_simulation mock returns DTO.
- run_simulation ngspice unavailable returns controlled ApiError.
- simulation_history returns last runs.
```

### Optional real ngspice integration tests

Add ignored/opt-in tests:

```text
real_ngspice_availability_smoke_test
real_ngspice_runs_rc_low_pass_ac_sweep
real_ngspice_runs_basic_transient
```

They should only run when:

```text
HOTSAS_RUN_NGSPICE_INTEGRATION=1
```

### Frontend tests

Add tests:

```text
apps/desktop-tauri/src/screens/__tests__/SimulationResultsScreen.test.tsx
```

Check:

```text
- renders Simulation Results screen;
- Check ngspice button calls backend;
- unavailable ngspice message displayed;
- engine selector works;
- Run AC Sweep button calls backend.runSimulation;
- result card displays status and warnings;
- history displays completed run;
- errors are shown without crash.
```

---

## 21. Documentation

Создать:

```text
docs/simulation/NGSPICE_ADAPTER_V1.md
```

Документ должен объяснять:

```text
- что делает v1.8;
- какие analysis types поддержаны;
- как HotSAS ищет ngspice;
- HOTSAS_NGSPICE_PATH;
- что делать, если ngspice не найден;
- какие результаты парсятся;
- какие ограничения v1.8;
- чем Mock отличается от ngspice;
- почему UI не запускает ngspice напрямую;
- как включить optional integration tests;
- как v1.8 готовит v1.9 SPICE/Touchstone import.
```

Обновить:

```text
README.md
docs/testing/TESTING.md
docs/testing/latest_verification_log.md
docs/testing/verification_logs/
```

README после выполнения должен показывать:

```text
Current roadmap stage: v1.9 next

Completed:
- v1.8 — ngspice Adapter v1
```

---

## 22. Verification log requirement

Обязательно создать отдельный файл:

```text
docs/testing/verification_logs/v1.8_ngspice_adapter_v1.md
```

Обязательно обновить:

```text
docs/testing/latest_verification_log.md
```

Агент обязан предоставить этот файл пользователю для проверки здесь.

Минимальный состав verification log:

```text
- version/task;
- date;
- branch;
- commit before;
- commit after;
- git status before/after;
- OS/Rust/Cargo/Node/npm/ngspice environment;
- summary of changes;
- cargo fmt --check output;
- cargo test output;
- focused ngspice tests output;
- optional real ngspice integration status;
- npm format/typecheck/test/build output;
- tauri build status;
- manual UI smoke test;
- agent self-check;
- final status;
- next version.
```

---

## 23. Manual smoke test

Запуск:

```bash
cd "D:\Документы\vscode\HotSAS Studio\apps\desktop-tauri"
npm.cmd run tauri:dev
```

Проверить:

```text
[OK/FAIL] App starts
[OK/FAIL] RC demo project can be created
[OK/FAIL] Simulation Results screen opens
[OK/FAIL] Check ngspice button works
[OK/FAIL] If ngspice missing, UI shows friendly warning
[OK/FAIL] Mock engine still runs
[OK/FAIL] Auto engine fallback works
[OK/FAIL] Operating point run returns result or controlled unavailable error
[OK/FAIL] AC Sweep run returns result or controlled unavailable error
[OK/FAIL] Transient run returns result or controlled unavailable error
[OK/FAIL] Result card shows status/warnings/errors
[OK/FAIL] Graph series list/chart renders
[OK/FAIL] Export Center still opens
[OK/FAIL] Export CSV simulation data works if simulation result exists
[OK/FAIL] Selected Region tab still opens
[OK/FAIL] Component Library still opens
[OK/FAIL] Engineering Notebook still opens
```

Если у агента нет GUI-доступа:

```text
Manual UI smoke test: NOT RUN
Reason: no GUI access
```

---

## 24. Final checks

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
```

Желательно:

```bash
npm.cmd run tauri:build
```

Если ngspice установлен и доступен:

```powershell
cd "D:\Документы\vscode\HotSAS Studio\engine"
$env:HOTSAS_RUN_NGSPICE_INTEGRATION="1"
cargo test ngspice
```

---

## 25. Agent self-check before final answer

Перед финальным отчётом агент обязан открыть и проверить:

```text
engine/core/src/ngspice.rs or simulation models
engine/ports/src/lib.rs
engine/adapters/src/ngspice*
engine/application/src/services/*simulation*
engine/api/src/dto.rs
engine/api/src/facade.rs
apps/desktop-tauri/src-tauri/src/lib.rs
apps/desktop-tauri/src/api/index.ts
apps/desktop-tauri/src/types/index.ts
apps/desktop-tauri/src/store/index.ts
apps/desktop-tauri/src/screens/SimulationResultsScreen.tsx
docs/simulation/NGSPICE_ADAPTER_V1.md
docs/testing/verification_logs/v1.8_ngspice_adapter_v1.md
docs/testing/latest_verification_log.md
docs/testing/TESTING.md
README.md
```

Проверить:

```text
- no frontend ngspice process launch;
- no frontend netlist generation;
- no frontend ngspice parser;
- missing ngspice does not crash;
- mock simulation still available;
- Export Center not broken;
- README says v1.9 next;
- v1.8 listed in Completed;
- verification log contains exact test outputs and commit hash;
- separate verification log file exists.
```

---

## 26. Git commit and push

После успешных проверок:

```bash
git status --short
git add .
git commit -m "v1.8: add ngspice adapter v1"
git push origin main
git status --short
git log --oneline -5
```

Если нужен отдельный log update commit:

```bash
git commit -m "v1.8-fix: add commit hash to verification logs"
git push origin main
```

Финальный ответ агента должен содержать:

```text
- commit hash;
- push status;
- summary of implemented features;
- exact test results;
- path to verification log;
- whether ngspice was installed and real integration tests were run;
- current next stage: v1.9 — SPICE/Touchstone Import Foundation.
```

---

## 27. Acceptance criteria

v1.8 считается принятой, если:

```text
1. NgspiceSimulationAdapter exists.
2. SimulationEnginePort is used by application layer.
3. check_ngspice_availability works.
4. Missing ngspice returns friendly unavailable status.
5. Mock simulation still works.
6. Operating point / AC sweep / transient methods exist.
7. Parser handles supported output fixtures.
8. API facade exposes simulation methods.
9. Tauri commands exist and are registered.
10. Frontend Simulation Results UI exposes ngspice status and run controls.
11. Rust tests pass.
12. Frontend tests pass.
13. Build passes.
14. docs/simulation/NGSPICE_ADAPTER_V1.md exists.
15. docs/testing/verification_logs/v1.8_ngspice_adapter_v1.md exists.
16. latest_verification_log.md points to v1.8.
17. README says Current roadmap stage: v1.9 next.
18. Git commit and push completed.
```

Если ngspice не установлен на машине агента, v1.8 всё ещё может быть принята, если:

```text
- unit/fake-runner/parser tests pass;
- missing-ngspice path is tested;
- optional real integration tests are explicitly marked SKIPPED with reason;
- UI handles unavailable ngspice clearly.
```

---

## 28. Next version after v1.8

После v1.8 следующий этап:

```text
v1.9 — SPICE/Touchstone Import Foundation
```

v1.9 должен начать real model import:

```text
- .lib / .mod / .subckt detection;
- SPICE model registry;
- pin mapping;
- attach simulation model to ComponentDefinition;
- Touchstone .s1p/.s2p parser foundation.
```

Не начинать v1.9 внутри v1.8.
