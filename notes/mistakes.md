# Mistakes Log

| Fecha | Problema | Patron | Error | Correccion | Repetir |
| --- | --- | --- | --- | --- | --- |
| 2026-07-11 | Integration tests | Rust testing | Puse el test en `tests/patterns/`, Cargo no lo descubrio | Usar archivos directos como `tests/hashing_test.rs` o declarar modulos manualmente | 2026-07-13 |
| 2026-07-11 | Group Anagrams | Hashing | El orden de `HashMap` no es estable para asserts | Ordenar grupos y resultado en el test antes de comparar | 2026-07-14 |
