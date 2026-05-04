# ТЗ HotSAS Studio v1.10 — Internal Alpha EXE Build & v2.0 Readiness Gate

```text
Выполни HotSAS Studio v1.10 — Internal Alpha EXE Build & v2.0 Readiness Gate.
```

## 0. Смысл этапа

v1.10 — это не публичный релиз и не переход к v2.0. Это стабилизационный этап перед v2.0, который должен превратить набор уже реализованных foundation-модулей v1.0–v1.9 в проверяемую внутреннюю alpha/dev-сборку.

Главная цель: получить локально собранный Windows `.exe`, который можно передать/скопировать на другой ПК для внутренней проверки запуска, базовой навигации и ключевых пользовательских сценариев.

Важно:

```text
До достижения стадии v2.0 не выпускать публичный GitHub Release.
Разрешены только внутренние dev/alpha-сборки для проверки запуска на другом ПК.
```

---

## 1. Текущий контекст проекта

Проект уже прошёл:

```text
- v1.0 — Initial RC Low-Pass Vertical Slice
- v1.1.1 — Formatting + Build/Test Infrastructure
- v1.1.2 — Backend Test Expansion
- v1.1.3 — FormulaPackLoader + FormulaRegistry
- v1.1.4-fix / fix.2 — Generic Formula Engine Completion + Hygiene
- v1.1.5 — Exact E-Series Tables
- v1.2 — Project Package Storage .circuit
- v1.3 — Schematic Editor Foundations
- v1.4 — Engineering Notebook / Calculator Foundations
- v1.5 — Component Library Foundation
- v1.6 — Selected Region Analysis Foundation
- v1.7 — Export Center v1
- v1.8 — ngspice Adapter v1
- v1.9 — SPICE/Touchstone Import Foundation
```

Текущий roadmap stage после v1.9: `v1.10 next`.

v2.0 по дорожной карте должен стать первой стадией, где программа уже выглядит как самостоятельный инженерный инструмент. v1.10 должен быть последним/одним из последних readiness-gate этапов перед этим переходом.

---

## 2. Что изменится для пользователя после v1.10

После v1.10 пользователь должен получить не новую большую инженерную функцию, а более цельное и проверяемое приложение.

С точки зрения пользователя должно стать возможно:

```text
1. Скачать/получить внутреннюю alpha/dev-сборку `.exe` без установки Rust/Node/npm.
2. Запустить HotSAS Studio на Windows как обычное desktop-приложение.
3. Увидеть понятный стартовый экран и текущий статус программы.
4. Перейти по основным экранам:
   - Start / Schematic
   - Engineering Notebook
   - Formula Library
   - Component Library
   - Selected Region
   - Simulation Results
   - Import Models
   - Export Center
5. Создать или открыть RC demo project.
6. Проверить, что базовая схема, формулы, симуляция, импорт и экспорт не ломают друг друга.
7. Увидеть диагностический статус ключевых модулей:
   - Formula Registry
   - Component Library
   - Project Package Storage
   - Simulation Engine / ngspice availability
   - Import Models
   - Export Center
8. Получить понятное сообщение, если ngspice или внешний компонент окружения недоступен.
9. Передать `.exe`/zip на другой ПК для внутреннего smoke-test.
```

Текущий пользовательский статус после v1.10:

```text
Программа ещё не публичный продукт и не v2.0 beta.
Но это уже внутренняя alpha/dev-сборка, которую можно собрать, запустить, проверить и показать как технический прототип со связанной функциональностью.
```

---

## 3. Жёсткие ограничения scope

Запрещено:

```text
- не выпускать публичный GitHub Release;
- не создавать публичный release tag как стабильную версию;
- не позиционировать сборку как production-ready;
- не делать PCB editor;
- не делать routing/Gerber/DRC;
- не делать proprietary Altium file generation;
- не делать полноценный KiCad project export;
- не делать full symbolic solver;
- не делать полноценный SPICE model manager;
- не делать полноценный Touchstone graph viewer;
- не добавлять новые крупные инженерные расчётные модули;
- не переписывать архитектуру;
- не переносить бизнес-логику в React;
- не запускать ngspice напрямую из UI;
- не парсить SPICE/Touchstone во frontend;
- не коммитить `target/`, `dist/`, `.exe`, `.zip`, `.msi`, `.nsis`, `node_modules/`.
```

