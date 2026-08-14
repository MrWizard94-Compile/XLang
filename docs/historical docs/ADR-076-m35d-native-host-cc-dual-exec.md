# ADR-076: M35d — host C toolchain dual-exec + object emit pilot

**Status:** Accepted — implemented  
**Date:** 2026-08-10  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`, `SEC-INPUT-001`  
**Law fork:** F-NATIVE (reaffirmed 2026-08-10)  
**Depends on:** ADR-059, ADR-062, ADR-073  

## Context

M35c lowered SPEAK/multi-weave to ISO C. Roadmap asked for dual-exec with host
`cc` when available and a step toward object emit — without requiring a C
toolchain on every CI host.

## Decision

1. **`native_host_cc_dual_exec(source)`:** product-compile, VM run, AETH→C, then
   if `cc`/`clang`/`gcc` is found: `cc -c` object pilot + link/run and require
   native exit == VM exit.  
2. **Best-effort:** missing C toolchain returns `cc_available: false` without
   failure (matrix remains green).  
3. **Not LLVM/object product path** — object emit is diagnostic pilot only.  
4. Tracker: `native_host_cc_dual_exec_pilot() == true`.  

## Honesty

- No guarantee of host `cc` on Windows TP machines.  
- Still verified AETH → C only; no source transpile.  

## Links

- ADR-037, ADR-073, M35A-VALIDATION-MATRIX  

---

*End of ADR-076.*
