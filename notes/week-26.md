# Semana 26: Geometría y Temas Selectivos

## Objetivo

Practicar geometría computacional con una base pequeña, explicable y reusable para problemas de puntos, líneas y barridos.

## Estado del Avance Actual

- Problemas implementados acumulados: 111.
- Tests automatizados acumulados: 266.
- Módulo agregado: `src/patterns/geometry.rs`.
- Tests agregados: `tests/geometry_test.rs`.

## Problemas Implementados

| Problema | Patrón | API | Estado |
| --- | --- | --- | --- |
| Producto cruz | Geometría básica | `cross_product` | implementado |
| Orientación de puntos | Producto cruz | `orientation` | implementado |
| Convex Hull / Erect the Fence | Monotonic chain | `convex_hull` | implementado |

## Ideas Clave

- El producto cruz indica si tres puntos giran en sentido horario, antihorario o son colineales.
- `convex_hull` usa monotonic chain: ordena puntos, construye borde inferior y borde superior.
- Para conservar puntos colineales del borde, se eliminan solo los giros horarios estrictos.
- El resultado de `convex_hull` se devuelve sin duplicados para que sea estable y fácil de comparar.

## Complejidad

| Función | Tiempo | Espacio |
| --- | ---: | ---: |
| `cross_product` | `O(1)` | `O(1)` |
| `orientation` | `O(1)` | `O(1)` |
| `convex_hull` | `O(n log n)` | `O(n)` |

## Criterio de Cierre

- Producto cruz clasifica giros positivos, negativos y colineales.
- Orientación devuelve `Clockwise`, `CounterClockwise` o `Collinear`.
- Convex Hull conserva puntos colineales en el borde.
- Convex Hull maneja duplicados, entrada vacía y entradas pequeñas.
- README y wiki apuntan al bloque.
- `cargo fmt` y `cargo test` pasan.

## Siguiente Paso

Implementar `K Closest Points to Origin` y luego avanzar hacia `Max Points on a Line`.