Разрешено:

```text
- исправлять интеграционные баги между уже существующими модулями;
- добавить системную диагностику / readiness dashboard;
- добавить app/build info DTO и UI-панель статуса;
- улучшить Start screen / About / Diagnostics;
- добавить smoke-test checklist в документацию;
- добавить скрипт или инструкцию внутренней упаковки `.exe` в zip;
- добавить frontend/backend tests для диагностики и навигации;
- обновить README, TESTING, docs/builds, latest verification log;
- выполнить `npm.cmd run tauri:build` и зафиксировать путь/размер/checksum `.exe` в verification log;
- сделать commit и push исходного кода/документации, но не бинарников.
```

---

## 4. Обязательный preflight

Перед любыми изменениями выполнить из корня проекта:

```powershell
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
- Если есть незакоммиченные изменения до начала работы — записать их в verification log.
- Если есть пользовательские файлы ТЗ/заметок — не трогать.
```

Проверить текущую базу v1.9:

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

Если какая-то проверка падает до изменений, сначала зафиксировать это в verification log и исправлять минимально, не расширяя scope.

---

## 5. Backend/Core: build/app status models

Добавить или проверить наличие core-моделей для статуса приложения. Рекомендуемый файл:

```text
engine/core/src/app_diagnostics.rs
```

Подключить в:

```text
engine/core/src/lib.rs
```

### 5.1. AppDiagnosticsReport

```rust
pub struct AppDiagnosticsReport {
    pub app_name: String,
    pub app_version: String,
    pub roadmap_stage: String,
    pub build_profile: String,
    pub modules: Vec<ModuleDiagnostics>,
    pub checks: Vec<ReadinessCheck>,
    pub warnings: Vec<String>,
}
```

### 5.2. ModuleDiagnostics

```rust
pub struct ModuleDiagnostics {
    pub id: String,
    pub title: String,
    pub status: ModuleStatus,
    pub summary: String,
    pub details: BTreeMap<String, String>,
}
```

### 5.3. ModuleStatus

```rust
pub enum ModuleStatus {
    Ready,
    Limited,
    Unavailable,
    Unknown,
}
```

### 5.4. ReadinessCheck

```rust
pub struct ReadinessCheck {
    pub id: String,
    pub title: String,
    pub status: ReadinessStatus,
    pub message: String,
}
```

### 5.5. ReadinessStatus

```rust
pub enum ReadinessStatus {
    Pass,
    Warn,
    Fail,
    NotRun,
}
```

Все модели должны поддерживать:

```text
- Debug
- Clone
- PartialEq where useful
- Serialize
- Deserialize
```

Core не должен зависеть от Tauri/UI/adapters.

---

## 6. Application: Diagnostics / Readiness service

Создать сервис:

```text
engine/application/src/services/app_diagnostics.rs
```

Подключить его в модуль services/application exports.

### 6.1. AppDiagnosticsService

Сервис должен собирать статус уже существующих модулей:

```text
- Project Package Storage
- Formula Registry
- Engineering Notebook
- Component Library
- Schematic Editor
- Selected Region Analysis
- Export Center
- Simulation Engine / ngspice availability
- SPICE/Touchstone Import
```

Минимальные методы:

```rust
pub fn get_app_diagnostics(&self) -> AppDiagnosticsReport;

pub fn run_readiness_self_check(&self) -> AppDiagnosticsReport;
```

### 6.2. Что проверять

`get_app_diagnostics` должен быть быстрым и безопасным:

```text
- не запускать длительные симуляции;
- не требовать ngspice;
- не писать файлы;
- не зависать из-за внешнего процесса;
- возвращать controlled status.
```

Минимальные проверки:

