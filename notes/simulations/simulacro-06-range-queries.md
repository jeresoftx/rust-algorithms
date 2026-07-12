# Simulacro 06: Consultas por Rangos

## Objetivo

Practicar selección entre prefix sum, Difference Array, Fenwick Tree, Segment Tree e intervalos ordenados.

Duración total: 60 minutos.

## Reglas

- No consultar `src/patterns/range_queries.rs` antes de terminar.
- Antes de codificar, justificar la estructura elegida.
- Escribir pruebas para índices borde.
- Al terminar, registrar retro en `notes/review-queue.md`.

## Enunciado

Diseñar una estructura que soporte:

- construir desde un arreglo de enteros,
- actualizar un índice,
- consultar la suma de un rango inclusivo.

## Ejemplos

```text
nums = [1, 3, 5]
sum_range(0, 2) = 9
update(1, 2)
sum_range(0, 2) = 8
```

## Preguntas de Clarificación

- ¿Las consultas son inclusivas?
- ¿Qué se debe devolver si el rango es inválido?
- ¿Habrá muchas actualizaciones o solo muchas consultas?
- ¿Los valores pueden ser negativos?

## Plan Esperado

1. Descartar prefix sum simple si hay actualizaciones frecuentes.
2. Elegir Fenwick Tree para sumas con actualización puntual.
3. Guardar el arreglo original para calcular `delta` al actualizar.
4. Validar índices antes de consultar o modificar.

## Tests Mínimos

```rust
#[test]
fn updates_values_and_queries_ranges() {
    let mut query = RangeSumQuery::new(vec![1, 3, 5]);
    assert_eq!(query.sum_range(0, 2), Some(9));
    assert!(query.update(1, 2));
    assert_eq!(query.sum_range(0, 2), Some(8));
}

#[test]
fn rejects_invalid_ranges() {
    let query = RangeSumQuery::new(vec![1, 2, 3]);
    assert_eq!(query.sum_range(2, 1), None);
}
```

## Retro Después del Simulacro

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
