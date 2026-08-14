# M13 bounded LSP validation matrix

**Status:** M13a + M13b implemented (0.18.0) —
[DELIVERY_REPORT-2026-08-04-M13A-BOUNDED-LSP.md](DELIVERY_REPORT-2026-08-04-M13A-BOUNDED-LSP.md),
[DELIVERY_REPORT-2026-08-04-M13B-PROJECT-LSP.md](DELIVERY_REPORT-2026-08-04-M13B-PROJECT-LSP.md)  
**Date:** 2026-08-04  
**Design:** [DESIGN-M13-BOUNDED-LSP.md](DESIGN-M13-BOUNDED-LSP.md)  
**ADR:** [ADR-017](ADR-017-m13-bounded-lsp.md)

## Invariants

| ID | Evidence |
| --- | --- |
| M13-INV-001 | `aether lsp` stdio only |
| M13-INV-002 | Diagnostics from compile_source |
| M13-INV-003 | No compile/run of product AETH in LSP |
| M13-INV-004 | No server-side write of source/artifact |
| M13-INV-005 | Path jail for multi-file (M13b) |
| M13-INV-006 | format_source canonical |
| M13-INV-007 | Docs state bootstrap vs seed honesty |

## Positive (M13a)

| ID | Case | Expect |
| --- | --- | --- |
| P1 | initialize | capabilities include diagnostics, format, symbols, hover, definition |
| P2 | didOpen invalid yield type | publishDiagnostics AE-TYPE-001 |
| P3 | didOpen valid welcome | empty diagnostics |
| P4 | documentSymbol on multi-weave | weave symbols present |
| P5 | formatting | full document TextEdit matches format_source |
| P6 | hover on weave name | non-empty hover |
| P7 | definition jump to weave | Location to declaration |

## Negative (M13a)

| ID | Case | Expect |
| --- | --- | --- |
| N1 | TCP / network method | unsupported |
| N2 | Server writes file | none (audit) |
| N3 | `compile` artifact via LSP | no such method |
| N4 | Malformed JSON-RPC | error response, no panic |

## M13b (optional)

| ID | Case | Expect |
| --- | --- | --- |
| B1 | import unit definition | jumps to lib file under project |
| B2 | import outside project | diagnostic fail closed |

## Implementation checklist

### M13a

- [x] `aether lsp` command  
- [x] JSON-RPC framing  
- [x] Doc sync + diagnostics  
- [x] symbols, format, hover, definition  
- [x] Tests  
- [x] DOC-SYNC 0.17  
- [x] Delivery report  

### M13b

- [x] Project-aware imports  
- [x] Cross-file definition (exported only)  
- [x] Path jail via project units  

## Sign-off

| Gate | Status |
| --- | --- |
| Design + ADR-017 | **Accepted** |
| M13a implementation | **Done** (0.17.0) |
| M13b | **Done** (0.18.0) |
