# ROADMAP

Estado de avance de `rust-algorithms`, repositorio del núcleo técnico de
Jeresoft Academy para algoritmos y estructuras de datos en Rust.

No hay fechas límite: este es un proyecto de legado (RFC-0001 §1). Este archivo
solo orienta; el tablero detallado vive en
[`plan/plan-algoritmos-rust.md`](plan/plan-algoritmos-rust.md).

## Estado actual

El catálogo funcional de práctica contiene 190 problemas implementados y
probados, organizados por familias de patrones algorítmicos:

- Arrays, strings, hashing, two pointers y sliding window.
- Búsqueda binaria, stacks/queues, heaps, intervalos y greedy.
- Recursión, backtracking, árboles, tries y grafos.
- Programación dinámica, matemáticas/bits, matrices, geometría y range queries.

## Alineación RFC-0001

La brecha actual no es agregar problemas nuevos. Es alinear el repo con los
estándares publicables de la RFC-0001: gobernanza, clippy limpio,
doc-comments, benchmarks y property testing donde apliquen.

El checklist de esa brecha vive en
[`plan/estandar-rfc-0001.md`](plan/estandar-rfc-0001.md).

## Fuera de alcance por ahora

Problemas 191+, una nueva fase temática o cambios de fondo al plan de estudio no
forman parte de la alineación actual; se deciden aparte.
