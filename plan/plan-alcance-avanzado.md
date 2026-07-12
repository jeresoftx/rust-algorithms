# Plan de Alcance Avanzado

Este documento extiende el plan base después de la semana 16. Su objetivo es cubrir los temas que faltan sin romper el enfoque del repositorio: soluciones claras en Rust, tests automatizados, notas de estudio y documentación pública en la wiki.

## Objetivo

Ampliar el repositorio desde una base fuerte de patrones frecuentes hacia una preparación más completa en estructuras avanzadas, grafos ponderados, cadenas, consultas por rangos, matemáticas y simulacros de mayor dificultad.

## Punto de Partida

Estado actual del repo:

- Problemas implementados: 69.
- Tests automatizados: 162.
- Patrones cubiertos: hashing, two pointers, sliding window, stack, búsqueda binaria, backtracking, listas enlazadas, árboles, grafos básicos, montículos, intervalos, greedy y programación dinámica.
- Simulacros documentados: 3.

Brechas principales:

- Complejidad formal: Θ, Ω y análisis amortizado.
- Trie y algoritmos de cadenas.
- Grafos ponderados y algoritmos clásicos.
- Matemáticas y manipulación de bits.
- Divide and conquer avanzado.
- Segment tree y Fenwick tree.
- Suffix array, suffix tree y Aho-Corasick.
- Convex hull.
- Mayor volumen de problemas y más simulacros cronometrados.

## Principios de Expansión

1. Priorizar utilidad de entrevista antes que teoría enciclopédica.
2. Implementar primero la versión más común y explicable de cada algoritmo.
3. Acompañar cada módulo con tests, notas y una página o sección de wiki.
4. Mantener commits pequeños por tema terminado.
5. Medir avance por dominio real: resolver, explicar, probar y repetir.

## Metas de Volumen

El plan base sigue siendo el núcleo. La expansión agrega una ruta gradual:

| Hito | Problemas acumulados | Enfoque |
| --- | ---: | --- |
| Base consolidada | 100 | Repetición, casos borde y problemas medios frecuentes |
| Avanzado inicial | 140 | Tries, bits, matemáticas y grafos ponderados básicos |
| Avanzado completo | 190 | Segment tree, Fenwick, strings avanzados y MST |
| Largo plazo | 260+ | Problemas difíciles selectivos y simulacros mixtos |

La meta de 400 problemas queda como horizonte opcional de largo plazo. Para este repo conviene llegar primero a 190 problemas bien probados y documentados.

## Fase 1: Complejidad, Matemáticas y Bits

Duración sugerida: semanas 17 y 18.

Estado: en progreso.

Temas:

- Big O, Θ, Ω y amortizada.
- GCD, LCM, criba, exponenciación rápida y aritmética modular.
- Operaciones bit a bit, máscaras, conteo de bits y subconjuntos con bitmask.
- Divide and conquer básico aplicado.

Archivos sugeridos:

- `notes/complexity-cheatsheet.md`: creado.
- `src/patterns/math_bit.rs`: creado.
- `tests/math_bit_test.rs`: creado.
- `notes/week-17-18.md`: creado.

Problemas sugeridos:

- Single Number.
- Number of 1 Bits.
- Counting Bits.
- Missing Number.
- Reverse Bits.
- Power of Two.
- Pow(x, n).
- Sieve of Eratosthenes.
- Majority Element.
- Maximum Subarray.

Avance acumulado:

- Problemas implementados: 12.
- Tests agregados: 30.
- Funciones principales: `single_number`, `missing_number`, `count_ones`, `count_bits`, `reverse_bits`, `is_power_of_two`, `fast_pow`, `gcd`, `lcm`, `sieve`, `maximum_subarray` y `majority_element`.

Criterio de cierre:

- Explicar Θ y Ω sin confundirlos con Big O.
- Resolver problemas con máscaras sin convertirlos en fuerza bruta accidental.
- Tener al menos 10 problemas implementados y probados.

## Fase 2: Tries y Cadenas

Duración sugerida: semanas 19 y 20.

Estado: completada.

Temas:

- Trie básico.
- Búsqueda por prefijo.
- Wildcards con DFS.
- KMP.
- Aho-Corasick.
- Suffix array como estructura de consulta.
- Suffix tree solo como tema conceptual, salvo que haya tiempo para una implementación cuidada.

Archivos sugeridos:

- `src/patterns/tries.rs`: creado.
- `src/patterns/string_algorithms.rs`: creado.
- `tests/tries_test.rs`: creado.
- `tests/string_algorithms_test.rs`: creado.
- `notes/week-19-20.md`: creado.

