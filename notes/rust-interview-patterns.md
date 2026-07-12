# Patrones de Rust para Entrevistas

## Conteo con HashMap

```rust
let mut counts = std::collections::HashMap::new();

for value in values {
    *counts.entry(value).or_insert(0) += 1;
}
```

Útil para:

- Frecuencias de caracteres.
- Frecuencias de valores.
- Agrupación por llaves normalizadas.

## Pertenencia con HashSet

```rust
let mut seen = std::collections::HashSet::new();

for value in values {
    if !seen.insert(value) {
        return true;
    }
}
```

Útil para:

- Duplicados.
- Consultas rápidas de pertenencia.
- Secuencias consecutivas.

## Retorno con Option

```rust
pub fn search(values: Vec<i32>, target: i32) -> Option<usize> {
    for (index, value) in values.into_iter().enumerate() {
        if value == target {
            return Some(index);
        }
    }

    None
}
```

Útil cuando:

- Una respuesta puede no existir.
- Devolver valores centinela como `-1` haría menos idiomática la API.

## Tests Estables al Usar HashMap

Cuando el algoritmo devuelve grupos desde un `HashMap`, el orden no está garantizado. Ordena dentro del test antes de comparar.

```rust
for group in &mut result {
    group.sort();
}
result.sort();
```
