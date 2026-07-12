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
| 2026-07-12 | Listas enlazadas | Ownership | Manipular `next` directamente puede mover valores inesperadamente | Usar `take()` para transferir propiedad del siguiente nodo | 2026-07-19 |
| 2026-07-12 | Validate BST | Árboles | Comparar solo contra el padre no detecta descendientes inválidos | Pasar límites inferior/superior heredados por ancestros | 2026-07-20 |
| 2026-07-12 | Diameter of Binary Tree | Árboles | Confundir altura en nodos con diámetro en aristas | Devolver altura y actualizar diámetro con `left + right` | 2026-07-20 |
| 2026-07-12 | Construct Binary Tree | Árboles | Buscar raíz en inorder en cada llamada vuelve la solución O(n^2) | Crear `HashMap<valor, índice>` antes de recursar | 2026-07-21 |
| 2026-07-12 | Pacific Atlantic Water Flow | Grafos | Buscar desde cada celda hacia ambos océanos repite demasiados recorridos | Invertir la búsqueda desde los bordes y moverse a alturas mayores o iguales | 2026-07-22 |
| 2026-07-12 | Rotting Oranges | Grafos | Correr BFS separado por cada naranja podrida rompe el tiempo mínimo global | Usar BFS multisource con todas las fuentes iniciales en la misma cola | 2026-07-22 |
| 2026-07-12 | Course Schedule | Grafos | Detectar ciclo solo con conteo de aristas no prueba prerequisitos pendientes | Usar indegrees y verificar si el orden procesó todos los cursos | 2026-07-23 |
| 2026-07-12 | Redundant Connection | Grafos | Agregar la arista antes de revisar ciclo oculta la conexión redundante | Revisar `union`; si devuelve falso, esa arista cierra el ciclo | 2026-07-23 |
| 2026-07-12 | MedianFinder | Montículos | Insertar en el montículo correcto no basta si las mitades quedan desbalanceadas | Rebalancear después de cada inserción y consultar la mediana desde las raíces | 2026-07-24 |
| 2026-07-12 | Meeting Rooms II | Intervalos | Contar solapamientos sin liberar salas ya terminadas infla el resultado | Usar min heap de finales y liberar todo final `<= start` antes de asignar sala | 2026-07-24 |
| 2026-07-12 | Task Scheduler | Greedy | Calcular huecos solo para una tarea máxima falla cuando hay empate de frecuencia | Sumar cuántas tareas comparten `max_count` en el marco mínimo | 2026-07-25 |
| 2026-07-12 | Coin Change | Programación dinámica | No usar centinela hace difícil distinguir montos inalcanzables | Inicializar con `amount + 1` y convertir a `-1` al final si no mejora | 2026-07-26 |
| 2026-07-12 | Decode Ways | Programación dinámica | Tratar `0` como dígito válido produce decodificaciones falsas | Permitir `0` solo cuando forma un número de 10 a 26 con el dígito anterior | 2026-07-26 |
| 2026-07-12 | Partition Equal Subset Sum | Knapsack | Recorrer el target hacia adelante reutiliza el mismo número más de una vez | Recorrer en reversa para modelar selección 0/1 | 2026-07-27 |
| 2026-07-12 | Longest Common Subsequence | Programación dinámica | Sobrescribir una fila antes de leerla rompe la transición | Separar fila previa y fila actual, o controlar cuidadosamente los temporales | 2026-07-27 |
