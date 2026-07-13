# Semana 13: Montículos, Intervalos y Greedy

## Objetivo

Resolver problemas de prioridad, rangos y decisiones locales justificadas usando montículos, ordenamiento y conteo de frecuencias.

## Problemas Completados

| Problema | Patrón | Función | Tests |
| --- | --- | --- | --- |
| Kth Largest Element in an Array | Montículo mínimo acotado | `kth_largest` | 3 |
| Find Median from Data Stream | Dos montículos | `MedianFinder` | 3 |
| Merge Intervals | Ordenar y fusionar | `merge_intervals` | 2 |
| Insert Interval | Insertar y fusionar | `insert_interval` | 2 |
| Meeting Rooms | Ordenar por inicio | `can_attend_meetings` | 2 |
| Meeting Rooms II | Montículo mínimo de finales | `min_meeting_rooms` | 2 |
| Non-overlapping Intervals | Greedy por fin más temprano | `erase_overlap_intervals` | 2 |
| Task Scheduler | Greedy por frecuencia máxima | `task_scheduler` | 4 |
| Merge K Sorted Lists | Montículo mínimo por cabeza | `merge_k_sorted_lists` | 2 |
| K Closest Points to Origin | Montículo máximo acotado | `k_closest_points_heap` | 2 |
| Last Stone Weight | Simulación con max heap | `last_stone_weight` | 2 |
| Minimum Number of Arrows to Burst Balloons | Greedy por fin más temprano | `min_arrows_to_burst_balloons` | 2 |
| Jump Game II | Greedy por frontera | `jump_game_ii` | 2 |
| Gas Station | Greedy con reinicio | `can_complete_circuit` | 2 |

## Patrones Trabajados

### Montículo Mínimo Acotado

Se usa cuando solo importan los `k` mejores valores.

Invariante:

- El montículo nunca guarda más de `k` elementos.
- La raíz representa el menor valor entre los `k` mejores vistos.

En `k_closest_points_heap`, el heap conserva los `k` puntos con menor distancia cuadrada.

### Montículo por Cabezas

En `merge_k_sorted_lists`, el heap guarda la cabeza actual de cada lista.

Invariante:

- Cada extracción produce el siguiente valor globalmente menor.
- Después de consumir una cabeza, se inserta el siguiente nodo de esa misma lista.
- El heap contiene como máximo un nodo por lista activa.

### Dos Montículos

Para medianas en streaming:

- `lower` guarda la mitad menor como max heap.
- `upper` guarda la mitad mayor como min heap usando `Reverse`.
- `lower` puede tener como máximo un elemento más que `upper`.

### Simulación con Max Heap

En `last_stone_weight`, siempre se necesitan las dos piedras más pesadas.

Invariante:

- Si las piedras pesan igual, ambas desaparecen.
- Si son distintas, solo vuelve al heap la diferencia.
- El heap siempre representa las piedras restantes.

### Fusionar Intervalos

Se ordena por inicio y se compara cada intervalo con el último rango fusionado.

Invariante:

- El vector de respuesta siempre contiene intervalos no solapados.

### Salas de Reuniones

Para contar salas, un montículo mínimo guarda los finales activos.

Invariante:

- Antes de abrir una sala nueva, se liberan todas las salas cuyo final es menor o igual al inicio actual.

### Greedy por Fin Más Temprano

En intervalos no solapados, conviene conservar el intervalo que termina antes porque deja más espacio para los siguientes.

La misma idea aparece en `min_arrows_to_burst_balloons`: disparar al final más temprano cubre todos los globos que se solapan con ese punto.

### Greedy por Frecuencia Máxima

En Task Scheduler, la tarea más frecuente define la estructura mínima de bloques:

```text
(max_count - 1) * (cooldown + 1) + number_of_max_tasks
```

El resultado real es el máximo entre ese marco mínimo y el número total de tareas.

### Greedy por Frontera

En `jump_game_ii`, cada salto cubre una ventana de índices alcanzables.

Invariante:

- `current_end` marca hasta dónde llega el número actual de saltos.
- `farthest` acumula el mejor alcance del siguiente salto.
- Si la frontera no avanza, el final es inalcanzable.

### Greedy con Reinicio

En `can_complete_circuit`, si el tanque cae por debajo de cero, ningún punto dentro del segmento actual puede ser inicio válido.

Invariante:

- El balance total decide si existe solución.
- Cada reinicio descarta un prefijo imposible.

## Representación en Rust

Montículo máximo:

```rust
BinaryHeap<i32>
```

Montículo mínimo:

```rust
BinaryHeap<Reverse<i32>>
```

Intervalos:

```rust
Vec<(i32, i32)>
```

## Errores Comunes

- Usar sort completo para kth largest cuando un montículo acotado reduce memoria.
- No rebalancear los dos montículos después de insertar en median stream.
- No limitar el heap a `k` elementos cuando solo importan los mejores.
- Fusionar intervalos sin ordenarlos primero.
- Considerar solapados intervalos donde `end == start`.
- En Task Scheduler, olvidar que varias tareas pueden compartir la frecuencia máxima.
- En Jump Game II, contar saltos por índice en vez de por frontera.
- En Gas Station, elegir el mínimo local sin validar el balance total.

## Verificación

```bash
cargo test
```

Resultado esperado al cerrar el bloque:

```text
411 passed; 0 failed
```

## Siguiente Bloque

El siguiente bloque recomendado es:

- Consultas por rangos.
- Estructuras persistentes simples.
- Ventanas monotónicas.

Primeros problemas sugeridos:

- Range Sum Query 2D Immutable.
- Range Addition.
- Sliding Window Maximum.
- Queue Reconstruction by Height.
