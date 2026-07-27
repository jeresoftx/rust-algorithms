# Profundización 316-330: patrones mixtos y cierre de brechas

**Estado:** bloque preparado; no representa soluciones implementadas ni
contenido publicado.

**Trazabilidad:** [issue #14](https://github.com/jeresoftx/rust-algorithms/issues/14).

Este bloque termina la profundización 261-330. Su trabajo no es rellenar los
últimos números: usa los huecos restantes de la matriz de cobertura para
practicar decisiones que atraviesan familias de algoritmos.

## Concepto

Un patrón mixto aparece cuando el enunciado puede parecer pertenecer a una
familia, pero la solución correcta depende de una segunda propiedad: un string
se vuelve grafo, una consulta se vuelve evento, una decisión greedy requiere un
estado de programación dinámica o una optimización se reduce a factibilidad.

## Problema

Cerrar un milestone por numeración incentiva repetir variaciones de los mismos
patrones. Eso añade volumen sin mejorar diagnóstico. La profundización solo
queda lista para revisión cuando sus últimos candidatos explican qué brecha
cubren y qué alternativa se evita.

## Alternativas

1. **Repetir los problemas más conocidos.** Baja la incertidumbre, pero no
   cierra brechas.
2. **Agregar temas especializados sin preparación.** Puede elevar dificultad,
   pero rompe la progresión pedagógica.
3. **Elegir quince problemas mixtos por brecha y registrar la evidencia que
   deberá existir al ejecutarlos.** Mantiene continuidad, claridad y una salida
   revisable hacia el tramo final.

Se adopta la tercera alternativa.

## Candidatos formalizados

| # | Candidato | Dificultad | Familias | Brecha que cubre | Alternativa a comparar |
| ---: | --- | --- | --- | --- | --- |
| 316 | Minimum Window Subsequence | Hard | Strings + DP / two pointers | Ventanas con orden, no solo conteos | Probar todos los inicios frente a DP de subsecuencias. |
| 317 | Shortest Common Supersequence | Hard | Strings + DP | Reconstrucción desde LCS | Longitud solamente frente a reconstrucción explicable. |
| 318 | Palindrome Pairs | Hard | Strings + trie / hashing | Prefijos y sufijos con reverso | Comparar todos los pares frente a indexar fragmentos. |
| 319 | Word Break II | Hard | Strings + DP + backtracking | Enumeración con memoización | Backtracking puro frente a estados de sufijo. |
| 320 | Minimum Cost to Cut a Stick | Hard | DP por intervalo + ordenamiento | Decisiones de corte dependientes | Greedy local frente a costo de subintervalos. |
| 321 | Remove Boxes | Hard | DP tridimensional | Estado con contexto de color | Eliminar cajas de inmediato frente a conservar rachas. |
| 322 | Maximum Vacation Days | Hard | DP + grafo por semanas | Decisión temporal con conexiones | Elegir el vuelo de mejor ganancia local. |
| 323 | Parallel Courses III | Medium | Grafo + DP topológica | Duración acumulada en DAG | Contar niveles frente a propagar tiempos críticos. |
| 324 | Minimum Height Trees | Medium | Grafos + poda por grados | Centros de árbol | BFS desde cada nodo frente a pelar hojas. |
| 325 | Smallest Range Covering Elements from K Lists | Hard | Heap + ventana de rango | Frontera de múltiples listas | Aplanar y ordenar frente a mantener un representante por lista. |
| 326 | Falling Squares | Hard | Rangos + compresión + segment tree | Actualizaciones sobre intervalos | Simulación por posiciones frente a máximos por rango. |
| 327 | Range Module | Hard | Rangos + intervalos ordenados | Consultas dinámicas de cobertura | Lista lineal frente a estructura de intervalos mantenida. |
| 328 | Candy | Hard | Greedy en dos pasadas | Restricciones a ambos vecinos | Ajustar localmente frente a combinar pendientes izquierda/derecha. |
| 329 | Maximum Performance of a Team | Hard | Greedy + heap | Selección bajo producto de mínimos | Probar combinaciones frente a ordenar por eficiencia. |
| 330 | IPO | Hard | Greedy + heap | Proyectos habilitados por capital | Elegir máxima ganancia global frente a frontera factible. |

## Invariantes a verbalizar

| Grupo | Invariante que debe aparecer antes de codificar |
| --- | --- |
| Strings 316-319 | El estado distingue posición, prefijo/sufijo o corte; no se reutiliza trabajo sin una llave válida. |
| DP 320-322 | La transición fija una última decisión que separa subproblemas independientes. |
| Grafos 323-324 | El orden topológico o la poda reduce solo estados cuya información ya es definitiva. |
| Rangos 325-327 | La estructura representa una frontera o intervalo completo; actualizar un punto no debe destruir cobertura previa. |
| Greedy 328-330 | La elección local mantiene una frontera factible que puede justificarse por orden o reemplazo. |

## Evidencia requerida al ejecutar

Cada candidato posterior debe dejar:

- tests rojos para caso normal, borde y mínimo;
- una explicación de la solución ingenua y su costo;
- invariante escrita antes de la implementación;
- complejidad temporal y espacial;
- una retro para `notes/review-queue.md` si falla bajo tiempo;
- estado `pendiente`, `repetir`, `fallado` o `dominado` respaldado por la
  evidencia anterior.

## Descartes conscientes

| Tema | Motivo |
| --- | --- |
| Dynamic connectivity offline | Requiere una introducción específica a rollback de Union-Find. |
| Heavy-light decomposition | Es valiosa, pero excede las estructuras de rango ya explicadas en el repo. |
| Suffix automaton | Compite con la ruta de suffix array y necesita una unidad conceptual propia. |
| Min-cost max-flow | Es un curso corto de optimización de redes, no un candidato aislado. |

## Cierre de profundización

Con los rangos 261-290, 291-315 y 316-330 formalizados, el milestone queda
listo para revisión humana posterior. Esto no declara que 140 problemas nuevos
estén resueltos: el catálogo funcional continúa en 190 hasta que cada
implementación entre con TDD y evidencia revisable.

El siguiente milestone 331-400 debe elegir hard selectivos, simulacros y cierre
con la misma disciplina de selección.
