# ADR-038: law fork F-REGISTRY (optional package registry)

**Status:** Proposed / awaiting human authorization — **no product code**  
**Date:** 2026-08-05  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `SEC-INPUT-001`, `CONST-CONTRACT-001`  
**Related:** CLM-003, CLM-004, CLM-027, ADR-014 T-PKG, DESIGN-LAW-FORK-F-REGISTRY, HUMAN-AUTHORIZE-REGISTRY  

## Context

M18/M22 provide offline multi-package graphs without a network registry. Peer
ecosystems often need a registry. Default Aether law forbids product network.

## Decision (conditional)

1. Adopt [DESIGN-LAW-FORK-F-REGISTRY.md](DESIGN-LAW-FORK-F-REGISTRY.md) as the fork shape.  
2. **Do not implement** registry network until
   [HUMAN-AUTHORIZE-REGISTRY.md](HUMAN-AUTHORIZE-REGISTRY.md) §3 is signed.  
3. If authorized: signed offline-first registry; explicit fetch only; guest AETH
   remains network-free; compile offline when cache complete.  
4. Threat draft [THREAT_MODEL-v5-PACKAGE-REGISTRY.md](THREAT_MODEL-v5-PACKAGE-REGISTRY.md)
   becomes binding when authorized.  
5. First vertical slice still needs its own ADR + matrix after the fork is open.

## Consequences

### If rejected (default)

- Path/workspace packages only; strongest local-first claim  

### If accepted

- Host CLI network surface + supply-chain controls  
- Claim refinements for “no network” language  

## Links

- DESIGN-LAW-FORK-F-REGISTRY, HUMAN-AUTHORIZE-REGISTRY, threat v5  

---

*End of ADR-038.*
