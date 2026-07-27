# Consolidación 191-220: patrones frecuentes bajo presión

**Estado:** bloque preparado; no representa problemas implementados ni contenido
publicado.

**Trazabilidad:** [issue #5](https://github.com/jeresoftx/rust-algorithms/issues/5).

Este bloque prepara los primeros 30 elementos posteriores al hito 190. Su
objetivo no es abrir temas nuevos, sino reforzar patrones que ya existen en el
repo y usarlos bajo criterios de entrevista: claridad, tests, complejidad,
casos borde y explicación verbal.

## Concepto

La consolidación convierte conocimiento existente en respuesta confiable. Un
patrón está realmente dominado cuando puede reconocerse, explicarse, probarse y
adaptarse bajo tiempo, no solo cuando ya existe una función parecida en el repo.

## Problema

El repo ya cubre muchas familias frecuentes, pero el horizonte 400 necesita una
forma ordenada de elegir qué practicar después. Si el bloque 191-220 empieza
agregando problemas al azar, se pierde la señal de entrevista y se infla el
conteo.

Este documento formaliza 30 candidatos iniciales. Cada candidato deberá
ejecutarse después con TDD en PRs pequeños o sub-bloques, siguiendo
[`plantilla-bloque-autonomo-191-plus.md`](./plantilla-bloque-autonomo-191-plus.md).

## Alternativas

1. **Implementar 30 problemas en un solo PR.** Cierra el rango rápido, pero hace
   difícil revisar calidad, errores y aprendizaje.
2. **Elegir problemas conforme aparezcan.** Da flexibilidad, pero diluye el
   objetivo de consolidación.
3. **Formalizar los 30 candidatos y ejecutarlos después con TDD.** Mantiene el
   bloque auditable y permite commits pequeños.

Se adopta la tercera alternativa.

## Distribución del bloque

| Dificultad | Cantidad | Rol |
| --- | ---: | --- |
| Easy | 8 | Calentamiento, fluidez de Rust y bordes simples. |
| Medium | 20 | Núcleo de práctica para entrevista. |
| Hard | 2 | Señal selectiva sin distraer del objetivo de consolidación. |
| Total | 30 | Rango 191-220. |

## Candidatos formalizados

| # | Candidato | Dificultad | Familia | Tipo | Motivo pedagógico |
| ---: | --- | --- | --- | --- | --- |
| 191 | Move Zeroes, repetición cronometrada | Easy | Two pointers | Repetición dirigida | Reforzar invariante de escritura y lectura sin asignaciones extra. |
| 192 | Valid Palindrome II | Easy | Two pointers | Problema nuevo | Practicar bifurcación mínima al permitir borrar un carácter. |
| 193 | Is Subsequence | Easy | Two pointers | Problema nuevo | Entrenar avance asimétrico de punteros en cadenas. |
| 194 | Merge Sorted Array | Easy | Two pointers | Problema nuevo | Reforzar escritura desde el final y mutación in-place. |
| 195 | Find Pivot Index | Easy | Prefix sum | Problema nuevo | Practicar suma total, acumulado izquierdo y caso borde. |
| 196 | Running Sum of 1d Array | Easy | Prefix sum | Repetición dirigida | Calentamiento para sumas acumuladas con pruebas mínimas. |
| 197 | Backspace String Compare | Easy | Stack / two pointers | Problema nuevo | Comparar solución con stack contra recorrido inverso. |
| 198 | Baseball Game | Easy | Stack | Problema nuevo | Practicar pila simple y validación de comandos. |
| 199 | Remove Duplicates from Sorted Array II | Medium | Two pointers | Problema nuevo | Generalizar el patrón de escritura con límite de repeticiones. |
| 200 | Sort Colors | Medium | Two pointers | Problema nuevo | Practicar partición en una pasada y trade-off contra counting sort. |
| 201 | Subarray Sum Equals K, repetición cronometrada | Medium | Hashing / prefix sum | Repetición dirigida | Reforzar conteo de prefijos y negativos. |
| 202 | Continuous Subarray Sum | Medium | Hashing / prefix sum | Problema nuevo | Trabajar residuos y longitud mínima. |
| 203 | Contiguous Array | Medium | Hashing / prefix sum | Problema nuevo | Convertir binario en balance acumulado. |
| 204 | Longest Repeating Character Replacement | Medium | Sliding window | Problema nuevo | Mantener frecuencia máxima y ventana válida. |
| 205 | Permutation in String | Medium | Sliding window | Problema nuevo | Comparar conteos de ventana fija sin rehacer todo. |
| 206 | Find All Anagrams in a String, repetición | Medium | Sliding window / hashing | Repetición dirigida | Reforzar ventana fija y normalización de frecuencias. |
| 207 | Minimum Size Subarray Sum | Medium | Sliding window | Problema nuevo | Practicar contracción con suma positiva. |
| 208 | Daily Temperatures, repetición cronometrada | Medium | Monotonic stack | Repetición dirigida | Reforzar pila monotónica y distancia a siguiente mejor estado. |
| 209 | Online Stock Span | Medium | Monotonic stack | Problema nuevo | Convertir pila monotónica en estructura con estado incremental. |
| 210 | Asteroid Collision | Medium | Stack | Problema nuevo | Modelar choques, cancelación y casos sin interacción. |
| 211 | Decode String | Medium | Stack | Problema nuevo | Practicar pila de contexto y construcción de cadenas. |
| 212 | Search in Rotated Sorted Array II | Medium | Binary search | Problema nuevo | Manejar duplicados que degradan la condición monótona. |
| 213 | Find First and Last Position, repetición | Medium | Binary search | Repetición dirigida | Separar lower bound y upper bound con tests de bordes. |
| 214 | Find Peak Element, repetición | Medium | Binary search | Repetición dirigida | Reforzar búsqueda sobre propiedad local, no sobre orden total. |
| 215 | Koko Eating Bananas, repetición | Medium | Binary search on answer | Repetición dirigida | Explicar condición monótona y límites de respuesta. |
| 216 | Capacity To Ship Packages Within D Days, repetición | Medium | Binary search on answer | Repetición dirigida | Comparar límites inferiores/superiores y validación de capacidad. |
| 217 | Top K Frequent Elements, repetición | Medium | Hashing / heap | Repetición dirigida | Comparar heap, bucket sort y orden estable para tests. |
| 218 | Group Anagrams, repetición | Medium | Hashing / strings | Repetición dirigida | Reforzar llave canónica y costos de ordenar vs contar. |
| 219 | Minimum Window Substring, repetición hard | Hard | Sliding window | Repetición dirigida | Consolidar contracción con conteos requeridos y sobrantes. |
| 220 | Largest Rectangle in Histogram, repetición hard | Hard | Monotonic stack | Repetición dirigida | Reforzar pila monotónica con centinela y anchura correcta. |

## Criterio de TDD por candidato

Cada candidato se ejecutará en un PR posterior con:

- test rojo de caso normal;
- test rojo de caso borde;
- test rojo de caso mínimo;
- implementación mínima;
- refactor si mejora claridad;
- nota de complejidad temporal y espacial;
- registro de aprendizaje si fue repetición dirigida.

## Problemas descartados por ahora

| Candidato | Motivo |
| --- | --- |
| Suffix Tree | Demasiado especializado para consolidación 191-220. |
| The Skyline Problem | Ya existe en geometría y pertenece mejor a hard selectivo. |
| Word Search II | Mezcla tries y backtracking; conviene más adelante. |
| Median from Data Stream | Ya está cubierto; no aporta consolidación inmediata. |
| My Calendar II | Ya está en range queries; se reserva para repetición 241-260. |

## Evidencia esperada al cerrar sub-bloques

- funciones nuevas o repeticiones documentadas en `src/patterns`;
- tests en la familia correspondiente;
- notas de repetición cuando aplique;
- conteo actualizado solo si hay evidencia nueva;
- validaciones locales y remotas en verde.

## Ejecución registrada

| Candidato | Estado | Evidencia |
| --- | --- | --- |
| 192: Valid Palindrome II | Implementado y probado; pendiente de revisión humana como contenido educativo. | Issue #32, función `valid_palindrome_with_one_removal` y tests de bifurcación izquierda/derecha. |
| 193: Is Subsequence | Implementado y probado; pendiente de revisión humana como contenido educativo. | Issue #31, función `is_subsequence` y tests de orden, vacío y repetidos. |
| 194: Merge Sorted Array | Implementado y probado; pendiente de revisión humana como contenido educativo. | Issue #35, función `merge_sorted_array` y tests de mezcla, vacíos y duplicados. |
| 195: Find Pivot Index | Implementado y probado; pendiente de revisión humana como contenido educativo. | Issue #36, función `pivot_index` y tests de centro, extremos, ausencia y negativos. |
| 197: Backspace String Compare | Implementado y probado; pendiente de revisión humana como contenido educativo. | Issue #40, función `backspace_compare` y tests de coincidencia, diferencia y retrocesos encadenados. |
| 198: Baseball Game | Implementado y probado; pendiente de revisión humana como contenido educativo. | Issue #39, función `baseball_game` y tests de historial compuesto, operaciones y entradas inválidas. |
| 199: Remove Duplicates from Sorted Array II | Implementado y probado; pendiente de revisión humana como contenido educativo. | Issue #44, función `remove_duplicates_sorted_at_most_twice` y tests de repetidos, cortos y distintos. |
| 200: Sort Colors | Implementado y probado; pendiente de revisión humana como contenido educativo. | Issue #43, función `sort_colors` y tests de partición, repetidos y entradas mínimas. |
| 202: Continuous Subarray Sum | Implementado y probado; pendiente de revisión humana como contenido educativo. | Issue #48, función `continuous_subarray_sum` y tests de múltiplo, longitud mínima, negativos y divisor cero. |
| 203: Contiguous Array | Implementado y probado; pendiente de revisión humana como contenido educativo. | Issue #50, función `contiguous_array` y tests de balance total, entradas mínimas y ausencia de balance. |
| 204: Longest Repeating Character Replacement | Implementado y probado; pendiente de revisión humana como contenido educativo. | Issue #52, función `longest_repeating_character_replacement` y tests de presupuesto, contracción y texto vacío. |
| 205: Permutation in String | Implementado y probado; pendiente de revisión humana como contenido educativo. | Issue #51, función `permutation_in_string` y tests de coincidencia, ausencia y patrón vacío. |
| 207: Minimum Size Subarray Sum | Implementado y probado; pendiente de revisión humana como contenido educativo. | Issue #56, función `minimum_size_subarray_sum` y tests de ventana mínima, ausencia y objetivo no positivo. |
| 209: Online Stock Span | Implementado y probado; pendiente de revisión humana como contenido educativo. | Issue #58, tipo `StockSpanner` y tests de estado incremental, precios iguales y descenso. |
| 210: Asteroid Collision | Implementado y probado; pendiente de revisión humana como contenido educativo. | Issue #60, función `asteroid_collision` y tests de choque simple, mutuo, encadenado y ausencia. |
| 211: Decode String | Implementado y probado; pendiente de revisión humana como contenido educativo. | Issue #62, función `decode_string` y tests de repetición, anidamiento, fragmentos y entrada vacía. |
| 212: Search in Rotated Sorted Array II | Implementado y probado; pendiente de revisión humana como contenido educativo. | Issue #64, función `search_rotated_with_duplicates` y tests de presencia, ausencia y duplicados ambiguos. |

## Relación con el milestone

Este bloque abre el milestone `Horizonte 400 — Consolidación 191-260`. Los
issues #7 y #6 completan el tramo con simulacros cronometrados y repeticiones
dirigidas. Hasta que esos issues cierren, el milestone permanece abierto.
