# Novel R&D

```
Document: Novel R&D
Module ID: MOD-SPC-RND-001
Version: 1.1.0
Status: Binding
Authority: AGENTS.md
Applies When: Novel architectures, frontier computational systems, exploratory invention
Dependencies:
  - AGENTS.md
  - standards/TESTING.md
  - standards/ENGINEERING.md
  - specialist/IP-AND-INVENTION.md
Overrides: None
```

---

## RND-INVAR-001 — Invariants First

When inventing non-traditional paradigms, novel AI architectures, reversible dataflow, topological engines, static-memory systems, custom attractors, or frontier work:

* First articulate mathematical / information-theoretic invariants and guarantees.
* Implement invariants as executable checks (assertions, property tests, continuous monitoring) before or alongside main logic.

## RND-CORE-001 — Realized Core Claim

* Prefer deterministic, pure, side-effect-free cores that can be exhaustively tested.
* Deliver a complete, runnable prototype or kernel that demonstrates the **core claim**.
* Never leave “the interesting part” as a comment or TODO.

## RND-DOC-001 — Production-Level Design Docs

* Document novel design decisions, failure modes, and scaling properties with production-level rigor.
* Maintain living “Core Claims & Invariants” docs (see `IP-INVENTION-001`).

## RND-SPIKE-001 — Scoped Spikes Only

* Exploratory spikes only when explicitly scoped.
* Still satisfy completeness, zero-warning, and documentation standards so they can be promoted or discarded cleanly (`CONST-DONE-002`).

---

*Canonical home for novel systems process. Patent/IP documentation: IP-AND-INVENTION.md.*