Problemas sugeridos:

- Implement Trie.
- Design Add and Search Words Data Structure.
- Word Search II.
- Replace Words.
- Longest Common Prefix.
- Find All Anagrams in a String.
- Repeated Substring Pattern.
- Longest Duplicate Substring.
- Multi-pattern Search.

Avance acumulado:

- Problemas implementados: 8.
- Tests agregados: 26.
- Funciones y estructuras principales: `Trie`, `WordDictionary`, `replace_words`, `find_pattern_positions`, `find_anagram_starts`, `repeated_substring_pattern`, `longest_common_prefix` y `longest_duplicate_substring`.

Criterio de cierre:

- Implementar un trie con `insert`, `search` y `starts_with`.
- Explicar cuándo conviene trie sobre `HashMap` o sorting.
- Tener KMP probado con coincidencias parciales y patrones repetidos.

## Fase 3: Grafos Ponderados y Algoritmos Clásicos

Duración sugerida: semanas 21 a 23.

Estado: completada.

Temas:

- Dijkstra.
- Bellman-Ford.
- Floyd-Warshall.
- Prim.
- Kruskal.
- Tarjan para puentes y componentes fuertemente conectados.

Archivos sugeridos:

- `src/patterns/weighted_graphs.rs`: creado.
- `tests/weighted_graphs_test.rs`: creado.
- `notes/week-21-23.md`: creado.

Problemas sugeridos:

- Network Delay Time.
- Cheapest Flights Within K Stops.
- Path With Minimum Effort.
- Find Critical and Pseudo-Critical Edges in MST.
- Min Cost to Connect All Points.
- Critical Connections in a Network.
- Strongly Connected Components.
- Course Schedule reforzado con variantes.
- Shortest Path in Binary Matrix como puente entre BFS y pesos.

Avance acumulado:

- Problemas implementados: 12.
- Tests agregados: 27.
- Funciones principales: `dijkstra_shortest_paths`, `network_delay_time`, `bellman_ford_shortest_paths`, `floyd_warshall_all_pairs`, `prim_minimum_spanning_tree_weight`, `kruskal_minimum_spanning_tree_weight`, `cheapest_flight_within_k_stops`, `minimum_effort_path`, `critical_connections`, `strongly_connected_components`, `min_cost_connect_points` y `find_critical_and_pseudo_critical_edges`.

Criterio de cierre:

- Elegir correctamente entre BFS, Dijkstra, Bellman-Ford y Floyd-Warshall.
- Implementar MST con Prim y Kruskal.
- Tener tests para grafos desconectados, ciclos y pesos negativos cuando aplique.

## Fase 4: Consultas por Rangos

Duración sugerida: semanas 24 y 25.

Estado: iniciada.

Temas:

- Prefix sum avanzado.
- Difference array.
- Fenwick tree.
- Segment tree.
- Lazy propagation como extensión opcional.

Archivos sugeridos:

- `src/patterns/range_queries.rs`: creado.
- `tests/range_queries_test.rs`: creado.
- `notes/week-24-25.md`: creado.

Problemas sugeridos:

- Range Sum Query Immutable.
- Range Sum Query Mutable.
- Count of Smaller Numbers After Self.
- Reverse Pairs.
- Corporate Flight Bookings.
- Car Pooling.
- My Calendar I.
- My Calendar II.

Avance acumulado:

- Problemas implementados: 10.
- Tests agregados: 28.
- Funciones y estructuras principales: `FenwickTree`, `RangeSumQuery`, `SegmentTree`, `DifferenceArray`, `corporate_flight_bookings`, `car_pooling`, `count_smaller_numbers_after_self`, `reverse_pairs`, `MyCalendar` y `MyCalendarTwo`.

Criterio de cierre:

- Saber explicar la diferencia entre prefix sum, Fenwick y segment tree.
- Implementar consultas y actualizaciones con tests de índices borde.
- Documentar cuándo no vale la pena usar una estructura avanzada.

## Fase 5: Geometría y Temas Selectivos

Duración sugerida: semana 26.

Estado: iniciada.

Temas:

- Producto cruz.
- Orientación de puntos.
- Convex hull.
- Distancia Manhattan y euclidiana.
- Barrido de línea como introducción.

Archivos sugeridos:

- `src/patterns/geometry.rs`
- `tests/geometry_test.rs`
- `notes/week-26.md`

Problemas sugeridos:

