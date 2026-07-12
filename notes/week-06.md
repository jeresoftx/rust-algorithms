# Semana 06: Búsqueda Binaria

## Objetivo

Usar búsqueda binaria no solo para encontrar índices exactos, sino también para encontrar límites, pivotes y respuestas mínimas que cumplen una condición.

## Problemas Completados

| Problema | Patrón | Función | Tests |
| --- | --- | --- | --- |
| Binary Search | Búsqueda exacta | `binary_search` | 3 |
| Search Insert Position | Lower bound | `search_insert` | 3 |
| Search in Rotated Sorted Array | Mitad ordenada | `search_rotated` | 3 |
| Find Minimum in Rotated Sorted Array | Búsqueda de pivote | `find_min_rotated` | 3 |
| Koko Eating Bananas | Búsqueda sobre respuesta | `min_eating_speed` | 3 |
| Capacity To Ship Packages Within D Days | Búsqueda sobre respuesta | `ship_within_days` | 3 |

## Patrones Trabajados

### Búsqueda Exacta

Se usa cuando el arreglo está ordenado y queremos saber si un valor existe.

Invariante:

- El objetivo, si existe, está dentro del rango actual.
- Si `nums[mid] < target`, descartamos la mitad izquierda.
- Si `nums[mid] > target`, descartamos la mitad derecha.

### Lower Bound

Busca el primer índice cuyo valor es mayor o igual al objetivo.

Útil para:

- Posición de inserción.
- Primer valor que cumple una condición.
- Evitar ramas especiales cuando el valor no existe.

### Arreglo Rotado

En un arreglo ordenado y rotado, al menos una mitad alrededor de `mid` está ordenada.

Invariante:

- Detectar qué mitad está ordenada.
- Decidir si el objetivo pertenece a esa mitad.
- Descartar la mitad que no puede contener el objetivo.

### Búsqueda Sobre Respuesta

Se usa cuando la respuesta está en un rango numérico y existe una condición monótona.

Ejemplos:

- Si una velocidad permite terminar a tiempo, toda velocidad mayor también.
- Si una capacidad permite enviar a tiempo, toda capacidad mayor también.

Pasos:

1. Definir el rango de respuestas posibles.
2. Escribir una función `works(answer)`.
3. Usar búsqueda binaria para encontrar la mínima respuesta que funciona.

## Errores Comunes

- Usar `right = len - 1` en arreglos vacíos.
- No avanzar `left` con `mid + 1`.
- Mezclar rangos cerrados y semiabiertos.
- No probar objetivos ausentes.
- Definir mal la condición monótona.
- No hacer división redondeando hacia arriba en problemas de capacidad o velocidad.

## Verificación

```bash
cargo test
```

Resultado esperado al cerrar el bloque:

```text
67 passed; 0 failed
```

## Siguiente Bloque

El siguiente bloque recomendado es:

- Recursión.
- Backtracking.
- Linked lists.

Primeros problemas sugeridos:

- Subsets.
- Permutations.
- Combination Sum.
- Generate Parentheses.
- Word Search.
- Reverse Linked List.
