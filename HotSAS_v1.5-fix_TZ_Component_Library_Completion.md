# HotSAS Studio v1.5-fix — Component Library Completion, Verification, Documentation

## 0. Решение по текущей версии

По отчёту агента v1.5 заявлена как завершённая, но по видимому состоянию GitHub её нельзя принимать как полностью закрытую без фикс-этапа.

Причины:
- `latest_verification_log.md` всё ещё указывает на v1.4-fix, а не на v1.5.
- Файл `docs/testing/verification_logs/v1.5_component_library_foundation.md` в GitHub raw не найден.
- `README.md` показывает `Current roadmap stage: v1.5 in progress`, а не `v1.6 next`.
- Кодовые элементы v1.5 частично видны: `component_library.rs`, `component_seeds.rs`, `ComponentLibraryScreen`, component-library UI components, API/facade методы.
- Но без актуального verification log, README и полного локального прогона нельзя считать этап принятым.
- Отчёт агента указывает commit `2df2dc0`, но в видимой истории GitHub этот commit нужно отдельно проверить через `git fetch && git log`; возможна рассинхронизация local/remote или неполный push.

Итог:
- НЕ переходить сразу к `v1.6 — Selected Region Analysis Foundation`.
- Выполнить `v1.5-fix — Component Library Completion, Verification, Documentation`.
- Если в ходе preflight окажется, что commit v1.5 действительно есть локально, но не запушен — запушить.
- Если код есть, но docs/logs не обновлены — исправить только docs/logs/tests.
- Если тесты падают — исправить минимально в рамках v1.5.
- Только после PASS по v1.5-fix переходить к v1.6.

---

## 1. Цель этапа

Довести `v1.5 — Component Library Foundation` до состояния, где она подтверждена кодом, тестами, документацией, логами и GitHub push.

Главный результат:

```text
Пользователь открывает Component Library,
видит встроенную библиотеку минимум из 12 компонентов,
ищет/фильтрует компоненты,
смотрит details + symbol preview + footprint preview,
выбирает компонент схемы,
назначает библиотечный ComponentDefinition на schematic ComponentInstance,
изменение проходит через Rust backend,
React остаётся только UI/view layer,
проект проходит cargo/npm checks,
есть verification log v1.5,
README показывает v1.6 next.
```

---

## 2. Жёсткие ограничения scope

Запрещено:

```text
- не начинать v1.6 selected region analysis;
- не добавлять ngspice;
- не добавлять PCB editor;
- не добавлять routing;
- не добавлять Gerber;
- не добавлять KiCad export;
- не добавлять Altium export;
- не делать DBLib;
- не делать online lookup DigiKey/Mouser/LCSC;
- не делать lifecycle/pricing/stock lookup;
- не добавлять DC-DC calculators;
- не делать full symbolic solver;
- не переписывать Schematic Editor;
- не переписывать Engineering Notebook;
- не ломать .circuit package storage;
- не ломать RC vertical slice;
- не переносить component library state только во frontend;
- не давать React прямую запись в project storage.
```

Разрешено:

```text
- исправить v1.5 compile/type/test issues;
- проверить и подключить component_library/component_seeds modules;
- добавить/исправить ComponentLibraryPort;
- добавить/исправить JsonComponentLibraryStorage;
- добавить/исправить ComponentLibraryService;
- добавить/исправить API DTO/facade methods;
- добавить/исправить Tauri commands;
- добавить/исправить frontend API/types/store/components;
- добавить/исправить assign-to-schematic flow;
- добавить/исправить tests;
- отформатировать Rust/TS/Markdown;
- создать v1.5 verification log;
- обновить latest_verification_log.md;
- обновить README и docs/component_library;
- закоммитить и запушить.
```

---

## 3. Preflight: проверить фактический local/remote HEAD

Выполнить из корня проекта:

```powershell
cd "D:\Документы\vscode\HotSAS Studio"

git rev-parse --show-toplevel
git branch --show-current
git status --short
git remote -v
git fetch origin main
git log --oneline -15
git log --oneline origin/main -15
git rev-parse HEAD
git rev-parse origin/main
git diff --stat
git diff --name-only
git diff --stat origin/main..HEAD
git diff --name-only origin/main..HEAD
```

