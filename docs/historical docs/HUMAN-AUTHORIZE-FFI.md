# Human authorization checklist — M21 foreign ABI (T-FFI)

**Status:** Decision aid — pilot **authorized and implemented** in package 0.31.0
**Date:** 2026-08-04

**Law:** [ADR-025](ADR-025-m21-foreign-abi-pilot.md), [THREAT_MODEL-v3-FOREIGN-ABI.md](../Current%20state/THREAT_MODEL-v3-FOREIGN-ABI.md), [DESIGN-M21](DESIGN-M21-FOREIGN-ABI-PILOT.md)
**Constitution:** Agents **must not** implement FFI until this checklist is
explicitly completed by a human in writing (chat or commit message is enough).

---

## 1. Why this exists

ADR-025 blocks product foreign weaves / libloading until a human accepts residual
risk after threat review. This file is the **accept gate**, not an implement go
by itself until section 3 is signed.

## 2. Residual risk (must accept)

| ID | Risk | Accept? |
| --- | --- | --- |
| R1 | Operator-granted `.dll`/`.so` runs native code in the host process | ☐ |
| R2 | Hostile native code can corrupt the host (not sandboxed by AETH) | ☐ |
| R3 | No claim of memory safety for foreign code | ☐ |
| R4 | Explicit library path grant only (no PATH search) | ☐ |
| R5 | Pinned symbol signatures in source (no C header parser v1) | ☐ |

## 3. Implement authorization phrase

A human must write **exactly** (or clearly equivalent):

> **I authorize Aether M21 foreign ABI pilot implementation** under ADR-025 /
> threat model v3 residual risk acceptance.

Until that phrase appears, agents must keep FFI code out of the product path.

## 4. First implementable slice (only after §3)

1. Design freeze unchanged (DESIGN-M21 forms).
2. Host-only load after AETH verify; no seed-path FFI.
3. Copy-primitive pilot only (e.g. Whole/Text extent style).
4. Negative tests: missing grant, bad path, missing symbol.
5. DOC-SYNC + no “safe FFI” marketing claims.

## 5. Explicit non-authorization

Requests such as “continue deferred”, “implement everything”, or “do FFI next”
**without** the §3 phrase are **not** implement authorization.

---

*End of HUMAN-AUTHORIZE-FFI.md*
