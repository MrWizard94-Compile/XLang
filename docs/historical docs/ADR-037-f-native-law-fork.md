# ADR-037: law fork F-NATIVE (optional native backend)

**Status:** **Accepted** 2026-08-10 — fork open; first product slice ADR-059 (M35a)
**Date:** 2026-08-05 (authorized 2026-08-10)
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `SEC-INPUT-001`, `CONST-CONTRACT-001`
**Related:** CLM-010, ADR-014 T-NATIVE, DESIGN-LAW-FORK-F-NATIVE, HUMAN-AUTHORIZE-NATIVE

## Context

Maturity roadmap performance/deploy goals may require native deploy. Current law
keeps AETH-VM-only execution and prohibits source transpile (CLM-010).

## Decision (conditional)

1. Adopt [DESIGN-LAW-FORK-F-NATIVE.md](../Current%20state/DESIGN-LAW-FORK-F-NATIVE.md) as the fork shape.
2. **Authorized** via [HUMAN-AUTHORIZE-NATIVE.md](../Current%20state/HUMAN-AUTHORIZE-NATIVE.md) §3
   (2026-08-10).
3. Only **verified AETH → native**; VM remains default + reference;
   no Aether-source → C/Rust/JS product compiler.
4. Threat [THREAT_MODEL-v4-NATIVE-BACKEND.md](../Current%20state/THREAT_MODEL-v4-NATIVE-BACKEND.md)
   is **binding** for native slices.
5. First vertical slice: [ADR-059](ADR-059-f-native-authorized-m35a-aeth-to-c.md) M35a
   verified AETH → ISO C pure pilot.

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
