# ADR-056: BARP — host elaborate + seed emit multi-module contract

**Status:** Accepted — implemented (contract + honesty trackers)
**Date:** 2026-08-10
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `RND-INVAR-001`
**Program:** [ADR-043](ADR-043-bootstrap-authority-reduction.md) / [DESIGN-BARP-001](../Current%20state/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md)
**Depends on:** ADR-015, ADR-047, ADR-055

## Context

Full **seed-native multi-module elaboration** would require the seed forge
compiler to load multiple files. The forge ABI is
`compile [borrow source: Text] -> Bytes` with a single Text — no multi-file
host read capability is granted to seed. Therefore seed-native multi-file
elaboration is **not** available without a new multi-file forge ABI (future ADR).

Product multi-module path already: **host elaborates** the import graph into one
source, then **seed emits** bytecode (ADR-047).

## Decision

1. **Contract (lawful product multi-module path):** host elaborate → seed emit.
2. **Trackers:**
   - `host_elaborates_modules_seed_emits() == true`
   - `seed_native_multi_module_elaboration() == false` (honest)
3. **Single-file product** rejects raw `import unit` with `AE-SEED-012`
   (ADR-055).
4. **Do not claim** seed multi-file elaboration.

## Future (not this ADR)

Multi-file forge ABI or host callback for unit loads — requires new design +
threat model + dual-compare.

## Links

- ADR-015, ADR-047, ADR-055

---

*End of ADR-056.*
