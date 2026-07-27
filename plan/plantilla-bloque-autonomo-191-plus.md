# Plantilla de bloque autónomo para problemas 191+

**Estado:** guía operativa; requiere revisión humana antes de considerarse
material publicable.

**Trazabilidad:** [issue #4](https://github.com/jeresoftx/rust-algorithms/issues/4).

Esta plantilla define cómo ejecutar bloques del horizonte 400 sin perder la
trazabilidad de Jeresoft Academy. Cada bloque debe poder auditarse desde issue,
rama, commit, PR, validaciones y cierre.

## Antes de empezar

- [ ] El issue existe y está asignado a `jeresoftx`.
- [ ] El issue tiene milestone, labels y Project.
- [ ] El issue declara rango, familias, dificultad esperada y criterios de
      selección.
- [ ] La rama sale de `origin/main` actualizado.
- [ ] No hay cambios locales ajenos al bloque.
- [ ] El bloque no modifica RFC-0001, licencias, gobernanza fundacional ni
      decisiones de currículum.

## Estructura esperada del issue

```markdown
## Contexto

Qué tramo del horizonte 400 se trabaja y por qué importa.

## Trabajo

- [ ] Seleccionar candidatos con criterios explícitos.
- [ ] Implementar con TDD o documentar repetición/simulacro.
- [ ] Registrar complejidad y aprendizaje.
- [ ] Actualizar README, notas o plan cuando aplique.

## Criterios de aceptación

- Validaciones aplicables en verde.
- PR de un solo commit principal.
- Cierre con `Closes #N`.
- Coautor preservado en commit y squash merge.
```

## Flujo por PR

1. Revisar issue, milestone, labels, assignee y Project.
2. Marcar el item del Project como `In Progress`.
3. Crear una rama con prefijo coherente:
   - `docs/issue-N-descripcion`
   - `feat/issue-N-descripcion`
   - `test/issue-N-descripcion`
   - `chore/issue-N-descripcion`
4. Hacer cambios acotados al issue.
5. Validar localmente.
6. Crear un commit principal con:

```text
Closes #N

Co-authored-by: Joel Alvarez D. <124008575+joelalvarezduenas@users.noreply.github.com>
```

7. Subir la rama y crear PR con assignee, milestone y labels del issue.
8. Verificar metadata del PR.
9. Esperar checks remotos.
10. Fusionar con squash si todo está en verde y el cambio cumple revisión
    diferida.
11. Marcar el item del Project como `Done`.
12. Sincronizar `main`.

## Validaciones

Usar todas las validaciones aplicables al bloque:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo test --doc
cargo bench --no-run
git diff --check
```

Para cambios puramente documentales se conservan las mismas validaciones cuando
el costo sea bajo, porque protegen el repo completo y no solo el archivo editado.

## TDD por problema nuevo

- [ ] Escribir tests rojos con caso normal, borde y mínimo.
- [ ] Implementar la solución más clara que pase los tests.
- [ ] Refactorizar sin cambiar comportamiento.
- [ ] Agregar explicación de complejidad.
- [ ] Registrar aprendizaje o error si hubo tropiezo.

Si el bloque es una repetición dirigida, el equivalente de TDD es:

- [ ] escribir qué falló antes;
- [ ] resolver sin mirar la solución anterior;
- [ ] comparar diferencias;
- [ ] registrar qué invariante quedó más claro.

## Resumen de PR

Todo PR debe incluir:

```markdown
## Resumen

- Qué se agregó o cambió.
- Qué issue cierra.
- Qué queda fuera de alcance.

## Validaciones

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-targets`
- `cargo test --doc`
- `cargo bench --no-run`
- `git diff --check`

## Revisión diferida

Este PR se fusiona en modo de revisión diferida autorizado para Jeresoft
Academy. El cambio está dentro del plan aprobado, no modifica RFC-0001,
licencias, gobernanza fundacional ni decisiones de currículum, no agrega
dependencias no triviales, no usa `unsafe` y no marca contenido como `reviewed`
ni `published`.

Closes #N

Co-authored-by: Joel Alvarez D. <124008575+joelalvarezduenas@users.noreply.github.com>
```

## Límites de autonomía

Detenerse y pedir ayuda si el bloque:

- cambia el currículum o una decisión fundacional;
- requiere una dependencia externa no trivial;
- requiere `unsafe`;
- modifica licencias o gobernanza;
- necesita publicar contenido;
- exige credenciales o configuración externa;
- falla de forma persistente después de investigación razonable.

## Cierre del bloque

Al terminar un bloque, reportar:

- PRs fusionados con links;
- issues cerrados;
- validaciones ejecutadas;
- estado local;
- cuántos PRs coautorizados suma el bloque.

El bloque no marca material como `published` ni `reviewed`; solo deja evidencia
lista para revisión humana posterior.
