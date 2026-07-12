# Repetición Dirigida: Validate Binary Search Tree

Fecha: 2026-07-12.

## Resultado

Repetido.

Puntaje total: 24/25.

## Evidencia

- Se reforzó `is_valid_bst` con duplicados en el lado izquierdo y derecho.
- `cargo test is_valid_bst` pasó con 3 pruebas enfocadas.

## Invariante Repetido

- Cada nodo hereda un límite inferior y un límite superior desde todos sus ancestros.
- Los límites son estrictos: duplicados no forman un BST válido.
- No basta comparar contra el padre inmediato, porque un descendiente puede violar el límite de un ancestro.

## Retro

- Qué salió bien: la implementación ya usaba límites estrictos y rechazó duplicados correctamente.
- Qué faltaba: una prueba explícita para duplicados en ambos lados del árbol.
- Acción siguiente: repetir explicación de límites heredados el 2026-07-20.
