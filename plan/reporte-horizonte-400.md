# Reporte de cierre de planeación: Horizonte 400

**Estado:** evidencia interna preparada para revisión humana; no representa
contenido revisado ni publicado.

**Trazabilidad:** [issue #13](https://github.com/jeresoftx/rust-algorithms/issues/13).

## Propósito

Este reporte distingue la capacidad actual del repositorio de la ruta futura
que se preparó. Sirve para verificar alcance, decisiones y deuda visible antes
de decidir si el horizonte 400 se ejecuta, ajusta o queda como reserva de
estudio.

## Estado verificable

| Aspecto | Estado |
| --- | --- |
| Catálogo funcional | 195 problemas implementados y probados. |
| Familias de patrones | 19 módulos de patrones. |
| Tests automatizados | 472 pruebas deterministas/property y 9 doctests. |
| Simulacros documentados | 6 acumulados. |
| Horizonte 400 | Planeado y organizado; no ejecutado como catálogo funcional. |
| Contenido publicado o revisado | Ningún bloque 191-400 se marca como publicado o revisado. |

Los rangos 191-400 son candidatos, repeticiones, simulacros y protocolos de
trabajo. Solo incrementan el catálogo cuando entran implementaciones
individuales con TDD y evidencia revisable; hasta ahora, Valid Palindrome II e
Is Subsequence, Merge Sorted Array, Find Pivot Index y Backspace String Compare
(candidatos 192-195 y 197) elevaron el conteo de 190 a 195.

## Evidencia preparada

| Tramo | Documento | Intención pedagógica |
| --- | --- | --- |
| 191-220 | `consolidacion-191-220.md` | Consolidar patrones frecuentes. |
| 221-240 | `simulacros-221-240.md` | Practicar ejecución bajo tiempo y comunicación. |
| 241-260 | `repeticiones-241-260.md` | Repetir debilidades con retro y evidencia. |
| 261-290 | `profundizacion-261-290.md` | Combinar patrones y justificar composición. |
| 291-315 | `profundizacion-291-315.md` | Usar Medium y Hard por señal de entrevista. |
| 316-330 | `profundizacion-316-330.md` | Cerrar brechas con patrones mixtos. |
| 331-360 | `cierre-331-360.md` | Seleccionar Hards por técnica transferible. |
| 361-400 | `cierre-361-400.md` | Mezclar práctica final, repaso y auditoría. |

La guía `notes/explicacion-avanzada-trade-offs.md` complementa la rúbrica de
simulacros: ayuda a expresar restricciones, alternativa directa, invariante,
complejidad y pruebas. No reemplaza implementación ni demostración de
corrección.

## Decisiones pedagógicas consolidadas

- El número 400 es un horizonte de organización, no una meta de volumen.
- Los Hards se aceptan por técnica transferible, no por prestigio de dificultad.
- Una repetición cuenta solo si registra aprendizaje nuevo y evidencia.
- La alternativa simple aparece antes de la optimización para justificar una
  estructura avanzada.
- Tests, complejidad e invariantes son evidencia distinta; ninguna sustituye a
  las otras.

## Deuda visible y límites

- Los candidatos 196 y 198-400 todavía no son implementaciones funcionales nuevas.
- La dificultad del catálogo histórico de 190 problemas sigue sin normalizarse
  retroactivamente, por decisión de la matriz de cobertura.
- Las repeticiones necesitan ejecución real antes de pasar a `dominado`.
- Los simulacros futuros necesitan puntaje, retro y acción de seguimiento.
- Cualquier material publicable necesita revisión humana, aunque tenga CI en
  verde.

## Validaciones registradas

Los PRs de planeación y cierre del horizonte ejecutaron, cuando aplicaba:

```text
git diff --check
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo test --doc
cargo bench --no-run
```

Los checks remotos de los PRs fusionados quedaron en verde. Estas validaciones
demuestran salud del crate y consistencia del repositorio; no convierten por sí
solas un plan en contenido pedagógico publicado.

## Checklist de revisión humana semanal

- [ ] Confirmar que la selección 191-400 sigue alineada con el estudio real.
- [ ] Elegir un candidato y crear un issue específico de implementación con
      TDD.
- [ ] Revisar concepto, problema, alternativas, código, tests y complejidad
      antes de aprobar material nuevo.
- [ ] Revisar simulacros y cola de repaso antes de usar `dominado`.
- [ ] Decidir qué material puede pasar a revisión editorial o publicación.
- [ ] Decidir si el milestone de planeación se cierra o queda abierto para
      ejecutar candidatos en bloques posteriores.

## Conclusión

La planeación del horizonte 400 queda completa y trazable en GitHub, pero la
revisión humana conserva la decisión final. El siguiente trabajo técnico no es
"contar hasta 400": es elegir un candidato concreto, implementarlo con TDD y
demostrar que mejora el libro de ingeniería y el crate educativo.
