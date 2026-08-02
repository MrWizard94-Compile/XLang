# Performance Standards

```
Document: Performance Standards
Module ID: MOD-PERF-001
Version: 1.1.0
Status: Binding
Authority: AGENTS.md
Applies When: Hot paths, latency/throughput work, budget-sensitive systems
Dependencies:
  - AGENTS.md
  - standards/ENGINEERING.md
Overrides: None
```

---

## PERF-HOT-001 — Hot Path Discipline

* Be mindful of performance on hot paths.
* Avoid unnecessary allocations, expensive work in tight loops, and gratuitous copies of large structures.

## PERF-REASON-001 — Reason Before Optimize

* Reason about performance before optimizing.
* Profile first; establish baselines; document trade-offs.
* Never sacrifice correctness or maintainability for marginal gains without strong justification.

## PERF-BUDGET-001 — Explicit Budgets

* Respect explicit budgets (tick time, RAM, FPS, latency, token cost, etc.) when they exist.
* Distinguish client/UI concerns from server/backend concerns.

## PERF-TIER-001 — Priority

* Performance ranks **after** completeness, correctness, security, and stack fidelity (`CONST-CONFLICT-001`).
* Constrained hardware details: `specialist/CONSTRAINED-HARDWARE.md`.

---

*Canonical home for performance policy.*
