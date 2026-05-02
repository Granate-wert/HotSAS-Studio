# HotSAS Studio — Testing Guide

## Назначение

Этот документ описывает базовые команды проверки проекта HotSAS Studio.

## Проверка Rust engine

```bash
cd "D:\Документы\vscode\HotSAS Studio\engine"
cargo fmt --check
cargo test
```

Если нужно автоматически отформатировать Rust-код:

```bash
cargo fmt
```

## Проверка frontend

```bash
cd "D:\Документы\vscode\HotSAS Studio\apps\desktop-tauri"
npm.cmd install
npm.cmd run format:check
npm.cmd run typecheck
npm.cmd run build
```

Если нужно автоматически отформатировать frontend-код:

```bash
npm.cmd run format
```

## Запуск Tauri dev

```bash
cd "D:\Документы\vscode\HotSAS Studio\apps\desktop-tauri"
npm.cmd run tauri:dev
```

## Почему используется npm.cmd

На текущей Windows PowerShell среде `npm.ps1` может блокироваться Execution Policy.
Поэтому для команд используется `npm.cmd`.

## Минимальная проверка v1 vertical slice вручную

1. Запустить приложение через `npm.cmd run tauri:dev`.
2. Открыть стартовый экран.
3. Создать RC low-pass demo project.
4. Проверить, что схема отображается.
5. Рассчитать `fc`.
6. Получить ближайшее значение E24.
7. Сгенерировать SPICE netlist.
8. Запустить mock AC simulation.
9. Проверить, что график отображается.
10. Экспортировать Markdown report.
11. Экспортировать HTML report.
12. Сохранить JSON project.

## Backend test coverage

v1.1.2 covers:

- EngineeringValue parsing;
- Preferred E-series;
- RC low-pass formula;
- Circuit template and formula binding;
- SPICE netlist export;
- Markdown/HTML report export;
- JSON storage;
- API error DTO;
- Full backend vertical slice.

## Команды перед коммитом

```bash
cd "D:\Документы\vscode\HotSAS Studio\engine"
cargo fmt --check
cargo test

cd "D:\Документы\vscode\HotSAS Studio\apps\desktop-tauri"
npm.cmd run format:check
npm.cmd run typecheck
npm.cmd run build
```
