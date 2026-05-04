# ТЗ: HotSAS Studio v2.1 — Formula Library Expansion & Formula UX Hardening

## 0. Контекст текущего состояния

Проект HotSAS Studio дошёл до стадии:

```text
v2.0 — Product Beta Integration, Workflow Stabilization & Internal RC Build
```

Текущее состояние по последней проверке:

```text
Implementation commit: ca5d11e
Verification/docs commit: 3f05fb5

cargo fmt --check — PASS
cargo test — PASS, 245 Rust tests, 0 failures
npm run format:check — PASS
npm run typecheck — PASS
npm run test — PASS, 76 frontend tests, 0 failures
npm run build — PASS
npm run tauri:build — PASS

EXE:
apps/desktop-tauri/src-tauri/target/release/hotsas_desktop_tauri.exe
size: 12,337,152 bytes
SHA256: 15D1145D6031AD8A29B6404FFEEABD16A2CA87BFCC7A8FF46D15FECF8A8E064C

ZIP:
apps/desktop-tauri/src-tauri/target/release/HotSAS-Studio-v2.0-internal-rc-windows-x64.zip
size: 4,023,848 bytes
SHA256: A998030BBCD54A6D5E3FF512A473857A435D2F735624EB260860920AC7750FEA
```

Важное замечание по текущему состоянию:

```text
В полном v2.0 verification log осталось:
Git push completed: PENDING
Push status: PENDING

Но фактически commit 3f05fb5 уже находится в GitHub.
Перед началом v2.1 нужно исправить verification metadata.
```

---

## 1. Название этапа

```text
v2.1 — Formula Library Expansion & Formula UX Hardening
```

---

## 2. Главная цель этапа

Цель v2.1 — превратить Formula Library из базового набора формул в более полезную инженерную библиотеку для повседневных расчётов.

После v2.1 пользователь должен получить расширенную библиотеку формул с категориями, примерами, допущениями, ограничениями, validation hints и удобным UX для выбора/расчёта формул.

v2.1 не должен делать full symbolic solver, MathCAD/Wolfram, Lcapy/SymPy bridge или произвольный CAS. Это этап расширения curated formula packs и улучшения пользовательского workflow вокруг уже существующего FormulaEngine/FormulaRegistry.

---

## 3. Что изменится для пользователя

После выполнения v2.1 пользователь должен видеть в приложении:

```text
1. Formula Library станет заметно богаче:
   - базовые DC-цепи;
   - AC/импедансы;
   - RC/RL/RLC переходные процессы;
   - пассивные фильтры;
   - базовые op-amp топологии;
   - power/thermal helper formulas;
   - unit conversion / engineering helper formulas, если вписывается в текущий FormulaEngine.

2. В Formula Library появятся улучшенные категории:
   - Basic DC
   - AC & Impedance
   - Transient
   - Filters
   - Op-Amps
   - Power & Thermal
   - Utilities / Engineering Helpers

3. Для формулы пользователь должен видеть:
   - название;
   - категорию;
   - описание;
   - переменные;
   - единицы;
   - значения по умолчанию;
   - output values;
   - LaTeX/equation preview;
   - assumptions;
   - limitations;
   - examples;
   - warnings/validation hints.

4. Пользователь сможет:
   - искать формулы;
   - фильтровать по категории;
   - открыть details выбранной формулы;
   - ввести значения;
   - рассчитать результат через Rust backend;
   - увидеть результат с единицами;
   - увидеть предупреждения при некорректных входных данных;
   - отправить формулу/пример в Engineering Notebook, если это можно сделать без большого scope creep.

5. Product Beta / Guided Workflow должен показывать Formula Library как более зрелый модуль.
```

---

## 4. Жёсткие ограничения scope

### Запрещено

```text
- не делать full symbolic solver;
- не делать CAS;
- не подключать SymPy;
- не подключать Lcapy;
- не подключать math.js в frontend как расчётный движок;
- не переносить расчёты во frontend;
- не делать автоматический вывод формул для произвольных схем;
- не делать selected region symbolic transfer function beyond existing supported templates;
- не добавлять real PCB;
- не добавлять KiCad/Altium generation;
- не добавлять онлайн-загрузку formula packs;
- не добавлять внешнюю базу данных формул;
- не ломать существующие формулы:
  - rc_low_pass_cutoff
  - rc_high_pass_cutoff
  - ohms_law
  - voltage_divider
- не ломать Engineering Notebook;
- не ломать Product Beta workflow;
- не делать публичный GitHub Release/tag без отдельного разрешения пользователя.
```