```text
Formula Registry:
- formula packs доступны или хотя бы registry service инициализирован;
- rc_low_pass_cutoff / ohms_law / voltage_divider доступны, если текущий registry это поддерживает.

Component Library:
- built-in library загружается;
- ожидаемый минимум компонентов >= 12.

Export Center:
- list_export_capabilities возвращает 9 форматов.

Simulation:
- mock engine доступен;
- ngspice status: Available / Unavailable / Unknown.

Import Models:
- SPICE parser доступен;
- Touchstone parser доступен;
- supported extensions перечислены.

Project Package:
- .circuit storage service существует;
- save/load smoke можно выполнить только в temp dir, если метод `run_readiness_self_check` это явно делает.
```

`run_readiness_self_check` может выполнять более дорогие smoke checks, но без destructive effects:

```text
- создать RC demo project в памяти;
- проверить formula calculation;
- проверить export capability list;
- проверить mock simulation smoke;
- проверить SPICE parser на маленьком inline `.model`;
- проверить Touchstone parser на маленьком inline `.s2p`;
- если нужен temp dir — использовать временную директорию и очистить только свои файлы.
```

---

## 7. API DTO / facade

В `engine/api/src/dto.rs` добавить DTO:

```rust
pub struct AppDiagnosticsReportDto { ... }
pub struct ModuleDiagnosticsDto { ... }
pub struct ReadinessCheckDto { ... }
```

В `engine/api/src/facade.rs` добавить методы:

```rust
pub fn get_app_diagnostics(&self) -> Result<AppDiagnosticsReportDto, ApiError>;

pub fn run_readiness_self_check(&self) -> Result<AppDiagnosticsReportDto, ApiError>;
```

Требования:

```text
- не возвращать Rust enum напрямую в TS без контролируемого string mapping;
- все статусы отдавать как строки: ready/limited/unavailable/unknown, pass/warn/fail/not_run;
- ошибки маппить через существующий ApiError pattern;
- не нарушать dependency boundaries.
```

---

## 8. Tauri commands

В:

```text
apps/desktop-tauri/src-tauri/src/lib.rs
```

добавить команды:

```rust
#[tauri::command]
async fn get_app_diagnostics(...) -> Result<AppDiagnosticsReportDto, String>;

#[tauri::command]
async fn run_readiness_self_check(...) -> Result<AppDiagnosticsReportDto, String>;
```

Добавить их в `tauri::generate_handler!`.

Если в проекте используется Tauri capabilities/permissions, обновить соответствующий файл.

---

## 9. Frontend types / API / store

### 9.1. Types

В:

```text
apps/desktop-tauri/src/types/index.ts
```

добавить:

```ts
export interface AppDiagnosticsReportDto { ... }
export interface ModuleDiagnosticsDto { ... }
export interface ReadinessCheckDto { ... }
```

### 9.2. API

В:

```text
apps/desktop-tauri/src/api/index.ts
```

добавить методы:

```ts
getAppDiagnostics(): Promise<AppDiagnosticsReportDto>
runReadinessSelfCheck(): Promise<AppDiagnosticsReportDto>
```

Mock implementations тоже обновить, если они есть.

### 9.3. Store

В Zustand store добавить:

```text
appDiagnostics
readinessSelfCheckResult
diagnosticsLoading
diagnosticsError
```

И setters/actions:

```text
setAppDiagnostics
setReadinessSelfCheckResult
setDiagnosticsLoading
setDiagnosticsError
```

---

## 10. Frontend UI: Diagnostics / Internal Alpha screen

Создать экран:

```text
apps/desktop-tauri/src/screens/InternalAlphaScreen.tsx
```

или:

```text
apps/desktop-tauri/src/screens/DiagnosticsScreen.tsx
```

Рекомендуемое название в UI:

```text
Internal Alpha / Diagnostics
```

### 10.1. Что показывать

Экран должен показывать:

```text
- HotSAS Studio title
- roadmap stage: v1.10 internal alpha / v2.0 readiness
- app version/build profile, если доступно
- карточки модулей:
  - Formula Registry
  - Component Library
  - Schematic Editor
  - Engineering Notebook
  - Selected Region
  - Export Center
  - Simulation / ngspice
  - Import Models
- статус каждого модуля: Ready / Limited / Unavailable / Unknown
- warnings
- readiness checks
- кнопку Refresh diagnostics
- кнопку Run readiness self-check
```

