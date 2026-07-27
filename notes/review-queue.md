# Cola de Repaso

## Cómo Usarla

Registrar aquí problemas para repetir después de simulacros o errores detectados.

Estados:

- `pendiente`: aún no se repite.
- `repetido`: ya se resolvió otra vez.
- `dominado`: salió limpio bajo tiempo.

## Prioridad Alta

| Problema | Patrón | Motivo | Estado | Repetir |
| --- | --- | --- | --- | --- |
| Minimum Window Substring | Sliding window | Ventana y conteos suelen romperse al contraer | repetido | 2026-07-26 |
| Validate Binary Search Tree | Árboles | Requiere límites heredados, no solo comparar padre | repetido | 2026-07-20 |
| Course Schedule | Grafos | Topological sort y ciclos dirigidos | repetido | 2026-07-23 |
| Coin Change | Programación dinámica | Estado de minimización con centinela | repetido | 2026-07-26 |
| Partition Equal Subset Sum | Knapsack | Recorrido inverso para 0/1 | repetido | 2026-07-27 |

## Simulacros

| Fecha | Problema | Patrón | Puntaje total | Acción siguiente |
| --- | --- | --- | --- | --- |
| 2026-07-12 | Minimum Window Substring | Sliding window | 24/25 | Repetir sin mirar notas el 2026-07-26 |
| 2026-07-12 | Course Schedule | Grafos | 23/25 | Repetir `find_course_order` desde cero el 2026-07-23 |
| 2026-07-12 | Coin Change | Programación dinámica | 24/25 | Repetir con monedas no positivas y monto grande el 2026-07-26 |
| 2026-07-12 | Path With Minimum Effort | Grafos ponderados | 24/25 | Repetido con matriz de una sola fila; explicar Dijkstra minimax el 2026-07-19 |
| 2026-07-12 | Longest Duplicate Substring | Cadenas avanzadas | 21/25 | Repetir con rolling hash o suffix array el 2026-07-20 |
| 2026-07-12 | Range Query Mix | Consultas por rangos | 24/25 | Repetir `RangeSumQuery` desde cero el 2026-07-21 |

## Bloque 241-260

| Rango | Fuente | Estado | Acción |
| --- | --- | --- | --- |
| 241-246 | Repeticiones y simulacros existentes | Evidencia parcial documentada | Revisar notas existentes y crear notas individuales donde falte trazabilidad. |
| 247-260 | Simulacros con acción pendiente y candidatos reservados | Pendiente | Ejecutar como repeticiones dirigidas sin mirar solución antes de comparar. |

Este bloque se detalla en
[`plan/repeticiones-241-260.md`](../plan/repeticiones-241-260.md). Ningún
problema pendiente debe marcarse como dominado sin retro y validación.

## Profundización 261-330

| Rango | Enfoque | Estado | Acción |
| --- | --- | --- | --- |
| 261-290 | Patrones compuestos | Pendiente | Seleccionar una combinación, defender la alternativa simple y registrar su invariante. |
| 291-315 | Medium y Hard con alta señal | Pendiente | Ejecutar bajo TDD y cubrir el caso borde propio de la familia. |
| 316-330 | Patrones mixtos y brechas | Pendiente | Priorizar strings, grafos, DP, rangos o greedy según la señal de simulacro. |

Los tres rangos están formalizados como práctica futura; no representan
problemas dominados ni contenido publicado. Los detalles viven en
[`plan/profundizacion-261-290.md`](../plan/profundizacion-261-290.md),
[`plan/profundizacion-291-315.md`](../plan/profundizacion-291-315.md) y
[`plan/profundizacion-316-330.md`](../plan/profundizacion-316-330.md).

## Cierre 331-400

| Rango | Enfoque | Estado | Acción |
| --- | --- | --- | --- |
| 331-360 | Hard selectivos | Pendiente | Elegir por técnica transferible y explicar la reducción antes de implementar. |
| 361-390 | Práctica, repeticiones y simulacros | Pendiente | Ejecutar bajo TDD; documentar puntaje y retro si se usa como simulacro. |
| 391-400 | Auditoría y revisión humana | Pendiente | Registrar evidencia real y separar plan, práctica y contenido publicable. |

El bloque final se detalla en [`plan/cierre-361-400.md`](../plan/cierre-361-400.md).
Nada de este tramo debe marcarse como dominado, revisado o publicado sin la
evidencia y la revisión correspondientes.
