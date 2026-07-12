# Resultado Simulacro 06: Consultas por Rangos

Fecha: 2026-07-12.

## Problema

Range Sum Query Mutable.

## Resultado

Aprobado.

Puntaje total: 24/25.

| Dimensión | Puntaje 1-5 | Evidencia |
| --- | ---: | --- |
| Clarificación | 5 | Se cubrieron inclusividad, índices inválidos, negativos y entrada vacía. |
| Enfoque | 5 | Fenwick Tree es apropiado para actualización puntual y suma de rango. |
| Implementación | 5 | La estructura conserva valores originales y aplica deltas correctamente. |
| Pruebas | 5 | Se reforzó la suite con negativos, múltiples updates y estructura vacía. |
| Comunicación | 4 | Falta repetir la explicación verbal de Fenwick bajo tiempo. |

## Complejidad

- Construcción: `O(n log n)` en la implementación actual.
- Actualización puntual: `O(log n)`.
- Consulta de rango: `O(log n)`.
- Espacio: `O(n)`.

## Invariantes

- El arreglo original permite calcular `delta = nuevo - actual`.
- Fenwick guarda contribuciones parciales de prefijos.
- `sum_range(left, right)` se obtiene como `prefix(right) - prefix(left - 1)`.
- Las consultas inválidas devuelven `None`; las actualizaciones inválidas devuelven `false`.

## Retro

- Qué salió bien: se eligió la estructura adecuada y se reforzaron bordes reales.
- Qué falló: faltaban pruebas explícitas para negativos y entrada vacía.
- Error que se repite: no convertir todas las preguntas de clarificación en pruebas desde el primer avance.
- Acción de repaso: reimplementar `RangeSumQuery` desde cero con pruebas primero.
- Fecha para repetir: 2026-07-21.
