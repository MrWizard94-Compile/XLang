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

---

*End of M35A-VALIDATION-MATRIX.md*