Обязательно проверить commit из отчёта агента:

```powershell
git show --stat --oneline 2df2dc0
git branch --contains 2df2dc0
git branch -r --contains 2df2dc0
```

Интерпретация:

```text
1. Если 2df2dc0 существует локально и не существует в origin/main:
   - проверить git status;
   - убедиться, что это нужный v1.5 commit;
   - выполнить push:
     git push origin main

2. Если 2df2dc0 не существует:
   - считать отчёт агента неподтверждённым;
   - продолжить v1.5-fix по фактическому HEAD.

3. Если origin/main уже содержит v1.5 код, но логов нет:
   - не переписывать реализацию;
   - закрыть docs/logs/tests.

4. Если git status не clean:
   - не выполнять reset/clean;
   - записать список файлов в verification log;
   - править только в рамках v1.5-fix.
```

Запрещено:

```text
git reset
git clean
удаление пользовательских untracked-файлов
force push
```

---

## 4. Базовые проверки до изменений

Выполнить:

```powershell
cd "D:\Документы\vscode\HotSAS Studio\engine"
cargo fmt --check
cargo test
```

```powershell
cd "D:\Документы\vscode\HotSAS Studio\apps\desktop-tauri"
npm.cmd run format:check
npm.cmd run typecheck
npm.cmd run test
npm.cmd run build
```

Если что-то падает:
- сохранить полный вывод ошибки в verification log;
- исправлять минимально;
- после исправления повторить команду;
- не писать PASS без фактического вывода команды.

---

## 5. Проверить и исправить форматирование файлов

Сейчас часть raw-файлов в GitHub отображается как почти одна строка. Перед финальным логом нужно привести форматирование к нормальному виду.

Проверить:

```text
README.md
CHANGELOG.md
docs/testing/TESTING.md
docs/testing/latest_verification_log.md
docs/component_library/COMPONENT_MODEL.md
docs/component_library/COMPONENT_LIBRARY_FOUNDATION.md

engine/core/src/lib.rs
engine/core/src/component_library.rs
engine/core/src/component_seeds.rs
engine/ports/src/lib.rs
engine/application/src/services/component_library.rs
engine/application/src/services/app_services.rs
engine/adapters/src/lib.rs
engine/api/src/dto.rs
engine/api/src/facade.rs

apps/desktop-tauri/src-tauri/src/lib.rs
apps/desktop-tauri/src/api/index.ts
apps/desktop-tauri/src/types/index.ts
apps/desktop-tauri/src/store/index.ts или src/store.ts
apps/desktop-tauri/src/screens/ComponentLibraryScreen.tsx
apps/desktop-tauri/src/components/component-library/*.tsx
apps/desktop-tauri/src/components/component-library/__tests__/*.tsx
```

Команды:

```powershell
cd "D:\Документы\vscode\HotSAS Studio\engine"
cargo fmt
cargo fmt --check
```

```powershell
cd "D:\Документы\vscode\HotSAS Studio\apps\desktop-tauri"
npm.cmd run format
npm.cmd run format:check
```

Если Prettier не форматирует Markdown достаточно читаемо, поправить Markdown вручную.

---

## 6. Core: подтвердить component library domain

Проверить, что в `engine/core/src/lib.rs` подключены:

```rust
pub mod component_library;
pub mod component_seeds;

pub use component_library::*;
pub use component_seeds::*;
```

Проверить `engine/core/src/component_library.rs`:

