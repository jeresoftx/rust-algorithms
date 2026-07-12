# Repetición Dirigida: Path With Minimum Effort

Fecha: 2026-07-12.

## Resultado

Repetido.

Puntaje total: 24/25.

## Evidencia

- Se reforzó `minimum_effort_path` con una matriz de una sola fila.
- `cargo test minimum_effort_path` pasó con 5 pruebas enfocadas.

## Invariante Repetido

- `efforts[row][col]` guarda el menor esfuerzo conocido para llegar a esa celda.
- El esfuerzo de una ruta no es la suma de pesos: es el máximo salto de altura visto en la ruta.
- Al extraer del heap una celda con esfuerzo mayor al registrado, ese estado se descarta.

## Retro

- Qué salió bien: el modelo minimax se mantuvo claro incluso sin alternativas de ruta.
- Qué faltaba: un caso degenerado de una sola fila.
- Acción siguiente: repetir la explicación verbal de por qué BFS simple no basta cuando los saltos tienen costos distintos.
