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
| Clarificación | 4 | Se cubrieron matriz vacía, celda única, movimientos sin diagonal y retorno de esfuerzo mínimo. |
| Enfoque | 5 | Se eligió Dijkstra con costo minimax y se explicó por qué BFS simple no modela pesos. |
| Implementación | 4 | La solución existente ya cubría el patrón; se reforzó la suite con el segundo ejemplo y matriz vacía. |
| Pruebas | 5 | Quedaron cubiertos ejemplo principal, ruta alternativa, celda única y grid vacío. |
| Comunicación | 4 | La retro deja invariantes y siguiente repetición, pero conviene practicar explicación verbal cronometrada. |

Notas:

- Qué salió bien: la selección de Dijkstra minimax fue directa y las pruebas capturan los bordes principales.
- Qué falló: faltaba registrar el segundo ejemplo del enunciado en la suite.
- Error que se repite: dejar casos de clarificación como texto sin convertirlos en tests.
- Acción de repaso: repetir la explicación de Dijkstra minimax sin consultar notas.
- Fecha para repetir: 2026-07-19.
