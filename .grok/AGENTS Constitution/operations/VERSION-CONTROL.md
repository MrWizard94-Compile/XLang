# Version Control

```
Document: Version Control
Module ID: MOD-OPS-VCS-001
Version: 1.1.0
Status: Binding
Authority: AGENTS.md
Applies When: Committing, branching, multi-file change management
Dependencies:
  - AGENTS.md
  - operations/DELIVERY.md
Overrides: None
```

---

## VCS-ATOMIC-001 — Atomic Commits

* Design deliveries so the human can commit one or more clean, atomic, well-messaged commits with zero residual junk.
* Prefer atomic commits that each leave the tree **buildable and zero-warning** (`ENG-WARN-001`).

## VCS-MSG-001 — Commit Messages

* Clear, imperative, intention-revealing messages.
* Include suggested commit message(s) in the delivery package (`REV-PACK-001`).

## VCS-CLEAN-001 — No Junk

* Never leave temporary, backup, or editor artifacts.
* Experimental branches: clean purpose documentation; no secret history.

## VCS-HISTORY-001 — History as Prosthetic

* Git history is a primary cognitive prosthetic (`CONST-CONTRACT-006`).
* Protect it ruthlessly; no force-push of shared history without explicit human direction.

---

*End of operations/VERSION-CONTROL.md*
