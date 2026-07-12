# Resultado Simulacro 02: Course Schedule

Fecha: 2026-07-12.

## Problema

Course Schedule.

## Resultado

Aprobado.

Puntaje total: 23/25.

| Dimensión | Puntaje 1-5 | Evidencia |
| --- | ---: | --- |
| Clarificación | 4 | Se asumieron cursos de `0` a `num_courses - 1` y salida booleana. |
| Enfoque | 5 | Se usó ordenamiento topológico con indegrees para detectar ciclos. |
| Implementación | 5 | La solución existente delega en `find_course_order` y verifica si procesa todos los cursos. |
| Pruebas | 5 | `cargo test can_finish` validó el caso acíclico y el caso con ciclo. |
| Comunicación | 4 | La explicación queda documentada; falta repetirla sin notas bajo tiempo. |

## Complejidad

- Tiempo: `O(v + e)`.
- Espacio: `O(v + e)`.

## Invariantes

- Un curso con indegree `0` no tiene prerequisitos pendientes.
- Cada arista reduce una sola vez el indegree del curso dependiente.
- Si no se procesan todos los cursos, existe un ciclo o una dependencia de ciclo.

## Retro

- Qué salió bien: el patrón de grafo dirigido quedó identificado de inmediato.
- Qué falló: faltó convertir las preguntas de clarificación en más casos borde.
- Error que se repite: confiar demasiado en los ejemplos mínimos del enunciado.
- Acción de repaso: repetir `find_course_order` desde cero y explicar la dirección de aristas.
- Fecha para repetir: 2026-07-23.
