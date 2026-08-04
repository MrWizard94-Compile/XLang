# M14 host I/O capabilities validation matrix

**Status:** Design gate — **implementation not yet green**  
**Date:** 2026-08-04  
**Design:** [DESIGN-M14-HOST-IO-CAPABILITIES.md](DESIGN-M14-HOST-IO-CAPABILITIES.md)  
**ADR:** [ADR-018](ADR-018-m14-host-io-capabilities.md)  
**Threat:** [THREAT_MODEL-v2-CAPABLE-HOST.md](THREAT_MODEL-v2-CAPABLE-HOST.md)

## Invariants

| ID | Evidence |
| --- | --- |
| M14-INV-001 | I/O without grant → AE-HOST-003 |
| M14-INV-004 | path escape → AE-HOST-004 |
| M14-INV-003 | pure fixtures without grants still work |
| M14-INV-006 | oversize read/write → AE-HOST-005 |
| M14-INV-005 | no shell/network services installed |

## Positive

| ID | Case | Expect |
| --- | --- | --- |
| P1 | `--grant-read` + `read_text` | file contents as Text |
| P2 | `--grant-write` + `write_text` | file created under root |
| P3 | `--grant-env FOO` + `env_get` | value |
| P4 | pure `host-pilot` without grants | still exit 48 |
| P5 | seed≡bootstrap for pure host-pilot | regression |

## Negative

| ID | Case | Expect |
| --- | --- | --- |
| N1 | read without grant | fail closed |
| N2 | guest path `../x` | AE-HOST-004 |
| N3 | absolute guest path | AE-HOST-004 |
| N4 | env name not granted | fail closed |
| N5 | write outside root via symlink | fail closed |
| N6 | read > max size | AE-HOST-005 |

## Implementation checklist

- [ ] Grant table + path jail  
- [ ] Host catalog services  
- [ ] CLI `run` flags  
- [ ] Tests P*/N*  
- [ ] Example program  
- [ ] DOC-SYNC 0.19  
- [ ] Delivery report  

## Sign-off

| Gate | Status |
| --- | --- |
| Threat model v2 | **Ready** |
| Design + ADR-018 | **Ready / Accepted** |
| Implementation | Pending |
