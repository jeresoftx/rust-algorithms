# Simulacro 03: Coin Change

## Objetivo

Practicar programación dinámica de minimización, definiendo estado, transición y centinela antes de codificar.

Duración total: 45 minutos.

## Reglas

- No consultar `src/patterns/dynamic_programming.rs` antes de terminar.
- Escribir primero los casos de prueba.
- Definir explícitamente qué significa `dp[amount]`.
- Al terminar, registrar retro en `notes/review-queue.md`.

## Enunciado

Dado un arreglo de denominaciones `coins` y un `amount`, devolver el menor número de monedas necesario para formar ese monto.

Si no se puede formar el monto, devolver `-1`.

Se puede usar cada moneda tantas veces como sea necesario.

## Ejemplos

```text
coins = [1, 2, 5]
amount = 11
resultado = 3
```

```text
coins = [2]
amount = 3
resultado = -1
```

```text
coins = [1]
amount = 0
resultado = 0
```

## Preguntas de Clarificación

Antes de codificar, responder:

- ¿Las monedas pueden repetirse ilimitadamente?
- ¿Puede `amount` ser `0`?
- ¿Puede haber monedas con valor `0` o negativo?
- ¿Se necesita devolver combinación o solo cantidad mínima?

## Plan Esperado

1. Definir `dp[x]` como la menor cantidad de monedas para formar `x`.
2. Inicializar `dp[0] = 0`.
3. Usar un centinela mayor que cualquier respuesta posible.
4. Para cada monto, probar cada moneda que no exceda el monto.
5. Convertir el centinela final a `-1` si el monto sigue inalcanzable.

## Invariantes

- `dp[0]` siempre es `0`.
- Si `dp[x]` es centinela, `x` todavía no es alcanzable.
- La transición `dp[x] = min(dp[x], dp[x - coin] + 1)` solo es útil si `x - coin` ya es alcanzable.

## Tests Mínimos

Crear o reescribir tests equivalentes a:

```rust
#[test]
fn returns_minimum_number_of_coins() {
    assert_eq!(coin_change(vec![1, 2, 5], 11), 3);
}

#[test]
fn returns_minus_one_when_amount_cannot_be_formed() {
    assert_eq!(coin_change(vec![2], 3), -1);
}

#[test]
fn returns_zero_for_zero_amount() {
    assert_eq!(coin_change(vec![1, 2, 5], 0), 0);
}
```

## Señales de Buena Solución

- Complejidad temporal: O(amount * coins).
- Complejidad espacial: O(amount).
- Usa centinela claro para estados inalcanzables.
- No confunde este problema con knapsack 0/1.

## Retro Después del Simulacro

Completar:

| Dimensión | Puntaje 1-5 | Evidencia |
| --- | --- | --- |
| Clarificación |  |  |
| Enfoque |  |  |
| Implementación |  |  |
| Pruebas |  |  |
| Comunicación |  |  |

Notas:

- Qué salió bien:
- Qué falló:
- Error que se repite:
- Acción de repaso:
- Fecha para repetir:
