# Criterios de selección para problemas 191+

**Estado:** guía operativa; requiere revisión humana antes de considerarse
material publicable.

**Trazabilidad:** [issue #2](https://github.com/jeresoftx/rust-algorithms/issues/2).

Después del hito 190, el repositorio no necesita volumen por inercia. Necesita
problemas que mejoren la capacidad de razonar, explicar, probar y corregir bajo
condiciones parecidas a una entrevista técnica.

## Concepto

Un problema entra al horizonte 400 solo si cumple una función pedagógica clara.
La fuente puede ser Blind 75, NeetCode 150, Grind 169, LeetCode, CLRS, un
simulacro fallado o una brecha detectada en el repo; ninguna lista externa es
autoridad automática.

## Problema

Las listas populares son útiles para descubrir candidatos, pero pueden producir
tres errores:

- repetir problemas equivalentes sin mejorar una habilidad;
- agregar soluciones solo para subir el contador;
- mezclar problemas hard vistosos con bajo valor explicativo.

El horizonte 400 debe evitar esos errores. Cada problema nuevo debe dejar una
mejor explicación, una prueba más sólida, una repetición relevante o una
decisión documentada.

## Alternativas

1. **Seguir una lista externa de principio a fin.** Reduce decisiones, pero
   convierte el repo en copia de una curaduría ajena.
2. **Elegir problemas por gusto o dificultad.** Da libertad, pero puede dejar
   brechas importantes sin cubrir.
3. **Usar listas externas como insumo y decidir con criterios propios.** Permite
   aprovechar señales de entrevista sin perder el enfoque de Jeresoft Academy.

Se adopta la tercera alternativa.

## Criterios de entrada

Un candidato debe cumplir al menos uno de estos criterios:

- entrena un patrón que el repo todavía no domina bajo presión;
- combina dos o más patrones ya conocidos de una forma común en entrevistas;
- corrige una debilidad detectada en simulacros o repeticiones;
- agrega un caso borde que las soluciones actuales no enseñan bien;
- obliga a explicar una alternativa ingenua y una mejora razonable;
- introduce una técnica avanzada con uso práctico, no solo curiosidad teórica;
- mejora la fluidez de Rust en una situación frecuente de entrevista.

Si un problema no cumple ninguno, se descarta aunque aparezca en una lista
popular.

## Señales de descarte

Un problema debe descartarse o posponerse cuando:

- solo repite una solución ya dominada sin variación significativa;
- depende de trucos poco transferibles;
- exige una estructura demasiado especializada para el objetivo del bloque;
- requiere una dependencia externa no justificada;
- produce una implementación opaca difícil de enseñar;
- no permite escribir tests claros;
- empuja a optimizar antes de explicar la solución correcta.

Descartar un problema también es una decisión de ingeniería y debe registrarse
cuando el candidato parecía importante.

## Tipos de trabajo

| Tipo | Cuándo cuenta | Evidencia mínima |
| --- | --- | --- |
| Problema nuevo | Enseña una habilidad o combinación no cubierta. | Código, tests y complejidad. |
| Repetición dirigida | Corrige un fallo previo o una duda real. | Nota de qué cambió y por qué. |
| Simulacro | Mide ejecución bajo tiempo y comunicación. | Bitácora con enfoque, error y revisión. |
| Hard selectivo | Aporta una técnica transferible. | Alternativas, justificación y pruebas. |
| Descarte | Evita ruido o duplicación. | Motivo breve en nota o PR. |

## Fuentes permitidas

- **Blind 75:** buena señal para patrones frecuentes; no cubre todo el horizonte.
- **NeetCode 150:** útil para organizar por familias y dificultad.
- **Grind 169:** útil como inventario de práctica intensiva.
- **LeetCode:** fuente de enunciados y variantes; no define calidad pedagógica.
- **CLRS:** útil para fundamentos, pruebas e invariantes; no se copia como ruta
  de implementación completa.
- **Simulacros propios:** fuente prioritaria cuando revelan fallos reales.

Las fuentes ayudan a proponer candidatos; la decisión final vive en los issues
del repo.

## Checklist por candidato

- [ ] ¿Qué patrón o combinación entrena?
- [ ] ¿Qué brecha del repo atiende?
- [ ] ¿Cuál es la alternativa ingenua?
- [ ] ¿Cuál es la solución que se va a enseñar?
- [ ] ¿Qué casos borde deben probarse?
- [ ] ¿La solución puede explicarse en español claro?
- [ ] ¿El problema es nuevo, repetición, simulacro, hard selectivo o descarte?
- [ ] ¿La dificultad declarada aporta algo al plan?

## Definición de listo para un bloque

Un bloque de problemas 191+ puede empezar cuando:

- existe issue asignado a `jeresoftx`, con milestone, labels y Project;
- el issue declara rango, enfoque y criterios de selección;
- los candidatos iniciales están agrupados por tipo de trabajo;
- no se mezclan problemas nuevos con revisión final del bloque;
- el PR esperado puede cerrar exactamente un issue.

## Relación con el horizonte 400

Estos criterios complementan [`horizonte-400.md`](./horizonte-400.md). El
horizonte define la dirección; este documento define la puerta de entrada para
que el crecimiento sea deliberado.
