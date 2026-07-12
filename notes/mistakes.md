# Bitácora de Errores

| Fecha | Problema | Patrón | Error | Corrección | Repetir |
| --- | --- | --- | --- | --- | --- |
| 2026-07-11 | Integration tests | Rust testing | Puse el test en `tests/patterns/`, Cargo no lo descubrió | Usar archivos directos como `tests/hashing_test.rs` o declarar módulos manualmente | 2026-07-13 |
| 2026-07-11 | Group Anagrams | Hashing | El orden de `HashMap` no es estable para asserts | Ordenar grupos y resultado en el test antes de comparar | 2026-07-14 |
| 2026-07-12 | Subarray Sum Equals K | Prefix sum | Es fácil olvidar el prefijo inicial `0` | Inicializar el mapa con `{0: 1}` para contar subarreglos que empiezan en índice `0` | 2026-07-14 |
