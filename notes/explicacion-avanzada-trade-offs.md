# Guía de explicación avanzada para simulacros

**Estado:** guía de práctica; requiere revisión humana antes de considerarse
contenido publicado.

**Trazabilidad:** [issue #8](https://github.com/jeresoftx/rust-algorithms/issues/8).

Resolver un problema no termina al obtener una respuesta. En una entrevista,
la explicación hace visible el criterio: qué se entendió, qué se descartó, qué
invariante sostiene la solución y qué costo se acepta a cambio de qué mejora.

Esta guía sirve para ensayar esa comunicación. No sustituye tests, revisión de
código ni razonamiento de corrección.

## Secuencia de explicación

| Momento | Objetivo | Frase de arranque |
| --- | --- | --- |
| Clarificación | Convertir el enunciado en contrato | "Antes de elegir una técnica, quiero confirmar si..." |
| Ejemplo | Hacer visibles entradas y salida | "Con este caso pequeño, espero que... porque..." |
| Fuerza bruta | Encontrar el cuello de botella | "La versión directa sería..., pero cuesta... y falla cuando..." |
| Mejora | Nombrar la propiedad útil | "Puedo evitar ese trabajo repetido si mantengo..." |
| Invariante | Defender corrección durante el recorrido | "Después de cada paso, se conserva que..." |
| Complejidad | Contabilizar recursos reales | "Cada elemento entra y sale a lo más una vez; por eso..." |
| Validación | Probar el riesgo principal | "Antes de cerrar, probaría vacío, mínimo y el borde donde..." |
| Cierre | Declarar límites y alternativa | "Para entradas pequeñas elegiría..., pero con esta restricción prefiero..." |

Las frases son puntos de partida, no guion para recitar. Deben adaptarse al
problema y respaldarse con hechos del algoritmo.

## Preguntas de clarificación

Empezar por restricciones evita optimizar una versión imaginaria del problema.

- "¿La entrada puede estar vacía o contener valores repetidos?"
- "¿Necesitamos una respuesta cualquiera, todas las respuestas o una respuesta
  con desempate determinista?"
- "¿Los valores pueden ser negativos? Eso cambia si una ventana se puede
  contraer de forma segura."
- "¿El grafo es dirigido, tiene pesos o permite ciclos?"
- "¿La estructura se consulta una vez o se actualiza entre consultas?"
- "¿Hay límites de memoria, mutabilidad o latencia que hagan relevante una
  alternativa in-place?"

No preguntar por preguntar: cada pregunta debe poder cambiar el diseño.

## Cómo comparar alternativas

Una comparación útil no enumera técnicas; relaciona una limitación con una
decisión.

| Situación | Alternativa simple | Cuándo deja de servir | Alternativa justificada |
| --- | --- | --- | --- |
| Máximo de cada ventana | Recorrer toda la ventana | Ventanas grandes y solapadas | Deque monotónico con índices vigentes. |
| Rutas con peso positivo | BFS | Las aristas no cuestan lo mismo | Dijkstra con frontera de menor costo. |
| Consultas de rango estáticas | Sumar cada rango | Muchas consultas sobre el mismo arreglo | Prefix sums. |
| Consultas con actualizaciones | Recalcular prefijos | Cambios frecuentes invalidan prefijos | Fenwick o segment tree según operación. |
| Elegir proyectos factibles | Revisar todos en cada paso | La frontera habilitada cambia con capital | Heap de proyectos disponibles. |
| Palabras con prefijo común | Comparar cada palabra completa | Se repiten prefijos y búsquedas | Trie cuando el costo y alfabeto lo justifican. |

La pregunta de control es: "¿qué trabajo repetido deja de hacerse y qué costo
nuevo introduce la estructura?"

## Patrones de invariante

No basta con decir que una estructura "se mantiene ordenada". Describir qué
representa y por qué permite decidir el siguiente paso.

| Familia | Invariante comunicable |
| --- | --- |
| Two pointers | "Todo lo que quedó fuera de los punteros ya fue descartado o incorporado sin poder mejorar la respuesta." |
| Sliding window | "La ventana actual mantiene la condición requerida; al contraer, no pierdo una respuesta más corta válida." |
| Heap | "La raíz representa el mejor candidato entre los estados que ya son factibles." |
| Binary search on answer | "El predicado de factibilidad es falso antes de un límite y verdadero desde ese límite, o a la inversa." |
| DP | "Cada estado resume exactamente la mejor respuesta para un prefijo, índice o intervalo ya definido." |
| BFS | "Cuando extraigo un nodo, ya conozco la menor cantidad de aristas para llegar a él." |
| Dijkstra | "Cuando fijo un nodo de menor distancia pendiente, ningún camino futuro con pesos no negativos puede mejorarlo." |
| Union-Find | "Dos elementos comparten representante si y solo si pertenecen al componente conectado que se ha procesado." |
| Monotonic stack/deque | "Los elementos eliminados están dominados por uno más reciente y no podrán ser la mejor respuesta futura." |

## Validación narrada

Los tests se escriben y ejecutan; la entrevista además necesita explicar qué
riesgo cubre cada uno.

| Caso | Pregunta que responde | Ejemplo de frase |
| --- | --- | --- |
| Mínimo | ¿El estado base existe? | "Con un elemento, la transición no debe leer un vecino inexistente." |
| Vacío | ¿El contrato define ausencia? | "Para entrada vacía retorno... y evito indexar." |
| Repetidos | ¿El desempate o conteo es estable? | "Los duplicados no deben crear dos grupos o romper el límite estricto." |
| Límite | ¿El rango o acumulado se desborda? | "Uso este tipo porque la suma puede superar el tamaño de cada valor." |
| Regresión | ¿El error anterior vuelve a aparecer? | "Este caso reproduce la contracción incorrecta de la ventana." |

## Miniensayos por familia

### Búsqueda binaria sobre respuesta

"Primero calculo límites que necesariamente contienen la respuesta. Para una
capacidad dada puedo comprobar factibilidad en una pasada. Si una capacidad
funciona, cualquier capacidad mayor también funciona; por eso el predicado es
monótono y puedo buscar el mínimo válido. Probaría el límite inferior, el
superior y el caso de una sola carga."

### Grafo ponderado

"BFS sería correcta solo si cada arista tuviera el mismo costo. Aquí una ruta
con menos saltos puede costar más, así que mantengo una frontera priorizada por
distancia. Cuando extraigo el menor costo pendiente, los pesos no negativos
garantizan que esa distancia ya no mejorará. Probaría un nodo inalcanzable y
dos rutas donde la más corta en aristas no es la más barata."

### Programación dinámica por intervalo

"Elegir la primera operación deja dependencias cruzadas; en cambio, si fijo la
última operación, el intervalo se divide en dos partes independientes. El
estado representa el costo óptimo del intervalo abierto. Probaría un intervalo
vacío, uno con un corte y uno donde la elección greedy inmediata es mala."

### Heap y greedy

"Ordeno por la restricción que vuelve comparables las decisiones. El heap
guarda solo las opciones factibles y su raíz es la que conviene tomar o
reemplazar ahora. Explicaría qué decisión local queda protegida y probaría un
empate, una opción que todavía no es factible y una entrada vacía."

## Antipatrones de comunicación

- Decir "uso DP" sin definir el estado y la transición.
- Presentar la solución óptima sin mostrar qué limita a la versión directa.
- Llamar "O(n)" a un algoritmo con heap sin contar `log k`.
- Afirmar que un caso borde está cubierto sin describir el test.
- Usar una estructura avanzada solo porque aparece en el nombre del patrón.
- Recitar complejidades sin relacionarlas con operaciones concretas.

Cuando aparezca uno de estos síntomas, bajar un nivel: volver al ejemplo,
nombrar el estado y explicar una iteración completa.

## Registro posterior

Al terminar el simulacro, copiar la plantilla de
[`mock-interview-rubric.md`](./mock-interview-rubric.md) en
[`review-queue.md`](./review-queue.md) y registrar:

- la pregunta de clarificación que faltó;
- la alternativa que se eligió o descartó;
- la invariante que quedó débil;
- el test que habría detectado el error antes;
- una fecha de repetición si el puntaje fue menor a 20/25.

Esta guía y la rúbrica aportan evidencia de práctica. La revisión humana decide
si esa evidencia alcanza para convertir material en contenido publicable.