```rust
pub struct ComponentLibrary {
    pub id: String,
    pub title: String,
    pub version: String,
    pub components: Vec<ComponentDefinition>,
    pub symbols: Vec<SymbolDefinition>,
    pub footprints: Vec<FootprintDefinition>,
    pub simulation_models: Vec<SimulationModel>,
    pub metadata: BTreeMap<String, String>,
}

pub enum ComponentCategory {
    Resistor,
    Capacitor,
    Inductor,
    Diode,
    Led,
    OpAmp,
    Bjt,
    Mosfet,
    VoltageRegulator,
    Connector,
    Source,
    Ground,
    Generic,
}

pub struct ComponentLibraryQuery {
    pub search: Option<String>,
    pub category: Option<String>,
    pub tags: Vec<String>,
    pub manufacturer: Option<String>,
    pub has_symbol: Option<bool>,
    pub has_footprint: Option<bool>,
    pub has_simulation_model: Option<bool>,
}

pub struct ComponentLibrarySearchResult {
    pub components: Vec<ComponentDefinition>,
    pub total_count: usize,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
}

pub struct ComponentAssignment {
    pub instance_id: String,
    pub component_definition_id: String,
    pub selected_symbol_id: Option<String>,
    pub selected_footprint_id: Option<String>,
    pub selected_simulation_model_id: Option<String>,
}
```

Все модели:
- `Debug`
- `Clone`
- `PartialEq` где уместно
- `Serialize`
- `Deserialize`

Проверить конфликт `SymbolDefinition`:
- если есть `models.rs::SymbolDefinition` и `symbol.rs::SymbolDefinition`, зафиксировать в docs, какой тип используется для library preview;
- не делать большой refactor в этом этапе;
- component library может хранить ссылки `symbol_ids`, а preview брать через `seed_symbol_for_kind`.

---

## 7. Core: built-in seed library

Проверить `engine/core/src/component_seeds.rs`.

Обязательная функция:

```rust
pub fn built_in_component_library() -> ComponentLibrary
```

Минимум 12 компонентов:

```text
1. generic_resistor
2. generic_capacitor
3. generic_inductor
4. generic_diode
5. generic_led
6. generic_npn_bjt
7. generic_pnp_bjt
8. generic_n_mosfet
9. generic_p_mosfet
10. generic_op_amp
11. generic_voltage_source
12. ground_reference
```

Обязательные поля для каждого компонента:
- `id`
- `name`
- `category`
- `parameters` или metadata where appropriate
- `ratings` где уместно
- `symbol_ids`
- `footprint_ids` где уместно
- `tags`

Обязательные footprints:

```text
axial_resistor_placeholder
radial_capacitor_placeholder
inductor_placeholder
do_41_diode_placeholder
led_5mm_placeholder
to_92_placeholder
to_220_placeholder
soic8_placeholder
sot23_placeholder
ground_virtual_placeholder
```

Не делать реальные PCB footprints. Это только metadata placeholder.

---

## 8. Symbol mapping

Проверить, что seed-symbols существуют:

```text
resistor
capacitor
inductor
diode
led
bjt_npn
bjt_pnp
mosfet_n
mosfet_p
op_amp
voltage_source
ground
```

Если в `symbol.rs` не хватает diode/LED/BJT/MOSFET/op_amp/inductor:
- добавить минимальные pin definitions;
- не рисовать сложную графику;
- цель — preview + pin metadata.

Минимальные pin rules:
- resistor/capacitor/inductor/diode/LED: 2 pins;
- BJT: C/B/E;
- MOSFET: D/G/S;
- op-amp: `+`, `-`, `out`, optional supply pins;
- voltage source: p/n;
- ground: gnd.

---

## 9. Ports: ComponentLibraryPort

Проверить `engine/ports/src/lib.rs`.

Должен быть trait:

```rust
pub trait ComponentLibraryPort: Send + Sync {
    fn load_builtin_library(&self) -> Result<ComponentLibrary, PortError>;

    fn load_library_from_path(
        &self,
        path: &Path,
    ) -> Result<ComponentLibrary, PortError>;

    fn save_library_to_path(
        &self,
        path: &Path,
        library: &ComponentLibrary,
    ) -> Result<(), PortError>;
}
```

Если `Path` используется, импортировать:

```rust
use std::path::Path;
```

---

## 10. Adapters: JsonComponentLibraryStorage

Проверить `engine/adapters/src/lib.rs` или отдельный модуль.

Должна быть реализация:

```rust
pub struct JsonComponentLibraryStorage;
```

