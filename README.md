# Rust Algorithms

Repositorio de autoestudio de algoritmos y estructuras de datos en Rust, pensado para practicar resolución de problemas, análisis de complejidad y hábitos de entrevista técnica.

El objetivo es que este proyecto sirva como material de estudio reutilizable: cada solución debe tener una implementación clara, tests automatizados y notas que expliquen el patrón usado.

## Qué Contiene

- Soluciones de algoritmos organizadas por patrón.
- Tests de integración para validar cada ejercicio.
- Notas de estudio con ideas, invariantes y errores comunes.
- Un plan de práctica por semanas.
- Wiki con documentación resumida del avance.

## Estructura

```text
src/
  patterns/
    hashing.rs
    sliding_window.rs
    stack_queue.rs
    two_pointers.rs
tests/
  hashing_test.rs
  sliding_window_test.rs
  stack_queue_test.rs
  two_pointers_test.rs
notes/
  week-01.md
  week-02-03.md
  week-04-05.md
  rust-interview-patterns.md
  mistakes.md
plan/
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

Páginas destacadas:

- [Semana 1: Rust, Hashing, Arrays y Strings](https://github.com/jeresoftx/rust-algorithms/wiki/Semana-1-Rust-Hashing-Arrays-y-Strings)
- [Semanas 2 y 3: Arrays, Strings, Hashing y Sumas de Prefijos](https://github.com/jeresoftx/rust-algorithms/wiki/Semanas-2-y-3-Arrays-Strings-Hashing-y-Sumas-de-Prefijos)
- [Semanas 4 y 5: Two Pointers, Sliding Window y Stack](https://github.com/jeresoftx/rust-algorithms/wiki/Semanas-4-y-5-Two-Pointers-Sliding-Window-y-Stack)
- [Patrones de Rust para Entrevista](https://github.com/jeresoftx/rust-algorithms/wiki/Patrones-Rust-para-Entrevista)

## Estado Actual

- Semana documentada: 5
- Problemas implementados: 17
- Tests automatizados: 49
- Lenguaje: Rust

## Enfoque de Estudio

Este repositorio prioriza:

- Comprender patrones antes que memorizar soluciones.
- Practicar con tests desde el inicio.
- Explicar decisiones y compensaciones.
- Repetir problemas fallados hasta dominarlos.
- Mantener notas útiles para repaso futuro.
