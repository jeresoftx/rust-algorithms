# Simulacro 02: Course Schedule

## Objetivo

Practicar grafos dirigidos, detección de ciclos y ordenamiento topológico bajo límite de tiempo.

Duración total: 45 minutos.

## Reglas

- No consultar `src/patterns/graphs.rs` antes de terminar.
- Escribir primero los casos de prueba.
- Explicar por qué el grafo es dirigido.
- Al terminar, registrar retro en `notes/review-queue.md`.

## Enunciado

Dado `num_courses` y una lista de prerequisitos `(course, prerequisite)`, determinar si es posible terminar todos los cursos.

Cada par significa: para tomar `course`, primero se debe tomar `prerequisite`.

## Ejemplos

```text
num_courses = 2
prerequisites = [(1, 0)]
resultado = true
```

```text
num_courses = 2
prerequisites = [(1, 0), (0, 1)]
resultado = false
```

```text
num_courses = 4
prerequisites = [(1, 0), (2, 0), (3, 1), (3, 2)]
resultado = true
```

## Preguntas de Clarificación

Antes de codificar, responder:

- ¿Los cursos están numerados de `0` a `num_courses - 1`?
- ¿Puede haber prerequisitos repetidos?
- ¿Qué debe pasar si `num_courses` es `0`?
- ¿Solo hay que devolver booleano o también un orden válido?

## Plan Esperado

1. Construir lista de adyacencia desde prerequisito hacia curso.
2. Calcular indegree de cada curso.
3. Agregar a una cola todos los cursos con indegree `0`.
4. Procesar la cola, reduciendo indegrees de vecinos.
5. Si se procesan todos los cursos, no hay ciclo.

## Invariantes

- Un curso con indegree `0` no tiene prerequisitos pendientes.
- Cada arista se procesa una sola vez.
- Si quedan cursos sin procesar, pertenecen a un ciclo o dependen de un ciclo.

## Tests Mínimos

Crear o reescribir tests equivalentes a:

```rust
#[test]
fn returns_true_when_prerequisites_are_acyclic() {
    assert!(can_finish(4, vec![(1, 0), (2, 0), (3, 1), (3, 2)]));
}

#[test]
fn returns_false_when_prerequisites_have_cycle() {
    assert!(!can_finish(2, vec![(1, 0), (0, 1)]));
}
```

## Señales de Buena Solución

- Complejidad temporal: O(v + e).
- Complejidad espacial: O(v + e).
- No confunde dirección de aristas.
- Detecta ciclo por cantidad de cursos procesados.

## Retro Después del Simulacro

Completar:

| Dimensión | Puntaje 1-5 | Evidencia |
| --- | --- | --- |
| Clarificación |  |  |
| Enfoque |  |  |
| Implementación |  |  |
| Pruebas |  |  |
| Comunicación |  |  |

Notas:

- Qué salió bien:
- Qué falló:
- Error que se repite:
- Acción de repaso:
- Fecha para repetir:
