# Semana 26: Geometría y Temas Selectivos

## Objetivo

Practicar geometría computacional con una base pequeña, explicable y reusable para problemas de puntos, líneas y barridos.

## Estado del Avance Actual

- Problemas implementados acumulados: 190.
- Tests automatizados acumulados: 449.
- Módulo agregado: `src/patterns/geometry.rs`.
- Módulo agregado: `src/patterns/matrices.rs`.
- Tests agregados: `tests/geometry_test.rs`.
- Tests agregados: `tests/matrices_test.rs`.

## Problemas Implementados

| Problema | Patrón | API | Estado |
| --- | --- | --- | --- |
| Producto cruz | Geometría básica | `cross_product` | implementado |
| Orientación de puntos | Producto cruz | `orientation` | implementado |
| Convex Hull / Erect the Fence | Monotonic chain | `convex_hull` | implementado |
| K Closest Points to Origin | Ordenamiento por distancia | `k_closest_points` | implementado |
| Max Points on a Line | Pendiente normalizada | `max_points_on_a_line` | implementado |
| The Skyline Problem | Barrido de línea + multiset | `get_skyline` | implementado |
| Rotate Image | Transponer + reversa | `rotate_image` | implementado |
| Spiral Matrix | Límites decrecientes | `spiral_order` | implementado |
| Set Matrix Zeroes | Marcadores en primera fila/columna | `set_matrix_zeroes` | implementado |
| Search a 2D Matrix II | Búsqueda escalonada | `search_matrix_ii` | implementado |
| Valid Sudoku | Máscaras por fila, columna y caja | `is_valid_sudoku` | implementado |
| Game of Life | Estados transitorios in-place | `game_of_life` | implementado |
| Maximal Square | DP 2D comprimida | `maximal_square` | implementado |
| Insert Delete GetRandom O(1) | Vector + mapa de índices | `RandomizedSet` | implementado |

## Ideas Clave

- El producto cruz indica si tres puntos giran en sentido horario, antihorario o son colineales.
- `convex_hull` usa monotonic chain: ordena puntos, construye borde inferior y borde superior.
- Para conservar puntos colineales del borde, se eliminan solo los giros horarios estrictos.
- El resultado de `convex_hull` se devuelve sin duplicados para que sea estable y fácil de comparar.
- Para puntos cercanos al origen basta comparar distancia cuadrada; no hace falta calcular raíces.
- En empates de distancia, ordenar por coordenadas vuelve el resultado determinista.
- Max Points on a Line usa pendientes reducidas por `gcd` para evitar errores de precisión.
- Las líneas verticales se normalizan como `(1, 0)` y las horizontales como `(0, 1)`.
- Skyline convierte edificios en eventos de inicio y fin, mantiene alturas activas y emite puntos solo cuando cambia la altura máxima.
- Rotate Image combina transposición con reversa de filas para lograr rotación horaria in-place.
- Spiral Matrix consume bordes y encoge límites después de cada vuelta.
- Set Matrix Zeroes usa la primera fila y columna como marcadores sin memoria adicional proporcional.
- Valid Sudoku usa bits para detectar repetidos en filas, columnas y cajas de 3x3.
- Game of Life usa estados transitorios para aplicar cambios simultáneos sin copiar el tablero.
- Maximal Square calcula el lado máximo usando el mínimo entre izquierda, arriba y diagonal.
- RandomizedSet combina vector e índice para borrar en O(1) promedio con swap contra el último valor.

## Complejidad

| Función | Tiempo | Espacio |
| --- | ---: | ---: |
| `cross_product` | `O(1)` | `O(1)` |
| `orientation` | `O(1)` | `O(1)` |
| `convex_hull` | `O(n log n)` | `O(n)` |
| `k_closest_points` | `O(n log n)` | `O(n)` |
| `max_points_on_a_line` | `O(n^2)` | `O(n)` |
| `get_skyline` | `O(n log n)` | `O(n)` |
| `rotate_image` | `O(n^2)` | `O(1)` |
| `spiral_order` | `O(m * n)` | `O(1)` extra |
| `set_matrix_zeroes` | `O(m * n)` | `O(1)` |
| `search_matrix_ii` | `O(m + n)` | `O(1)` |
| `is_valid_sudoku` | `O(1)` | `O(1)` |
| `game_of_life` | `O(m * n)` | `O(1)` |
| `maximal_square` | `O(m * n)` | `O(n)` |
| `RandomizedSet` | `O(1)` promedio | `O(n)` |

## Criterio de Cierre

- Producto cruz clasifica giros positivos, negativos y colineales.
- Orientación devuelve `Clockwise`, `CounterClockwise` o `Collinear`.
- Convex Hull conserva puntos colineales en el borde.
- Convex Hull maneja duplicados, entrada vacía y entradas pequeñas.
- K Closest Points maneja empates, `k` mayor al tamaño y entrada vacía.
- Max Points on a Line maneja duplicados, líneas verticales, líneas horizontales y entradas pequeñas.
- Skyline maneja edificios traslapados, edificios contiguos con misma altura y entradas inválidas.
- Rotate Image rechaza matrices no cuadradas.
- Spiral Matrix maneja matrices cuadradas y una sola fila.
- Set Matrix Zeroes cubre ceros centrales y matriz vacía.
- Valid Sudoku acepta tableros parciales válidos y rechaza duplicados.
- Game of Life actualiza simultáneamente y rechaza tablero vacío.
- Maximal Square devuelve área máxima y cero cuando no hay unos.
- Search Matrix II encuentra y rechaza objetivos en matriz ordenada.
- RandomizedSet cubre inserción, borrado, duplicados y conjunto vacío.
- README y wiki apuntan al bloque.
- `cargo fmt` y `cargo test` pasan.

## Siguiente Paso

El hito de 190 problemas queda cerrado. El siguiente paso del repo puede ser mantenimiento, revisión de explicaciones o una ruta opcional de largo plazo.
