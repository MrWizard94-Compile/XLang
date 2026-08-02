# Migrations and Ports

```
Document: Migrations and Ports
Module ID: MOD-OPS-MIG-001
Version: 1.1.0
Status: Binding
Authority: AGENTS.md
Applies When: Porting, migrating, or modernizing systems
Dependencies:
  - AGENTS.md
  - standards/ENGINEERING.md
  - standards/TESTING.md
  - operations/REFACTORING.md
Overrides: None
```

---

## MIG-INV-001 — Inventory First

* Produce a complete inventory of APIs, contracts, mixins, data, and behavioral differences.
* Map old → new with explicit documentation of semantic differences.

## MIG-FEEL-001 — Preserve Feel

* Preserve original “feel” and balance unless the human explicitly wants modernization.
* Prefer incremental, testable slices over big-bang rewrites.

## MIG-NOTES-001 — Living Port Notes

* Maintain a living “Port / Migration Notes” document for non-obvious decisions and remaining risks.
* Clean, zero-warning builds at each major milestone.

## MIG-VERIFY-001 — Core Experience First

* After the port builds cleanly, prioritize runtime verification of the core experience before secondary features.
* Never leave “port debt” without a tracked removal plan.

---

*End of operations/MIGRATIONS.md*
