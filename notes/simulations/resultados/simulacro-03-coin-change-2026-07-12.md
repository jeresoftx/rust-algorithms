# Resultado Simulacro 03: Coin Change

Fecha: 2026-07-12.

## Problema

Coin Change.

## Resultado

Aprobado.

Puntaje total: 24/25.

| Dimensión | Puntaje 1-5 | Evidencia |
| --- | ---: | --- |
| Clarificación | 5 | Se validó que las monedas son ilimitadas, `amount = 0` devuelve `0` y la salida es cantidad mínima. |
| Enfoque | 5 | Se usó DP de minimización con centinela para estados inalcanzables. |
| Implementación | 5 | La implementación ignora monedas no positivas y mantiene `dp[0] = 0`. |
| Pruebas | 5 | `cargo test coin_change` validó mínimo, monto imposible y monto cero. |
| Comunicación | 4 | La transición está clara; conviene repetirla sin consultar notas. |

## Complejidad

- Tiempo: `O(amount * coins)`.
- Espacio: `O(amount)`.

## Invariantes

- `dp[x]` representa la menor cantidad de monedas para formar `x`.
- `dp[0]` siempre vale `0`.
- El centinela distingue montos todavía inalcanzables.
- La transición solo mejora `dp[x]` cuando `x - coin` ya tiene una respuesta válida.

## Retro

- Qué salió bien: el estado de DP y la transición quedaron definidos antes de revisar el resultado.
- Qué falló: faltan pruebas explícitas para monedas repetidas y monedas no positivas.
- Error que se repite: documentar el centinela, pero no siempre probar sus bordes.
- Acción de repaso: repetir Coin Change con un caso de monedas no positivas y uno de monto grande.
- Fecha para repetir: 2026-07-26.
