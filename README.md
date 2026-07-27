# Rust Algorithms

Repositorio de autoestudio de algoritmos y estructuras de datos en Rust, pensado para practicar resolución de problemas, análisis de complejidad y hábitos de entrevista técnica.

El objetivo es que este proyecto sirva como material de estudio reutilizable: cada solución debe tener una implementación clara, tests automatizados y notas que expliquen el patrón usado.

## Qué Contiene

- Soluciones de algoritmos organizadas por patrón.
- Tests de integración para validar cada ejercicio.
- Notas de estudio con ideas, invariantes y errores comunes.
- Un plan de práctica por semanas.
- Una ruta avanzada para ampliar el alcance después del plan base.
- Wiki con documentación resumida del avance.

## Estructura

```text
AGENTS.md
ROADMAP.md
LICENSE.md
LICENSE-MIT
LICENSE-APACHE
LICENSE-CC-BY-SA-4.0.md
.github/
  workflows/
    ci.yml
src/
  patterns/
    binary_search.rs
    dynamic_programming.rs
    graphs.rs
    greedy.rs
    hashing.rs
    heaps.rs
    intervals.rs
    linked_lists.rs
    matrices.rs
    math_bit.rs
    range_queries.rs
    recursion_backtracking.rs
    sliding_window.rs
    stack_queue.rs
    string_algorithms.rs
    trees.rs
    tries.rs
    two_pointers.rs
    weighted_graphs.rs
tests/
  binary_search_test.rs
  dynamic_programming_test.rs
  graphs_test.rs
  greedy_test.rs
  hashing_test.rs
  heaps_test.rs
  intervals_test.rs
  linked_lists_test.rs
  matrices_test.rs
  math_bit_test.rs
  range_queries_test.rs
  recursion_backtracking_test.rs
  sliding_window_test.rs
  stack_queue_test.rs
  string_algorithms_test.rs
  trees_test.rs
  tries_test.rs
  two_pointers_test.rs
  weighted_graphs_test.rs
  property_algorithms_test.rs
benches/
  algorithm_families.rs
diagrams/
  core-patterns.mmd
  math-geometry.mmd
  optimization-patterns.mmd
  recursive-structures.mmd
notes/
  week-01.md
  week-02-03.md
  week-04-05.md
  week-06.md
  week-07-08.md
  week-09-10.md
  week-11-12.md
  week-13.md
  week-14-15.md
  week-16.md
  week-17-18.md
  week-19-20.md
  week-21-23.md
  week-24-25.md
  complexity-cheatsheet.md
  mock-interview-rubric.md
  review-queue.md
  simulations/
    simulacro-01-minimum-window-substring.md
    simulacro-02-course-schedule.md
    simulacro-03-coin-change.md
  rust-interview-patterns.md
  mistakes.md
plan/
  criterios-seleccion-191-plus.md
  consolidacion-191-220.md
  estandar-rfc-0001.md
  horizonte-400.md
  matriz-cobertura-400.md
  plantilla-bloque-autonomo-191-plus.md
  plan-alcance-avanzado.md
  repeticiones-241-260.md
  simulacros-221-240.md
  plan-algoritmos-rust.md
```

## Gobernanza

- `AGENTS.md` es la guía de arranque para humanos e IA en este repositorio.
- `ROADMAP.md` apunta al plan vivo sin duplicarlo.
- `plan/estandar-rfc-0001.md` registra la alineación de este repo con el Manual Fundacional RFC-0001.
- `LICENSE.md` resume la doble licencia: código bajo `MIT OR Apache-2.0`; contenido educativo bajo `CC BY-SA 4.0`.

## Cómo Usarlo

Ejecutar los tests:

```bash
cargo test
```

Formatear el código:

```bash
cargo fmt
```

Lint y verificación completa:

```bash
cargo fmt --check
cargo clippy --all-targets
cargo test
```

Compilar benchmarks sin ejecutarlos:

```bash
cargo bench --no-run
```

Para estudiar un problema:

1. Leer el enunciado y escribir ejemplos.
2. Identificar el patrón principal.
3. Escribir tests antes de implementar.
4. Resolver con una solución clara.
5. Anotar complejidad temporal y espacial.
6. Registrar errores o bloqueos en la bitácora.

## Wiki

La wiki es el punto de entrada recomendado para estudiar el avance, revisar patrones y seguir las semanas del plan:

[Wiki de Rust Algorithms](https://github.com/jeresoftx/rust-algorithms/wiki)

Planes locales:

- [Plan base de algoritmos en Rust](plan/plan-algoritmos-rust.md)
- [Plan de alcance avanzado](plan/plan-alcance-avanzado.md)
- [Horizonte 400](plan/horizonte-400.md)
- [Criterios de selección para problemas 191+](plan/criterios-seleccion-191-plus.md)
- [Consolidación 191-220](plan/consolidacion-191-220.md)
- [Matriz de cobertura Easy, Medium y Hard](plan/matriz-cobertura-400.md)
- [Plantilla de bloque autónomo para problemas 191+](plan/plantilla-bloque-autonomo-191-plus.md)
- [Repeticiones 241-260](plan/repeticiones-241-260.md)
- [Simulacros 221-240](plan/simulacros-221-240.md)
- [Profundización 261-290](plan/profundizacion-261-290.md)
- [Profundización 291-315](plan/profundizacion-291-315.md)
- [Profundización 316-330](plan/profundizacion-316-330.md)
- [Cierre 331-360](plan/cierre-331-360.md)
- [Cierre 361-400](plan/cierre-361-400.md)
- [Reporte de cierre de planeación del Horizonte 400](plan/reporte-horizonte-400.md)
- [Guía de explicación avanzada para simulacros](notes/explicacion-avanzada-trade-offs.md)
- [Cierre del hito de 140 problemas](plan/cierre-hito-140.md)
- [Cierre del hito de 190 problemas](plan/cierre-hito-190.md)

Páginas destacadas:

- [Plan de Alcance Avanzado](https://github.com/jeresoftx/rust-algorithms/wiki/Plan-de-Alcance-Avanzado)
- [Cierre del Hito de 190 Problemas](https://github.com/jeresoftx/rust-algorithms/wiki/Cierre-del-Hito-190)
- [Guía de Complejidad](https://github.com/jeresoftx/rust-algorithms/wiki/Guia-de-Complejidad)
- [Semana 1: Rust, Hashing, Arrays y Strings](https://github.com/jeresoftx/rust-algorithms/wiki/Semana-1-Rust-Hashing-Arrays-y-Strings)
- [Semanas 2 y 3: Arrays, Strings, Hashing y Sumas de Prefijos](https://github.com/jeresoftx/rust-algorithms/wiki/Semanas-2-y-3-Arrays-Strings-Hashing-y-Sumas-de-Prefijos)
- [Semanas 4 y 5: Two Pointers, Sliding Window y Stack](https://github.com/jeresoftx/rust-algorithms/wiki/Semanas-4-y-5-Two-Pointers-Sliding-Window-y-Stack)
- [Semana 6: Búsqueda Binaria](https://github.com/jeresoftx/rust-algorithms/wiki/Semana-6-Busqueda-Binaria)
- [Semanas 7 y 8: Recursión, Backtracking y Listas Enlazadas](https://github.com/jeresoftx/rust-algorithms/wiki/Semanas-7-y-8-Recursion-Backtracking-y-Linked-Lists)
- [Semanas 9 y 10: Árboles](https://github.com/jeresoftx/rust-algorithms/wiki/Semanas-9-y-10-Arboles)
- [Semanas 11 y 12: Grafos](https://github.com/jeresoftx/rust-algorithms/wiki/Semanas-11-y-12-Grafos)
- [Semana 13: Montículos, Intervalos y Greedy](https://github.com/jeresoftx/rust-algorithms/wiki/Semana-13-Monticulos-Intervalos-y-Greedy)
- [Semanas 14 y 15: Programación Dinámica](https://github.com/jeresoftx/rust-algorithms/wiki/Semanas-14-y-15-Programacion-Dinamica)
- [Semana 16: Simulacros y Repaso](https://github.com/jeresoftx/rust-algorithms/wiki/Semana-16-Simulacros-y-Repaso)
- [Semanas 17 y 18: Complejidad, Matemáticas y Bits](https://github.com/jeresoftx/rust-algorithms/wiki/Semanas-17-y-18-Complejidad-Matematicas-y-Bits)
- [Semanas 19 y 20: Tries y Cadenas](https://github.com/jeresoftx/rust-algorithms/wiki/Semanas-19-y-20-Tries-y-Cadenas)
- [Semanas 21 a 23: Grafos Ponderados](https://github.com/jeresoftx/rust-algorithms/wiki/Semanas-21-a-23-Grafos-Ponderados)
- [Semanas 24 y 25: Consultas por Rangos](https://github.com/jeresoftx/rust-algorithms/wiki/Semanas-24-y-25-Consultas-por-Rangos)
- [Semana 26: Geometría y Temas Selectivos](https://github.com/jeresoftx/rust-algorithms/wiki/Semana-26-Geometria-y-Temas-Selectivos)
- [Semanas 27 y 28: Simulacros Avanzados](https://github.com/jeresoftx/rust-algorithms/wiki/Semanas-27-y-28-Simulacros-Avanzados)
- [Patrones de Rust para Entrevista](https://github.com/jeresoftx/rust-algorithms/wiki/Patrones-Rust-para-Entrevista)

## Estado Actual

- Semana documentada: 27-28
- Problemas implementados: 206
- Tests automatizados: 498 pruebas deterministas/property + 9 doctests
- Ruta avanzada: hito 190 completado; horizonte 400 documentado como plan
  opcional futuro
- Lenguaje: Rust

## Benchmarks y Property Testing

Las dependencias `criterion` y `proptest` son solo de desarrollo. Se agregan
porque el repo no solo comprueba respuestas: también enseña complejidad,
invariantes y regresiones de rendimiento donde la señal es real.

| Familia | Benchmarks | Property testing | Decision |
| --- | --- | --- | --- |
| `binary_search` | Si | Si | Bench para `search_insert`; property contra `partition_point`, porque la invariante de lower-bound es generativa. |
| `dynamic_programming` | Si | No por ahora | Bench para LIS; property tests requieren oraculos exponenciales acotados y se posponen para no agregar ruido. |
| `range_queries` | Si | Si | Bench de Fenwick; property compara prefijos contra un vector ingenuo tras actualizaciones. |
| `string_algorithms` | Si | No por ahora | Bench de KMP; property testing queda fuera hasta definir generadores ASCII/Unicode por algoritmo. |
| `weighted_graphs` | Si | No por ahora | Bench de Dijkstra; propiedades de grafos requieren generadores conectados/ponderados especificos. |
| `two_pointers` | No por ahora | Si | `sorted_squares` se prueba por multiconjunto ordenado; no se mide porque el coste O(n) ya queda claro y barato. |
| Resto de familias | No por ahora | No por ahora | La cobertura determinista actual expresa mejor los casos borde; se agregaran benches/properties solo cuando haya una invariante o regresion concreta que medir. |

## Enfoque de Estudio

Este repositorio prioriza:

- Comprender patrones antes que memorizar soluciones.
- Practicar con tests desde el inicio.
- Explicar decisiones y compensaciones.
- Repetir problemas fallados hasta dominarlos.
- Mantener notas útiles para repaso futuro.