Методы:
- `load_builtin_library()` возвращает `built_in_component_library()`;
- `load_library_from_path(path)` читает JSON;
- `save_library_to_path(path, library)` пишет pretty JSON и создаёт parent directories.

Ошибки:
- invalid JSON → controlled `PortError`;
- missing file → controlled `PortError`;
- IO error → controlled `PortError`.

---

## 11. Application: ComponentLibraryService

Проверить `engine/application/src/services/component_library.rs`.

Должны быть методы:

```rust
load_builtin_library()
list_components(library)
search_components(library, query)
get_component(library, component_id)
get_symbol_for_component(library, component_id)
get_footprints_for_component(library, component_id)
assign_component_to_instance(project, assignment)
```

Требования:
- поиск по id/name/category/description/part_number/tags;
- фильтр по category;
- фильтр по tags;
- фильтр по manufacturer;
- фильтр `has_symbol`;
- фильтр `has_footprint`;
- фильтр `has_simulation_model`;
- assignment должен менять только выбранный `ComponentInstance`;
- если instance не найден — controlled error;
- если component id не найден — controlled error;
- не трогать wiring/nets/parameters без необходимости.

Проверить `AppServices`:
- создаётся `ComponentLibraryService`;
- есть accessor `component_library_service()`;
- constructor принимает `Arc<dyn ComponentLibraryPort>` или конкретный adapter в composition root.

---

## 12. API DTO / Facade

Проверить `engine/api/src/dto.rs`.

Нужны DTO:

```rust
ComponentLibraryDto
ComponentSummaryDto
ComponentDetailsDto
ComponentSearchRequestDto
ComponentSearchResultDto
AssignComponentRequestDto
FootprintDto
SimulationModelDto
KeyValueDto
```

`ComponentSummaryDto` должен содержать:
- id
- name
- category
- manufacturer
- part_number
- description
- tags
- has_symbol
- has_footprint
- has_simulation_model

`ComponentDetailsDto` должен содержать:
- все summary fields;
- parameters;
- ratings;
- symbol_ids;
- footprint_ids;
- simulation_models;
- datasheets;
- tags;
- metadata;
- symbol_preview;
- footprint_previews.

Проверить `engine/api/src/facade.rs`.

Нужны методы:

```rust
pub fn load_builtin_component_library(&self) -> Result<ComponentLibraryDto, ApiError>;

pub fn list_components(&self) -> Result<Vec<ComponentSummaryDto>, ApiError>;

pub fn search_components(
    &self,
    request: ComponentSearchRequestDto,
) -> Result<ComponentSearchResultDto, ApiError>;

pub fn get_component_details(
    &self,
    component_id: String,
) -> Result<ComponentDetailsDto, ApiError>;

pub fn assign_component_to_selected_instance(
    &self,
    request: AssignComponentRequestDto,
) -> Result<ProjectDto, ApiError>;
```

Требования:
- Facade держит backend-owned current `ComponentLibrary`;
- UI не должен передавать всю library обратно в backend;
- assignment проверяет, что component существует в library;
- assignment требует current_project;
- result assignment возвращает обновлённый `ProjectDto`.

---

## 13. Tauri boundary

Проверить `apps/desktop-tauri/src-tauri/src/lib.rs`.

Нужны команды:

```rust
#[tauri::command]
async fn load_builtin_component_library(
    state: State<'_, AppState>,
) -> Result<ComponentLibraryDto, String>;

#[tauri::command]
async fn list_components(
    state: State<'_, AppState>,
) -> Result<Vec<ComponentSummaryDto>, String>;

#[tauri::command]
async fn search_components(
    state: State<'_, AppState>,
    request: ComponentSearchRequestDto,
) -> Result<ComponentSearchResultDto, String>;

#[tauri::command]
async fn get_component_details(
    state: State<'_, AppState>,
    component_id: String,
) -> Result<ComponentDetailsDto, String>;

#[tauri::command]
async fn assign_component_to_selected_instance(
    state: State<'_, AppState>,
    request: AssignComponentRequestDto,
) -> Result<ProjectDto, String>;
```

Добавить в `generate_handler!`.

