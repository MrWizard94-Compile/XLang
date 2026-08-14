# ADR-023: deeper resource↔effect requires destruction model first

**Status:** Accepted (direction) — foundation **M19a/M19b Policy A shipped**; Policy B deferred  
**Date:** 2026-08-04  
**Related:** [ADR-020](ADR-020-m16-resource-effect.md), [DESIGN-M19](DESIGN-M19-DEEPER-RESOURCE-EFFECT.md), [ADR-027](ADR-027-m19a-explicit-release.md), [ADR-028](ADR-028-m19b-nursery-resource.md)

## Decision

1. Do **not** implement free-on-raise or Policy B cancel-destroy without a dedicated ADR.  
2. Adopt DESIGN-M19: explicit resource end-of-life (`release`) before abortive/resource
   composition — **done in M19a**. Nursery×resource Policy A is **done in M19b**.  
3. M16 handle-over-live-resources remains the proven T-RX pilot.

## Consequences

Honest deferral; agents must not “just add raise in main with arena.”

---

*End of ADR-023.*
