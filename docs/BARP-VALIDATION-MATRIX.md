# BARP validation matrix — seed diagnostic authority reduction

| ID | Check | Expected |
| --- | --- | --- |
| BARP-SPEAK-001 | Empty source | Seed SPEAK `AE-SEED-005`, `origin: seed-speak`, blank Bytes result |
| BARP-SPEAK-002 | Missing `world` | Seed SPEAK `AE-SEED-006` |
| BARP-SPEAK-003 | Non-main weave without `main` | Seed SPEAK `AE-SEED-004` |
| BARP-SPEAK-004 | Raw `import unit` in a single source | Seed SPEAK `AE-SEED-012` |
| BARP-SPEAK-005 | Input starts with or follows a line feed with ASCII tab | Seed SPEAK `AE-SEED-003`; bounded tab-indentation pilot |
| BARP-SPEAK-006 | Input starts with or follows a line feed with `fn ` | Seed SPEAK `AE-SEED-007`; bounded top-level legacy-`fn` pilot |
| BARP-SPEAK-007 | Valid source text containing escaped `\\nfn` inside a Text literal | Does not trigger the lexical `fn` pilot |
| BARP-SPEAK-008 | Seed source rebuild through bootstrap, product, and forge | Byte-identical to checked-in `seed/aether_seed.aeth` |
| BARP-SPEAK-009 | Full conformance tracker | Remains `false`; no diagnostic-parity claim |
| BARP-SPEAK-010 | Tab plus legacy `fn ` at a source-line start | Stable priority emits `AE-SEED-003`; lower-priority pilot does not replace it |
| BARP-SPEAK-010 | Tab plus legacy `fn ` at a source-line start | Stable priority emits `AE-SEED-003`; lower-priority pilot does not replace it |
| BARP-SPEAK-010 | Tab plus legacy `fn ` at a source-line start | Stable priority emits `AE-SEED-003`; lower-priority pilot does not replace it |

## Boundary

This matrix covers only the named seed-SPEAK pilot conditions. It does not claim
seed-internal parser packets, full indentation diagnostics, complete legacy
syntax classification, source spans, or seed-native multi-file elaboration.

---

*End of BARP-VALIDATION-MATRIX.md.*
