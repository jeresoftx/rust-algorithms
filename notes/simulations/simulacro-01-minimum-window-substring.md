# Simulacro 01: Minimum Window Substring

## Objetivo

Practicar hashing + sliding window bajo límite de tiempo, explicando invariantes y casos borde antes de codificar.

Duración total: 45 minutos.

## Reglas

- No consultar `src/patterns/sliding_window.rs` antes de terminar.
- Escribir primero los casos de prueba.
- Explicar el enfoque antes de codificar.
- Al terminar, registrar retro en `notes/review-queue.md`.

## Enunciado

Dadas dos cadenas `source` y `target`, devolver la subcadena más corta de `source` que contenga todos los caracteres de `target`, respetando multiplicidad.

Si no existe una ventana válida, devolver una cadena vacía.

## Ejemplos

```text
source = "ADOBECODEBANC"
target = "ABC"
resultado = "BANC"
```

```text
source = "a"
target = "a"
resultado = "a"
```

```text
source = "a"
target = "aa"
resultado = ""
```

## Preguntas de Clarificación

Antes de codificar, responder:

- ¿Importa la multiplicidad de caracteres?
- ¿Qué debe pasar si `target` está vacío?
- ¿La comparación distingue mayúsculas y minúsculas?
- ¿Qué tipo de caracteres se espera manejar?

## Plan Esperado

1. Contar caracteres requeridos en `target`.
2. Expandir `right` para incluir caracteres de `source`.
3. Mantener cuántos caracteres requeridos siguen faltando.
4. Cuando la ventana sea válida, contraer `left` mientras siga siendo válida.
5. Guardar la mejor ventana vista.

## Invariantes

- `missing == 0` significa que la ventana actual cubre `target`.
- El mapa de conteos indica cuántas apariciones faltan o sobran dentro de la ventana.
- Solo se puede mover `left` con cuidado: si al sacar un carácter requerido vuelve a faltar, la ventana deja de ser válida.

## Tests Mínimos

Crear o reescribir tests equivalentes a:

```rust
#[test]
fn returns_shortest_substring_covering_target_counts() {
    assert_eq!(min_window("ADOBECODEBANC", "ABC"), "BANC");
}

#[test]
fn returns_empty_when_target_cannot_be_covered() {
    assert_eq!(min_window("a", "aa"), "");
}

#[test]
fn handles_single_character_match() {
    assert_eq!(min_window("a", "a"), "a");
}
```

## Señales de Buena Solución

- Complejidad temporal: O(n + m).
- Complejidad espacial: O(k), donde `k` es el número de caracteres distintos relevantes.
- No reconstruye substrings en cada paso.
- No pierde multiplicidad cuando `target` contiene caracteres repetidos.

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
