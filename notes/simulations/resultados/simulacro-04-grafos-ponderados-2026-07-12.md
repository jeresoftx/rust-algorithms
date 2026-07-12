# Resultado Simulacro 04: Grafos Ponderados

Fecha: 2026-07-12.

## Problema

Path With Minimum Effort.

## Resultado

Aprobado.

Puntaje total: 22/25.

| Dimensión | Puntaje 1-5 | Evidencia |
| --- | ---: | --- |
| Clarificación | 4 | Se validaron matriz vacía, celda única, movimientos sin diagonal y salida esperada. |
| Enfoque | 5 | Dijkstra minimax modela el esfuerzo como el máximo salto acumulado. |
| Implementación | 4 | La implementación existente era correcta; se reforzó cobertura de pruebas. |
| Pruebas | 5 | Se agregaron el segundo ejemplo del enunciado y matriz vacía. |
| Comunicación | 4 | La explicación queda clara en notas; falta repetirla en voz alta bajo tiempo. |

## Complejidad

- Tiempo: `O(m * n * log(m * n))`.
- Espacio: `O(m * n)`.

## Invariantes

- `efforts[row][col]` guarda el menor esfuerzo conocido para llegar a esa celda.
- Al extraer una celda del heap, un costo mayor al registrado se descarta.
- El costo hacia un vecino es `max(esfuerzo_actual, diferencia_de_altura)`.

## Retro

- Qué salió bien: se identificó el patrón correcto y la solución evita BFS simple.
- Qué falló: faltaban pruebas explícitas para el segundo ejemplo y entrada vacía.
- Error que se repite: no convertir todas las preguntas de clarificación en tests.
- Acción de repaso: repetir explicación de Dijkstra minimax sin consultar notas.
- Fecha para repetir: 2026-07-19.