### Разрешено

```text
- расширить YAML formula packs;
- добавить новые curated formulas;
- расширить FormulaDefinition metadata;
- добавить assumptions/limitations/examples;
- добавить validation hints;
- улучшить Formula Library UI;
- улучшить search/filter/grouping;
- добавить tests для новых formula packs;
- добавить tests для новых вычисляемых formulas;
- добавить docs по formula library v2.1;
- обновить Product Beta readiness;
- собрать внутренний EXE/ZIP, если изменения затрагивают UI/workflow;
- создать отдельный verification log;
- commit + push.
```

---

## 5. Обязательный preflight-fix по v2.0

Перед началом функциональных изменений v2.1 агент обязан исправить текущий documentation/verification debt.

### 5.1. Исправить полный v2.0 verification log

Файл:

```text
docs/testing/verification_logs/v2.0_product_beta_integration.md
```

Заменить:

```text
Git push completed: PENDING
Push status: PENDING
```

на:

```text
Git push completed: PASS
Push status: origin/main OK
Implementation commit: ca5d11e
Verification log update commit: 3f05fb5
```

Если в файле есть другие `PENDING`, связанные именно с Git push, тоже заменить на фактический статус.

### 5.2. Проверить latest verification log

Файл:

```text
docs/testing/latest_verification_log.md
```

Должен содержать краткую сводку v2.0:

```text
Version: v2.0 — Product Beta Integration
Implementation commit: ca5d11e
Verification log update commit: 3f05fb5
cargo test: PASS, 245 tests
npm test: PASS, 76 tests
tauri:build: PASS
EXE SHA256: ...
ZIP SHA256: ...
Public GitHub Release: NO
Public release tag: NO
```

Если такой сводки уже достаточно — не переписывать весь файл, только аккуратно дополнить.

### 5.3. TESTING.md

Проверить:

```text
docs/testing/TESTING.md
```

Добавить, если отсутствует, раздел:

```text
Manual v2.0 Product Beta Smoke Check
```

Минимальные пункты:

```text
[ ] App starts from EXE
[ ] Product Beta screen opens
[ ] Diagnostics opens
[ ] Guided workflow cards are visible
[ ] Formula Library opens
[ ] Component Library opens
[ ] Import Models opens
[ ] Simulation screen opens
[ ] Export Center opens
[ ] No crash on navigation between main screens
```

Если manual smoke не запускался, оставить `NOT RUN`, но сам чеклист должен быть в документации.

---

## 6. Preflight перед изменениями

