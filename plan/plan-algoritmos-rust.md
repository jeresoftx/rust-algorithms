# Plan de Autoestudio: Algoritmos en Rust para Entrevistas Técnicas

**Objetivo:** prepararme para entrevistas técnicas usando este repo como bitácora práctica de algoritmos, patrones de resolución y simulacros en Rust.

**Duración sugerida:** 16 semanas.

**Ritmo base:** 5 días por semana, 90 a 150 minutos por día.

**Resultado esperado:** resolver problemas medium con consistencia, explicar trade-offs con claridad, escribir Rust correcto bajo tiempo y llegar a simulacros de entrevista con un método repetible.

---

## Principios

1. Practicar patrones, no listas infinitas.
2. Escribir cada solución con tests y análisis de complejidad.
3. Repetir problemas fallados hasta poder resolverlos sin mirar notas.
4. Priorizar claridad sobre trucos.
5. Simular entrevistas antes de sentirse listo.

---

## Estructura Recomendada del Repo

Crear esta estructura conforme avances:

```text
rust-algorithms/
  Cargo.toml
  src/
    lib.rs
    patterns/
      arrays_strings.rs
      two_pointers.rs
      sliding_window.rs
      hashing.rs
      stack_queue.rs
      binary_search.rs
      recursion_backtracking.rs
      linked_lists.rs
      trees.rs
      graphs.rs
      heaps.rs
      intervals.rs
      dynamic_programming.rs
      greedy.rs
    utils/
      mod.rs
      tree_node.rs
      list_node.rs
  tests/
    patterns/
  notes/
    mistakes.md
    rust-interview-patterns.md
    complexity-cheatsheet.md
  plan/
    plan-algoritmos-rust.md
```

Cada solución debería incluir:

- Nombre del problema.
- Link o fuente.
- Patrón principal.
- Idea en 3 a 6 líneas.
- Complejidad temporal y espacial.
- Tests para caso normal, borde y caso pequeño.
- Nota de error si te atoraste.

---

## Rutina Semanal

### Lunes a Jueves

1. Calentamiento: 10 minutos repasando errores anteriores.
2. Estudio de patrón: 20 a 30 minutos.
3. Problema principal: 45 a 70 minutos.
4. Refactor y tests: 15 a 25 minutos.
5. Nota final: 5 minutos en `notes/mistakes.md`.

### Viernes

1. Rehacer 2 problemas fallados sin mirar la solución.
2. Resolver 1 problema nuevo cronometrado.
3. Actualizar checklist semanal.

### Sábado Opcional

Simulacro ligero o repaso de Rust.

### Domingo

Descanso. Si quieres tocar el repo, solo lectura de notas.

---

## Semana 1: Base de Rust y Complejidad

**Meta:** tener base fluida para escribir soluciones sin pelear con el lenguaje.

Temas:

- `Vec`, slices, `String`, `&str`, `HashMap`, `HashSet`, `VecDeque`, `BinaryHeap`.
- Ownership y borrowing aplicados a ejercicios.
- Complejidad Big O.
- Esqueleto de tests en Rust.

Práctica:

- 8 a 10 problemas easy.
- 2 problemas medium sencillos.

Criterio de avance:

- Puedes escribir una función con tests en menos de 10 minutos.
- Puedes explicar complejidad sin dudar.

---

## Semanas 2 y 3: Arrays, Strings y Hashing

**Meta:** dominar los problemas más comunes de conteo, frecuencia, índices y transformaciones.

Patrones:

- Frequency map.
- Prefix sum.
- Difference array básico.
- String normalization.
- Anagramas.
- Detección de duplicados.

Problemas sugeridos:

- Two Sum.
- Valid Anagram.
- Group Anagrams.
- Top K Frequent Elements.
- Product of Array Except Self.
- Longest Consecutive Sequence.
- Subarray Sum Equals K.

Criterio de avance:

- Identificas rápidamente cuándo usar `HashMap` vs ordenar.
- Puedes explicar el trade-off entre tiempo, memoria y mutabilidad.

---

## Semanas 4 y 5: Two Pointers, Sliding Window y Stack

**Meta:** reconocer ventanas, pares ordenados y estructuras monotonic.

Patrones:

- Two pointers en arreglo ordenado.
- Fast/slow pointers.
- Ventana fija.
- Ventana variable.
- Stack monotonic.
- Parentheses matching.

Problemas sugeridos:

- Valid Palindrome.
- 3Sum.
- Container With Most Water.
- Best Time to Buy and Sell Stock.
- Longest Substring Without Repeating Characters.
- Minimum Window Substring.
- Valid Parentheses.
- Daily Temperatures.
- Largest Rectangle in Histogram.

Criterio de avance:

