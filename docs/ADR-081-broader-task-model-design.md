# ADR-081: Broader task model — handles, timeouts, parallelism (design only)

**Status:** Accepted design direction — **not implemented**  
**Date:** 2026-08-10  
**Related Rule IDs:** `DOC-ADR-001`, `CONST-DEP-001`, `RND-INVAR-001`  
**Depends on:** ADR-042 (M19e), ADR-036 (Policy B)  

## Context

Roadmap item 4: broader task cancellation / handles only through a new ADR.
M19e provides checkpoint-only active-frame cancel under AETH v12. Operators and
maturity roadmaps ask for task handles, timeouts, and optional parallelism —
each expands runtime authority and must not ship as a silent extension.

## Decision (design bounds)

1. **This ADR authorizes design only.** No opcodes, VM, seed, or CLI ship here.  
2. **Candidate surface (future vertical ADRs required):**  
   - Task **handles** as first-class values (typed, non-forgeable)  
   - Explicit **timeouts** at nursery or task-frame boundaries  
   - Optional **parallel** scheduler behind a law fork or capability gate  
3. **Invariants that must hold in any implementable follow-on:**  
   - Verify-before-run retained  
   - No ambient host capability leak  
   - VM remains reference semantics; dual-compare for claimed corpora  
   - Seed Profile honesty for any new emission  
4. **Stop conditions:** implementing handles/timeouts/parallel without a new
   implementable ADR + matrix + residual-risk table.  

## Non-goals

Implementation, new AETH version, or claiming M19e already provides handles.

## Links

- ADR-042, DESIGN-M19E, ROADMAP, ROADMAP-MAINSTREAM-MATURITY  

---

*End of ADR-081.*