Выполнить из корня проекта:

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
- Если git status не clean — записать это в verification log.
- Если есть untracked TZ/log/export/photo files пользователя — не трогать.
```

Базовые проверки перед работой:

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

## 7. Formula pack expansion

### 7.1. Где работать

Проверить существующие packs:

```text
shared/formula_packs/basic_electronics.yaml
shared/formula_packs/filters.yaml
shared/formula_packs/op_amp.yaml
shared/formula_packs/smps.yaml
```

Разрешено добавить новые файлы:

```text
shared/formula_packs/ac_impedance.yaml
shared/formula_packs/transient.yaml
shared/formula_packs/power_thermal.yaml
shared/formula_packs/utilities.yaml
```

Либо расширить существующие packs, если текущий loader лучше работает с меньшим количеством файлов.

### 7.2. Минимальный набор новых формул

Добавить не менее **25 новых формул**, но лучше 30–40, если это укладывается в текущий engine без хрупких костылей.

Обязательный минимум по категориям:

#### Basic DC

```text
1. power_from_voltage_current: P = V * I
2. power_from_current_resistance: P = I^2 * R
3. power_from_voltage_resistance: P = V^2 / R
4. series_resistors: R_total = R1 + R2
5. parallel_resistors: R_total = (R1 * R2) / (R1 + R2)
6. current_divider_two_resistors: I_R2 = I_total * R1 / (R1 + R2)
```

#### AC & Impedance

```text
7. capacitive_reactance: Xc = 1 / (2*pi*f*C)
8. inductive_reactance: Xl = 2*pi*f*L
9. rc_time_constant: tau = R * C
10. rl_time_constant: tau = L / R
11. series_rl_impedance_magnitude: |Z| = sqrt(R^2 + Xl^2)
12. series_rc_impedance_magnitude: |Z| = sqrt(R^2 + Xc^2)
```

#### Transient

```text
13. capacitor_charge_voltage: Vc = Vfinal * (1 - exp(-t/(R*C)))
14. capacitor_discharge_voltage: Vc = Vinitial * exp(-t/(R*C))
15. time_to_capacitor_threshold: t = -R*C*ln(1 - Vtarget/Vfinal)
16. inductor_current_rise: I = Ifinal * (1 - exp(-t*R/L))
17. inductor_current_decay: I = Iinitial * exp(-t*R/L)
```

Если текущий FormulaEngine не поддерживает `exp`, `ln`, `sqrt`, такие формулы можно добавить как `unsupported_expression` / placeholder formula definitions, но тогда нужно ясно показать в UI/documentation, что они registered but not yet evaluable.

Лучше в v2.1 добавить engine support для ограниченного набора math functions:

```text
sqrt()
exp()
ln()
log10()
pow()
```

Только если это можно сделать безопасно и покрыть тестами.

#### Filters

```text
18. rc_high_pass_cutoff: fc = 1 / (2*pi*R*C)
19. rl_low_pass_cutoff: fc = R / (2*pi*L)
20. rl_high_pass_cutoff: fc = R / (2*pi*L)
21. lc_resonant_frequency: f0 = 1 / (2*pi*sqrt(L*C))
22. rlc_quality_factor_series: Q = (1/R)*sqrt(L/C)
23. rlc_bandwidth: BW = f0 / Q
```

#### Op-Amps

```text
24. inverting_op_amp_gain: Av = -Rf / Rin
25. non_inverting_op_amp_gain: Av = 1 + Rf / Rg
26. op_amp_voltage_follower: Vout = Vin
27. summing_amplifier_two_inputs: Vout = -Rf*(V1/R1 + V2/R2)
28. differential_amplifier_gain: Vout = (R2/R1)*(V2 - V1)
29. integrator_time_constant: tau = R * C
30. differentiator_gain_factor: K = R * C
```

#### Power & Thermal

```text
31. resistor_temperature_rise: dT = P * theta
32. led_series_resistor: R = (Vs - Vf) / I
33. linear_regulator_power_loss: P = (Vin - Vout) * Iout
34. efficiency_percent: eta = Pout / Pin * 100
35. heatsink_required_theta: theta = (Tmax - Tambient) / P
```

### 7.3. Metadata для каждой новой формулы

Каждая новая формула должна иметь:

```yaml
id:
title:
category:
description:
variables:
equations:
outputs:
assumptions:
limitations:
examples:
```

Если текущая модель `FormulaDefinition` ещё не поддерживает `assumptions`, `limitations`, `examples`, нужно добавить backward-compatible optional fields.

Пример:

```yaml
  - id: power_from_voltage_current
    title: Power from Voltage and Current
    category: basic/dc
    description: Electrical power calculated from voltage and current.
    variables:
      V:
        unit: V
        description: Voltage
        default: 5
      I:
        unit: A
        description: Current
        default: 20m
    equations:
      - id: power
        latex: "P = V I"
        expression: "P = V * I"
        solve_for:
          - P
    outputs:
      P:
        unit: W
        description: Power
    assumptions:
      - "DC or RMS values are used consistently."
    limitations:
      - "Does not model transient or reactive power."
    examples:
      - title: "5 V at 20 mA"
        inputs:
          V: 5
          I: 20m
        expected_outputs:
          P: 100m
```

---

## 8. FormulaEngine v2.1 improvements

### 8.1. Проверить текущий SimpleFormulaEngine

Проверить фактический файл, где реализован generic formula evaluation.

Нужно определить, какие операции уже поддерживаются:

```text
+ - * /
parentheses
pi
variables
engineering values
```

### 8.2. Разрешённое расширение expression evaluator

Если текущий evaluator не поддерживает нужные функции, добавить минимум:

```text
sqrt(x)
exp(x)
ln(x)
log10(x)
pow(x, y)
abs(x)
```

Приоритет:

```text
1. sqrt
2. ln
3. exp
4. pow
5. abs/log10
```

Если добавление всего набора слишком рискованно, добавить только `sqrt`, `ln`, `exp`, потому что они нужны для LC/RLC/transient formulas.

### 8.3. Validation

Для формул добавить controlled validation:

```text
- missing variable → error;
- unsupported expression → controlled error, not panic;
- division by zero → error;
- negative value under sqrt → error;
- ln domain error → error;
- exp overflow → error or warning;
- NaN/Infinity output → error.
```

Frontend не должен вычислять эти ошибки сам. UI только показывает backend response.

---

## 9. Core/API DTO extensions

### 9.1. Formula metadata

Если ещё нет, добавить optional fields:

```rust
pub struct FormulaDefinition {
    ...
    pub assumptions: Vec<String>,
    pub limitations: Vec<String>,
    pub examples: Vec<FormulaExample>,
}
```

Если нельзя менять existing struct без широкого refactor, добавить `metadata: BTreeMap<String, String>` и DTO-level examples. Но предпочтительно сделать явные модели.

Пример моделей:

```rust
pub struct FormulaExample {
    pub title: String,
    pub inputs: BTreeMap<String, String>,
    pub expected_outputs: BTreeMap<String, String>,
    pub notes: Option<String>,
}
```

DTO:

```rust
pub struct FormulaExampleDto {
    pub title: String,
    pub inputs: Vec<FormulaExampleValueDto>,
    pub expected_outputs: Vec<FormulaExampleValueDto>,
    pub notes: Option<String>,
}

pub struct FormulaExampleValueDto {
    pub name: String,
    pub value: String,
}
```

### 9.2. Formula calculation result warnings

Если в result DTO нет `warnings`, добавить:

```rust
pub warnings: Vec<String>
```

или использовать уже существующий warnings/error model.

---

## 10. Application layer

Расширить/проверить:

```text
FormulaRegistryService
FormulaService
ProductWorkflowService / ProductBeta readiness, если он есть
```

### 10.1. Formula registry

Должен уметь:

```text
- загрузить все новые packs;
- вернуть categories;
- искать по title/id/category/tags/description;
- отдать details с assumptions/limitations/examples;
- не падать на unsupported placeholder formulas.
```

### 10.2. Formula calculation

Должен уметь:

```text
- рассчитать новые supported formulas;
- вернуть controlled unsupported для formulas, которые пока не поддержаны evaluator;
- корректно показывать missing inputs;
- корректно обрабатывать invalid values.
```

### 10.3. Product Beta readiness

Обновить v2.0 Product Beta / diagnostics readiness:

```text
Formula Library module:
- formula pack count updated;
- formula count updated;
- supported formula count;
- unsupported/placeholder formula count;
- warning if formulas loaded but some are not evaluable.
```

---

## 11. API / Tauri

Добавить или проверить методы:

```rust
list_formula_packs
list_formulas
list_formula_categories
get_formula
calculate_formula
search_formulas
```

Если `search_formulas` отсутствует, добавить:

```rust
pub fn search_formulas(
    &self,
    query: FormulaSearchRequestDto,
) -> Result<FormulaSearchResultDto, ApiError>
```

Tauri command:

```rust
#[tauri::command]
async fn search_formulas(...)
```

Если UI уже фильтрует локально после `list_formulas`, можно не добавлять command, но backend search предпочтительнее для будущего роста.

---

## 12. Frontend UX

### 12.1. Formula Library Screen

Улучшить `FormulaLibraryScreen`:

```text
- category sidebar or category chips;
- search box;
- formula count;
- formula pack metadata;
- selected formula details;
- assumptions section;
- limitations section;
- examples section;
- variables input table;
- calculate button;
- result card;
- warnings/errors card;
- unsupported expression badge if formula exists but not evaluable yet.
```

### 12.2. Examples UX

Для формулы с examples:

```text
- показывать список examples;
- кнопка "Use example inputs";
- подставлять values в input fields;
- пользователь нажимает Calculate;
- результат должен совпадать с expected output в допустимой точности.
```

### 12.3. Notebook integration — optional but recommended

Если уже есть стабильные notebook API:

```text
- кнопка "Send to Notebook" / "Add formula call to Notebook";
- создаёт notebook input вроде:
  power_from_voltage_current(V=5, I=20m)
