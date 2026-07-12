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

## Montículo Mínimo con Reverse

```rust
let mut heap = BinaryHeap::new();

for value in values {
    heap.push(Reverse(value));

    if heap.len() > k {
        heap.pop();
    }
}
```

Útil para:

- Kth largest.
- Mantener solo los mejores `k` candidatos.
- Modelar prioridades mínimas con `BinaryHeap`.

## Dos Montículos para Mediana

```rust
let mut lower = BinaryHeap::new();
let mut upper = BinaryHeap::new();

lower.push(value);

if lower.len() > upper.len() + 1 {
    upper.push(Reverse(lower.pop().unwrap()));
}
```

Útil para:

- Median stream.
- Mantener dos mitades balanceadas.
- Consultar mediana en O(1).

## Fusionar Intervalos

```rust
intervals.sort_unstable_by_key(|&(start, end)| (start, end));

for (start, end) in intervals {
    match merged.last_mut() {
        Some((_, previous_end)) if start <= *previous_end => {
            *previous_end = (*previous_end).max(end);
        }
        _ => merged.push((start, end)),
    }
}
```

Útil para:

- Merge intervals.
- Insert interval.
- Normalizar rangos antes de analizarlos.

## Greedy por Fin Más Temprano

```rust
intervals.sort_unstable_by_key(|&(start, end)| (end, start));

let mut last_end = intervals[0].1;

for (start, end) in intervals.into_iter().skip(1) {
    if start >= last_end {
        last_end = end;
    }
}
```

Útil para:

- Intervalos no solapados.
- Selección máxima de eventos compatibles.
- Minimizar remociones.

## DP 1D con Compresión

```rust
let mut two_back = base_two_back;
let mut one_back = base_one_back;

for item in items {
    let current = transition(two_back, one_back, item);
    two_back = one_back;
    one_back = current;
}
```

Útil para:

- Escaleras.
- Robo de casas.
- Decodificación.

## DP de Minimización

```rust
let unreachable = target + 1;
let mut dp = vec![unreachable; target + 1];
dp[0] = 0;

for amount in 1..=target {
    for coin in &coins {
        if *coin <= amount {
            dp[amount] = dp[amount].min(dp[amount - *coin] + 1);
        }
    }
}
```

Útil para:

- Coin Change.
- Costos mínimos.
- Caminos mínimos sin pesos negativos.

## Knapsack 0/1

```rust
let mut dp = vec![false; target + 1];
dp[0] = true;

for value in values {
    for current in (value..=target).rev() {
        dp[current] = dp[current] || dp[current - value];
    }
}
```

Útil para:

- Partition Equal Subset Sum.
- Subset sum.
- Decisiones donde cada elemento se usa una vez.

## DP 2D con Fila Comprimida

```rust
let mut previous = vec![0; cols + 1];

for row in rows {
    let mut current = vec![0; cols + 1];

    for col in 0..cols {
        current[col + 1] = transition(&previous, &current, row, col);
    }

    previous = current;
}
```

Útil para:

- Longest Common Subsequence.
- Problemas de grid.
- Comparación de dos secuencias.