- Puedes decidir si una ventana crece, se contrae o se reinicia.
- Puedes mantener invariantes simples mientras codificas.

---

## Semana 6: Binary Search

**Meta:** usar búsqueda binaria también en respuestas, no solo en arreglos.

Patrones:

- Buscar índice exacto.
- Lower bound / upper bound.
- Binary search on answer.
- Condición monótona.

Problemas sugeridos:

- Binary Search.
- Search Insert Position.
- Search in Rotated Sorted Array.
- Find Minimum in Rotated Sorted Array.
- Koko Eating Bananas.
- Capacity To Ship Packages Within D Days.

Criterio de avance:

- Puedes definir `lo`, `hi` y la condición monótona antes de escribir código.

---

## Semanas 7 y 8: Recursión, Backtracking y Listas Enlazadas

**Meta:** construir soluciones recursivas claras y manejar punteros/listas en Rust con calma.

Patrones:

- Decision tree.
- DFS con path mutable.
- Permutaciones y subsets.
- Pruning.
- Reversal de linked list.
- Fast/slow pointer en listas.

Problemas sugeridos:

- Subsets.
- Permutations.
- Combination Sum.
- Generate Parentheses.
- Word Search.
- Reverse Linked List.
- Merge Two Sorted Lists.
- Linked List Cycle.
- Reorder List.

Criterio de avance:

- Puedes dibujar el árbol de decisiones antes de codificar.
- Puedes implementar listas enlazadas en Rust sin bloquearte demasiado en ownership.

---

## Semanas 9 y 10: Árboles

**Meta:** dominar DFS, BFS y recursión sobre árboles binarios.

Patrones:

- Preorder, inorder, postorder.
- DFS con retorno de información.
- BFS por niveles.
- Lowest common ancestor.
- Validación de BST.
- Construcción de árboles.

Problemas sugeridos:

- Maximum Depth of Binary Tree.
- Invert Binary Tree.
- Diameter of Binary Tree.
- Balanced Binary Tree.
- Same Tree.
- Subtree of Another Tree.
- Binary Tree Level Order Traversal.
- Validate Binary Search Tree.
- Lowest Common Ancestor of a BST.
- Construct Binary Tree from Preorder and Inorder Traversal.

Criterio de avance:

- Puedes decidir si el estado vive en parámetros, retorno o estructura auxiliar.

---

## Semanas 11 y 12: Grafos

**Meta:** reconocer representaciones y recorridos de grafos con seguridad.

Patrones:

- DFS/BFS en matriz.
- DFS/BFS en adjacency list.
- Connected components.
- Cycle detection.
- Topological sort.
- Union Find.
- Shortest path con BFS.

Problemas sugeridos:

- Number of Islands.
- Clone Graph.
- Max Area of Island.
- Pacific Atlantic Water Flow.
- Course Schedule.
- Rotting Oranges.
- Walls and Gates.
- Redundant Connection.
- Accounts Merge.

Criterio de avance:

- Puedes elegir representación antes de codificar.
- Puedes explicar visited, pila de recursión y componentes.

---

## Semana 13: Montículos, Intervalos y Greedy

**Meta:** resolver scheduling, prioridades y decisiones locales justificadas.

Patrones:

- Min heap / max heap.
- Merge intervals.
- Sweep line básico.
- Greedy con prueba intuitiva.

Problemas sugeridos:

- Kth Largest Element in an Array.
- Find Median from Data Stream.
- Merge Intervals.
- Insert Interval.
- Meeting Rooms.
- Meeting Rooms II.
- Non-overlapping Intervals.
- Task Scheduler.

Criterio de avance:

- Puedes justificar por qué un heap o sort simplifica el problema.

---

## Semanas 14 y 15: Dynamic Programming

**Meta:** pasar de recursión a memoización y tabulación cuando convenga.

Patrones:

- 1D DP.
- 2D DP.
- Knapsack básico.
- Longest common subsequence.
- State compression.
- Decision DP.

Problemas sugeridos:

- Climbing Stairs.
- House Robber.
- Coin Change.
- Longest Increasing Subsequence.
- Word Break.
- Unique Paths.
- Longest Common Subsequence.
- Partition Equal Subset Sum.
- Decode Ways.

Criterio de avance:

- Puedes definir estado, transición, base cases y orden de cómputo.

---

## Semana 16: Simulacros de Entrevista

**Meta:** convertir conocimiento en performance de entrevista.

Rutina:

- 3 simulacros de 45 minutos.
- 2 sesiones de revisión profunda.
- 1 día de behavioral stories.
- 1 día de repaso de errores frecuentes.

Formato de simulacro:

1. 5 minutos: clarificar problema y ejemplos.
2. 5 minutos: proponer enfoque y complejidad.
3. 25 minutos: implementar.
4. 5 minutos: probar manualmente.
5. 5 minutos: discutir mejoras.

