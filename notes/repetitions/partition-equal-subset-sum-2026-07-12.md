# Repetición Dirigida: Partition Equal Subset Sum

Fecha: 2026-07-12.

## Resultado

Repetido.

Puntaje total: 24/25.

## Evidencia

- Se reforzó `can_partition` con un caso que fallaría si se reutilizara un valor dentro de la misma iteración: `[1, 2, 5]`.
- Se agregó cobertura para ceros: `[0, 0]`.
- `cargo test can_partition` pasó con 3 pruebas enfocadas.

## Invariante Repetido

- `dp[x]` indica si algún subconjunto de los valores ya procesados alcanza suma `x`.
- Recorrer `target` en reversa evita que el mismo número se use más de una vez.
- Si la suma total es impar, no existe partición en dos subconjuntos iguales.

## Retro

- Qué salió bien: el patrón 0/1 quedó expresado por el recorrido inverso.
- Qué faltaba: una prueba explícita contra reutilización accidental del mismo valor.
- Acción siguiente: repetir la explicación de por qué el recorrido hacia adelante cambia el problema a unbounded knapsack.
