# Profundización 261-290: patrones compuestos

**Estado:** bloque preparado; no representa soluciones implementadas ni
contenido publicado.

**Trazabilidad:** [issue #10](https://github.com/jeresoftx/rust-algorithms/issues/10).

Este bloque abre la profundización posterior a la consolidación 191-260. La
meta no es encadenar técnicas por exhibición: es aprender a detectar cuándo un
problema exige que dos ideas se sostengan mutuamente y cuándo una solución más
simple conserva mejor la claridad.

## Concepto

Un patrón compuesto combina una estructura de búsqueda, representación o
estado con una segunda técnica que restringe el espacio de soluciones. Por
ejemplo, una búsqueda binaria sobre la respuesta necesita además un predicado
de factibilidad; Dijkstra necesita una estructura de prioridad y una
invariante sobre distancias ya asentadas.

## Problema

Después de practicar familias aisladas, es fácil reconocer palabras clave y
elegir una técnica demasiado pronto. Esa reacción produce soluciones opacas:
un heap sin justificar, programación dinámica donde bastaba greedy o un árbol
de rangos para una entrada estática.

Los problemas 261-290 entrenan una pregunta previa a la implementación:
¿qué subproblema resuelve cada técnica y qué evidencia muestra que la
combinación es necesaria?

## Alternativas

1. **Agregar problemas por dificultad nominal.** Da variedad, pero no explica
   qué habilidad compuesta se está entrenando.
2. **Resolver siempre con la estructura más potente disponible.** Puede pasar
   pruebas, pero oculta costos y aumenta el riesgo de bugs.
3. **Seleccionar candidatos con una alternativa simple y un disparador claro
   para la composición.** Conserva la explicación, permite comparar costos y
   deja evidencia útil para simulacros.

Se adopta la tercera alternativa.

## Regla de selección

Un candidato entra al bloque solo si cumple al menos una condición:

- mezcla dos patrones con responsabilidades distinguibles;
- fuerza a defender una estructura avanzada contra una alternativa lineal o
  de ordenamiento;
- expone una invariante que no aparece en problemas de patrón único;
- reutiliza una familia del repositorio para evitar sumar tema nuevo sin
  propósito.

Cada ejecución posterior debe incluir tests de caso normal, borde y mínimo,
complejidad temporal y espacial, y una explicación de por qué la alternativa
simple deja de ser suficiente. El estado inicial de todos los candidatos es
`pendiente`; ninguno cuenta como problema implementado o dominado.

## Candidatos formalizados

| # | Candidato | Dificultad | Composición | Alternativa simple | Decisión que debe defenderse |
| ---: | --- | --- | --- | --- | --- |
| 261 | Merge K Sorted Lists | Hard | Heap + listas enlazadas | Concatenar y ordenar | El heap conserva una frontera de `k` elementos y evita ordenar todo de nuevo. |
| 262 | Find Median from Data Stream | Hard | Dos heaps + invariantes de tamaño | Ordenar al consultar | El costo se desplaza a inserción para responder mediana establemente. |
| 263 | Sliding Window Maximum | Hard | Ventana deslizante + deque monotónico | Máximo por ventana | El deque descarta dominados y conserva índices vigentes. |
| 264 | Shortest Subarray with Sum at Least K | Hard | Prefijos + deque monotónico | Ventana deslizante positiva | Los negativos invalidan la contracción habitual. |
| 265 | Minimum Cost to Connect Sticks | Medium | Heap + greedy | Elegir pares arbitrarios | Unir los menores preserva la propiedad de optimalidad local. |
| 266 | Reorganize String | Medium | Conteos + heap | Reordenamiento exhaustivo | La frecuencia máxima gobierna factibilidad y elección siguiente. |
| 267 | Task Scheduler | Medium | Conteos + greedy | Simulación de todos los órdenes | Los huecos dependen de la frecuencia dominante, no del orden incidental. |
| 268 | Kth Smallest Element in a Sorted Matrix | Medium | Búsqueda binaria + conteo monotónico | Heap sobre filas | El rango de valores permite contar sin extraer cada candidato. |
| 269 | Split Array Largest Sum | Hard | Búsqueda binaria sobre respuesta + greedy | Programación dinámica cuadrática | La factibilidad de un límite es monótona. |
| 270 | Minimum Days to Make M Bouquets | Medium | Búsqueda binaria sobre respuesta + rachas | Probar todos los días | La madurez por día induce un predicado monotónico. |
| 271 | Path With Minimum Effort | Medium | Dijkstra minimax + heap | BFS sin prioridad | La métrica minimiza el máximo salto, no el número de aristas. |
| 272 | Network Delay Time | Medium | Grafo ponderado + Dijkstra | BFS | Los pesos positivos cambian la frontera correcta. |
| 273 | Cheapest Flights Within K Stops | Medium | Grafo + DP acotada por capas | Dijkstra sin estado | El límite de escalas forma parte del estado. |
| 274 | Course Schedule III | Hard | Heap + greedy por fecha | Elegir cursos cortos solamente | El heap permite reemplazar la decisión más costosa ya tomada. |
| 275 | Meeting Rooms III | Hard | Dos heaps + simulación temporal | Ordenar y recorrer una sala | Hay que separar salas ocupadas de salas libres y respetar desempates. |
| 276 | Car Pooling | Medium | Diferencias + prefix sum | Ordenar eventos y contar pasajeros | Un arreglo de cambios expresa ocupación acumulada con claridad. |
| 277 | My Calendar III | Hard | Barrido de eventos + mapa ordenado | Revisar intervalos uno por uno | Cada reserva modifica puntos de cambio, no cada instante del rango. |
| 278 | Count of Range Sum | Hard | Prefijos + divide and conquer | Enumerar todos los subarreglos | El merge cuenta pares válidos entre mitades ordenadas. |
| 279 | Reverse Pairs | Hard | Merge sort + dos punteros | Comparar cada par | El orden parcial vuelve lineal el conteo cruzado. |
| 280 | Number of Longest Increasing Subsequence | Medium | DP + conteo | Enumerar subsecuencias | Longitud y cantidad son estados diferentes que deben actualizarse juntos. |
| 281 | Russian Doll Envelopes | Hard | Ordenamiento + LIS | DP sobre todos los pares | El desempate descendente evita apilar sobres del mismo ancho. |
| 282 | Longest Increasing Path in a Matrix | Hard | DFS memoizada + grafo implícito | DFS sin memoria | La memoización transforma subrutas repetidas en estados compartidos. |
| 283 | Word Ladder | Hard | BFS + patrones intermedios | Comparar cada palabra con todas | La indexación por comodines reduce vecinos candidatos. |
| 284 | Alien Dictionary | Hard | Grafo de precedencia + topological sort | Ordenar alfabéticamente | Las restricciones derivan de la primera diferencia entre palabras. |
| 285 | Evaluate Division | Medium | Grafo ponderado + DFS/BFS | Sustitución simbólica | El producto de aristas modela cocientes transitivos. |
| 286 | Accounts Merge | Medium | Union-Find + hashing | Comparar listas por pares | Los correos son la identidad conectiva entre cuentas. |
| 287 | Minimum Genetic Mutation | Medium | BFS + generación de vecinos | Buscar rutas exhaustivas | Cada mutación válida forma una arista no ponderada. |
| 288 | Word Search II | Hard | Trie + backtracking | Buscar cada palabra por separado | El prefijo compartido poda búsquedas duplicadas. |
| 289 | Concatenated Words | Hard | Trie/DP + segmentación | Probar combinaciones de palabras | El prefijo y los cortes comparten subproblemas. |
| 290 | Longest Duplicate Substring | Hard | Búsqueda binaria + rolling hash | Comparar todos los substrings | La longitud candidata es monótona y el hash evita comparar cada texto completo. |

## Ejecución registrada

| Candidato | Estado | Evidencia |
| --- | --- | --- |
| 264: Shortest Subarray with Sum at Least K | Implementado y probado; pendiente de revisión humana como contenido educativo. | Issue #66, función `shortest_subarray_at_least_k` y tests de negativos, ausencia y ventana mínima. |
| 265: Minimum Cost to Connect Sticks | Implementado y probado; pendiente de revisión humana como contenido educativo. | Issue #68, función `minimum_cost_to_connect_sticks` y tests de caso típico, extremos y repetidos. |
| 268: Kth Smallest Element in a Sorted Matrix | Implementado y probado; pendiente de revisión humana como contenido educativo. | Issue #72, función `kth_smallest_in_sorted_matrix` y tests de conteo, bordes y matriz mínima. |
| 270: Minimum Days to Make M Bouquets | Implementado y probado; pendiente de revisión humana como contenido educativo. | Issue #73, función `minimum_days_to_make_bouquets` y tests de día mínimo, imposibilidad y rachas. |
| 266: Reorganize String | Implementado y probado; pendiente de revisión humana como contenido educativo. | Issue #70, función `reorganize_string` y tests de reorganización posible, imposible y vacío. |

## Distribución de señal

| Familia principal | Candidatos | Pregunta recurrente |
| --- | ---: | --- |
| Heaps y greedy | 261-267, 274-275 | ¿Qué frontera mínima debe mantenerse? |
| Búsqueda binaria y factibilidad | 268-270, 290 | ¿Por qué el predicado cambia solo en una dirección? |
| Grafos y estado | 271-273, 282-287 | ¿Qué información debe viajar con cada nodo? |
| Rangos, prefijos y orden parcial | 276-279 | ¿Qué operación permite evitar revisar cada par o instante? |
| DP, tries y backtracking | 280-281, 288-289 | ¿Qué subproblema o prefijo se reutiliza? |

## Protocolo de ejecución

1. Explicar el enunciado con un ejemplo pequeño y anotar restricciones.
2. Presentar una solución ingenua y su cuello de botella.
3. Nombrar las dos técnicas y la responsabilidad de cada una.
4. Escribir tests rojos: normal, borde y mínimo; agregar regresión si aparece
   un error real.
5. Implementar la versión más clara que cumpla la invariante.
6. Explicar complejidad, memoria y cuándo la alternativa simple volvería a ser
   preferible.
7. Registrar resultado en la cola de repaso sin marcar `dominado` sin evidencia
   de ejecución.

## Descartes conscientes

| Tema | Motivo para no incluirlo todavía |
| --- | --- |
| Suffix tree desde cero | Aporta complejidad de implementación desproporcionada para la señal de entrevista del bloque. |
| Link-cut tree | Es una estructura especializada; no fortalece las brechas actuales del repositorio. |
| Flujo máximo con optimizaciones avanzadas | Requiere una ruta conceptual propia, no una inserción aislada en un bloque mixto. |
| Segment tree persistente | Conviene solo después de una necesidad de consulta versionada documentada. |

## Relación con el milestone

Este documento cubre el primer tramo del milestone
`Horizonte 400 — Profundización 261-330`. Los bloques 291-315 y 316-330 deben
subir la dificultad o cerrar brechas sin repetir candidatos solo para completar
la numeración. Todo material resultante queda pendiente de revisión humana.
