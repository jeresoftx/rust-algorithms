# Semana 16: Simulacros y Repaso

## Objetivo

Convertir el estudio acumulado en práctica de entrevista: resolver bajo tiempo, explicar decisiones, probar casos borde y revisar errores con método.

## Estado del Repo al Iniciar el Bloque

- Problemas implementados: 69.
- Tests automatizados: 162.
- Patrones cubiertos: hashing, two pointers, sliding window, stack, búsqueda binaria, backtracking, listas enlazadas, árboles, grafos, montículos, intervalos, greedy y programación dinámica.
- Comando base de verificación: `cargo test`.

## Rutina de la Semana

| Día | Trabajo | Resultado esperado |
| --- | --- | --- |
| 1 | Simulacro 1: arrays/hash + two pointers | Resolver en 45 minutos y registrar retro |
| 2 | Rehacer 3 errores de `notes/mistakes.md` | Reducir bugs repetidos |
| 3 | Simulacro 2: árboles/grafos | Practicar explicación de invariantes |
| 4 | Repaso dirigido: programación dinámica | Definir estado y transición antes del código |
| 5 | Simulacro 3: mezcla aleatoria | Medir consistencia bajo tiempo |
| 6 | Historias conductuales y comunicación | Preparar ejemplos breves con estructura STAR |
| 7 | Revisión final | Elegir patrones débiles para repetir |

## Formato de Simulacro

Duración total: 45 minutos.

1. Minutos 0 a 5: aclarar entrada, salida, restricciones y casos borde.
2. Minutos 5 a 10: proponer brute force, mejorar enfoque y explicar complejidad.
3. Minutos 10 a 35: implementar solución con tests.
4. Minutos 35 a 40: probar casos normales y borde.
5. Minutos 40 a 45: explicar mejoras, trade-offs y riesgos.

## Simulacros Sugeridos

### Simulacro 1: Hashing y Ventanas

Problemas candidatos:

- Two Sum.
- Group Anagrams.
- Minimum Window Substring.
- Longest Substring Without Repeating Characters.

Meta:

- Mantener invariantes de conteo sin perder edge cases.

Artefacto listo:

- `notes/simulations/simulacro-01-minimum-window-substring.md`

### Simulacro 2: Árboles y Grafos

Problemas candidatos:

- Validate Binary Search Tree.
- Binary Tree Level Order Traversal.
- Number of Islands.
- Course Schedule.

Meta:

- Explicar estado visitado, límites heredados y orden de recorrido.

### Simulacro 3: Programación Dinámica y Greedy

Problemas candidatos:

- Coin Change.
- Decode Ways.
- Partition Equal Subset Sum.
- Task Scheduler.

Meta:

- Definir `dp[i]`, transición, casos base y orden de cómputo antes de escribir código.

## Rúbrica Rápida

Cada simulacro se califica de 1 a 5:

| Dimensión | 1 | 3 | 5 |
| --- | --- | --- | --- |
| Clarificación | Empieza a codificar sin preguntas | Confirma ejemplos básicos | Define restricciones y casos borde |
| Enfoque | No justifica patrón | Explica idea general | Compara alternativas y complejidad |
| Implementación | Bugs estructurales | Solución funcional con ajustes | Código claro, idiomático y probado |
| Pruebas | Solo caso feliz | Casos normales y un borde | Casos borde, negativos y regresión |
| Comunicación | Silencios largos | Explica partes clave | Narra decisiones con orden |

## Criterio de Cierre

El bloque se considera listo cuando:

- Se completan 3 simulacros de 45 minutos.
- Cada simulacro deja una entrada en `notes/review-queue.md`.
- Se repiten al menos 5 errores de `notes/mistakes.md`.
- `cargo test` sigue pasando.
- La wiki queda actualizada como bitácora pública del avance.

## Siguiente Paso

Después de esta semana, el trabajo ya no es sumar módulos al azar. El foco debe ser:

- Repetición espaciada.
- Simulacros cronometrados.
- Explicación verbal.
- Revisión de errores recurrentes.
