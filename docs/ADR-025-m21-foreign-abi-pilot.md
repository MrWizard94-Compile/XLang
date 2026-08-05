# ADR-025: foreign ABI pilot is design-gated (T-FFI)

**Status:** Accepted (design direction) — **implementation blocked**  
**Date:** 2026-08-04  

## Decision

1. Adopt draft threat model v3 and DESIGN-M21 as the FFI direction.  
2. **Do not implement** foreign weaves/libloading until a human explicitly
   authorizes implementation after reviewing threat residual risk — use the
   checklist and exact phrase in [HUMAN-AUTHORIZE-FFI.md](HUMAN-AUTHORIZE-FFI.md).  
3. Pure host (M8) and grant I/O (M14) remain the only host call surfaces.
   Requests to “continue deferred” are **not** implement authorization.

## Rationale

FFI is the highest ownership/hostile-input risk track (ADR-014). Design without
code preserves Constitution gates.

---

*End of ADR-025.*
