# Profundización 291-315: medium y hard con alta señal

**Estado:** bloque preparado; no representa soluciones implementadas ni
contenido publicado.

**Trazabilidad:** [issue #9](https://github.com/jeresoftx/rust-algorithms/issues/9).

Este tramo aumenta la dificultad con criterio. Un problema Hard no es más
valioso por su etiqueta: entra solo cuando obliga a construir una explicación
transferible sobre estado, corrección, límites o trade-offs que no aparecieron
con suficiente claridad en los bloques anteriores.

## Concepto

La señal de entrevista mide cuánto aprendizaje reutilizable deja un problema.
Un candidato tiene alta señal cuando permite explicar una solución ingenua,
derivar una mejora, sostener una invariante y defender el costo de esa mejora
ante restricciones concretas.

## Problema

Escalar dificultad sin filtro suele producir dos errores: coleccionar Hards
opacos o usar estructuras avanzadas para entradas que un recorrido simple
resuelve mejor. Ninguna conducta enseña juicio técnico.

Los problemas 291-315 se reservan para combinaciones donde la complejidad
adicional responde a una limitación explícita, no a una preferencia por código
ingenioso.

## Alternativas

1. **Seleccionar solo por popularidad.** Ofrece reconocimiento de enunciados,
   pero no asegura una habilidad nueva.
2. **Priorizar todos los Hard disponibles.** Aumenta la fricción y puede
   ocultar vacíos de fundamentos.
3. **Elegir Medium y Hard por señal, con descarte documentado.** Mantiene un
   plan honesto y refuerza la capacidad de decidir entre alternativas.

Se adopta la tercera alternativa.

## Criterios de alta señal

Cada candidato debe cumplir al menos dos puntos:

- introduce un estado que no se puede omitir sin perder corrección;
- tiene una alternativa ingenua fácil de explicar y comparar;
- exige un caso borde representativo, no solo el caso feliz;
- entrena una familia con brecha visible en la matriz de cobertura;
- produce una lección que se puede reutilizar en más de un enunciado.

La ejecución posterior conserva TDD: tests rojos de caso normal, borde y
mínimo; implementación mínima; refactor solo si mejora claridad; complejidad y
nota de aprendizaje. Ningún candidato de la tabla incrementa por sí mismo el
conteo de problemas implementados.

## Candidatos formalizados

| # | Candidato | Dificultad | Familia | Alternativa ingenua | Habilidad o trade-off |
| ---: | --- | --- | --- | --- | --- |
| 291 | Trapping Rain Water | Hard | Two pointers / stack | Medir agua desde cada posición | Dos máximos laterales se pueden mantener sin arreglos auxiliares completos. |
| 292 | First Missing Positive | Hard | In-place hashing | Usar `HashSet` | Los índices del arreglo representan el rango útil sin memoria adicional. |
| 293 | Median of Two Sorted Arrays | Hard | Partición binaria | Fusionar arreglos | La partición correcta depende de tamaños y centinelas, no de construir el resultado. |
| 294 | Serialize and Deserialize Binary Tree | Hard | Árboles / recorrido | Comparar árboles directamente | Una codificación debe preservar estructura y valores nulos. |
| 295 | Binary Tree Maximum Path Sum | Hard | DFS + estado de retorno | Enumerar caminos | El valor que retorna al padre no es el mejor camino global. |
| 296 | Count Complete Tree Nodes | Medium | Árboles + búsqueda binaria | Recorrer todos los nodos | La completitud permite contar subárboles perfectos de una vez. |
| 297 | Kth Smallest Element in a BST | Medium | Inorder + pila | Recolectar y ordenar | El recorrido ordenado puede detenerse en el rango requerido. |
| 298 | Recover Binary Search Tree | Medium | Inorder + detección local | Reordenar todos los valores | Dos inversiones en el orden revelan los nodos intercambiados. |
| 299 | Maximum Sum BST in Binary Tree | Hard | DFS + resumen de subárbol | Verificar cada subárbol repetidamente | Un resumen con límites, suma y validez evita trabajo duplicado. |
| 300 | Regular Expression Matching | Hard | DP bidimensional | Backtracking sin memoria | El estado `(i, j)` evita repetir sufijos bajo `.` y `*`. |
| 301 | Wildcard Matching | Hard | DP / greedy | Probar todas las expansiones | Comparar DP completa contra backtracking greedy con puntos de retorno. |
| 302 | Edit Distance | Medium | DP | Probar inserciones, borrados y reemplazos | La última operación reduce el problema a prefijos definidos. |
| 303 | Burst Balloons | Hard | DP por intervalo | Elegir el primer globo globalmente | Elegir el último globo divide el intervalo en subproblemas independientes. |
| 304 | Strange Printer | Hard | DP por intervalo | Imprimir cada carácter separado | Los extremos iguales permiten compartir una operación. |
| 305 | Palindrome Partitioning II | Hard | DP + expansión de centros | Probar todos los cortes | La palindromía precomputada reduce la transición de cortes. |
| 306 | Maximum Profit in Job Scheduling | Hard | Ordenamiento + DP + binary search | Comparar todos los empleos previos | La búsqueda encuentra el último empleo compatible. |
| 307 | Russian Doll Envelopes | Hard | Ordenamiento + LIS | DP cuadrática | El desempate descendente protege la invariante de LIS. |
| 308 | Swim in Rising Water | Hard | Búsqueda binaria + BFS/DFS | Simular todos los tiempos | La alcanzabilidad a un nivel dado es monótona. |
| 309 | Minimum Obstacle Removal to Reach Corner | Hard | 0-1 BFS | Dijkstra general | Pesos binarios permiten una deque más directa que un heap. |
| 310 | Critical Connections in a Network | Hard | DFS + low-link | Quitar cada arista y probar conectividad | `low-link` identifica puentes en una sola exploración. |
| 311 | Reconstruct Itinerary | Hard | Grafo + Hierholzer | Probar todos los itinerarios | Consumir aristas una vez construye un camino euleriano ordenado. |
| 312 | Minimum Cost to Make at Least One Valid Path in a Grid | Hard | 0-1 BFS | Dijkstra general | Seguir la flecha cuesta cero; cambiarla cuesta uno. |
| 313 | Maximum XOR of Two Numbers in an Array | Medium | Trie de bits | Comparar cada par | El trie elige el bit opuesto más alto disponible. |
| 314 | Find Median from Data Stream | Hard | Dos heaps | Ordenar en cada consulta | Mantener balanceado el flujo hace barata la consulta. |
| 315 | Design Search Autocomplete System | Hard | Trie + heap/conteos | Buscar todos los prefijos | El prefijo compartido y la selección top-k tienen responsabilidades distintas. |

## Casos borde que no se negocian

| Familia | Riesgo | Caso mínimo que debe aparecer en tests |
| --- | --- | --- |
| Arreglos in-place | Índices fuera de rango o duplicados | Arreglo vacío, un valor y valores fuera de `1..=n`. |
| Árboles | Confundir estado local con global | Árbol vacío, nodo único y violación en un descendiente. |
| DP | Estado base mal inicializado | Prefijo vacío, coincidencia vacía y transición imposible. |
| Grafos | Perder dirección, peso o restricciones | Nodo aislado, ciclo y frontera de costo cero/uno. |
| Tries / heaps | Desempates no deterministas | Prefijo ausente, empate de frecuencia y consulta sin resultados. |

## Descartes conscientes

| Candidato descartado | Motivo |
| --- | --- |
| Sudoku Solver | Ya existe como ejemplo clásico de backtracking; no agrega composición nueva a este tramo. |
| N-Queens II | Cuenta soluciones, pero la señal se solapa con backtracking base ya cubierto. |
| LFU Cache | Combina varias estructuras, pero merece un capítulo de diseño de estructuras y no una práctica aislada. |
| Basic Calculator III | Es útil, pero conviene una ruta dedicada de parsing para explicar precedencia sin atajos. |
| Falling Squares | Exige compresión y segment tree; se reserva para una brecha de rangos explícita. |

## Protocolo de explicación

Para cada ejecución, la explicación verbal debe responder en este orden:

1. ¿Qué restricción rompe la solución ingenua?
2. ¿Cuál es el estado o estructura mínima para conservar la información útil?
3. ¿Qué invariante protege la corrección durante cada paso?
4. ¿Qué caso borde puede invalidar la intuición inicial?
5. ¿En qué rango de entradas volvería a preferirse la solución simple?

La guía de trade-offs del issue #8 ampliará este protocolo para simulacros; no
sustituye los tests ni una demostración informal de corrección.

## Relación con el milestone

Este documento cubre el tramo medio de
`Horizonte 400 — Profundización 261-330`. El bloque 316-330 debe usar las
brechas restantes, mantener continuidad de numeración y dejar el milestone
listo para revisión humana, sin declarar contenido publicado.
