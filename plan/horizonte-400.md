# Horizonte 400: ruta opcional posterior al hito 190

**Estado:** plan futuro; no representa contenido publicado ni revisado.

**Trazabilidad:** [issue #1](https://github.com/jeresoftx/rust-algorithms/issues/1).

Este documento integra el conocimiento útil del borrador local
`plan/DSA_Google_Study_Plan.md` sin convertirlo en un segundo plan canónico. El
repositorio ya completó el hito de 190 problemas; el horizonte 400 es una ruta
opcional de profundización para entrevistas técnicas, no una corrección del plan
base.

## Concepto

Un horizonte de problemas es una dirección de práctica, no una meta de volumen
por sí misma. En algoritmos, resolver más ejercicios solo aporta valor cuando
cada problema entrena una habilidad nueva, refuerza una debilidad real o mejora
la comunicación bajo presión.

El número 400 se usa aquí como marco de largo plazo para organizar práctica,
simulacros y repetición deliberada después de tener una base sólida en Rust.

## Problema

El borrador DSA proponía una ruta de 12 semanas con temas frecuentes de
entrevistas Google L3-L5 y una meta aproximada de:

- 120 problemas Easy;
- 220 problemas Medium;
- 60 problemas Hard.

La mayoría de los temas ya existe en el plan base y en el alcance avanzado:
arrays, strings, hashing, listas, stacks, queues, árboles, heaps, tries, grafos,
programación dinámica, backtracking, greedy, bits, matemáticas, árboles de rango
y cadenas avanzadas.

Mantener ese borrador como archivo separado crearía dos problemas:

- duplicaría rutas que ya están documentadas;
- podría sugerir que el repo aún no cerró su hito actual, aunque el hito 190 ya
  está completo.

## Alternativas

1. **Conservar el borrador DSA como plan paralelo.** Sería rápido, pero dejaría
   dos fuentes de verdad con solapamientos y nombres distintos para el mismo
   avance.
2. **Descartar el borrador completo.** Evitaría duplicación, pero perdería la
   señal útil sobre preparación de entrevistas y distribución por dificultad.
3. **Integrar solo lo nuevo como horizonte futuro.** Mantiene el plan canónico
   limpio y rescata lo valioso: orientación a entrevista, volumen opcional,
   simulacros y proporción de dificultad.

Se adopta la tercera alternativa.

## Justificación

RFC-0001 privilegia claridad sobre acumulación. Para este repositorio, la
claridad significa separar tres niveles:

- `plan/plan-algoritmos-rust.md`: ruta base de autoestudio;
- `plan/plan-alcance-avanzado.md`: expansión que ya llevó el repo al hito 190;
- `plan/horizonte-400.md`: horizonte opcional posterior, todavía sin ejecutar.

Así el repo conserva una historia honesta: el estado actual está cerrado en 190
problemas, y cualquier crecimiento hacia 400 debe entrar por issues, milestones
y PRs revisables.

## Cobertura ya existente

| Tema del borrador DSA | Estado en el repo |
| --- | --- |
| Complejidad Big O, Θ, Ω y amortizada | Cubierto en `notes/complexity-cheatsheet.md` y fase avanzada. |
| Arrays, strings y hashing | Cubierto en plan base, módulos y tests existentes. |
| Linked lists, stacks, queues, trees, BST, heaps y tries | Cubierto por módulos de patrones y semanas documentadas. |
| BFS, DFS, topological sort y union-find | Cubierto en grafos básicos. |
| Dijkstra, Bellman-Ford, Floyd-Warshall, Prim, Kruskal y Tarjan | Cubierto en grafos ponderados y fase avanzada. |
| Programación dinámica, memoización, tabulación, knapsack, LIS, LCS y edit distance | Cubierto en programación dinámica. |
| Backtracking, greedy, matemáticas y bits | Cubierto en módulos y notas avanzadas. |
| Segment tree, Fenwick tree, Aho-Corasick, suffix array y convex hull | Cubierto en alcance avanzado. |
| Suffix tree | Tratado como tema conceptual; no se implementa sin necesidad pedagógica. |
| Meta 400 problemas | Nuevo como horizonte opcional, no como estado actual. |

## Ruta futura propuesta

| Milestone | Rango | Enfoque |
| --- | ---: | --- |
| Diseño y gobernanza | N/A | Evitar duplicación, definir criterios y preparar plantilla de bloques. |
| Consolidación | 191-260 | Reforzar patrones frecuentes, simulacros cronometrados y repeticiones. |
| Profundización | 261-330 | Combinar patrones, problemas medium avanzados y trade-offs. |
| Cierre | 331-400 | Hard selectivos, revisión de calidad y reporte final. |

La distribución 120/220/60 por dificultad se conserva como hipótesis de trabajo,
pero se valida en un issue separado antes de usarla como guía operativa.

La puerta de entrada para aceptar o descartar candidatos vive en
[`criterios-seleccion-191-plus.md`](./criterios-seleccion-191-plus.md).

La distribución por dificultad y familia vive en
[`matriz-cobertura-400.md`](./matriz-cobertura-400.md).

El flujo operativo para cada issue y PR vive en
[`plantilla-bloque-autonomo-191-plus.md`](./plantilla-bloque-autonomo-191-plus.md).

El primer bloque de consolidación vive en
[`consolidacion-191-220.md`](./consolidacion-191-220.md).

El bloque de simulacros cronometrados vive en
[`simulacros-221-240.md`](./simulacros-221-240.md).

El cierre por repeticiones dirigidas vive en
[`repeticiones-241-260.md`](./repeticiones-241-260.md).

El primer bloque de patrones compuestos de profundización vive en
[`profundizacion-261-290.md`](./profundizacion-261-290.md).

El bloque Medium y Hard con alta señal vive en
[`profundizacion-291-315.md`](./profundizacion-291-315.md).

El cierre de profundización con patrones mixtos vive en
[`profundizacion-316-330.md`](./profundizacion-316-330.md).

## Reglas para avanzar

- Cada bloque debe entrar por un issue asignado a `jeresoftx`, con milestone,
  labels y Project.
- Cada PR debe cerrar exactamente un issue con `Closes #N`.
- El avance autónomo debe usar un commit principal y squash merge con coautor.
- Los problemas nuevos deben incluir tests, explicación de complejidad y nota de
  aprendizaje cuando aplique.
- Las repeticiones cuentan solo si documentan qué cambió entre el fallo y la
  solución limpia.
- Ningún bloque se marca como publicado sin revisión humana.

## Estado del borrador DSA

El archivo `plan/DSA_Google_Study_Plan.md` se descarta como fuente local porque
su contenido quedó integrado aquí y en los issues del Project. No debe
recrearse salvo que Joel decida abrir una RFC o una nueva ruta de entrevistas
con alcance distinto.
