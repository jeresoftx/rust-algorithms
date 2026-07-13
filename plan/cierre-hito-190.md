# Cierre del Hito de 190 Problemas

## Objetivo

Ampliar el repositorio de 140 a 190 problemas implementados, probados y documentados.

Este hito es contenido del repo/curso. No incluye repeticiones personales, simulacros ejecutados por el estudiante ni bitácoras de entrenamiento individual.

## Estado Inicial

- Problemas implementados: 149.
- Tests automatizados: 367.
- Meta del hito: 190 problemas.
- Faltan: 41 problemas.

## Enfoque

La meta no es solo aumentar volumen. Cada bloque debe reforzar un patrón que conviene reconocer, explicar y probar:

- Elegir el algoritmo por sus invariantes, no por memoria.
- Escribir tests antes de implementar.
- Documentar la idea central y la complejidad.
- Mantener los ejemplos lo bastante pequeños para estudiar el patrón.
- Evitar mezclar entrenamiento personal con documentación pública.

## Bloques de Trabajo

| Bloque | Tema | Problemas estimados | Meta acumulada |
| --- | --- | ---: | ---: |
| 1 | Strings avanzados | 5 | Completado: 145 |
| 2 | Grafos y Union-Find | 7 | En progreso: 149 |
| 3 | Árboles y BST | 6 | 158 |
| 4 | Programación dinámica avanzada | 7 | 165 |
| 5 | Heaps, intervalos y greedy | 6 | 171 |
| 6 | Range queries y estructuras | 6 | 177 |
| 7 | Backtracking y combinatoria | 5 | 182 |
| 8 | Geometría, matrices y selección final | 8 | 190 |

## Bloque 1: Strings Avanzados

Objetivo: reforzar búsqueda y análisis de cadenas más allá de KMP básico.

Problemas sugeridos:

- Rabin-Karp Pattern Search.
- Z Function.
- Longest Palindromic Substring.
- Count Palindromic Substrings.
- Shortest Palindrome.

Criterio de cierre:

- Tests con coincidencias superpuestas.
- Tests con entradas vacías o sin coincidencias.
- Notas comparando KMP, rolling hash y expansión por centros.

Avance:

- Problemas agregados: 5.
- Tests agregados: 11.
- APIs agregadas: `rabin_karp_positions`, `z_function`, `longest_palindromic_substring`, `count_palindromic_substrings` y `shortest_palindrome`.

## Bloque 2: Grafos y Union-Find

Objetivo: completar variantes frecuentes de conectividad, ordenamiento topológico y grafos bipartitos.

Problemas sugeridos:

- Number of Connected Components. Implementado.
- Graph Valid Tree. Implementado.
- Is Graph Bipartite. Implementado.
- Possible Bipartition.
- Number of Provinces. Implementado.
- Evaluate Division.
- Alien Dictionary.

Avance:

- Problemas agregados: 4 de 7.
- Tests agregados: 8.
- APIs agregadas: `count_connected_components`, `graph_valid_tree`, `is_bipartite` y `find_circle_num`.

## Bloque 3: Árboles y BST

Objetivo: ampliar recorridos, consultas y serialización de árboles.

Problemas sugeridos:

- Kth Smallest Element in a BST.
- Binary Tree Right Side View.
- Path Sum.
- Path Sum II.
- Serialize and Deserialize Binary Tree.
- Construct Binary Tree from Inorder and Postorder.

## Bloque 4: Programación Dinámica Avanzada

Objetivo: practicar estados 2D, intervalos, conteos y decisiones con restricciones.

Problemas sugeridos:

- Palindromic Substrings.
- Longest Palindromic Substring.
- Best Time to Buy and Sell Stock with Cooldown.
- House Robber III.
- Target Sum.
- Combination Sum IV.
- Maximum Product Subarray.

## Bloque 5: Heaps, Intervalos y Greedy

Objetivo: fortalecer selección incremental, planificación y estructuras de prioridad.

Problemas sugeridos:

- Merge K Sorted Lists.
- K Closest Points reforzado con heap.
- Meeting Rooms II reforzado.
- Non-overlapping Intervals reforzado.
- Minimum Number of Arrows to Burst Balloons.
- Jump Game II.

## Bloque 6: Range Queries y Estructuras

Objetivo: profundizar en consultas, actualizaciones y estructuras para rangos.

Problemas sugeridos:

- Range Sum Query 2D Immutable.
- Range Addition.
- Count Range Sum.
- Sliding Window Maximum.
- Queue Reconstruction by Height.
- Snapshot Array.

## Bloque 7: Backtracking y Combinatoria

Objetivo: cubrir generación, poda y conteo sin repetir estados accidentalmente.

Problemas sugeridos:

- Combination Sum II.
- Palindrome Partitioning.
- Letter Combinations of a Phone Number.
- N-Queens.
- Subsets II.

## Bloque 8: Geometría, Matrices y Selección Final

Objetivo: cerrar el hito con problemas selectivos de alto valor.

Problemas sugeridos:

- Rotate Image.
- Spiral Matrix.
- Set Matrix Zeroes.
- Word Search reforzado.
- Valid Sudoku.
- Game of Life.
- Maximal Square.
- Insert Delete GetRandom O(1).

## Rutina por Bloque

1. Agregar pruebas rojas.
2. Implementar solución clara.
3. Ejecutar pruebas enfocadas.
4. Actualizar notas y plan.
5. Actualizar README y wiki.
6. Ejecutar `cargo fmt`.
7. Ejecutar `cargo test`.
8. Crear commit pequeño.
9. Empujar repo y wiki.

## Avances Registrados

| Bloque | Problemas agregados | Tests agregados | Estado |
| --- | ---: | ---: | --- |
| Strings avanzados | 5 | 11 | completado |
| Grafos y Union-Find | 4 | 8 | en progreso |

## Criterio de Cierre del Hito

- `README.md` reporta 190 problemas.
- `plan/plan-alcance-avanzado.md` marca el hito 190 como completado.
- La wiki refleja el mismo estado.
- La suite completa pasa.
- Las repeticiones personales siguen separadas del avance del repo.
