# M14 host I/O capabilities validation matrix

**Status:** Implementation green (package 0.19.0)  
**Date:** 2026-08-04  
**Design:** [DESIGN-M14-HOST-IO-CAPABILITIES.md](DESIGN-M14-HOST-IO-CAPABILITIES.md)  
**ADR:** [ADR-018](ADR-018-m14-host-io-capabilities.md)  
**Threat:** [THREAT_MODEL-v2-CAPABLE-HOST.md](../Current%20state/THREAT_MODEL-v2-CAPABLE-HOST.md)

## Invariants

| ID | Evidence |
| --- | --- |
| M14-INV-001 | I/O without grant → AE-HOST-003 (`m14_read_text_*`, `m14_write_*`) |
| M14-INV-004 | path escape / absolute → AE-HOST-004 |
| M14-INV-003 | pure fixtures without grants still work (`m14_pure_host_pilot_*`, host-pilot exit 48) |
| M14-INV-006 | oversize read → AE-HOST-005 |
| M14-INV-005 | no shell/network services installed (catalog closed) |

## Positive

| ID | Case | Expect | Evidence |
| --- | --- | --- | --- |
| P1 | `--grant-read` + `read_text` | file contents as Text | core + CLI tests; exit 6 for `"Aether"` |
| P2 | `--grant-write` + `write_text` | file created under root | `m14_write_text_*`; `"hello"` → exit 5 |
| P3 | `--grant-env FOO` + `env_get` | value | `m14_write_text_and_env_get_*` |
| P4 | pure `host-pilot` without grants | still exit 48 | pure pilot + seed dual-compare |
| P5 | seed≡bootstrap for host-io declarations | compile match | `seed_profile_compiler_forges_m14_*` |
| P6 | `read_bytes` under grant | Bytes extent | `m14_read_bytes_and_size_limit` |

## Negative

| ID | Case | Expect | Evidence |
| --- | --- | --- | --- |
| N1 | read without grant | fail closed AE-HOST-003 | core tests |
| N2 | guest path `../x` | AE-HOST-004 | core tests |
| N3 | absolute guest path | AE-HOST-004 | core tests |
| N4 | env name not granted | fail closed | core tests |
| N5 | write without grant | AE-HOST-003 | core tests |
| N6 | read > max size | AE-HOST-005 | core tests |

## Implementation checklist

- [x] Grant table + path jail  
- [x] Host catalog services  
- [x] CLI `run` flags  
- [x] Tests P*/N*  
- [x] Example programs  
- [x] DOC-SYNC 0.19  
- [x] Delivery report  

## Sign-off

| Gate | Status |
| --- | --- |
| Threat model v2 | **Ready** |
| Design + ADR-018 | **Accepted / Implemented** |
| Implementation | **Green (0.19.0)** |
