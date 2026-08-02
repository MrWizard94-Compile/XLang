# Module Index

```
Document: Module Index
Module ID: MOD-GOV-IDX-001
Version: 1.2.1
Status: Binding (governance)
Authority: AGENTS.md
Applies When: Loading modules; amending pack structure; verifying pack integrity
Dependencies:
  - AGENTS.md
  - PACK.md
  - RULE-REGISTRY.md
  - INTEGRITY.md
Overrides: None
```

Every Level-3+ document has a stable **Module ID** (`MOD-…-001` form). Adding a module requires updating this index, `AGENTS.md` folder map, and `INTEGRITY.md` required-file list in the same delivery.

**Pack version:** see `VERSION` (currently **5.0.1** LOCKED).

---

## Always-load set

| Path | Module ID | Version |
|------|-----------|---------|
| `AGENTS.md` | MOD-CONST-ROOT-001 | see file |
| `SOP.md` | MOD-SOP-PROD-001 | see file |
| `constitution/03-DEFINITION-OF-DONE.md` | MOD-CONST-DONE-001 | 1.1.0 |
| `standards/ENGINEERING.md` | MOD-ENG-001 | 1.1.0 |
| `standards/TESTING.md` | MOD-TEST-001 | 1.1.0 |
| `standards/DOCUMENTATION.md` | MOD-DOC-001 | 1.1.0 |

---

## Governance

| Path | Module ID |
|------|-----------|
| `VERSION` | pack semver stamp |
| `PACK.md` | MOD-GOV-PACK-001 |
| `LOCK.md` | MOD-GOV-LOCK-001 |
| `ADOPT.md` | MOD-GOV-ADOPT-001 |
| `RULE-REGISTRY.md` | MOD-GOV-REG-001 |
| `MODULE-INDEX.md` | MOD-GOV-IDX-001 |
| `INTEGRITY.md` | MOD-GOV-INT-001 |
| `Modularize.md` | MOD-GOV-DESIGN-001 (advisory design history) |
| `README.md` | MOD-GOV-README-001 (entry map) |

---

## Constitution detail

| Path | Module ID |
|------|-----------|
| `constitution/00-CORE-LAW.md` | MOD-CONST-CORE-001 |
| `constitution/01-AUTHORITY-AND-PRECEDENCE.md` | MOD-CONST-AUTH-001 |
| `constitution/02-HUMAN-AI-CONTRACT.md` | MOD-CONST-CONTRACT-001 |
| `constitution/03-DEFINITION-OF-DONE.md` | MOD-CONST-DONE-001 |

---

## Standards

| Path | Module ID | Load trigger |
|------|-----------|--------------|
| `standards/ENGINEERING.md` | MOD-ENG-001 | Always |
| `standards/TESTING.md` | MOD-TEST-001 | Always |
| `standards/DOCUMENTATION.md` | MOD-DOC-001 | Always |
| `standards/SECURITY.md` | MOD-SEC-001 | Untrusted input, auth, secrets, ship |
| `standards/PERFORMANCE.md` | MOD-PERF-001 | Hot paths, budgets |
| `standards/DEPENDENCIES.md` | MOD-DEP-001 | Add/upgrade/remove deps |
| `standards/OBSERVABILITY.md` | MOD-OBS-001 | Failure modes, production diag |

---

## Operations

| Path | Module ID | Load trigger |
|------|-----------|--------------|
| `operations/DELIVERY.md` | MOD-OPS-DEL-001 | Presenting work |
| `operations/VERSION-CONTROL.md` | MOD-OPS-VCS-001 | Commits, branches |
| `operations/RELEASES.md` | MOD-OPS-REL-001 | Ship / distribute |
| `operations/REFACTORING.md` | MOD-OPS-REF-001 | Large refactors |
| `operations/MIGRATIONS.md` | MOD-OPS-MIG-001 | Ports / migrations |
| `operations/PROJECT-LIFECYCLE.md` | MOD-OPS-LIFE-001 | Birth / archive / tooling DX |

---

## Collaboration

| Path | Module ID | Load trigger |
|------|-----------|--------------|
| `collaboration/MULTI-AGENT.md` | MOD-COL-MA-001 | Multiple agents |
| `collaboration/CONTEXT-SWITCHING.md` | MOD-COL-CTX-001 | Multi-project / fresh context |
| `collaboration/REVIEW-PACKAGING.md` | MOD-COL-REV-001 | Any human handoff |
| `collaboration/INSTITUTIONAL-MEMORY.md` | MOD-COL-MEM-001 | Knowledge base / health |

---

## Specialist

| Path | Module ID | Load trigger |
|------|-----------|--------------|
| `specialist/NOVEL-RND.md` | MOD-SPC-RND-001 | Novel architectures |
| `specialist/IP-AND-INVENTION.md` | MOD-SPC-IP-001 | Invention / patent hygiene |
| `specialist/NETWORKING.md` | MOD-SPC-NET-001 | Protocols / multiplayer |
| `specialist/LOW-LEVEL-SAFETY.md` | MOD-SPC-LOW-001 | Unsafe / mixins / bytecode |
| `specialist/CONSTRAINED-HARDWARE.md` | MOD-SPC-HW-001 | RAM/CPU/GPU/cost constraints |

---

## Templates

| Path | Module ID |
|------|-----------|
| `templates/MANIFEST.template.md` | MOD-TPL-MANIFEST-001 |
| `templates/ADR.template.md` | MOD-TPL-ADR-001 |
| `templates/AUDIT.template.md` | MOD-TPL-AUDIT-001 |
| `templates/DELIVERY-REPORT.template.md` | MOD-TPL-DELIVERY-001 |
| `templates/PROJECT-OVERRIDE.template.md` | MOD-TPL-OVERRIDE-001 |
| `templates/PROJECT-POINTER.template.md` | MOD-TPL-POINTER-001 |

---

## Tools

| Path | Purpose |
|------|---------|
| `tools/verify-pack.ps1` | Automated pack integrity + portability checks (`GOV-INT-*`, `GOV-PORT-001`) |
| `tools/self-audit.ps1` | Run the pack against itself (`CONST-GATE-001` meta-audit → `reports/SELF-AUDIT-latest.md`) |
| `tools/run-full-constitution-self.ps1` | Full constitution + SOP self-run → `reports/FULL-CONSTITUTION-SELF-RUN-latest.md` |
| `tools/write-checksums.ps1` | Portable SHA256 inventory → `reports/CHECKSUMS.sha256` |

---

*End of MODULE-INDEX.md*
