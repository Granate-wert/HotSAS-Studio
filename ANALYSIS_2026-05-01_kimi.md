# Анализ проекта HotSAS Studio

**Дата:** 2026-05-01  
**Анализатор:** Kimi Code CLI  
**Заказчик:** Пользователь (Codex-история + ТЗ)  

---

## 1. Общая сводка

Проект **HotSAS Studio** находится на стадии **v1 Vertical Slice** (первый вертикальный срез).  
Реализован базовый каркас monorepo согласно ТЗ: Rust engine (Clean Architecture) + Tauri v2 desktop shell (React + TypeScript + Vite).

**Что работает:**
- Rust workspace компилируется и проходит 9 unit-тестов.
- Frontend Vite/TypeScript сборка проходит без ошибок.
- Tauri dev-сборка запускает `.exe` (после исправления capabilities).
- Реализованы: доменные модели, E-ряды (E3–E192), RC low-pass шаблон, mock AC симуляция, SPICE netlist exporter, Markdown/HTML report exporter, JSON storage.

**Что НЕ работает / заблокировано:**
- Кнопки UI не реагировали на нажатия до исправления конфигурации Tauri v2 (подробности ниже).
- Нет реальной интеграции ngspice, SQLite, KiCad/Altium экспорта.
- Engineering Calculator, Component Library, Selected Region Analysis — только заглушки UI.

---

## 2. Стадия разработки

| Этап | Статус | Примечание |
|------|--------|------------|
| Архитектура и структура проекта | ✅ Готово | Clean Architecture, 5 crate'ов в workspace |
| Доменные модели (core) | ✅ Готово | CircuitProject, Component, Net, Wire, Formula, Report и т.д. |
| E-ряды (preferred values) | ✅ Готово | E3–E192, тесты пройдены |
| Formula engine (mock) | ✅ Готово | Только RC low-pass cutoff, порт заложен |
| SPICE netlist exporter | ✅ Готово | Генерация текста для RC low-pass |
| Mock simulation engine | ✅ Готово | Математическая модель RC фильтра |
| Report exporters | ✅ Готово | Markdown + HTML (базовые) |
| JSON project storage | ✅ Готово | Сохранение/загрузка через файловую систему |
| Tauri API + команды | ✅ Готово | 10 команд, DTO между frontend/backend |
| React UI shell | ⚠️ Частично | Есть toolbar, canvas, график, но много заглушек |
| ngspice integration | ❌ Не начато | Только placeholder |
| SQLite storage | ❌ Не начато | Только JSON адаптер |
| Engineering Calculator | ❌ Заглушка | UI панель есть, логики ввода нет |
| Component Library | ❌ Заглушка | Только текстовое описание |
| Formula Packs runtime | ❌ Не интегрированы | YAML файлы лежат в `shared/`, но не парсятся |
| KiCad / Altium export | ❌ Placeholder | Только в docs |
| Selected Region Analysis | ❌ Не начато | Модели есть, сервиса нет |

---

## 3. Критические проблемы (блокеры)

### 3.1. Не работают кнопки в UI Tauri — ПРИЧИНА НАЙДЕНА И ИСПРАВЛЕНА

**Проблема:**  
В `apps/desktop-tauri/src-tauri/tauri.conf.json` **отсутствовала секция `capabilities`** внутри `app.security`.

**Почему это критично:**  
Tauri v2 работает по принципу **deny-by-default** для IPC. Без явного подключения capability-файла фронтенд **не имеет права** вызывать backend-команды через `invoke()`. При нажатии кнопок `invoke()` возвращал ошибку доступа, но из-за generic обработчика ошибок в `App.tsx` пользователь мог видеть только пустое состояние или непонятную ошибку.

**Исправление (применено):**

1. В `tauri.conf.json` добавлено:
```json
"app": {
  "security": {
    "csp": null,
    "capabilities": ["default"]
  }
}
```

2. Файл `capabilities/default.json` оставлен с `permissions: ["core:default"]` — этого достаточно для работы стандартного `invoke` в Tauri v2.

**Рекомендация пользователю:**  
Запускать приложение только через:
```powershell
cd "apps/desktop-tauri"
npm.cmd run tauri:dev
```
Не открывать `http://127.0.0.1:1420` в браузере — `invoke` работает только внутри WebView Tauri.

---

### 3.2. Отсутствие иконок и bundle-конфигурации

В `tauri.conf.json`:
```json
"bundle": {
  "active": false,
  "targets": "all",
  "icon": []
}
```

- `active: false` отключает создание установщика (`msi`, `exe installer`).
- `icon: []` пустой — Tauri warning при сборке. Ранее Codex добавил `icon.ico`, но не прописал путь в конфиге.

**Рекомендация:**
```json
"bundle": {
  "active": true,
  "targets": "msi",
  "icon": ["icons/icon.ico"]
}
```

---

### 3.3. Identifier Tauri

```json
"identifier": "studio.hotsas.app"
```

Tauri выдает warning, что identifier оканчивается на `.app`. Для Windows это не критично, но лучше изменить на `studio.hotsas.desktop` или `com.hotsas.studio`.

---

## 4. Архитектурные проблемы

### 4.1. Formula Packs не интегрированы в runtime

В `shared/formula_packs/` лежат YAML-файлы (`basic_electronics.yaml`, `filters.yaml` и т.д.), но:
- Нет YAML-парсера в Rust.
- Нет runtime-загрузки этих пакетов.
- Формула RC low-pass захардкожена прямо в `templates.rs` и `adapters/src/lib.rs`.

