# M11 language modules validation matrix

**Status:** Design gate for M11 — **implementation not yet green**  
**Date:** 2026-08-04  
**Design:** [DESIGN-M11-LANGUAGE-MODULES.md](DESIGN-M11-LANGUAGE-MODULES.md)  
**ADR:** [ADR-015](ADR-015-m11-language-modules.md)

## Invariants under test

| Invariant | Evidence |
| --- | --- |
| M11-INV-001 grammar | Parse import/export/qualified call; reject bad placement |
| M11-INV-002 export | Private weave not callable via alias |
| M11-INV-003 DAG | Cycle fails `AE-MOD-004` |
| M11-INV-004 one main | Two mains in cone fail; entry has main |
| M11-INV-005 lib main | Lib with `main` fails `AE-MOD-006` |
| M11-INV-006 project closed | Import path not in units fails `AE-MOD-002` |
| M11-INV-007 single-file | `aether compile` rejects `import unit` `AE-MOD-007` |
| M11-INV-008 build | `project build` one AETH; verify; run expected exit |
| M11-INV-009 honesty | Docs/tests assert bootstrap-hosted multi-module until M11b |
| M11-INV-010 limits | Host weave export rejected; record import not in pilot |
| M11-INV-011 worlds | Duplicate world names fail `AE-MOD-005` |

## Positive corpus (M11a)

| ID | Case | Expect |
| --- | --- | --- |
| P1 | `math.double` import, yield 42 | build+run exit 42 |
| P2 | Transitive import A→B→C | resolve; run |
| P3 | Diamond DAG (A→B, A→C, B→D, C→D) | single load of D; run |
| P4 | Exported `raises Whole` + handle in main | M4 rules hold |
| P5 | `project verify` multi-module project | pass locks/paths |
| P6 | Lib without `main` | verify+build OK |
| P7 | M10 `project-multi` still independent verify | no regression |

## Negative corpus (M11a)

| ID | Case | Expect |
| --- | --- | --- |
| N1 | `call math.secret` private | `AE-MOD-003` |
| N2 | Import `../escape.ae` | `AE-MOD-002` / path |
| N3 | Import path not in project units | `AE-MOD-002` |
| N4 | Cycle A↔B | `AE-MOD-004` |
| N5 | Duplicate alias | `AE-MOD-005` |
| N6 | Lib defines `main` | `AE-MOD-006` |
| N7 | Entry missing `main` | `AE-MOD-006` |
| N8 | Single-file compile with import | `AE-MOD-007` |
| N9 | `export host weave` | reject |
| N10 | Import main unit as lib dependency | reject |
| N11 | Unknown alias in call | reject |
| N12 | Export name clash across imports without qualify | N/A if always qualified; duplicate export in one unit reject |

## M11b corpus (seed)

| ID | Case | Expect |
| --- | --- | --- |
| S1 | Module corpus seed ≡ bootstrap AETH bytes | identity |
| S2 | Self-host still holds | multi-gen seed |
| S3 | MANIFEST switches multi-module to seed-hosted only after S1 | DOC-SYNC |

## Non-tests (out of M11)

- Registry / URL import  
- Import records/shapes  
- LSP navigation  
- Generics  

## Implementation checklist

### M11a

- [ ] Parser/grammar  
- [ ] Graph resolve + cycles  
- [ ] Bootstrap multi-source compile  
- [ ] `project build`  
- [ ] Single-file reject import  
- [ ] Example `examples/project-modules`  
- [ ] Core/CLI tests P*/N*  
- [ ] DOC-SYNC (0.12 language / 0.14 package suggested)  
- [ ] Delivery report; claim bootstrap-hosted multi-module  

### M11b

- [ ] Seed multi-module strategy implemented  
- [ ] Dual-compare S1–S2  
- [ ] MANIFEST authority switch  
- [ ] Delivery report  

## Sign-off

| Gate | Status |
| --- | --- |
| Design + ADR-015 | **Ready / Accepted** |
| M11a implementation | Pending |
| M11b seed proof | Pending |
