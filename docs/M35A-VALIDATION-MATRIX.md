# M35a validation matrix — verified AETH → C pure pilot

| ID | Check | Expected |
| --- | --- | --- |
| M35A-001 | verify before lower | invalid AETH rejected |
| M35A-002 | pure Whole main lowers to C | C contains main/yield exit |
| M35A-003 | product source path seed compile then lower | dual product bytes verify |
| M35A-004 | foreign/host/nursery rejected for pilot | FAIL closed |
| M35A-005 | no Aether-source→C without AETH | only verified AETH input |
| M35B-001 | bind + sum + yield lowers with locals | C has locals[] and + |
| M35B-002 | VM exit matches for bind sum fixture | exit 42 |
| M35C-001 | speak Text + multi-weave call lowers | C has fputs + aether_fn_ CALL |
| M35C-002 | VM exit + stdout for multi-weave speak fixture | exit 42, stdout `ok` |
| M35C-003 | dual-run helper: VM exit + successful C lower | `native_dual_run_vm_exit` |
| M35D-001 | host cc dual-exec best-effort without cc | `cc_available=false` OK |
| M35D-002 | when cc present, native exit matches VM | FAIL closed on diverge |
| M35E-001 | native object emit without cc | FAIL closed AE-NATIVE-004 |
| M35E-002 | native object emit with cc | object file produced |

---

*End of M35A-VALIDATION-MATRIX.md*
