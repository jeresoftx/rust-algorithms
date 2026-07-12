# Simulacro 04: Grafos Ponderados

## Objetivo

Practicar selección entre BFS, Dijkstra y búsqueda binaria sobre respuesta en un problema con pesos no negativos.

Duración total: 60 minutos.

## Reglas

- No consultar `src/patterns/weighted_graphs.rs` antes de terminar.
- Escribir primero los casos de prueba.
- Explicar por qué BFS simple no basta cuando hay pesos.
- Al terminar, registrar retro en `notes/review-queue.md`.

## Enunciado

Dada una matriz de alturas, moverse arriba, abajo, izquierda o derecha. El esfuerzo de una ruta es la diferencia máxima de alturas entre dos celdas consecutivas. Devolver el esfuerzo mínimo para ir de la esquina superior izquierda a la esquina inferior derecha.

## Ejemplos

```text
heights = [[1,2,2],[3,8,2],[5,3,5]]
resultado = 2
```

```text
heights = [[1,2,3],[3,8,4],[5,3,5]]
resultado = 1
```

## Preguntas de Clarificación

- ¿La matriz puede estar vacía?
- ¿Las alturas pueden repetirse?
- ¿Se permite moverse en diagonal?
- ¿Se pide el camino o solo el esfuerzo mínimo?

## Plan Esperado

1. Modelar cada celda como nodo.
2. Usar Dijkstra donde el costo de llegar a una celda es el máximo esfuerzo visto hasta ahí.
3. Relajar vecinos con `max(costo_actual, abs(altura_actual - altura_vecina))`.
4. Terminar al extraer la celda destino.

## Tests Mínimos

```rust
#[test]
fn returns_minimum_effort_path() {
    let heights = vec![vec![1, 2, 2], vec![3, 8, 2], vec![5, 3, 5]];
    assert_eq!(minimum_effort_path(heights), 2);
}

#[test]
fn handles_single_cell() {
    assert_eq!(minimum_effort_path(vec![vec![7]]), 0);
}
```

## Retro Después del Simulacro

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