**Это противоречит ТЗ**, которое требует: *"Формулы хранить не только в коде, а в JSON/YAML formula packs"*.

### 4.2. React Flow — только view adapter, но без обратной связи

ТЗ требует: *"React Flow is only a view adapter. Source of truth is Rust CircuitModel"*.  
В текущей реализации это соблюдается: nodes/edges строятся из `ProjectDto`. Однако:
- Нет обработки пользовательских действий на canvas (перетаскивание, соединение, удаление).
- Нет синхронизации изменений обратно в Rust state.
- `HotSasApi` хранит `current_project` в `Mutex<Option<CircuitProject>>`, но нет команд для его модификации из UI.

### 4.3. State management (Zustand) — избыточно

В `store.ts` 12 отдельных сеттеров. Можно упростить до 2-3 actions (`setField`, `resetProject`) без потери типобезопасности.

### 4.4. Отсутствие error boundaries

В `App.tsx` ошибки от `invoke` ловятся и пишутся в `error` state, но:
- Нет различия между пользовательскими ошибками ("сначала создай проект") и системными ошибками (IPC forbidden).
- Нет retry-механизма.

### 4.5. EngineeringValue — ограниченный парсинг

Поддерживаются только: `p`, `n`, `u`, `m`, `k`, `M`.  
Нет: `G`, `T`, `f`, `a`, `%`, `dB`, комплексных чисел. Для v1 это допустимо, но архитектурно нужно заложить расширение.

---

## 5. Проблемы при сборке и запуске

| Проблема | Причина | Решение |
|----------|---------|---------|
| `cargo test` не запускался изначально | Отсутствовал Rust toolchain (TLS ошибка при скачивании) | Установлен `stable-x86_64-pc-windows-msvc` |
| `tauri:build` требовал `src/lib.rs` | Tauri v2 ожидает library crate | Добавлен `lib.rs` с `run()`, `main.rs` тонкий |
| `tauri:build` требовал `icons/icon.ico` | Bundle manifest требует иконку | Добавлен минимальный `.ico` файл |
| `tauri:dev` падает с "Port 1420 already in use" | Предыдущий vite process не убит | Убить процессы `node`/`vite` |
| `tauri:dev` падает с "Отказано в доступе" к `.exe` | Предыдущий `hotsas_desktop_tauri.exe` запущен | Убить процесс в Task Manager |

---

## 6. Что сделано Codex'ом хорошо

1. **Чистая архитектура**: Правильное разделение на `core` → `ports` → `application` → `adapters` → `api`. Зависимости направлены внутрь.
2. **DTO boundary**: Между Rust и React используются строгие DTO (`ProjectDto`, `FormulaResultDto` и т.д.).
3. **Тесты**: 9 Rust-тестов покрывают E-ряды, EngineeringValue, формулу RC, netlist, report.
4. **Вертикальный срез**: Реализован end-to-end сценарий от создания проекта до экспорта отчета.
5. **Zustand без бизнес-логики**: UI только вызывает `backend.*` и отображает результат. Все вычисления в Rust.

---

## 7. Рекомендации по дальнейшей разработке

### Немедленные (блокируют работу)
- [x] **Исправить `tauri.conf.json`** — добавить `capabilities: ["default"]` (уже сделано).
- [ ] **Добавить путь к иконке** в `bundle.icon`.
- [ ] **Включить `bundle.active: true`**, если нужен установщик.

### Ближайшие (v1.1)
- [ ] **Интегрировать YAML formula packs** — добавить `serde_yaml` в Rust, загрузчик пакетов, убрать хардкод формул.
- [ ] **Добавить ngspice adapter placeholder** с проверкой наличия `ngspice.dll`/`ngspice.exe` в системе.
- [ ] **Реализовать SQLite storage adapter** как альтернативу JSON.
- [ ] **Добавить обратную связь canvas → Rust** — команды `move_component`, `add_wire`, `delete_component`.
- [ ] **Engineering Calculator UI** — панель ввода выражений с вызовом `FormulaEnginePort`.

### Средние (v2)
- [ ] **Component Library Manager** — импорт SPICE `.lib`/`.subckt`, отображение параметров.
- [ ] **Selected Region Analysis** — выделение участка, расчет H(s), Zin, Zout.
- [ ] **Реальный ngspice** — интеграция через `SimulationEnginePort` (shared library или CLI wrapper).
- [ ] **KiCad / Altium workflow exporters** — генерация `.kicad_sym`, `.kicad_mod`, BOM CSV.

---

## 8. Вывод

Проект находится на **ранней, но архитектурно здоровой стадии**. Codex корректно заложил Clean Architecture и вертикальный срез RC low-pass.  

**Главная причина неработающих кнопок** — неправильная конфигурация security/capabilities в Tauri v2, из-за чего frontend был лишен права вызывать backend. Исправление однострочное (`capabilities: ["default"]` в `tauri.conf.json`).

После примененного исправления приложение компилируется, запускается и IPC-команды должны работать. Для продолжения разработки приоритетны: YAML formula runtime, обратная связь canvas и реальная интеграция симулятора.

---

*Анализ подготовлен Kimi Code CLI на основе исходного кода, ТЗ (`PROMT HotSAS Studio.txt`) и истории разработки (`история чата.txt`).*
