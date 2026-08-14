# ADR-038: law fork F-REGISTRY (optional package registry)

**Status:** **Accepted** 2026-08-10 — fork open; first product slice ADR-060 (M24a)
**Date:** 2026-08-05 (authorized 2026-08-10)
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `SEC-INPUT-001`, `CONST-CONTRACT-001`
**Related:** CLM-003, CLM-004, CLM-027, ADR-014 T-PKG, DESIGN-LAW-FORK-F-REGISTRY, HUMAN-AUTHORIZE-REGISTRY

## Context

M18/M22 provide offline multi-package graphs without a network registry. Peer
ecosystems often need a registry. Default Aether law forbids product network.

## Decision (conditional)

1. Adopt [DESIGN-LAW-FORK-F-REGISTRY.md](../Current%20state/DESIGN-LAW-FORK-F-REGISTRY.md) as the fork shape.
2. **Authorized** via [HUMAN-AUTHORIZE-REGISTRY.md](../Current%20state/HUMAN-AUTHORIZE-REGISTRY.md) §3
   (2026-08-10).
3. Signed offline-first registry direction; explicit fetch only (later); guest AETH
   remains network-free; compile offline when cache complete.
4. Threat [THREAT_MODEL-v5-PACKAGE-REGISTRY.md](../Current%20state/THREAT_MODEL-v5-PACKAGE-REGISTRY.md)
   is **binding** for registry slices.
5. First vertical slice: [ADR-060](ADR-060-f-registry-authorized-m24a-offline-cache.md)
   M24a offline digest-bound cache (no network).

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
