# Bitácora de Errores

| Fecha | Problema | Patrón | Error | Corrección | Repetir |
| --- | --- | --- | --- | --- | --- |
| 2026-07-11 | Integration tests | Rust testing | Puse el test en `tests/patterns/`, Cargo no lo descubrió | Usar archivos directos como `tests/hashing_test.rs` o declarar módulos manualmente | 2026-07-13 |
| 2026-07-11 | Group Anagrams | Hashing | El orden de `HashMap` no es estable para asserts | Ordenar grupos y resultado en el test antes de comparar | 2026-07-14 |
| 2026-07-12 | Subarray Sum Equals K | Prefix sum | Es fácil olvidar el prefijo inicial `0` | Inicializar el mapa con `{0: 1}` para contar subarreglos que empiezan en índice `0` | 2026-07-14 |
| 2026-07-12 | 3Sum | Two pointers | Es fácil devolver triplets duplicados | Saltar duplicados del índice fijo y de ambos punteros después de encontrar una solución | 2026-07-15 |
| 2026-07-12 | Minimum Window Substring | Sliding window | Contraer la ventana demasiado pronto rompe los conteos requeridos | Solo contraer mientras `missing == 0` y ajustar conteos al mover `left` | 2026-07-16 |
| 2026-07-12 | Largest Rectangle in Histogram | Stack monotónico | Sin centinela final quedan barras pendientes sin cerrar | Agregar altura `0` al final para vaciar la pila | 2026-07-16 |
| 2026-07-12 | Binary Search | Búsqueda binaria | Mezclar rangos cerrados y semiabiertos produce off-by-one | Elegir una convención por función y actualizar `left`/`right` de forma consistente | 2026-07-17 |
| 2026-07-12 | Koko Eating Bananas | Búsqueda sobre respuesta | Dividir con truncamiento subestima las horas necesarias | Usar división redondeando hacia arriba: `(value + divisor - 1) / divisor` | 2026-07-17 |
| 2026-07-12 | Backtracking | Recursión | Olvidar deshacer una decisión contamina otras ramas | Hacer `push`, recursión y `pop` siempre juntos | 2026-07-18 |
| 2026-07-12 | Word Search | DFS | Reutilizar una celda en el mismo camino produce falsos positivos | Marcar visitado temporalmente y restaurar al volver | 2026-07-18 |
| 2026-07-12 | Linked Lists | Ownership | Manipular `next` directamente puede mover valores inesperadamente | Usar `take()` para transferir propiedad del siguiente nodo | 2026-07-19 |
| 2026-07-12 | Validate BST | Árboles | Comparar solo contra el padre no detecta descendientes inválidos | Pasar límites inferior/superior heredados por ancestros | 2026-07-20 |
| 2026-07-12 | Diameter of Binary Tree | Árboles | Confundir altura en nodos con diámetro en aristas | Devolver altura y actualizar diámetro con `left + right` | 2026-07-20 |
| 2026-07-12 | Construct Binary Tree | Árboles | Buscar raíz en inorder en cada llamada vuelve la solución O(n^2) | Crear `HashMap<valor, índice>` antes de recursar | 2026-07-21 |