Criterio de avance:

- Resuelves mediums frecuentes en 35 a 45 minutos.
- Hablas mientras piensas sin perder estructura.
- Tienes historias STAR preparadas para liderazgo, conflicto, ambigüedad, impacto y aprendizaje.

---

## Checklist por Problema

Antes de marcar un problema como terminado:

- [ ] Entendí el enunciado y escribí ejemplos.
- [ ] Identifiqué el patrón principal.
- [ ] Expliqué el algoritmo en voz alta.
- [ ] Escribí tests.
- [ ] Pasé los tests.
- [ ] Anoté complejidad temporal y espacial.
- [ ] Registré errores o bloqueos.
- [ ] Rehice el problema otro día si fallé o miré solución.

---

## Sistema de Repaso

Usa cuatro estados:

- `nuevo`: primera vez.
- `fallado`: no salió sin ayuda.
- `repetir`: salió con dificultad o con bugs importantes.
- `dominado`: salió limpio en tiempo.

Cadencia:

- Problema fallado: repetir en 2 días.
- Problema con dificultad: repetir en 1 semana.
- Problema dominado: repetir en 3 a 4 semanas.

Mantener una tabla en `notes/mistakes.md`:

```markdown
| Fecha | Problema | Patrón | Error | Corrección | Repetir |
| --- | --- | --- | --- | --- | --- |
| 2026-07-11 | Two Sum | Hashing | Busqué pares con O(n^2) | Usar HashMap valor -> índice | 2026-07-13 |
```

---

## Métricas de Progreso

Medir cada semana:

- Problemas resueltos.
- Problemas repetidos.
- Porcentaje sin mirar solución.
- Tiempo promedio por problema medium.
- Bugs de Rust frecuentes.
- Patrones más débiles.

Metas acumuladas:

- Semana 4: 35 a 45 problemas.
- Semana 8: 75 a 90 problemas.
- Semana 12: 120 a 140 problemas.
- Semana 16: 160 a 190 problemas y 5 a 8 simulacros.

Calidad > cantidad: un problema bien revisado vale más que tres copiados.

---

## Preparación de Entrevistas Técnicas

Una buena entrevista técnica suele valorar:

- Claridad para explorar requisitos.
- Pensamiento estructurado.
- Correctness antes de micro-optimización.
- Manejo de edge cases.
- Comunicación durante la solución.
- Capacidad de mejorar una primera solución.

Practicar frases:

- "Primero voy a confirmar constraints y casos borde."
- "La solución brute force sería..., pero cuesta..."
- "Hay una propiedad monótona, así que podemos usar binary search."
- "Voy a mantener este invariante de la ventana."
- "Probemos con un caso pequeño y uno borde."

Behavioral:

Preparar 5 historias STAR:

- Liderazgo.
- Conflicto tecnico.
- Ambigüedad.
- Error propio y aprendizaje.
- Impacto medible.

---

## Primeros 7 Días

Día 1:

- Inicializar proyecto Rust.
- Crear estructura base.
- Resolver Two Sum y Valid Anagram.

Día 2:

- Repasar `HashMap`, `HashSet`, slices y strings.
- Resolver Contains Duplicate y Group Anagrams.

Día 3:

- Resolver Product of Array Except Self.
- Documentar complejidad y edge cases.

Día 4:

- Resolver Top K Frequent Elements.
- Comparar heap vs bucket sort.

Día 5:

- Rehacer un problema fallado.
- Resolver Longest Consecutive Sequence.

Día 6:

- Simulacro corto: 1 easy + 1 medium.
- Registrar errores.

Día 7:

- Descanso o lectura ligera de notas.

---

## Senales de que Vas Bien

- Antes de codificar, puedes decir qué patrón aplica.
- Tus tests capturan bordes sin exagerar.
- Tus soluciones se vuelven más pequeñas y claras.
- Fallas menos por Rust y más por razonamiento real.
- Puedes explicar por qué una solución es correcta.

## Senales de Ajuste

- Si pasas más de 3 días atorado en Rust, dedicar 1 sesión solo a lenguaje.
- Si resuelves muchos easy pero pocos medium, bajar cantidad y subir profundidad.
- Si miras soluciones demasiado pronto, usar un límite: 25 minutos de intento real, luego pista mínima, luego solución.
- Si olvidas patrones, hacer tarjetas pequeñas con invariantes y ejemplos.

---

## Próxima Acción Recomendada

1. Crear `Cargo.toml`, `src/lib.rs`, `notes/mistakes.md` y el primer modulo `src/patterns/hashing.rs`.
2. Resolver `Two Sum` con tests.
3. Registrar la solución como la primera entrada del repositorio.
