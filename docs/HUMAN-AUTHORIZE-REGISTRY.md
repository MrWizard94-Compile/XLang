# Human authorization checklist — F-REGISTRY (package registry law fork)

**Status:** **Authorized** 2026-08-10 (human §3 phrase accepted as clearly equivalent;
residual-risk bounds of the full §3 text remain **binding**)  
**Date:** 2026-08-05 (authorized 2026-08-10)  

**Law:** [ADR-038](ADR-038-f-registry-law-fork.md), [ADR-060](ADR-060-f-registry-authorized-m24a-offline-cache.md), [DESIGN-LAW-FORK-F-REGISTRY.md](DESIGN-LAW-FORK-F-REGISTRY.md), [THREAT_MODEL-v5-PACKAGE-REGISTRY.md](THREAT_MODEL-v5-PACKAGE-REGISTRY.md)  
**Constitution:** Product registry paths require this authorization plus a vertical ADR + matrix.

---

## 1. Why this exists

Default law is offline path/workspace packages only (M18/M22). A registry
introduces host network and supply-chain risk. That is a **human** decision.

## 2. Residual risk (accepted with authorization)

| ID | Risk | Accept? |
| --- | --- | --- |
| G1 | Host CLI gains a network client | ☑ (fetch not in M24a) |
| G2 | Compromised keys / bad packages if verify fails open | ☑ |
| G3 | Cache poisoning without digest binding | ☑ |
| G4 | “Offline-first” must remain true when cache is complete | ☑ |
| G5 | Guest AETH still has **no** network | ☑ |

## 3. Law-fork authorization phrase

A human must write **exactly** (or clearly equivalent):

> **I authorize Aether law fork F-REGISTRY** under ADR-038 / threat model v5 residual risk acceptance: signed offline-first registry with optional explicit fetch, guest AETH remains network-free, compile never requires network when cache is complete.

**Recorded authorization (2026-08-10):** human wrote `I authorize Aether law fork F-REGISTRY`
(clearly equivalent intent); full residual-risk bounds above are enforced as fork law.

## 4. After §3 — still not free-form implement

Authorization opens the fork. Vertical ADR + matrix still required (`CONST-DEP-001`).
**M24a** (ADR-060): offline digest-bound cache pin/verify (no network) is the first product slice.

## 5. Explicit non-authorization

“Continue”, “do law forks”, or “implement registry” **without** the §3 phrase
are **not** F-REGISTRY implement authorization.

---

*End of HUMAN-AUTHORIZE-REGISTRY.md*
