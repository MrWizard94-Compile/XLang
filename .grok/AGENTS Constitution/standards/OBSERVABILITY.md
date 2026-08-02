# Observability Standards

```
Document: Observability Standards
Module ID: MOD-OBS-001
Version: 1.1.0
Status: Binding
Authority: AGENTS.md
Applies When: Non-trivial subsystems, failure modes, production diagnostics
Dependencies:
  - AGENTS.md
  - standards/SECURITY.md
Overrides: None
```

---

## OBS-LOG-001 — Structured Logging

* Non-trivial subsystems expose clear logging at appropriate levels.
* Prefer structured (key-value or JSON) logging when the platform supports it.
* Respect logger hierarchies; avoid spam on hot paths; provide debug toggles where useful.

## OBS-FAIL-001 — Diagnosable Failures

* Critical failure modes must be diagnosable from logs + clear context.
* Prefer fail-fast with clear messages over silent corruption.
* When introducing new failure modes, document them and the recovery path in the same delivery.

## OBS-PRIV-001 — No Sensitive Logs

* Never log secrets, tokens, or PII (`SEC-SECRET-001`).

## OBS-NOVEL-001 — Novel Systems Metrics

* For novel systems: log key metrics (norms, energy, convergence, invariant violations) at appropriate frequency.
* Detail of novel R&D process: `specialist/NOVEL-RND.md`.

---

*Canonical home for logging, diagnostics, and failure-mode observability.*
