# ADR-023: deeper resource↔effect requires destruction model first

**Status:** Accepted (direction) — **implementation deferred**  
**Date:** 2026-08-04  
**Related:** [ADR-020](ADR-020-m16-resource-effect.md), [DESIGN-M19](DESIGN-M19-DEEPER-RESOURCE-EFFECT.md)

## Decision

1. Do **not** implement raise/forward+resource or nursery+resource in this cycle.  
2. Adopt DESIGN-M19: next implementable foundation is explicit resource end-of-life
   (`release` or equivalent) before abortive/resource composition.  
3. M16 handle-over-live-resources remains the proven T-RX pilot.

## Consequences

Honest deferral; agents must not “just add raise in main with arena.”

---

*End of ADR-023.*
