# Semana 26: Geometría y Temas Selectivos

## Objetivo

Practicar geometría computacional con una base pequeña, explicable y reusable para problemas de puntos, líneas y barridos.

## Estado del Avance Actual

- Problemas implementados acumulados: 119.
- Tests automatizados acumulados: 300.
- Módulo agregado: `src/patterns/geometry.rs`.
- Tests agregados: `tests/geometry_test.rs`.

## Problemas Implementados

| Problema | Patrón | API | Estado |
| --- | --- | --- | --- |
| Producto cruz | Geometría básica | `cross_product` | implementado |
| Orientación de puntos | Producto cruz | `orientation` | implementado |
| Convex Hull / Erect the Fence | Monotonic chain | `convex_hull` | implementado |
| K Closest Points to Origin | Ordenamiento por distancia | `k_closest_points` | implementado |
| Max Points on a Line | Pendiente normalizada | `max_points_on_a_line` | implementado |
| The Skyline Problem | Barrido de línea + multiset | `get_skyline` | implementado |

## Ideas Clave

- El producto cruz indica si tres puntos giran en sentido horario, antihorario o son colineales.
- `convex_hull` usa monotonic chain: ordena puntos, construye borde inferior y borde superior.
- Para conservar puntos colineales del borde, se eliminan solo los giros horarios estrictos.
- El resultado de `convex_hull` se devuelve sin duplicados para que sea estable y fácil de comparar.
- Para puntos cercanos al origen basta comparar distancia cuadrada; no hace falta calcular raíces.
- En empates de distancia, ordenar por coordenadas vuelve el resultado determinista.
- Max Points on a Line usa pendientes reducidas por `gcd` para evitar errores de precisión.
- Las líneas verticales se normalizan como `(1, 0)` y las horizontales como `(0, 1)`.
- Skyline convierte edificios en eventos de inicio y fin, mantiene alturas activas y emite puntos solo cuando cambia la altura máxima.

## Complejidad

| Función | Tiempo | Espacio |
| --- | ---: | ---: |
| `cross_product` | `O(1)` | `O(1)` |
| `orientation` | `O(1)` | `O(1)` |
| `convex_hull` | `O(n log n)` | `O(n)` |
| `k_closest_points` | `O(n log n)` | `O(n)` |
| `max_points_on_a_line` | `O(n^2)` | `O(n)` |
| `get_skyline` | `O(n log n)` | `O(n)` |

## Criterio de Cierre

- Producto cruz clasifica giros positivos, negativos y colineales.
- Orientación devuelve `Clockwise`, `CounterClockwise` o `Collinear`.
- Convex Hull conserva puntos colineales en el borde.
- Convex Hull maneja duplicados, entrada vacía y entradas pequeñas.
- K Closest Points maneja empates, `k` mayor al tamaño y entrada vacía.
- Max Points on a Line maneja duplicados, líneas verticales, líneas horizontales y entradas pequeñas.
- Skyline maneja edificios traslapados, edificios contiguos con misma altura y entradas inválidas.
- README y wiki apuntan al bloque.
- `cargo fmt` y `cargo test` pasan.

## Siguiente Paso

Iniciar la fase 6 con simulacros avanzados y repetición dirigida.
