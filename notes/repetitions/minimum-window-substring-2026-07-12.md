# Repetición Dirigida: Minimum Window Substring

Fecha: 2026-07-12.

## Resultado

Repetido.

Puntaje total: 24/25.

## Evidencia

- Se reforzó `min_window` con prueba de multiplicidad: `source = "AAABBC"`, `target = "AABC"`.
- Se agregó prueba para `target` vacío.
- `cargo test --test sliding_window_test min_window` pasó con 5 pruebas enfocadas.
- `cargo test --quiet` pasó con 283 pruebas.

## Invariante Repetido

- `missing == 0` significa que la ventana actual cubre todas las apariciones requeridas.
- Solo se contrae mientras la ventana siga siendo válida.
- Al mover `left`, si el conteo de ese byte queda positivo, vuelve a faltar un carácter requerido.

## Retro

- Qué salió bien: la implementación ya respetaba multiplicidad y contracción controlada.
- Qué faltaba: pruebas explícitas para objetivo con caracteres repetidos y objetivo vacío.
- Acción siguiente: repetir la explicación sin mirar notas el 2026-07-26.
