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
  plan-alcance-avanzado.md
  plan-algoritmos-rust.md
```

## Cómo Usarlo

Ejecutar los tests:

```bash
cargo test
```

Formatear el código:

```bash
cargo fmt
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

Páginas destacadas:

- [Plan de Alcance Avanzado](https://github.com/jeresoftx/rust-algorithms/wiki/Plan-de-Alcance-Avanzado)
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
- [Patrones de Rust para Entrevista](https://github.com/jeresoftx/rust-algorithms/wiki/Patrones-Rust-para-Entrevista)

## Estado Actual

- Semana documentada: 26
- Problemas implementados: 111
- Tests automatizados: 266
- Ruta avanzada: fase 5 iniciada
- Lenguaje: Rust

## Enfoque de Estudio

Este repositorio prioriza:

- Comprender patrones antes que memorizar soluciones.
- Practicar con tests desde el inicio.
- Explicar decisiones y compensaciones.
- Repetir problemas fallados hasta dominarlos.
- Mantener notas útiles para repaso futuro.
