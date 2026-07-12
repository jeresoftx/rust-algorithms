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

## Patrones Trabajados

### Montículo Mínimo Acotado

Se usa cuando solo importan los `k` mejores valores.

Invariante:

- El montículo nunca guarda más de `k` elementos.
- La raíz representa el menor valor entre los `k` mejores vistos.

### Dos Montículos

Para medianas en streaming:

- `lower` guarda la mitad menor como max heap.
- `upper` guarda la mitad mayor como min heap usando `Reverse`.
- `lower` puede tener como máximo un elemento más que `upper`.

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

### Greedy por Frecuencia Máxima

En Task Scheduler, la tarea más frecuente define la estructura mínima de bloques:

```text
(max_count - 1) * (cooldown + 1) + number_of_max_tasks
```

El resultado real es el máximo entre ese marco mínimo y el número total de tareas.

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
- Fusionar intervalos sin ordenarlos primero.
- Considerar solapados intervalos donde `end == start`.
- En Task Scheduler, olvidar que varias tareas pueden compartir la frecuencia máxima.

## Verificación

```bash
cargo test
```

Resultado esperado al cerrar el bloque:

```text
142 passed; 0 failed
```

## Siguiente Bloque

El siguiente bloque recomendado es:

- Programación dinámica.
- Memoización.
- Tabulación.
- Estados 1D y 2D.

Primeros problemas sugeridos:

- Climbing Stairs.
- House Robber.
- Coin Change.
- Longest Increasing Subsequence.
- Word Break.
