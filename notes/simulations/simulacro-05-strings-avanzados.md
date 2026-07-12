# Simulacro 05: Cadenas Avanzadas

## Objetivo

Practicar diseño de búsqueda de patrones y razonamiento sobre colisiones, prefijos y complejidad.

Duración total: 60 minutos.

## Reglas

- No consultar `src/patterns/string_algorithms.rs` antes de terminar.
- Explicar una solución fuerza bruta y por qué no escala.
- Elegir entre KMP, rolling hash, suffix array o búsqueda binaria sobre longitud.
- Al terminar, registrar retro en `notes/review-queue.md`.

## Enunciado

Dada una cadena `s`, devolver cualquier subcadena duplicada de longitud máxima. Si no existe, devolver cadena vacía.

## Ejemplos

```text
s = "banana"
resultado = "ana"
```

```text
s = "abcd"
resultado = ""
```

## Preguntas de Clarificación

- ¿Se puede devolver cualquier respuesta si hay empate?
- ¿La cadena contiene solo ASCII en minúsculas?
- ¿Qué tamaño máximo puede tener `s`?
- ¿Se acepta una solución `O(n^2 log n)` para práctica o buscamos `O(n log n)`?

## Plan Esperado

1. Proponer fuerza bruta y descartar por costo.
2. Usar búsqueda binaria sobre la longitud de la subcadena.
3. Verificar duplicados de una longitud con hashing o estructura ordenada.
4. Guardar una respuesta válida cuando exista duplicado.
5. Cuidar colisiones si se usa rolling hash.

## Tests Mínimos

```rust
#[test]
fn returns_longest_duplicate_substring() {
    assert_eq!(longest_duplicate_substring("banana"), "ana");
}

#[test]
fn returns_empty_when_no_duplicate_exists() {
    assert_eq!(longest_duplicate_substring("abcd"), "");
}
```

## Retro Después del Simulacro

| Dimensión | Puntaje 1-5 | Evidencia |
| --- | --- | --- |
| Clarificación | 4 | Se asumió entrada ASCII para trabajar por bytes y se cubrieron cadenas vacías y de un carácter. |
| Enfoque | 4 | Se eligió búsqueda binaria sobre longitud con verificación por ventanas repetidas. |
| Implementación | 4 | La función es correcta para práctica, pero no usa rolling hash ni suffix array. |
| Pruebas | 5 | Se cubrieron `banana`, ausencia de duplicados, repetición completa y entradas pequeñas. |
| Comunicación | 4 | Quedó explícito el trade-off de complejidad y la mejora recomendada. |

Notas:

- Qué salió bien: la búsqueda binaria sobre longitud separó bien decisión y optimización.
- Qué falló: la verificación con `HashSet` de ventanas sigue pagando costo por comparar contenido.
- Error que se repite: aceptar una solución clara sin anotar de inmediato la alternativa óptima.
- Acción de repaso: repetir con rolling hash doble o suffix array.
- Fecha para repetir: 2026-07-20.
