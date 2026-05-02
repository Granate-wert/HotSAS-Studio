# Preferred Values / E-Series

## Назначение

E-series используются для подбора стандартных номиналов компонентов (резисторов, конденсаторов, катушек индуктивности).

## Поддерживаемые ряды

- E3
- E6
- E12
- E24
- E48
- E96
- E192

## Реализация

Начиная с v1.1.5 все ряды используют статические таблицы.
E48/E96/E192 больше не генерируются приближённо через формулу.

## nearest

Возвращает ближайший номинал по абсолютной ошибке.

## lower

Возвращает ближайший номинал <= requested value (inclusive).

## higher

Возвращает ближайший номинал >= requested value (inclusive).

## error percent

```text
error = abs(selected - requested) / requested * 100
```

## Ограничения

- value должен быть положительным;
- NaN и Infinity запрещены;
- temperature/tolerance/manufacturer availability не учитываются.
