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

## DFS en Matriz

```rust
fn dfs(grid: &[Vec<i32>], visited: &mut [Vec<bool>], row: usize, col: usize) {
    if visited[row][col] || grid[row][col] == 0 {
        return;
    }

    visited[row][col] = true;

    for (next_row, next_col) in neighbors(row, col, grid.len(), grid[0].len()) {
        dfs(grid, visited, next_row, next_col);
    }
}
```

Útil para:

- Islas.
- Componentes conectados en una matriz.
- Áreas máximas.

## BFS Multisource

```rust
let mut queue = VecDeque::new();

for row in 0..rows {
    for col in 0..cols {
        if is_source(row, col) {
            queue.push_back((row, col, 0));
        }
    }
}

while let Some((row, col, distance)) = queue.pop_front() {
    for (next_row, next_col) in neighbors(row, col, rows, cols) {
        if can_visit(next_row, next_col) {
            queue.push_back((next_row, next_col, distance + 1));
        }
    }
}
```

Útil para:

- Distancias mínimas en matriz sin pesos.
- Propagación simultánea.
- Problemas con varias fuentes iniciales.

## Ordenamiento Topológico

```rust
let mut queue = VecDeque::new();

for (node, &indegree) in indegrees.iter().enumerate() {
    if indegree == 0 {
        queue.push_back(node);
    }
}

while let Some(node) = queue.pop_front() {
    order.push(node);

    for &neighbor in &graph[node] {
        indegrees[neighbor] -= 1;
        if indegrees[neighbor] == 0 {
            queue.push_back(neighbor);
        }
    }
}
```

Útil para:

- Prerequisitos.
- Detección de ciclos en grafos dirigidos.
- Ordenar tareas con dependencias.

## Union Find

```rust
fn find(parent: &mut Vec<usize>, value: usize) -> usize {
    if parent[value] != value {
        parent[value] = find(parent, parent[value]);
    }

    parent[value]
}
```

Útil para:

- Componentes conectados dinámicos.
- Detección de ciclos en grafos no dirigidos.
- Agrupar elementos equivalentes.
