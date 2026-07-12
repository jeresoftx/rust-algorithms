# Resultado Simulacro 05: Cadenas Avanzadas

Fecha: 2026-07-12.

## Problema

Longest Duplicate Substring.

## Resultado

Aprobado con mejora pendiente.

Puntaje total: 21/25.

| Dimensión | Puntaje 1-5 | Evidencia |
| --- | ---: | --- |
| Clarificación | 4 | Se asumió entrada ASCII y se cubrieron entradas pequeñas. |
| Enfoque | 4 | Búsqueda binaria sobre longitud separa el problema de decisión del óptimo. |
| Implementación | 4 | La solución funciona, pero la verificación por ventanas no es la opción más eficiente. |
| Pruebas | 5 | Cubre duplicado largo, sin duplicado, repetición de un carácter y entradas pequeñas. |
| Comunicación | 4 | Se documentó el trade-off y la alternativa de repaso. |

## Complejidad

- Tiempo: `O(n^2 log n)` en peor caso por hashing/comparación de ventanas de longitud variable.
- Espacio: `O(n)`.

## Invariantes

- Si existe un duplicado de longitud `L`, también existe alguno de longitud menor.
- La búsqueda binaria conserva la mejor longitud encontrada.
- La verificación por longitud revisa ventanas del mismo tamaño y detecta la primera repetida.

## Retro

- Qué salió bien: el diseño por decisión + búsqueda binaria quedó claro.
- Qué falló: faltó implementar una verificación más fuerte con rolling hash o suffix array.
- Error que se repite: no convertir de inmediato el objetivo de complejidad en restricción dura.
- Acción de repaso: repetir con rolling hash doble o suffix array.
- Fecha para repetir: 2026-07-20.