### 10.2. Навигация

Добавить экран в Workbench/левую навигацию:

```text
Diagnostics
```

или:

```text
Internal Alpha
```

Не ломать существующие экраны.

### 10.3. UX-правила

```text
- Если backend command упал — показать controlled error.
- Если ngspice unavailable — это warning, а не crash.
- Не делать автоматический тяжелый self-check при каждом рендере.
- Self-check запускать только по кнопке пользователя.
```

---

## 11. Интеграционные проверки пользовательских сценариев

В v1.10 нужно не столько добавлять новые функции, сколько проверить связность существующих.

Минимальный manual smoke сценарий:

```text
1. Запустить приложение через `npm.cmd run tauri:dev`.
2. Открыть Start / Schematic.
3. Создать RC demo project.
4. Убедиться, что схема отображается.
5. Открыть Formula Library.
6. Проверить, что формулы доступны.
7. Выполнить расчёт rc_low_pass_cutoff или ohms_law.
8. Открыть Engineering Notebook.
9. Проверить assignment + formula call.
10. Открыть Component Library.
11. Проверить built-in components.
12. Открыть Selected Region / Region tab.
13. Выбрать R1+C1 и выполнить Preview/Analyze, если доступно.
14. Открыть Simulation Results.
15. Проверить Mock simulation и controlled ngspice status.
16. Открыть Import Models.
17. Вставить простой SPICE `.model` и проверить import report.
18. Вставить простой Touchstone `.s2p` и проверить import report.
19. Открыть Export Center.
20. Проверить preview для Markdown/HTML/SPICE/BOM/ComponentLibrary/SVG.
21. Открыть Diagnostics/Internal Alpha.
22. Проверить Refresh diagnostics.
23. Проверить Run readiness self-check.
```

Manual smoke нужно записать в verification log как таблицу `[OK]/[FAIL]/[NOT RUN]`.

---

## 12. Обязательная сборка EXE

Это обязательный пункт v1.10.

Выполнить:

```powershell
cd "D:\Документы\vscode\HotSAS Studio\apps\desktop-tauri"
npm.cmd run tauri:build
```

После сборки найти `.exe`. Ожидаемый путь обычно:

```text
apps/desktop-tauri/src-tauri/target/release/hotsas_desktop_tauri.exe
```

Если имя отличается — записать фактический путь.

### 12.1. Проверить EXE

Выполнить PowerShell-проверку:

```powershell
$exe = Resolve-Path "D:\Документы\vscode\HotSAS Studio\apps\desktop-tauri\src-tauri\target\release\hotsas_desktop_tauri.exe"
Get-Item $exe | Select-Object FullName, Length, LastWriteTime
Get-FileHash $exe -Algorithm SHA256
```

Проверить subsystem, чтобы `.exe` не открывал консоль, если это уже было настроено:

```powershell
$bytes = [System.IO.File]::ReadAllBytes($exe)
$peOffset = [BitConverter]::ToInt32($bytes, 0x3C)
$optionalHeaderOffset = $peOffset + 24
$subsystem = [BitConverter]::ToUInt16($bytes, $optionalHeaderOffset + 0x44)
$kind = switch ($subsystem) { 2 { 'Windows GUI' } 3 { 'Windows CUI console' } default { 'Unknown' } }
"Subsystem=$subsystem ($kind)"
```

### 12.2. Внутренняя alpha-упаковка

Создать zip локально, но не коммитить его:

```powershell
$releaseDir = "D:\Документы\vscode\HotSAS Studio\apps\desktop-tauri\src-tauri\target\release"
$exe = Join-Path $releaseDir "hotsas_desktop_tauri.exe"
$zip = Join-Path $releaseDir "HotSAS-Studio-v1.10-internal-alpha-windows-x64.zip"
Compress-Archive -LiteralPath $exe -DestinationPath $zip -Force
Get-Item $zip | Select-Object FullName, Length, LastWriteTime
Get-FileHash $zip -Algorithm SHA256
```

Правила:

```text
- EXE/ZIP не добавлять в git.
- Не создавать GitHub Release.
- Не создавать публичный tag release.
- В verification log указать путь, размер, SHA256 и статус сборки.
```

