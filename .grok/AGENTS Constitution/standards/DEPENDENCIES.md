# Dependency Standards

```
Document: Dependency Standards
Module ID: MOD-DEP-001
Version: 1.1.0
Status: Binding
Authority: AGENTS.md
Applies When: Adding, upgrading, or removing third-party dependencies
Dependencies:
  - AGENTS.md
  - standards/SECURITY.md
  - standards/ENGINEERING.md
Overrides: None
```

---

## DEP-PIN-001 — Pin Versions

* Pin exact versions in lockfiles.
* Avoid floating ranges in production code.
* Prefer well-maintained, widely-used libraries with clear, permissive licenses.

## DEP-MIN-001 — Minimal Dependencies

* Before adding a dependency: necessity, size, security track record, maintenance status, license compatibility.
* Prefer zero or minimal new dependencies when a few hundred lines of well-tested code suffice.
* Document why each non-trivial dependency was chosen.

## DEP-AUDIT-001 — Ongoing Hygiene

* Periodically audit known vulnerabilities and outdated packages.
* Prefer upgrades and root-cause fixes over suppressions (`ENG-WARN-001`).

## DEP-REPRO-001 — Lockfiles & Reproducibility

* Builds must be reproducible via lockfiles (`ENG-REPRO-001`).
* Do not commit secrets in dependency config.

---

*Canonical home for dependency management. Security angle cross-links SEC-SUPPLY-001.*
