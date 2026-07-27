# Matriz de cobertura Easy, Medium y Hard

**Estado:** herramienta de planeación; no representa contenido publicado.

**Trazabilidad:** [issue #3](https://github.com/jeresoftx/rust-algorithms/issues/3).

Esta matriz convierte la meta aproximada de 400 problemas en una herramienta de
decisión. No clasifica el valor de una persona ni promete dificultad exacta: la
dificultad cambia entre plataformas, empresas y variantes del mismo problema.

## Concepto

La dificultad sirve para balancear práctica. Un exceso de problemas Easy puede
dar falsa confianza; un exceso de problemas Hard puede enseñar trucos aislados
sin reforzar fundamentos. El equilibrio útil para entrevistas L3-L5 suele poner
el peso en problemas Medium, con Easy para fluidez y Hard para técnicas
transferibles.

## Problema

El repo tiene 190 problemas implementados y probados, pero no conserva una
etiqueta canónica de dificultad por problema. Forzar una clasificación retroactiva
sería ruidoso: algunos ejercicios son Easy por enunciado, Medium por Rust, o
Hard solo si se exige una optimización específica.

La matriz debe ayudar a decidir el futuro sin reescribir la historia del repo.

## Alternativas

1. **Etiquetar retroactivamente los 190 problemas.** Haría el tablero más
   cuantitativo, pero puede crear precisión falsa.
2. **Ignorar dificultad y avanzar solo por familias.** Evita ruido, pero pierde
   una señal útil para entrevistas.
3. **Usar dificultad como objetivo futuro y familias como control de cobertura.**
   Balancea preparación de entrevistas con claridad pedagógica.

Se adopta la tercera alternativa.

## Inventario actual

| Métrica | Estado |
| --- | --- |
| Problemas implementados | 190 |
| Dificultad canónica por problema | No normalizada |
| Familias cubiertas | 19 módulos de patrones |
| Simulacros documentados | 6 acumulados |
| Property testing | Presente donde hay invariantes simples |
| Benchmarks | Presentes donde hay señal de complejidad |

La dificultad de los 190 problemas actuales no se infiere automáticamente. Solo
se normaliza hacia adelante, cuando el issue del bloque declare fuente,
dificultad y motivo de selección.

## Distribución objetivo

| Dificultad | Objetivo total | Rol pedagógico |
| --- | ---: | --- |
| Easy | 120 | Fluidez, calentamiento, bordes simples y velocidad de Rust. |
| Medium | 220 | Núcleo de entrevista: patrones, composición y trade-offs. |
| Hard | 60 | Técnicas selectivas, reducciones y razonamiento profundo. |
| Total | 400 | Horizonte opcional completo. |

Esta distribución no obliga a completar exactamente cada categoría. Funciona
como alarma: si un bloque agrega demasiado de una dificultad sin motivo, el PR
debe justificarlo.

## Matriz por tramo

| Tramo | Easy | Medium | Hard | Enfoque |
| --- | ---: | ---: | ---: | --- |
| 191-220 | 8 | 20 | 2 | Consolidación de patrones frecuentes. |
| 221-240 | 4 | 14 | 2 | Simulacros cronometrados y comunicación. |
| 241-260 | 4 | 12 | 4 | Repeticiones dirigidas y brechas reales. |
| 261-290 | 4 | 20 | 6 | Patrones compuestos. |
| 291-315 | 2 | 15 | 8 | Medium avanzados y hard con alta señal. |
| 316-330 | 2 | 8 | 5 | Cierre de profundización. |
| 331-360 | 0 | 15 | 15 | Hard selectivos y técnicas transferibles. |
| 361-400 | 0 | 26 | 14 | Cierre, repaso, simulacro y revisión final. |

La suma de la matriz futura agrega 210 problemas o repeticiones formalizadas al
hito 190. Si una repetición dirigida cuenta dentro del rango, debe registrar el
aprendizaje nuevo; de lo contrario, no suma al horizonte.

## Brechas por familia

| Familia | Señal actual | Prioridad futura |
| --- | --- | --- |
| Arrays, hashing y strings base | Muy cubierta | Repeticiones rápidas y variantes de entrevista. |
| Two pointers y sliding window | Cubierta | Problemas compuestos con conteo, orden o hashing. |
| Stack, queue, heap e intervalos | Cubierta | Scheduling, streaming y estructuras monotónicas. |
| Búsqueda binaria | Cubierta | Binary search on answer y condiciones monótonas mixtas. |
| Árboles y BST | Cubierta | Construcción, serialización y LCA con variantes. |
| Grafos básicos | Cubierta | Modelado correcto bajo presión. |
| Grafos ponderados | Cubierta avanzada | Problemas de rutas con restricciones múltiples. |
| Programación dinámica | Cubierta | DP con estados no obvios y compresión. |
| Backtracking | Cubierta | Pruning y combinación con tries o bitmasks. |
| Range queries | Cubierta avanzada | Aplicaciones donde Fenwick o segment tree sí se justifican. |
| Cadenas avanzadas | Cubierta inicial | Reforzar KMP, rolling hash y búsqueda multipatrón. |
| Matemáticas, bits y geometría | Cubierta selectiva | Mantener como hard selectivo, no como enciclopedia. |

## Uso en issues

Cada issue de práctica 191+ debe declarar:

- rango del bloque;
- distribución esperada Easy/Medium/Hard;
- familias priorizadas;
- fuente de candidatos;
- problemas descartados si eran candidatos razonables;
- razón pedagógica del bloque.

## Revisión

La matriz se revisa al cierre de cada milestone. Si la práctica real muestra que
otra distribución enseña mejor, se ajusta con PR documentado. La prioridad sigue
siendo calidad, explicación y evidencia, no alcanzar el número exacto.
