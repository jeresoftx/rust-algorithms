# Guía de Complejidad

Esta guía resume el vocabulario mínimo para explicar costos con claridad durante la resolución de problemas.

## Notación

| Notación | Qué describe | Cómo explicarla |
| --- | --- | --- |
| Big O | Cota superior | El algoritmo no crece peor que esta función, ignorando constantes |
| Θ | Cota ajustada | El algoritmo crece al mismo orden por arriba y por abajo |
| Ω | Cota inferior | El algoritmo necesita al menos este costo en los casos relevantes |

Ejemplo:

- Recorrer un arreglo completo de `n` elementos es `O(n)`, `Θ(n)` y `Ω(n)`.
- Buscar en una tabla hash puede ser `O(1)` promedio, pero `O(n)` en el peor caso si hay demasiadas colisiones.
- Binary search es `O(log n)` porque descarta la mitad del espacio en cada paso.

## Complejidad Amortizada

La complejidad amortizada reparte operaciones costosas sobre muchas operaciones baratas.

Ejemplo clásico:

- Insertar en un `Vec` suele costar `O(1)`.
- Cuando el arreglo interno se llena, crecerlo puede costar `O(n)`.
- Aun así, una secuencia larga de inserciones tiene costo amortizado `O(1)` por inserción.

Frase útil:

> Una operación individual puede ser cara, pero el costo promedio por operación en una secuencia larga se mantiene bajo.

## Costos Comunes

| Patrón | Tiempo típico | Espacio típico |
| --- | ---: | ---: |
| Recorrido lineal | `O(n)` | `O(1)` |
| Hash map de frecuencias | `O(n)` | `O(n)` |
| Sorting | `O(n log n)` | depende del algoritmo |
| Binary search | `O(log n)` | `O(1)` |
| DFS/BFS en grafo | `O(V + E)` | `O(V)` |
| Heap push/pop | `O(log n)` | `O(n)` |
| DP 1D | `O(n)` | `O(n)` o `O(1)` comprimido |
| DP 2D | `O(n * m)` | `O(n * m)` |
| Backtracking | exponencial | profundidad de recursión + resultado |

## Preguntas de Revisión

Antes de cerrar una solución:

1. ¿Qué tamaño controla la entrada principal?
2. ¿Hay ciclos anidados reales o solo ciclos secuenciales?
3. ¿La estructura auxiliar crece con la entrada?
4. ¿La complejidad cambia entre promedio y peor caso?
5. ¿Hay una operación oculta costosa, como `sort`, `clone` o búsqueda lineal?

## Frases para Entrevista

- "La cota temporal es `O(n)` porque cada elemento entra y sale de la ventana a lo sumo una vez."
- "El espacio es `O(k)` porque el mapa guarda como máximo `k` claves distintas."
- "La operación es amortizada `O(1)`: algunos crecimientos cuestan más, pero no ocurren en cada inserción."
- "La cota ajustada es `Θ(n)` porque cualquier solución debe mirar todos los elementos para decidir."
- "El peor caso se degrada cuando la estructura auxiliar pierde la propiedad que asumimos."
