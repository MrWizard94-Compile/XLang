# ADR-025: foreign ABI pilot (T-FFI)

**Status:** Accepted — **pilot implemented in package 0.31.0** after human residual-risk acceptance  
**Date:** 2026-08-04 (design); implement authorize 2026-08-04  
**Related Rule IDs:** `SEC-INPUT-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`, `CONST-DEP-001`

## Context

FFI is the highest ownership/hostile-input risk track (ADR-014). Design was
blocked until a human accepted residual risk and issued the authorize phrase in
[HUMAN-AUTHORIZE-FFI.md](HUMAN-AUTHORIZE-FFI.md).

## Decision

1. Adopt [DESIGN-M21-FOREIGN-ABI-PILOT.md](DESIGN-M21-FOREIGN-ABI-PILOT.md).  
2. **Implement** a Whole-only foreign weave pilot with explicit `--grant-lib KEY=PATH`.  
3. Host-side load after AETH verify; seed product path does not emit foreign yet.  
4. Scoped `unsafe` only in `crates/xlang-core/src/ffi.rs` for libloading (workspace
   `unsafe_code` is `deny`, not `forbid`, for this reason).  
5. Pure host (M8) and grant I/O (M14) remain the default host surfaces.  
6. Package **0.31.0**.  

## Human authorization (recorded)

> I authorize Aether M21 foreign ABI pilot implementation under ADR-025 /
> threat model v3 residual risk acceptance.

## Consequences

### Positive

- Bounded native interop under operator grant  
- Fail closed without grant / missing symbol  

### Costs

- Residual native-code risk on the host process  
- Bootstrap-only compile for foreign sources until seed dual-compare  
- Scoped unsafe for libloading  

### Risks

| Risk | Mitigation |
| --- | --- |
| Hostile library | Explicit path grant; residual risk documented |
| PATH search | Forbidden |
| Seed/bootstrap drift | Foreign not seed-emitted until dual-compare |

## Links

- DESIGN-M21, threat v3, HUMAN-AUTHORIZE-FFI, M21 matrix, AETHER_0.31  

---

*End of ADR-025.*
