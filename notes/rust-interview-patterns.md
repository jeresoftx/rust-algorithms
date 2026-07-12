# Rust Interview Patterns

## HashMap Count

```rust
let mut counts = std::collections::HashMap::new();

for value in values {
    *counts.entry(value).or_insert(0) += 1;
}
```

Use for:

- Character frequencies.
- Value frequencies.
- Grouping by normalized keys.

## HashSet Membership

```rust
let mut seen = std::collections::HashSet::new();

for value in values {
    if !seen.insert(value) {
        return true;
    }
}
```

Use for:

- Duplicates.
- Fast membership.
- Consecutive sequence checks.

## Option Return

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

Use when:

- A result may not exist.
- Returning sentinel values like `-1` would make the API less idiomatic.

## Sorting a Result for Stable Tests

When the algorithm returns groups from a `HashMap`, order is not guaranteed. Sort inside the test before asserting.

```rust
for group in &mut result {
    group.sort();
}
result.sort();
```