Проверить Tauri capabilities/permissions:
- если проект использует `capabilities/default.json`, команды должны быть разрешены;
- если commands не требуют explicit capabilities в текущей конфигурации, это записать в verification log.

---

## 14. Frontend API / Types / Store

Проверить `apps/desktop-tauri/src/types/index.ts`:
- DTO должны соответствовать backend DTO;
- использовать snake_case, если Tauri сериализует Rust DTO как snake_case;
- не делать camelCase, если backend отдаёт snake_case.

Проверить `apps/desktop-tauri/src/api/index.ts`:
- методы только через `invoke`;
- нет вычислений/поиска/assignment logic в React;
- wrapper methods:

```ts
loadBuiltinComponentLibrary()
listComponents()
searchComponents(request)
getComponentDetails(componentId)
assignComponentToSelectedInstance(request)
```

Проверить Zustand store:
- `selectedLibraryComponentId`
- `selectedLibraryComponent`
- `componentSearchResult`
- setters
- не хранить source-of-truth library mutation logic;
- store только UI/session state.

---

## 15. Frontend UI

Проверить/исправить `ComponentLibraryScreen.tsx`.

Функции экрана:
- при mount вызвать `loadBuiltinComponentLibrary`;
- показать title/version/count;
- показать search/filter panel;
- показать component table/cards;
- при выборе компонента вызвать `getComponentDetails`;
- показать details panel;
- показать symbol preview;
- показать footprint preview;
- если выбран schematic component — показать assign panel;
- assign вызывает backend command и обновляет project в store;
- loading overlay и error alert есть.

Компоненты:

```text
ComponentSearchPanel.tsx
ComponentTable.tsx
ComponentDetailsPanel.tsx
ComponentSymbolPreview.tsx
ComponentFootprintPreview.tsx
AssignComponentPanel.tsx
```

Требования:
- search input;
- category select;
- tags input or quick tags;
- checkboxes:
  - has symbol
  - has footprint
  - has simulation model
- reset filters;
- table columns:
  - Name
  - Category
  - Tags
  - Symbol
  - Footprint
  - Sim model
- details:
  - Parameters
  - Ratings
  - Metadata
  - Symbol IDs
  - Footprint IDs
  - Simulation models
- assign button disabled if no schematic component or no library component selected.

---

## 16. Tests: Rust

Добавить/проверить тесты.

### Core

`engine/core/tests/component_library_tests.rs`

```text
- built_in_library_has_at_least_12_components
- built_in_library_contains_required_component_ids
- generic_resistor_has_expected_parameters_and_symbol
- generic_capacitor_has_expected_parameters_and_symbol
- generic_op_amp_has_metadata_slew_rate_if_unit_not_supported
- built_in_footprints_have_expected_ids
- component_categories_display_as_stable_lowercase_strings
- component_library_serializes_deserializes
```

### Adapters

`engine/adapters/tests/component_library_storage_tests.rs`

```text
- load_builtin_library_returns_seed_library
- save_and_load_library_json_roundtrip
- save_library_creates_parent_directories
- load_missing_library_returns_error
- load_invalid_json_returns_error
```

### Application

`engine/application/tests/component_library_service_tests.rs`

```text
- search_by_name
- search_by_category
- search_by_tag
- search_by_manufacturer
- filter_has_symbol
- filter_has_footprint
- filter_has_simulation_model
- get_component_success
- get_component_missing_returns_error
- get_symbol_for_component_returns_preview
- get_footprints_for_component_returns_previews
- assign_component_to_instance_updates_definition_symbol_footprint_model
- assign_missing_instance_returns_error
```

### API

`engine/api/tests/component_library_api_tests.rs`

```text
- load_builtin_component_library_returns_12_plus_components
- list_components_returns_summaries
- search_components_returns_filtered_results
- get_component_details_returns_symbol_and_footprint_preview
- assign_component_without_project_returns_state_error
- create_project_then_assign_generic_resistor_to_R1
- assign_unknown_component_returns_error
- assign_unknown_instance_returns_error
```

---

## 17. Tests: Frontend

Добавить/проверить:

