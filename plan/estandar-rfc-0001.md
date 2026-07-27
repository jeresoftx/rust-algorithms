# Checklist: alinear el repo con RFC-0001

El catálogo de algoritmos (190 problemas implementados; ver
[plan-algoritmos-rust.md](./plan-algoritmos-rust.md)) está funcionalmente
completo para su estado actual, pero no cumple todavía el estándar publicable
del manual (RFC-0001 §13, §15, §20). Este documento es el tablero de esa brecha
específica.

Nada aquí tiene fecha límite (RFC-0001 §1); es una lista de qué falta, no un
sprint.

## 1. Gobernanza del repositorio (§15 — plantilla obligatoria)

- Registro §2: la gobernanza convierte el catálogo en un repositorio legible como
  libro de ingeniería; la alternativa era dejar convenciones implícitas, pero
  RFC-0001 §15 exige archivos canónicos para que humanos e IA trabajen igual.

- [x] Crear `AGENTS.md` (obligatorio en todo repositorio, §20). Instanciar el
      canónico con `{colección}` = núcleo técnico (algoritmos) y `{tema}` =
      algoritmos, estructuras de datos y patrones de resolución en Rust.
- [x] Crear `ROADMAP.md` en la raíz. No duplicar
      `plan/plan-algoritmos-rust.md`; puede ser un puntero corto a él.
- [x] Doble licencia (§15): agregar `LICENSE-APACHE` y una licencia de
      contenido `LICENSE-CC-BY-SA-4.0.md` para `notes/`, `plan/`, diagramas y
      Markdown; actualizar `Cargo.toml` (`license = "MIT OR Apache-2.0"`) y
      dejar `LICENSE.md` como índice de ambas.
- [x] Crear `diagrams/` con diagramas Mermaid por familia algorítmica
      (patrones base, estructuras recursivas, optimización, matemáticas y
      geometría).
- [x] CI en `.github/workflows/`: `cargo fmt --check`, `cargo clippy` sin
      advertencias, `cargo test` en cada push (§7).

## 2. Código (§13 — estándares)

- Registro §2: las advertencias de clippy deben resolverse porque este repo
  enseña a razonar sobre algoritmos y Rust a la vez; la alternativa de
  silenciarlas globalmente habría ocultado problemas reales. Solo se justifica
  una excepción local cuando el índice explícito comunica mejor el algoritmo
  (`rotate_image`, transposición in-place).

- [x] Limpiar las advertencias actuales de `cargo clippy --all-targets`
      (`needless_range_loop`, `ptr_arg`, `collapsible_match`,
      `too_many_arguments`) o justificar por escrito cada excepción que se
      mantenga.
- Registro §2: la API pública necesita documentación ejecutable porque el
  catálogo funciona también como material de estudio; la alternativa de dejar
  solo tests de integración valida comportamiento, pero no enseña uso desde la
  interfaz pública.

- [x] Agregar doc-comments (`///`) a la API pública de cada familia: qué hace,
      un ejemplo ejecutable (doctest) y notas de complejidad cuando aplique.
- Registro §2: los benchmarks aplican cuando ayudan a observar complejidad o
  regresiones de familias sensibles al tamaño de entrada; la alternativa de
  medir todo produciría ruido y tiempo de mantenimiento sin valor didáctico.
  Se agregó `criterion` como dependencia de desarrollo porque Criterion es el
  estándar práctico para microbenchmarks estables en Rust.

- [x] Decidir y documentar, por familia, si los benchmarks aplican. Donde no
      apliquen, declararlo explícitamente en el README o documentación
      correspondiente en vez de omitirlo en silencio (§14). Donde sí apliquen,
      agregar `criterion` y una carpeta `benches/`.
- Registro §2: property testing aplica donde hay invariantes simples y
  generadores confiables (lower-bound, cuadrados ordenados, aritmética,
  Fenwick); se agregó `proptest` como dependencia de desarrollo para probar
  familias de entradas sin inflar casos manuales.

- [x] Misma decisión y documentación para property testing (`proptest`): dónde
      aporta y dónde se declara que no aplica.

## 3. Consistencia con lo ya construido

- Registro §2: el README debe ser una puerta de entrada consistente con la
  gobernanza nueva; la alternativa de dejar `AGENTS.md`, `ROADMAP.md`,
  licencias y diagramas descubiertos solo por exploración haría más difícil
  retomar el repo en frío.

- [x] Una vez agregado `AGENTS.md`/`ROADMAP.md`, revisar que el README
      principal siga apuntando a las mismas fuentes de verdad sin quedar
      desactualizado.
- [x] Confirmar que `Cargo.toml` (`license`) y el nuevo `LICENSE.md` no se
      contradigan.

## Fuera de este checklist

Problemas 191+, una nueva fase de estudio o expansión de alcance no forman parte
de esta brecha; se deciden aparte.
