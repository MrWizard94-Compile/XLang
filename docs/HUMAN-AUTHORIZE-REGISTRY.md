# Human authorization checklist — F-REGISTRY (package registry law fork)

**Status:** Decision aid — **not authorized** (no product registry network)  
**Date:** 2026-08-05  

**Law:** [ADR-038](ADR-038-f-registry-law-fork.md), [DESIGN-LAW-FORK-F-REGISTRY.md](DESIGN-LAW-FORK-F-REGISTRY.md), [THREAT_MODEL-v5-PACKAGE-REGISTRY.md](THREAT_MODEL-v5-PACKAGE-REGISTRY.md)  
**Constitution:** Agents **must not** add product network package fetch until
section 3 is completed by a human in writing.

---

## 1. Why this exists

Default law is offline path/workspace packages only (M18/M22). A registry
introduces host network and supply-chain risk. That is a **human** decision.

## 2. Residual risk (must accept)

| ID | Risk | Accept? |
| --- | --- | --- |
| G1 | Host CLI gains a network client | ☐ |
| G2 | Compromised keys / bad packages if verify fails open | ☐ |
| G3 | Cache poisoning without digest binding | ☐ |
| G4 | “Offline-first” must remain true when cache is complete | ☐ |
| G5 | Guest AETH still has **no** network | ☐ |

## 3. Law-fork authorization phrase

A human must write **exactly** (or clearly equivalent):

> **I authorize Aether law fork F-REGISTRY** under ADR-038 / threat model v5 residual risk acceptance: signed offline-first registry with optional explicit fetch, guest AETH remains network-free, compile never requires network when cache is complete.

Until that phrase appears, agents must not:

- add registry fetch/publish product commands  
- open sockets from the CLI for packages  
- weaken “no network” claims without DOC-SYNC  

## 4. After §3 — still not free-form implement

Authorization opens the fork. A **vertical ADR** (M24-class) + matrix is still
required before product code (`CONST-DEP-001`).

## 5. Explicit non-authorization

“Continue”, “do law forks”, or “implement registry” **without** the §3 phrase
are **not** F-REGISTRY implement authorization.

---

*End of HUMAN-AUTHORIZE-REGISTRY.md*