```text
apps/desktop-tauri/src/components/component-library/__tests__/ComponentLibraryComponents.test.tsx
apps/desktop-tauri/src/screens/__tests__/ComponentLibraryScreen.test.tsx
```

Проверки:

```text
- ComponentSearchPanel renders search/category/filter controls.
- Search button calls handler with request.
- Reset button calls handler.
- ComponentTable renders components.
- Selecting row calls onSelect.
- ComponentDetailsPanel renders parameters/ratings/metadata.
- Symbol preview renders pins.
- Footprint preview renders package/pads metadata.
- AssignComponentPanel disables button if no schematic component.
- AssignComponentPanel enables button when both selections exist.
- ComponentLibraryScreen loads builtin library on mount.
- ComponentLibraryScreen shows loading and errors.
- ComponentLibraryScreen can select component and show details.
- ComponentLibraryScreen calls assign backend and updates project.
```

Тестовый setup:
- если `ResizeObserver` нужен React Flow/Mantine — mock в `test-setup.ts`;
- избегать ambiguous selectors;
- use accessible labels where possible.

---

## 18. Manual smoke test

Запустить:

```powershell
cd "D:\Документы\vscode\HotSAS Studio\apps\desktop-tauri"
npm.cmd run tauri:dev
```

Проверить:

```text
[OK/FAIL] App opens
[OK/FAIL] RC demo project creates
[OK/FAIL] Schematic renders
[OK/FAIL] R1 can be selected
[OK/FAIL] Property panel shows R1
[OK/FAIL] Component Library opens
[OK/FAIL] Built-in library loads
[OK/FAIL] Shows >=12 components
[OK/FAIL] Search "resistor" works
[OK/FAIL] Category "resistor" works
[OK/FAIL] Generic Resistor details open
[OK/FAIL] Symbol preview visible
[OK/FAIL] Footprint preview visible
[OK/FAIL] Assign Generic Resistor to R1 works
[OK/FAIL] Schematic project state updates
[OK/FAIL] Formula Library still works
[OK/FAIL] Engineering Notebook still works
[OK/FAIL] .circuit save/load still works
[OK/FAIL] RC vertical slice still works
```

Если UI smoke test не выполнялся — указать `NOT RUN`, не писать PASS.

---

## 19. Documentation

Создать:

```text
docs/component_library/COMPONENT_LIBRARY_FOUNDATION.md
```

Содержание:
- цель v1.5;
- что входит в built-in library;
- что такое ComponentDefinition;
- что такое ComponentInstance;
- как работает symbol preview;
- как работает footprint preview;
- как работает assign-to-schematic;
- что пока placeholder;
- чего нет:
  - no PCB editor;
  - no KiCad/Altium export;
  - no online lookup;
  - no lifecycle/pricing;
  - no real SPICE model import.

Обновить:

```text
docs/component_library/COMPONENT_MODEL.md
docs/testing/TESTING.md
README.md
CHANGELOG.md если используется
```

README после завершения v1.5:

```text
Current roadmap stage: v1.6 next

Completed:
- v1.2 — Project Package Storage .circuit
- v1.3 — Schematic Editor Foundations
- v1.4 — Engineering Notebook / Calculator Foundations
- v1.5 — Component Library Foundation
```

Не переписывать README полностью.

---

## 20. Verification log requirement

Создать:

```text
docs/testing/verification_logs/v1.5_component_library_foundation.md
```

Обновить:

```text
docs/testing/latest_verification_log.md
```

Формат v1.5 log смотри в отдельном файле:

```text
HotSAS_v1.5-fix_verification_log_template.md
```

Обязательное правило:
- log должен содержать реальные команды и реальные результаты;
- если команда не запускалась — `NOT RUN`;
- если команда падала до исправления — указать failed attempt и final status после исправления;
- не писать PASS без вывода команды.

---

## 21. Финальные проверки

Команды:

```powershell
cd "D:\Документы\vscode\HotSAS Studio\engine"
cargo fmt --check
cargo test
```

```powershell
cd "D:\Документы\vscode\HotSAS Studio\apps\desktop-tauri"
npm.cmd run format:check
npm.cmd run typecheck
npm.cmd run test
npm.cmd run build
npm.cmd run tauri:build
```

