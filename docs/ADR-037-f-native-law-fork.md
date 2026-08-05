# ADR-037: law fork F-NATIVE (optional native backend)

**Status:** Proposed / awaiting human authorization — **no product code**  
**Date:** 2026-08-05  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `SEC-INPUT-001`, `CONST-CONTRACT-001`  
**Related:** CLM-010, ADR-014 T-NATIVE, DESIGN-LAW-FORK-F-NATIVE, HUMAN-AUTHORIZE-NATIVE  

## Context

Maturity roadmap performance/deploy goals may require native deploy. Current law
keeps AETH-VM-only execution and prohibits source transpile (CLM-010).

## Decision (conditional)

1. Adopt [DESIGN-LAW-FORK-F-NATIVE.md](DESIGN-LAW-FORK-F-NATIVE.md) as the fork shape.  
2. **Do not implement** native product paths until
   [HUMAN-AUTHORIZE-NATIVE.md](HUMAN-AUTHORIZE-NATIVE.md) §3 is signed.  
3. If authorized: only **verified AETH → native**; VM remains default + reference;
   no Aether-source → C/Rust/JS product compiler.  
4. Threat draft [THREAT_MODEL-v4-NATIVE-BACKEND.md](THREAT_MODEL-v4-NATIVE-BACKEND.md)
   becomes binding when authorized.  
5. First vertical slice still needs its own ADR + matrix after the fork is open.

## Consequences

### If rejected (default)

- Stay VM-only; optional future JIT under separate ADR without full F-NATIVE  

### If accepted

- Law rewrites in AGENTS / CORE_CLAIMS / MANIFEST  
- Higher residual risk and toolchain complexity  
- Dual-run discipline mandatory  

## Links

- DESIGN-LAW-FORK-F-NATIVE, HUMAN-AUTHORIZE-NATIVE, threat v4  

---

*End of ADR-037.*
