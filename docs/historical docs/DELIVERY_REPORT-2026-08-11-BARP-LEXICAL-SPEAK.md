# Delivery Report — BARP lexical seed-SPEAK pilot

**Date:** 2026-08-11
**Status:** Delivered — full gate PASS
**Scope:** ADR-102, bounded seed-side diagnostic authority reduction
**Rule IDs:** `CONST-DEP-001`, `CONST-GATE-001`, `DOC-SYNC-001`,
`RND-INVAR-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`

---

## Delivered behavior

The checked-in Aether seed compiler now emits structured
`aether.seed-error/v1` SPEAK packets with `origin: "seed-speak"` for two
additional fail-closed lexical conditions before normal source parsing:

- `AE-SEED-003` for an ASCII tab at the start of the source or immediately
  after a source line feed;
- `AE-SEED-007` for `fn ` at the start of the source or immediately after a
  source line feed.

`AE-SEED-003` wins deterministically when both conditions occur at a source
line start. A Text literal containing the escaped characters `\\nfn ` does not
match because the pilot searches actual source line-feed boundaries.

The increment changes neither the Aether language surface, AETH format, VM,
host grants, nor default product authority. It regenerates the checked-in seed
artifact from the Aether source through bootstrap.

## Evidence

| Check | Result |
| --- | --- |
| Bootstrap check + canonical seed formatting | PASS |
| Targeted direct-forge packet regression | PASS — tabs, `fn `, priority, and escaped-literal negative |
| Targeted seed packet ABI integration test | PASS |
| Seed bootstrap versus checked-in artifact proof | PASS |
| Constitution pack integrity | PASS — version 5.0.1 (`GOV-INT-001`) |
| Full gate | PASS — `pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode full` |
| Full-gate seed identity | `BC0527A2E870C33899BEBAC64098B661946C214A5311E5EC9819C5A450522773` |

The full gate included formatting, deny-warning Clippy, all workspace tests,
32 top-level seed-versus-bootstrap example comparisons, project flows, and
bootstrap = product = forge = checked-in seed proof.

## Honesty boundary

This is a six-code seed-SPEAK pilot only:
`AE-SEED-003/004/005/006/007/012`. It does not claim complete indentation or
legacy-syntax diagnostics, packet source spans, a complete seed error-packet
matrix, or seed-native multi-file elaboration. The corresponding completeness
trackers remain `false`.

## Linked records

- [ADR-102](ADR-102-barp-seed-speak-lexical-edge-pilot.md)
- [BARP validation matrix](../Current%20state/BARP-VALIDATION-MATRIX.md)
- [BARP design](../Current%20state/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)
- [Seed Profile](../Current%20state/SEED_PROFILE.md)

---

*End of DELIVERY_REPORT-2026-08-11-BARP-LEXICAL-SPEAK.md*
