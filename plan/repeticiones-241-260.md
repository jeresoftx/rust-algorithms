# Repeticiones 241-260: cierre dirigido de consolidación

**Estado:** cierre operativo documentado; no representa contenido publicado ni
capítulos revisados.

**Trazabilidad:** [issue #6](https://github.com/jeresoftx/rust-algorithms/issues/6).

Este bloque cierra el tramo 191-260 con una decisión deliberada: después de
consolidar candidatos y preparar simulacros, los últimos 20 espacios no agregan
temas nuevos por inercia. Se usan para repetir problemas donde ya existe señal
de riesgo: errores anteriores, simulacros con acción pendiente, patrones que
suelen romperse bajo tiempo o soluciones correctas que todavía necesitan mejor
explicación.

## Concepto

Una repetición dirigida no es volver a copiar una solución. Es rehacer un
problema con memoria mínima, bajo una restricción explícita y con una pregunta
pedagógica concreta.

La repetición cuenta como evidencia solo si deja:

- motivo de repetición;
- tests o referencia a tests existentes;
- invariante principal;
- complejidad temporal y espacial;
- retro breve;
- estado final: dominado, repetir o fallado.

## Problema

El riesgo después de 240 espacios no es la falta de variedad. El riesgo es
confundir avance con acumulación y cerrar huecos débiles con problemas nuevos
que no atacan el patrón fallado.

Este documento convierte la cola de repaso y los simulacros en un bloque
auditable para revisión humana posterior.

## Alternativas

1. **Agregar 20 problemas nuevos.** Aumenta el conteo, pero debilita la señal
   de aprendizaje.
2. **Repetir solo lo que se recuerde en el momento.** Es cómodo, pero no deja
   trazabilidad.
3. **Cerrar con repeticiones dirigidas desde evidencia existente.** Mantiene el
   foco en patrones débiles y evita inflar el curso.

Se adopta la tercera alternativa.

## Fuentes de selección

El bloque se alimenta de:

- [`notes/review-queue.md`](../notes/review-queue.md);
- [`notes/repetitions/`](../notes/repetitions/);
- [`notes/simulations/`](../notes/simulations/);
- candidatos reservados en
  [`consolidacion-191-220.md`](./consolidacion-191-220.md).

## Bloque formalizado

| # | Problema | Familia | Tipo | Evidencia actual | Acción de cierre |
| ---: | --- | --- | --- | --- | --- |
| 241 | Minimum Window Substring | Sliding window | Repetición confirmada | Nota de repetición y tests enfocados | Reexplicar multiplicidad sin mirar notas. |
| 242 | Validate Binary Search Tree | Árboles | Repetición confirmada | Nota de repetición y tests de duplicados | Reexplicar límites heredados estrictos. |
| 243 | Course Schedule | Grafos | Repetición confirmada | Cola de repaso y simulacro | Rehacer detección de ciclo con indegree. |
| 244 | Coin Change | Programación dinámica | Repetición confirmada | Cola de repaso y simulacro | Reexplicar centinela y monto inalcanzable. |
| 245 | Path With Minimum Effort | Grafos ponderados | Repetición confirmada | Nota de repetición y simulacro | Defender Dijkstra minimax contra BFS. |
| 246 | Partition Equal Subset Sum | Knapsack | Repetición confirmada | Nota de repetición y tests de reutilización | Explicar por qué el recorrido inverso importa. |
| 247 | Longest Duplicate Substring | Cadenas avanzadas | Repetición pendiente | Simulacro con 21/25 | Repetir con rolling hash doble o suffix array. |
| 248 | RangeSumQuery | Range queries | Repetición pendiente | Simulacro con acción pendiente | Reimplementar Fenwick Tree con tests primero. |
| 249 | Move Zeroes | Two pointers | Repetición dirigida | Candidato 191 | Rehacer in-place con invariante de escritura. |
| 250 | Running Sum of 1d Array | Prefix sum | Repetición dirigida | Candidato 196 | Explicar acumulado destructivo contra vector nuevo. |
| 251 | Subarray Sum Equals K | Prefix sum / hashing | Repetición dirigida | Candidato 201 | Rehacer conteo de prefijos con negativos. |
| 252 | Find All Anagrams in a String | Sliding window | Repetición dirigida | Candidato 206 | Rehacer ventana fija y conteos equivalentes. |
| 253 | Daily Temperatures | Monotonic stack | Repetición dirigida | Candidato 208 | Explicar pila decreciente y distancia. |
| 254 | Find First and Last Position | Binary search | Repetición dirigida | Candidato 213 | Separar lower bound y upper bound. |
| 255 | Find Peak Element | Binary search | Repetición dirigida | Candidato 214 | Explicar propiedad local, no orden total. |
| 256 | Koko Eating Bananas | Binary search on answer | Repetición dirigida | Candidato 215 | Justificar condición monótona y límites. |
| 257 | Capacity To Ship Packages Within D Days | Binary search on answer | Repetición dirigida | Candidato 216 | Validar capacidad sin perder días. |
| 258 | Top K Frequent Elements | Hashing / heap | Repetición dirigida | Candidato 217 | Comparar heap, bucket y sort. |
| 259 | Group Anagrams | Hashing / strings | Repetición dirigida | Candidato 218 | Comparar llave por ordenamiento contra conteo. |
| 260 | My Calendar II | Range queries | Repetición reservada | Candidato descartado para 191-220 | Repetir como cierre de intervalos y solapamientos. |

## Regla de ejecución

Cada repetición debe ejecutarse en un espacio limpio:

1. leer solo el enunciado;
2. escribir al menos tres tests: caso normal, borde y mínimo;
3. resolver sin consultar la implementación actual;
4. comparar contra la implementación existente solo al final;
5. registrar retro en `notes/review-queue.md`;
6. si el resultado queda debajo de 20/25, devolverlo a la cola.

## Estados de cierre

| Estado | Significado |
| --- | --- |
| Dominado | Resuelto bajo tiempo, con tests, invariantes y explicación clara. |
| Repetir | Correcto, pero con duda, ruido de comunicación o bug corregido tarde. |
| Fallado | No alcanzó solución correcta bajo tiempo. |
| Pendiente | Programado para repetición posterior; no cuenta como dominado. |

## Evidencia ya disponible

Las siguientes repeticiones ya tienen evidencia escrita:

- [`minimum-window-substring-2026-07-12.md`](../notes/repetitions/minimum-window-substring-2026-07-12.md);
- [`validate-binary-search-tree-2026-07-12.md`](../notes/repetitions/validate-binary-search-tree-2026-07-12.md);
- [`path-with-minimum-effort-2026-07-12.md`](../notes/repetitions/path-with-minimum-effort-2026-07-12.md);
- [`partition-equal-subset-sum-2026-07-12.md`](../notes/repetitions/partition-equal-subset-sum-2026-07-12.md).

Course Schedule y Coin Change aparecen como repetidos en la cola de repaso y
con simulacro documentado. Si durante revisión humana se requiere mayor
trazabilidad, se deben crear notas individuales de repetición antes de moverlos
a dominado.

## Criterio para no inflar avance

Los problemas 247-260 quedan formalizados como repetición dirigida, no como
dominio probado. Su estado inicial es pendiente hasta que exista evidencia de
ejecución.

Este bloque cierra el milestone como plan de repetición y auditoría, no como
declaración de publicación del contenido.
