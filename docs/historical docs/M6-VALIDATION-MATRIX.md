# M6 explicit layout shapes validation matrix

**Status:** Implementation gate for Aether 0.9 / M6 — satisfied by
[DELIVERY_REPORT-2026-08-03-M6-EXPLICIT-LAYOUT.md](DELIVERY_REPORT-2026-08-03-M6-EXPLICIT-LAYOUT.md)

**Date:** 2026-08-03

**Design:** [M6 explicit layout shapes](DESIGN-M6-EXPLICIT-LAYOUT-SHAPES.md)

## Invariants under test

| Invariant | Semantic evidence | Product evidence |
| --- | --- | --- |
| M6-INV-001 shape product | Pure model accepts only Whole field products in order. | Parser/formatter/authoring round-trip shape declarations. |
| M6-INV-002 explicit layout | Model requires rows or columns keyword. | Missing/unknown layout rejected with `AE-LAYOUT-001`. |
| M6-INV-003 dual-layout equivalence | Same put/get script yields identical cells for both layouts. | Dual fixtures exit identically; loads match. |
| M6-INV-004 arena bounds | Capacity and byte reservation are closed. | Over-capacity allocate dims; capacity 0/1025 rejected. |
| M6-INV-005 shape analysis | Wrong field/shape rejected in pure model. | `AE-LAYOUT-002` for unknown field or unknown shape. |
| M6-INV-006 resource boundary | Tables cannot escape pure model boundary. | No table weave results; effect boundary rejects live tables. |
| M6-INV-007 v9 provenance | Layout tag present in artifact model. | Decoder/verifier accept table ops only in v9. |
| M6-INV-008 seed parity | Canonical corpus deterministic. | Seed/bootstrap byte identity + self-host rebuild. |
| M6-INV-009 methodology | Harness definition fixed. | Delivery report records metrics without overclaim. |

## Required corpus

| Area | Positive | Negative / hostile |
| --- | --- | --- |
| Shape | 1 and 8 Whole fields; canonical format. | Empty shape; non-Whole field; nested shape; duplicate field. |
| Table | rows and columns allocate/store/load. | Unknown layout; unknown shape; store wrong field. |
| Equivalence | Shared script on both layouts. | — |
| Capacity | 1 and 1024. | 0, 1025, arena exhaustion. |
| AETH | Deterministic v9 emission. | Table opcode under v8; truncated immediate. |
| Seed | Shipped example dual-compare. | No false diagnostic-parity claim. |
| Performance | Fixed 256-element harness both layouts. | Benefit claim without numbers. |

## Acceptance evidence

Constitution gate commands, test counts, seed hashes, dual-layout run results,
harness metrics, and known limits must appear in the delivery report. Happy-path
`store`/`load` alone is not M6 acceptance.
