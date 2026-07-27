# Simulacros 221-240: práctica cronometrada de entrevista

**Estado:** bloque preparado; no representa soluciones implementadas ni
contenido publicado.

**Trazabilidad:** [issue #7](https://github.com/jeresoftx/rust-algorithms/issues/7).

Este bloque convierte 20 espacios del horizonte 400 en práctica bajo tiempo. No
busca acumular soluciones: busca medir comunicación, elección de patrón,
corrección, pruebas y recuperación después de errores.

## Concepto

Un simulacro es una situación controlada para observar comportamiento real. El
objetivo no es salir perfecto, sino producir evidencia: qué se clarificó, qué se
asumió, qué patrón se eligió, qué se probó y qué debe repetirse.

## Problema

Después de 190 problemas, el riesgo principal ya no es desconocer todos los
patrones. El riesgo es reconocerlos tarde, explicarlos con ruido o romper casos
borde por presión de tiempo.

Los problemas 221-240 se preparan como sesiones cronometradas de 45 a 60
minutos para entrenar esa ejecución.

## Alternativas

1. **Agregar 20 problemas nuevos sin simulacro.** Aumenta el conteo, pero no
   mide comunicación ni manejo de presión.
2. **Hacer simulacros libres sin registrar retro.** Practica velocidad, pero no
   deja aprendizaje revisable.
3. **Diseñar simulacros repetibles con rúbrica.** Mantiene evidencia y permite
   separar dominado, fallado y repetir.

Se adopta la tercera alternativa.

## Reglas del bloque

- Duración sugerida por sesión: 45 minutos para Medium; 60 minutos para Hard.
- No mirar la solución existente antes de terminar.
- Escribir tests primero cuando el simulacro implique código.
- Narrar en voz alta: requisitos, alternativa ingenua, patrón y complejidad.
- Registrar retro en `notes/review-queue.md`.
- Si el puntaje total es menor a 20/25, el problema entra a repetición dirigida.

## Distribución del bloque

| Dificultad | Cantidad | Rol |
| --- | ---: | --- |
| Easy | 4 | Calentamiento y control de ansiedad técnica. |
| Medium | 14 | Núcleo de entrevista cronometrada. |
| Hard | 2 | Profundidad selectiva con revisión posterior. |
| Total | 20 | Rango 221-240. |

## Simulacros formalizados

| # | Simulacro | Dificultad | Familia | Duración | Evidencia esperada |
| ---: | --- | --- | --- | ---: | --- |
| 221 | Valid Palindrome II | Easy | Two pointers | 45 min | Tests de borrado izquierdo/derecho y explicación de bifurcación. |
| 222 | Merge Sorted Array | Easy | Two pointers | 45 min | Escritura desde el final y complejidad O(n + m). |
| 223 | Find Pivot Index | Easy | Prefix sum | 45 min | Casos borde al inicio, centro y final. |
| 224 | Backspace String Compare | Easy | Stack / two pointers | 45 min | Comparación de dos enfoques y trade-off de memoria. |
| 225 | Sort Colors | Medium | Two pointers | 45 min | Invariante de tres regiones y tests de repetidos. |
| 226 | Continuous Subarray Sum | Medium | Prefix sum / hashing | 45 min | Residuos, longitud mínima y k cero cuando aplique. |
| 227 | Contiguous Array | Medium | Prefix sum / hashing | 45 min | Transformación 0 -> -1 y primer índice por balance. |
| 228 | Longest Repeating Character Replacement | Medium | Sliding window | 45 min | Frecuencia máxima y ventana permisiva. |
| 229 | Permutation in String | Medium | Sliding window | 45 min | Ventana fija y conteos equivalentes. |
| 230 | Minimum Size Subarray Sum | Medium | Sliding window | 45 min | Contracción con números positivos. |
| 231 | Online Stock Span | Medium | Monotonic stack | 45 min | Estado incremental y pila de pares. |
| 232 | Asteroid Collision | Medium | Stack | 45 min | Colisiones encadenadas y destrucción mutua. |
| 233 | Decode String | Medium | Stack | 45 min | Pila de conteos/contextos y anidación. |
| 234 | Search in Rotated Sorted Array II | Medium | Binary search | 45 min | Duplicados y degradación controlada. |
| 235 | Koko Eating Bananas | Medium | Binary search on answer | 45 min | Condición monótona y límites. |
| 236 | Capacity To Ship Packages Within D Days | Medium | Binary search on answer | 45 min | Validación de capacidad y límites correctos. |
| 237 | Top K Frequent Elements | Medium | Hashing / heap | 45 min | Comparar heap, bucket y costo de ordenamiento. |
| 238 | Group Anagrams | Medium | Hashing / strings | 45 min | Llave canónica y costos de sorting. |
| 239 | Minimum Window Substring | Hard | Sliding window | 60 min | Multiplicidad, contracción y retro completa. |
| 240 | Largest Rectangle in Histogram | Hard | Monotonic stack | 60 min | Centinela, anchura y explicación visual. |

## Rúbrica

Usar [`notes/mock-interview-rubric.md`](../notes/mock-interview-rubric.md) para
registrar:

- clarificación;
- enfoque;
- implementación;
- pruebas;
- comunicación.

Puntaje recomendado:

| Resultado | Acción |
| --- | --- |
| 23-25 | Marcar como dominado si no hubo errores relevantes. |
| 20-22 | Registrar como repetir si hubo tropiezos o dudas. |
| 15-19 | Repetición dirigida obligatoria. |
| 1-14 | Rehacer fuera de tiempo con explicación completa. |

## Estados

| Estado | Definición |
| --- | --- |
| Dominado | Resuelto en tiempo, con tests y explicación clara. |
| Repetir | Correcto, pero con bugs, dudas o comunicación débil. |
| Fallado | No se llegó a solución correcta bajo tiempo. |
| Descartado | El simulacro no aportó señal y se reemplaza con justificación. |

## Salida esperada por simulacro

Cada simulacro debe dejar:

- entrada en `notes/review-queue.md`;
- tests o referencia a tests existentes;
- complejidad temporal y espacial;
- decisión de estado;
- fecha tentativa de repetición si no quedó dominado.

## Relación con el bloque 191-220

Los simulacros 221-240 reutilizan candidatos de
[`consolidacion-191-220.md`](./consolidacion-191-220.md). La diferencia es el
modo de trabajo: aquí importa la ejecución bajo tiempo y la retro, no solo la
selección del candidato.