Если собирается release exe:

```powershell
Get-Item "D:\Документы\vscode\HotSAS Studio\apps\desktop-tauri\src-tauri\target\release\hotsas_desktop_tauri.exe"
Compress-Archive -LiteralPath "D:\Документы\vscode\HotSAS Studio\apps\desktop-tauri\src-tauri\target\release\hotsas_desktop_tauri.exe" -DestinationPath "D:\Документы\vscode\HotSAS Studio\apps\desktop-tauri\src-tauri\target\release\HotSAS-Studio-v1.5-windows-x64.zip" -Force
```

---

## 22. Git commit / push

После PASS:

```powershell
cd "D:\Документы\vscode\HotSAS Studio"

git status --short
git diff --stat
git add README.md CHANGELOG.md docs engine apps
git diff --cached --stat
git commit -m "v1.5-fix: complete component library verification and docs"
git push origin main
git status --short
git log --oneline -5
```

Если основной v1.5 commit ещё не был запушен, а фикс только закрывает недостающие docs/logs:

```text
Commit message:
v1.5: Component Library Foundation

или если основной commit уже есть:
v1.5-fix: Component Library Completion, Verification, Documentation
```

Не добавлять:
- `node_modules`
- `target`
- `dist`
- `.env`
- личные файлы ТЗ/истории чата
- exe/zip в git, если проект не хранит release artifacts в репозитории.

---

## 23. Acceptance criteria

`v1.5-fix` считается завершённым только если:

```text
1. Фактический local HEAD и origin/main проверены.
2. Commit из отчёта агента `2df2dc0` проверен или расхождение описано.
3. Нет незадокументированных local changes.
4. `ComponentLibraryScreen` рабочий, не placeholder.
5. Built-in component library содержит минимум 12 компонентов.
6. Есть generic resistor/capacitor/inductor/diode/LED/BJT/MOSFET/op-amp/source/ground.
7. Есть symbol preview.
8. Есть footprint preview.
9. Есть search/filter.
10. Есть component details.
11. Есть assign component to schematic instance.
12. Assignment обновляет backend project.
13. React не пишет component library напрямую.
14. Backend остаётся source of truth.
15. ComponentLibraryPort есть.
16. JsonComponentLibraryStorage есть.
17. ComponentLibraryService есть.
18. API/Tauri commands есть.
19. Frontend API/types/store есть.
20. Tests добавлены или подтверждены.
21. COMPONENT_LIBRARY_FOUNDATION.md создан.
22. COMPONENT_MODEL.md обновлён.
23. TESTING.md обновлён.
24. README показывает v1.6 next и completed v1.5.
25. latest_verification_log.md указывает на v1.5.
26. v1.5 verification log создан.
27. cargo fmt --check PASS.
28. cargo test PASS.
29. npm.cmd run format:check PASS.
30. npm.cmd run typecheck PASS.
31. npm.cmd run test PASS.
32. npm.cmd run build PASS.
33. npm.cmd run tauri:build PASS или честно NOT RUN/FAIL с причиной.
34. Изменения закоммичены.
35. Изменения запушены.
36. ngspice/PCB/DC-DC/KiCad/Altium/symbolic solver/online lookup не добавлены.
```

---

## 24. Следующая версия после принятия v1.5-fix

После успешного v1.5-fix переходить к:

```text
v1.6 — Selected Region Analysis Foundation
```

Но начинать v1.6 можно только когда:
- latest verification log уже v1.5;
- README показывает v1.6 next;
- cargo/npm checks зелёные;
- GitHub содержит финальный commit.

Краткий будущий фокус v1.6:
- `SelectedCircuitRegion`
- `RegionPort`
- `BoundaryNet`
- `RegionAnalysisRequest`
- `RegionAnalysisResult`
- selection from schematic UI
- backend detection of boundary nets
- RC low-pass template recognition for selected `R1+C1`
- result: H(s), fc, mock graph data, report-ready section
- no universal symbolic solver promise
- no real ngspice yet
