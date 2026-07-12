# Semana 01: Fundamentos de Rust, Hashing, Arrays y Strings

## Objetivo

Construir suficiente fluidez en Rust para resolver ejercicios comunes de entrevista sin atascarse con sintaxis, ownership o colecciones estándar.

## Problemas Completados

| Problema | Patrón | Función | Tests |
| --- | --- | --- | --- |
| Two Sum | Búsqueda de complemento con hash map | `two_sum` | 3 |
| Valid Anagram | Conteo de frecuencias | `valid_anagram` | 3 |
| Contains Duplicate | Pertenencia en set | `contains_duplicate` | 2 |
| Group Anagrams | Llave canónica | `group_anagrams` | 2 |
| Product of Array Except Self | Productos prefijo/sufijo | `product_except_self` | 3 |
| Top K Frequent Elements | Conteo y ordenamiento por frecuencia | `top_k_frequent` | 2 |
| Longest Consecutive Sequence | Inicios de secuencia en set | `longest_consecutive` | 3 |

## Conceptos de Rust Practicados

- Ownership e iteración con `Vec<T>`.
- `HashMap` para conteos, índices y agrupaciones.
- `HashSet` para consultas rápidas de pertenencia.
- `Option` para funciones que pueden no encontrar una respuesta.
- Tests de integración en `tests/hashing_test.rs`.
- `cargo fmt` y `cargo test` como ciclo básico de verificación.

## Patrones de Entrevista

### Búsqueda de Complemento

Útil cuando el problema pide dos valores que se combinan para llegar a un objetivo.

Invariante:

- Antes de procesar el índice `i`, el mapa contiene valores de índices `< i`.
- Si `target - nums[i]` existe en el mapa, encontramos la respuesta.

### Conteo de Frecuencias

Útil al comparar conteos de caracteres, encontrar valores comunes o ordenar por ocurrencia.

Preguntas útiles:

- ¿Necesito conteos exactos o solo presencia?
- ¿Ordenar es más simple que usar hashing?
- ¿El orden de salida importa?

### Llaves Canónicas

Útil cuando los valores pueden normalizarse a una misma representación.

Ejemplo:

- `"eat"`, `"tea"` y `"ate"` se normalizan como `"aet"`.

### Prefijo/Sufijo

Útil cuando cada índice necesita información acumulada desde la izquierda y desde la derecha.

Invariante:

- La primera pasada guarda el producto antes de cada índice.
- La pasada inversa multiplica por el producto después de cada índice.

### Inicios de Secuencia

Útil para encontrar secuencias consecutivas en una colección desordenada.

Invariante:

- Solo empezamos a contar en `x` cuando `x - 1` está ausente.
- Esto evita contar varias veces la misma secuencia.

## Comandos

```bash
cargo fmt
cargo test
```

## Retrospectiva

- El patrón más fuerte esta semana fue la búsqueda directa con hash.
- El hábito más importante fue escribir el test antes de la implementación.
- La siguiente semana debe introducir two pointers y sliding window, porque complementan muy bien los ejercicios de hashing en entrevistas técnicas.
