# Documentation Standards

```
Document: Documentation Standards
Module ID: MOD-DOC-001
Version: 1.1.0
Status: Binding
Authority: AGENTS.md
Applies When: Any public API, design decision, or non-obvious logic changes
Dependencies:
  - AGENTS.md
  - constitution/03-DEFINITION-OF-DONE.md
Overrides: None
```

---

## DOC-SYNC-001 — Synchronized Documentation

* Documentation must match the code exactly after every delivery.
* Document public APIs, complex internals, non-obvious behavior, and important design decisions.

## DOC-WHY-001 — Explain Why

* Docs explain **why** something exists, the problem it solves, intended use, invariants/guarantees, and known limits or trade-offs.
* Avoid over-documenting the trivial; focus where long-term value multiplies.

## DOC-PRIORITY-001 — Prioritized Surfaces

Prioritize documentation for:

* Extension points and SPIs  
* Configuration mechanisms  
* Custom protocols and serialization formats  
* Complex subsystems  
* Novel inventions (link `specialist/IP-AND-INVENTION.md` and `NOVEL-RND.md`)

## DOC-ADR-001 — Architectural Decisions

* Material architectural choices use ADRs (template: `templates/ADR.template.md`).
* When a delivery changes architectural assumptions, update knowledge-base docs in the **same** delivery (`collaboration/INSTITUTIONAL-MEMORY.md`).

## DOC-START-001 — Start-Here Paths

* Major projects provide a short Start Here path: root AGENTS pointer + local DESIGN / AGENTS files.

---

*Canonical home for documentation sync and standards. Institutional memory process lives in collaboration/INSTITUTIONAL-MEMORY.md.*