```

Если это слишком большой scope, оставить documented TODO и не делать.

### 12.4. Product Beta screen

Обновить Product Beta / Guided Workflow:

```text
- Formula Library card должна показывать updated formula count;
- Formula expansion статус должен быть visible;
- readiness self-check должен учитывать formula packs.
```

---

## 13. Tests — Rust

Добавить tests.

### 13.1. Formula pack loader tests

```text
engine/application/tests/formula_library_expansion_tests.rs
```

Проверить:

```text
- все новые YAML packs загружаются;
- нет duplicate formula ids;
- у каждой формулы есть title/category/description;
- у каждой supported formula есть variables/equations/outputs;
- assumptions/limitations/examples корректно парсятся;
- category list содержит expected categories.
```

### 13.2. Formula calculation tests

Добавить тесты минимум для 15 supported formulas:

```text
- power_from_voltage_current: 5 V * 20m A = 0.1 W
- power_from_current_resistance: 2m A, 10k Ohm = 0.04 W
- power_from_voltage_resistance: 5 V, 10k Ohm = 0.0025 W
- series_resistors: 1k + 2k = 3k
- parallel_resistors: 10k || 10k = 5k
- capacitive_reactance: f=1k, C=100n -> approx 1591.55 Ohm
- inductive_reactance: f=1k, L=10m -> approx 62.8319 Ohm
- rc_time_constant: 10k * 100n = 1ms
- rl_time_constant: L/R
- lc_resonant_frequency: L=10m, C=100n -> approx 5032.92 Hz
- inverting_op_amp_gain: Rf=100k, Rin=10k -> -10
- non_inverting_op_amp_gain: Rf=90k, Rg=10k -> 10
- led_series_resistor: Vs=5, Vf=2, I=20m -> 150 Ohm
- linear_regulator_power_loss: Vin=12, Vout=5, Iout=0.5 -> 3.5 W
- efficiency_percent: Pout=8, Pin=10 -> 80 %
```

### 13.3. Error tests

```text
- missing variable;
- division by zero;
- sqrt negative;
- ln invalid;
- unsupported expression;
- invalid engineering value;
- NaN/Infinity blocked.
```

### 13.4. API tests

```text
engine/api/tests/formula_library_expansion_api_tests.rs
```

Проверить:

```text
- list_formula_categories returns expanded categories;
- get_formula returns assumptions/limitations/examples;
- calculate_formula returns warnings/errors correctly;
- search_formulas returns relevant formulas.
```

---

## 14. Tests — frontend

Добавить/обновить:

```text
apps/desktop-tauri/src/screens/__tests__/FormulaLibraryScreen.test.tsx
```

Минимум 8 тестов:

```text
1. renders expanded Formula Library title and counts;
2. shows category filters;
3. filters formulas by category;
4. search finds formula by title/id;
5. selecting formula shows assumptions/limitations;
6. example button fills input values;
7. calculate displays result card;
8. unsupported formula displays controlled badge/warning;
9. backend error displays error card;
10. Product Beta formula count/readiness visible if relevant.
```

---

## 15. Documentation

Создать:

```text
docs/formula_library/FORMULA_LIBRARY_EXPANSION_V2_1.md
```

Документ должен описывать:

```text
- цель v2.1;
- новые категории;
- количество формул;
- supported vs placeholder formulas;
- supported expression functions;
- examples;
- assumptions/limitations;
- frontend workflow;
- known limitations.
```

Обновить:

```text
README.md
docs/testing/TESTING.md
docs/testing/latest_verification_log.md
```

README после завершения:

```text
Current roadmap stage: v2.2 next