---

## 13. Внутренний запуск на другом ПК

Если есть доступ ко второму Windows-ПК, выполнить:

```text
1. Скопировать HotSAS-Studio-v1.10-internal-alpha-windows-x64.zip.
2. Распаковать в локальную папку без кириллицы в пути, например C:\HotSAS-alpha\.
3. Запустить hotsas_desktop_tauri.exe.
4. Проверить, открывается ли окно.
5. Пройти minimal smoke:
   - Start screen opens
   - Schematic opens
   - Formula Library opens
   - Component Library opens
   - Simulation screen opens
   - Import Models opens
   - Export Center opens
   - Diagnostics opens
```

Если второго ПК нет — отметить в verification log:

```text
Second PC smoke test: NOT RUN
Reason: no second Windows machine available
```

Это не блокер для v1.10, но обязательно должно быть честно отражено.

---

## 14. Тесты

### 14.1. Rust tests

Добавить/обновить тесты:

```text
engine/application/tests/app_diagnostics_tests.rs
engine/api/tests/app_diagnostics_api_tests.rs
```

Минимум:

```text
- diagnostics report contains expected module IDs;
- component library module reports ready/limited, not panic;
- export center module reports 9 capabilities;
- simulation module handles ngspice unavailable as warning/limited, not error;
- import models module reports SPICE and Touchstone support;
- readiness self-check returns checks with pass/warn/fail/not_run statuses;
- API DTO conversion preserves module statuses.
```

### 14.2. Frontend tests

Добавить:

```text
apps/desktop-tauri/src/screens/__tests__/DiagnosticsScreen.test.tsx
```

Минимум:

```text
- renders diagnostics title;
- loads module cards;
- shows Ready/Limited/Unavailable statuses;
- Refresh diagnostics calls backend API;
- Run readiness self-check calls backend API;
- shows backend error message if command fails;
- does not automatically run heavy self-check on first render.
```

### 14.3. Existing tests

Все существующие тесты должны остаться зелёными:

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

---

## 15. Документация

Создать:

```text
docs/builds/INTERNAL_ALPHA_BUILD.md
```

Содержимое:

```text
- что такое internal alpha build;
- почему это не публичный релиз;
- как собрать `.exe`;
- где лежит `.exe`;
- как создать zip;
- как проверить SHA256;
- как запускать на другом ПК;
- какие ограничения есть до v2.0;
- known limitations.
```

Создать или обновить:

```text
docs/user_manual/QUICK_START_ALPHA.md
```

Содержимое:

```text
- как запустить внутреннюю alpha-сборку;
- какие экраны можно проверить;
- минимальный smoke test;
- что делать, если ngspice unavailable;
- что пока не поддерживается.
```

Обновить:

```text
README.md
docs/testing/TESTING.md
docs/testing/latest_verification_log.md
```

README после успешного v1.10:

```text
Current roadmap stage: v2.0 next / v2.0 beta preparation
Completed:
- v1.10 — Internal Alpha EXE Build & v2.0 Readiness Gate
```

Если команда проекта предпочитает не писать `v2.0 next` до отдельного решения — можно указать:

```text
Current roadmap stage: v2.0 preparation
```

Но обязательно отметить, что это не публичный релиз.

---

## 16. Verification log requirement

Обязательно создать отдельный файл:

```text
docs/testing/verification_logs/v1.10_internal_alpha_build_and_v2_readiness_gate.md
```

И обновить:

```text
docs/testing/latest_verification_log.md
```

В verification log обязательно включить:

```text
- Implementation commit
- Verification log update commit, если отдельный
- cargo fmt --check output summary
- cargo test output summary
- npm run format:check output summary
- npm run typecheck output summary
- npm run test output summary
- npm run build output summary
- npm run tauri:build output summary
- EXE path
- EXE size
- EXE SHA256
- ZIP path, если создан
- ZIP size
- ZIP SHA256
- Windows subsystem check result
- Manual UI smoke table
- Second PC smoke table or NOT RUN reason
- Agent self-check checklist
```

Агент обязан предоставить этот файл пользователю здесь для проверки.