- Erect the Fence.
- K Closest Points to Origin.
- Max Points on a Line.
- The Skyline Problem como reto opcional.

Avance acumulado:

- Problemas implementados: 5.
- Tests agregados: 12.
- Funciones y estructuras principales: `Point`, `Orientation`, `cross_product`, `orientation`, `convex_hull`, `k_closest_points` y `max_points_on_a_line`.

Criterio de cierre:

- Implementar convex hull con casos colineales.
- Explicar el papel del producto cruz.
- Resolver al menos 4 problemas de geometría o barrido.

## Fase 6: Simulacros Avanzados y Repetición

Duración sugerida: semanas 27 y 28.

Estado: iniciada.

Temas:

- Simulacros mixtos de 45 a 60 minutos.
- Repetición de problemas fallados.
- Explicación verbal de complejidad.
- Selección de problemas difíciles con alto valor de aprendizaje.

Archivos sugeridos:

- `notes/week-27-28.md`
- `notes/simulations/simulacro-04-grafos-ponderados.md`
- `notes/simulations/simulacro-05-strings-avanzados.md`
- `notes/simulations/simulacro-06-range-queries.md`

Rutina:

1. Resolver 1 problema cronometrado.
2. Registrar enfoque, errores y complejidad.
3. Rehacer 1 problema fallado sin mirar solución.
4. Actualizar `notes/review-queue.md`.

Avance acumulado:

- Simulacros avanzados creados: 3.
- Simulacros acumulados ejecutados: 6 de 6.
- Problemas repetidos: 4 de 20.
- Problemas implementados: 116 de 140 como mínimo.
- Faltan para cierre mínimo: 24 problemas implementados y 16 repeticiones.
- Archivos creados: `notes/week-27-28.md`, `notes/simulations/simulacro-04-grafos-ponderados.md`, `notes/simulations/simulacro-05-strings-avanzados.md` y `notes/simulations/simulacro-06-range-queries.md`.
- Enfoque: práctica cronometrada, explicación verbal y repetición dirigida.

Criterio de cierre:

- Completar 6 simulacros acumulados.
- Tener al menos 20 problemas repetidos.
- Llegar a 140 problemas implementados como mínimo.
- Mantener `cargo test` pasando.

## Orden Recomendado de Implementación

1. Crear `notes/complexity-cheatsheet.md`.
2. Agregar `math_bit` con 5 problemas pequeños.
3. Agregar `tries` con implementación básica y 2 problemas.
4. Agregar `string_algorithms` con KMP.
5. Agregar `weighted_graphs` con Dijkstra y Bellman-Ford.
6. Agregar MST con Prim y Kruskal.
7. Agregar Tarjan para puentes.
8. Agregar `range_queries` con Fenwick.
9. Agregar segment tree.
10. Agregar `geometry` con convex hull.
11. Crear 3 simulacros avanzados.
12. Actualizar wiki y README después de cada bloque.

## Backlog Inicial Priorizado

| Prioridad | Tema | Primer entregable |
| --- | --- | --- |
| 1 | Complejidad | Guía corta de Big O, Θ, Ω y amortizada |
| 2 | Bits | `single_number`, `count_bits`, `reverse_bits` |
| 3 | Matemáticas | `gcd`, `fast_pow`, `sieve` |
| 4 | Trie | `Trie::insert`, `Trie::search`, `Trie::starts_with` |
| 5 | KMP | `find_pattern_positions` |
| 6 | Dijkstra | Camino mínimo con pesos no negativos |
| 7 | Bellman-Ford | Detección de ciclo negativo |
| 8 | MST | Prim y Kruskal |
| 9 | Fenwick | Prefix sum mutable |
| 10 | Segment tree | Range query mutable |
| 11 | Tarjan | Puentes en grafo no dirigido |
| 12 | Convex hull | Monotonic chain |

## Definición de Terminado por Bloque

Un bloque avanzado queda terminado cuando:

- Tiene código en `src/patterns`.
- Tiene tests en `tests`.
- Tiene nota semanal en `notes`.
- Tiene entrada de errores o repaso cuando hubo tropiezos.
- El README apunta al avance relevante.
- La wiki refleja el bloque.
- `cargo fmt` y `cargo test` pasan.
- Se hizo commit pequeño y push.

## Siguiente Acción

Empezar por Fase 1:

1. Crear la guía de complejidad.
2. Agregar el módulo `math_bit`.
3. Implementar 5 problemas pequeños con tests.
4. Documentar el bloque en `notes/week-17-18.md`.
5. Publicar el resumen en la wiki.