Completed:
- v2.1 — Formula Library Expansion & Formula UX Hardening
```

---

## 16. Verification log

Агент обязан создать отдельный файл:

```text
docs/testing/verification_logs/v2.1_formula_library_expansion.md
```

Файл должен содержать полный лог:

```text
- дата;
- branch;
- implementation commit;
- verification log update commit;
- git status before/after;
- cargo fmt --check output;
- cargo test output;
- formula-focused cargo tests output;
- npm run format:check output;
- npm run typecheck output;
- npm run test output;
- npm run build output;
- npm run tauri:build output;
- EXE path;
- EXE size;
- EXE SHA256;
- ZIP path, если создаётся;
- ZIP size;
- ZIP SHA256;
- public release created: NO;
- release tag created: NO;
- manual smoke test status;
- self-check по пунктам ТЗ.
```

Важно:

```text
Даже если EXE/ZIP собирается, не создавать публичный GitHub Release/tag без отдельного разрешения пользователя.
```

---

## 17. Обязательные проверки

После реализации выполнить:

```bash
cd "D:\Документы\vscode\HotSAS Studio\engine"
cargo fmt --check
cargo test
cargo test formula
cargo test formula_library
```

```bash
cd "D:\Документы\vscode\HotSAS Studio\apps\desktop-tauri"
npm.cmd run format:check
npm.cmd run typecheck
npm.cmd run test
npm.cmd run build
npm.cmd run tauri:build
```

Если какие-то focused test names не совпадают с реальными, использовать фактические имена тестов и указать это в verification log.

---

## 18. Manual smoke check

Если есть доступ к GUI:

```text
[ ] App starts
[ ] Product Beta screen opens
[ ] Formula Library opens
[ ] Expanded categories visible
[ ] Search works
[ ] Category filter works
[ ] Select formula works
[ ] Assumptions/limitations/examples visible
[ ] Use example inputs works
[ ] Calculate works for at least:
    [ ] Ohm’s law
    [ ] Power formula
    [ ] RC time constant
    [ ] LC resonant frequency
    [ ] Op-amp gain
[ ] Unsupported formula shows controlled warning
[ ] Product Beta readiness still opens
[ ] Export Center still opens
[ ] Diagnostics still opens
```

Если GUI недоступен:

```text
Manual UI smoke test: NOT RUN
Reason: ...
```

---

## 19. Git

После успешных проверок:

```bash
git status --short
git add .
git commit -m "v2.1 — Formula Library Expansion and Formula UX Hardening"
git push origin main
```

Если verification log commit делается отдельно:

```bash
git commit -m "docs(v2.1): add formula library verification log"
git push origin main
```

В verification log обязательно указать:

```text
Implementation commit: <hash>
Verification log update commit: <hash>
Push status: origin/main OK
```

---

## 20. Agent self-check

В конце агент обязан пройти checklist и записать его в verification log:

```text
[ ] v2.0 PENDING push metadata fixed
[ ] latest_verification_log.md updated
[ ] TESTING.md updated
[ ] Formula packs expanded
[ ] At least 25 new formulas added
[ ] No duplicate formula ids
[ ] Categories visible in UI
[ ] Formula examples supported in DTO/UI
[ ] Assumptions/limitations visible in UI
[ ] Supported formulas calculate through Rust backend
[ ] Frontend does not calculate formulas
[ ] Unsupported formulas return controlled warnings/errors
[ ] Product Beta readiness updated
[ ] README updated to v2.2 next
[ ] v2.1 verification log created
[ ] cargo fmt --check PASS
[ ] cargo test PASS
[ ] npm format:check PASS
[ ] npm typecheck PASS
[ ] npm test PASS
[ ] npm build PASS
[ ] tauri:build PASS
[ ] EXE path/size/SHA256 recorded
[ ] Public GitHub Release not created
[ ] Public release tag not created
[ ] Commit and push completed
```

---

## 21. Acceptance criteria

v2.1 считается выполненной, если:

```text
1. Исправлен documentation debt v2.0.
2. Formula Library содержит расширенный curated formula set.
3. Добавлено минимум 25 новых формул.
4. Новые formulas загружаются из YAML packs.
5. Новые supported formulas считаются через Rust backend.
6. UI показывает категории, поиск, details, examples, assumptions, limitations.
7. Product Beta/Diagnostics учитывают расширенную Formula Library.
8. Все тесты проходят.
9. EXE собран через tauri:build.
10. Создан отдельный verification log.
11. README/TESTING/latest log обновлены.
12. Изменения закоммичены и запушены в origin/main.
```

---

## 22. Следующий этап после v2.1

После принятия v2.1 следующий логичный этап:

```text
v2.2 — Component Library Expansion & Pin Mapping UX
```

Предварительный смысл v2.2:

```text
- расширить component library;
- добавить больше реальных/типовых component definitions;
- улучшить pin mapping UX для imported SPICE models;
- связать imported models с components более удобно;
- улучшить BOM/export readiness.
```
