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
| Validate Binary Search Tree | Árboles | Requiere límites heredados, no solo comparar padre | pendiente | 2026-07-20 |
| Course Schedule | Grafos | Topological sort y ciclos dirigidos | repetido | 2026-07-23 |
| Coin Change | Programación dinámica | Estado de minimización con centinela | pendiente | 2026-07-26 |
| Partition Equal Subset Sum | Knapsack | Recorrido inverso para 0/1 | pendiente | 2026-07-27 |

## Simulacros

| Fecha | Problema | Patrón | Puntaje total | Acción siguiente |
| --- | --- | --- | --- | --- |
| 2026-07-12 | Minimum Window Substring | Sliding window | 24/25 | Repetir sin mirar notas el 2026-07-26 |
| 2026-07-12 | Course Schedule | Grafos | 23/25 | Repetir `find_course_order` desde cero el 2026-07-23 |
| Pendiente | Coin Change | Programación dinámica | Sin evaluar | Ejecutar `notes/simulations/simulacro-03-coin-change.md` |
| 2026-07-12 | Path With Minimum Effort | Grafos ponderados | 22/25 | Repetir explicación sin consultar notas el 2026-07-19 |
| 2026-07-12 | Longest Duplicate Substring | Cadenas avanzadas | 21/25 | Repetir con rolling hash o suffix array el 2026-07-20 |
| 2026-07-12 | Range Query Mix | Consultas por rangos | 24/25 | Repetir `RangeSumQuery` desde cero el 2026-07-21 |