---

## 17. Agent self-check

Перед завершением агент обязан пройти checklist:

```text
[ ] v1.9 состояние проверено перед изменениями.
[ ] Git status до изменений записан.
[ ] Никакие пользовательские untracked-файлы не удалены.
[ ] App diagnostics models добавлены/проверены.
[ ] AppDiagnosticsService добавлен/проверен.
[ ] API DTO/facade methods добавлены.
[ ] Tauri commands добавлены в generate_handler.
[ ] Frontend API/types/store обновлены.
[ ] Diagnostics/Internal Alpha screen добавлен.
[ ] Diagnostics screen подключён в навигацию.
[ ] Existing screens не сломаны.
[ ] React не содержит бизнес-логики диагностики beyond display/API calls.
[ ] ngspice unavailable обрабатывается controlled warning.
[ ] Tests добавлены/обновлены.
[ ] docs/builds/INTERNAL_ALPHA_BUILD.md создан.
[ ] docs/user_manual/QUICK_START_ALPHA.md создан или обновлён.
[ ] README обновлён.
[ ] TESTING.md обновлён.
[ ] latest_verification_log.md обновлён.
[ ] v1.10 verification log создан.
[ ] cargo fmt --check PASS.
[ ] cargo test PASS.
[ ] npm run format:check PASS.
[ ] npm run typecheck PASS.
[ ] npm run test PASS.
[ ] npm run build PASS.
[ ] npm run tauri:build PASS.
[ ] `.exe` собран.
[ ] `.exe` path/size/SHA256 записаны.
[ ] `.zip` internal alpha archive создан или причина отсутствия записана.
[ ] EXE/ZIP не добавлены в git.
[ ] Git commit создан.
[ ] Git push выполнен.
[ ] Публичный GitHub Release НЕ создан.
[ ] Публичный release tag НЕ создан.
[ ] Итоговый verification log предоставлен пользователю.
```

---

## 18. Commit / push

После успешных проверок:

```powershell
git status --short
git add README.md docs engine apps shared
git commit -m "v1.10 — Internal Alpha EXE Build and v2.0 Readiness Gate"
git push origin main
```

Если verification log обновляется отдельным коммитом:

```powershell
git add docs/testing/latest_verification_log.md docs/testing/verification_logs/v1.10_internal_alpha_build_and_v2_readiness_gate.md
git commit -m "docs(v1.10): update verification log with commit hash"
git push origin main
```

В итоговом отчёте указать:

```text
Implementation commit: <hash>
Verification log update commit: <hash or same>
Push: origin/main OK
```

---

## 19. Acceptance criteria

v1.10 считается выполненной только если:

```text
1. Все preflight-команды выполнены и записаны.
2. Диагностический экран/отчёт добавлен.
3. Пользователь видит readiness/status основных модулей.
4. Existing modules v1.0–v1.9 не сломаны.
5. Добавлены backend/frontend tests.
6. Созданы internal alpha build docs.
7. Создан user quick start alpha doc.
8. Создан отдельный verification log file.
9. latest_verification_log.md обновлён.
10. README обновлён до v2.0 preparation / v2.0 next.
11. cargo fmt --check PASS.
12. cargo test PASS.
13. npm run format:check PASS.
14. npm run typecheck PASS.
15. npm run test PASS.
16. npm run build PASS.
17. npm run tauri:build PASS.
18. `.exe` реально собран.
19. `.exe` path/size/SHA256 записаны.
20. Internal alpha zip создан или честно указано, почему нет.
21. EXE/ZIP не закоммичены.
22. Публичный GitHub Release не создан.
23. Изменения закоммичены и запушены.
24. Verification log предоставлен пользователю здесь.
```

---

## 20. Итоговый результат для пользователя

После выполнения v1.10 пользователь должен получить:

```text
- подтверждённую внутреннюю Windows alpha/dev-сборку `.exe`;
- документацию, как эту сборку собрать и проверить;
- диагностический экран состояния модулей;
- verification log с тестами и параметрами EXE;
- GitHub main с актуальным README и docs;
- готовность двигаться к v2.0 без публичного релиза.
```
