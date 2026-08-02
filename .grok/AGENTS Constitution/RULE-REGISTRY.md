# Rule Registry

```
Document: Rule Registry
Module ID: MOD-GOV-REG-001
Version: 1.2.1
Status: Binding (governance)
Authority: AGENTS.md
Applies When: Creating, amending, or citing Rule IDs; audits; delivery reports
Dependencies:
  - AGENTS.md
  - PACK.md
  - LOCK.md
Overrides: None
```

**Purpose:** Exactly one registry of stable Rule IDs. No new Rule ID may be invented in a module without updating this file in the **same** delivery.

**Format:** `DOMAIN-TOPIC-NNN` (three digits). IDs are **immutable** once published; deprecate rather than reuse.

---

## Registry rules

| Rule ID | Statement |
|---------|-----------|
| **GOV-REG-001** | This file is the sole authority for which Rule IDs exist. |
| **GOV-REG-002** | Each Rule ID has exactly one **canonical home** path. Other files may cite the ID; they must not redefine it. |
| **GOV-REG-003** | Alias IDs are forbidden. Use the canonical ID only (`ENG-WARN-001`, not `CONST-WARN-001`). |
| **GOV-REG-004** | Deprecated IDs remain listed with status `Deprecated` and a successor. |

---

## Level 1 — Constitution

| Rule ID | Summary | Canonical home | Status |
|---------|---------|----------------|--------|
| CONST-GATE-001 | Unified 15-point pre-delivery gate | `AGENTS.md` | Active |
| CONST-COMPLETE-001 | No partial delivery / no stubs / no TODOs as ship | `constitution/00-CORE-LAW.md` | Active |
| CONST-DEP-001 | Dependency-first | `constitution/00-CORE-LAW.md` | Active |
| CONST-ONESHOT-001 | One-shot completeness / complete slices | `constitution/00-CORE-LAW.md` | Active |
| CONST-EMPIRE-001 | Empire-wide scope of law | `constitution/00-CORE-LAW.md` | Active |
| CONST-AUTH-001 | Four levels of law; lower never overrides higher | `constitution/01-AUTHORITY-AND-PRECEDENCE.md` | Active |
| CONST-AUTH-002 | Precedence stack | `constitution/01-AUTHORITY-AND-PRECEDENCE.md` | Active |
| CONST-CONFLICT-001 | Conflict resolution order | `constitution/01-AUTHORITY-AND-PRECEDENCE.md` | Active |
| CONST-ONEHOME-001 | One canonical home per rule | `constitution/01-AUTHORITY-AND-PRECEDENCE.md` | Active |
| CONST-AMEND-001 | Complete-file amendment process | `constitution/01-AUTHORITY-AND-PRECEDENCE.md` | Active |
| CONST-CONTRACT-001 | Human sovereign; AI owns implementation + audit | `constitution/02-HUMAN-AI-CONTRACT.md` | Active |
| CONST-CONTRACT-002 | Completeness obligation | `constitution/02-HUMAN-AI-CONTRACT.md` | Active |
| CONST-CONTRACT-003 | Priority of human change requests | `constitution/02-HUMAN-AI-CONTRACT.md` | Active |
| CONST-CONTRACT-004 | Invention mandate | `constitution/02-HUMAN-AI-CONTRACT.md` | Active |
| CONST-CONTRACT-005 | Cognitive and resource respect | `constitution/02-HUMAN-AI-CONTRACT.md` | Active |
| CONST-CONTRACT-006 | History as cognitive prosthetic | `constitution/02-HUMAN-AI-CONTRACT.md` | Active |
| CONST-DONE-001 | Machine-checkable Definition of Done | `constitution/03-DEFINITION-OF-DONE.md` | Active |
| CONST-DONE-002 | Narrow spike exception | `constitution/03-DEFINITION-OF-DONE.md` | Active |
| CONST-DONE-003 | Rationale for Done strictness | `constitution/03-DEFINITION-OF-DONE.md` | Active |

---

## Level 2 — Process

| Rule ID | Summary | Canonical home | Status |
|---------|---------|----------------|--------|
| SOP-PHASE-001 | SOP phase order is binding for product-to-delivery work | `SOP.md` | Active |
| SOP-GATE-001 | Each implement/audit/ship step cites required Rule IDs | `SOP.md` | Active |

---

## Standards

| Rule ID | Summary | Canonical home | Status |
|---------|---------|----------------|--------|
| ENG-WARN-001 | Zero warnings/errors; root-cause fix; no silent suppress | `standards/ENGINEERING.md` | Active |
| ENG-STACK-001 | Version & stack fidelity | `standards/ENGINEERING.md` | Active |
| ENG-RESEARCH-001 | Research-first when uncertain | `standards/ENGINEERING.md` | Active |
| ENG-CODE-001 | Code quality / null safety / simplicity | `standards/ENGINEERING.md` | Active |
| ENG-FW-001 | Framework & stack idioms | `standards/ENGINEERING.md` | Active |
| ENG-REPRO-001 | Reproducibility & offline-first builds | `standards/ENGINEERING.md` | Active |
| ENG-SCOPE-001 | Empire-wide engineering applicability | `standards/ENGINEERING.md` | Active |
| TEST-BEHAVIOR-001 | Tests against intended behavior | `standards/TESTING.md` | Active |
| TEST-QUALITY-001 | Meaningful tests | `standards/TESTING.md` | Active |
| TEST-DOMAIN-001 | Domain-specific test mandates | `standards/TESTING.md` | Active |
| TEST-REGRESSION-001 | No silent test weakening | `standards/TESTING.md` | Active |
| SEC-INPUT-001 | Untrusted input handling | `standards/SECURITY.md` | Active |
| SEC-PRIV-001 | Least privilege | `standards/SECURITY.md` | Active |
| SEC-SURFACE-001 | Forbidden insecure patterns | `standards/SECURITY.md` | Active |
| SEC-SECRET-001 | Secrets handling | `standards/SECURITY.md` | Active |
| SEC-LIB-001 | Prefer vetted security libraries | `standards/SECURITY.md` | Active |
| SEC-SUPPLY-001 | Supply-chain security angle | `standards/SECURITY.md` | Active |
| DOC-SYNC-001 | Docs synchronized with code | `standards/DOCUMENTATION.md` | Active |
| DOC-WHY-001 | Explain why | `standards/DOCUMENTATION.md` | Active |
| DOC-PRIORITY-001 | Prioritized documentation surfaces | `standards/DOCUMENTATION.md` | Active |
| DOC-ADR-001 | ADRs for architectural decisions | `standards/DOCUMENTATION.md` | Active |
| DOC-START-001 | Start-Here paths | `standards/DOCUMENTATION.md` | Active |
| PERF-HOT-001 | Hot-path discipline | `standards/PERFORMANCE.md` | Active |
| PERF-REASON-001 | Reason/profile before optimize | `standards/PERFORMANCE.md` | Active |
| PERF-BUDGET-001 | Explicit performance budgets | `standards/PERFORMANCE.md` | Active |
| PERF-TIER-001 | Performance ranks after higher laws | `standards/PERFORMANCE.md` | Active |
| DEP-PIN-001 | Pin dependency versions | `standards/DEPENDENCIES.md` | Active |
| DEP-MIN-001 | Minimal dependencies | `standards/DEPENDENCIES.md` | Active |
| DEP-AUDIT-001 | Ongoing dependency hygiene | `standards/DEPENDENCIES.md` | Active |
| DEP-REPRO-001 | Lockfiles & reproducible deps | `standards/DEPENDENCIES.md` | Active |
| OBS-LOG-001 | Structured logging | `standards/OBSERVABILITY.md` | Active |
| OBS-FAIL-001 | Diagnosable failure modes | `standards/OBSERVABILITY.md` | Active |
| OBS-PRIV-001 | No secrets/PII in logs | `standards/OBSERVABILITY.md` | Active |
| OBS-NOVEL-001 | Novel-system metrics | `standards/OBSERVABILITY.md` | Active |

---

## Operations

| Rule ID | Summary | Canonical home | Status |
|---------|---------|----------------|--------|
| OPS-DEL-001 | Clean delivery packages | `operations/DELIVERY.md` | Active |
| OPS-DEL-002 | Complete file replacements default | `operations/DELIVERY.md` | Active |
| OPS-DEL-003 | Quality metrics when asked | `operations/DELIVERY.md` | Active |
| OPS-DEL-004 | Cross-link only (no restate) | `operations/DELIVERY.md` | Active |
| VCS-ATOMIC-001 | Atomic green commits | `operations/VERSION-CONTROL.md` | Active |
| VCS-MSG-001 | Intention-revealing commit messages | `operations/VERSION-CONTROL.md` | Active |
| VCS-CLEAN-001 | No junk artifacts | `operations/VERSION-CONTROL.md` | Active |
| VCS-HISTORY-001 | Protect shared history | `operations/VERSION-CONTROL.md` | Active |
| REL-PACKAGE-001 | Complete release artifacts | `operations/RELEASES.md` | Active |
| REL-DETERM-001 | Deterministic release artifacts | `operations/RELEASES.md` | Active |
| REL-UPGRADE-001 | Upgrade paths documented | `operations/RELEASES.md` | Active |
| REL-HARDEN-001 | Pre-ship security/quality controls | `operations/RELEASES.md` | Active |
| REF-PLAN-001 | Plan large refactors | `operations/REFACTORING.md` | Active |
| REF-GREEN-001 | Always-green incremental refactors | `operations/REFACTORING.md` | Active |
| REF-SAFE-001 | Safe refactor patterns | `operations/REFACTORING.md` | Active |
| MIG-INV-001 | Inventory before port/migration | `operations/MIGRATIONS.md` | Active |
| MIG-FEEL-001 | Preserve original feel unless directed | `operations/MIGRATIONS.md` | Active |
| MIG-NOTES-001 | Living port notes | `operations/MIGRATIONS.md` | Active |
| MIG-VERIFY-001 | Core experience verification first | `operations/MIGRATIONS.md` | Active |
| LIFE-PHASE-001 | Respect project lifecycle phase | `operations/PROJECT-LIFECYCLE.md` | Active |
| LIFE-NODEAD-001 | No half-dead projects | `operations/PROJECT-LIFECYCLE.md` | Active |
| LIFE-TOOL-001 | Tooling / DX expectations | `operations/PROJECT-LIFECYCLE.md` | Active |
| LIFE-BIRTH-001 | Project birth via RepoForge + constitution attach | `operations/PROJECT-LIFECYCLE.md` | Active |
| LIFE-PROCESS-001 | Lifecycle vs SOP ownership split | `operations/PROJECT-LIFECYCLE.md` | Active |

---

## Collaboration

| Rule ID | Summary | Canonical home | Status |
|---------|---------|----------------|--------|
| AI-COORD-001 | Ownership before multi-agent work | `collaboration/MULTI-AGENT.md` | Active |
| AI-COORD-002 | Parallel boundaries + single integrator | `collaboration/MULTI-AGENT.md` | Active |
| AI-COORD-003 | One coherent multi-agent package | `collaboration/MULTI-AGENT.md` | Active |
| AI-COORD-004 | No parallel root SOUL / governance edits | `collaboration/MULTI-AGENT.md` | Active |
| CTX-SELF-001 | Self-contained deliveries | `collaboration/CONTEXT-SWITCHING.md` | Active |
| CTX-GRAPH-001 | Cross-project dependency graph | `collaboration/CONTEXT-SWITCHING.md` | Active |
| CTX-ASSUME-001 | No silent context assumptions | `collaboration/CONTEXT-SWITCHING.md` | Active |
| CTX-ONBOARD-001 | Onboarding path for new AIs/humans | `collaboration/CONTEXT-SWITCHING.md` | Active |
| REV-PACK-001 | Zero-friction human review packaging | `collaboration/REVIEW-PACKAGING.md` | Active |
| REV-PACK-002 | Rule ID traceability in delivery reports | `collaboration/REVIEW-PACKAGING.md` | Active |
| REV-PACK-003 | Review packaging ≠ release packaging | `collaboration/REVIEW-PACKAGING.md` | Active |
| MEM-LIVE-001 | Living knowledge base | `collaboration/INSTITUTIONAL-MEMORY.md` | Active |
| MEM-SAME-001 | Same-delivery knowledge updates | `collaboration/INSTITUTIONAL-MEMORY.md` | Active |
| MEM-FIRST-001 | Knowledge base as first-class source | `collaboration/INSTITUTIONAL-MEMORY.md` | Active |
| MEM-HEALTH-001 | Empire health signals | `collaboration/INSTITUTIONAL-MEMORY.md` | Active |

---

## Specialist

| Rule ID | Summary | Canonical home | Status |
|---------|---------|----------------|--------|
| RND-INVAR-001 | Invariants first for novel systems | `specialist/NOVEL-RND.md` | Active |
| RND-CORE-001 | Fully realized core claim | `specialist/NOVEL-RND.md` | Active |
| RND-DOC-001 | Production-level novel design docs | `specialist/NOVEL-RND.md` | Active |
| RND-SPIKE-001 | Explicitly scoped spikes only | `specialist/NOVEL-RND.md` | Active |
| IP-INVENTION-001 | Dated invention notes + executable claim | `specialist/IP-AND-INVENTION.md` | Active |
| IP-PATENT-001 | Patent hygiene / attorney-ready language | `specialist/IP-AND-INVENTION.md` | Active |
| IP-SECRET-001 | No trade secrets in public repos | `specialist/IP-AND-INVENTION.md` | Active |
| IP-DOC-001 | Invention docs meet completeness standards | `specialist/IP-AND-INVENTION.md` | Active |
| NET-PROTO-001 | Versioned robust protocols | `specialist/NETWORKING.md` | Active |
| NET-AUTH-001 | Authoritative source of truth | `specialist/NETWORKING.md` | Active |
| NET-DOC-001 | Full protocol documentation | `specialist/NETWORKING.md` | Active |
| NET-CHATTY-001 | Avoid chatty hot-path protocols | `specialist/NETWORKING.md` | Active |
| LOW-RISK-001 | Extreme care for low-level techniques | `specialist/LOW-LEVEL-SAFETY.md` | Active |
| LOW-REASON-001 | Documented reason for low-level use | `specialist/LOW-LEVEL-SAFETY.md` | Active |
| LOW-TEMP-001 | No untracked temporary low-level hacks | `specialist/LOW-LEVEL-SAFETY.md` | Active |
| HW-RESPECT-001 | Real hardware/cognitive/economic constraints | `specialist/CONSTRAINED-HARDWARE.md` | Active |
| HW-MEM-001 | Allocation discipline | `specialist/CONSTRAINED-HARDWARE.md` | Active |
| HW-ALT-001 | Document cost; provide lighter alternative | `specialist/CONSTRAINED-HARDWARE.md` | Active |
| COST-TOKEN-001 | Token/compute economics awareness | `specialist/CONSTRAINED-HARDWARE.md` | Active |

---

## Governance / pack integrity

| Rule ID | Summary | Canonical home | Status |
|---------|---------|----------------|--------|
| GOV-REG-001 | Registry is sole Rule ID authority | `RULE-REGISTRY.md` | Active |
| GOV-REG-002 | One canonical home per Rule ID | `RULE-REGISTRY.md` | Active |
| GOV-REG-003 | No alias Rule IDs | `RULE-REGISTRY.md` | Active |
| GOV-REG-004 | Deprecate; do not reuse IDs | `RULE-REGISTRY.md` | Active |
| GOV-INT-001 | Pack integrity checks must pass | `INTEGRITY.md` | Active |
| GOV-INT-002 | Required file set present | `INTEGRITY.md` | Active |
| GOV-INT-003 | All cited Rule IDs exist in registry | `INTEGRITY.md` | Active |
| GOV-INT-004 | All module manifests valid | `INTEGRITY.md` | Active |
| GOV-SYNC-001 | Mirror / multi-copy discipline | `INTEGRITY.md` | Active |
| GOV-INT-005 | Link integrity | `INTEGRITY.md` | Active |
| GOV-INT-006 | Anti-fragmentation (gate only in root) | `INTEGRITY.md` | Active |
| GOV-OVR-001 | Project override discipline | `INTEGRITY.md` | Active |
| GOV-PORT-001 | Path independence (no host absolute paths in law) | `PACK.md` | Active |
| GOV-PORT-002 | Self-contained drop-in pack | `PACK.md` | Active |
| GOV-PORT-003 | Stable identity under move/rename | `PACK.md` | Active |
| GOV-PORT-004 | Copy and mirror discipline | `PACK.md` | Active |
| GOV-PORT-005 | No host coupling in binding docs | `PACK.md` | Active |
| GOV-PORT-006 | Universal domain applicability | `PACK.md` | Active |
| GOV-LOCK-001 | No casual amendment; human + complete replace + verify | `LOCK.md` | Active |
| GOV-LOCK-002 | Non-law surfaces (reports, archive, docs, Modularize) | `LOCK.md` | Active |
| GOV-LOCK-003 | One active editor surface for amendments | `LOCK.md` | Active |
| GOV-LOCK-004 | Forbidden silent weakenings of core quality rules | `LOCK.md` | Active |
| GOV-LOCK-005 | Release checklist before lock-ready claim | `LOCK.md` | Active |

---

## Deprecated aliases (never use in new writing)

| Forbidden alias | Use instead | Notes |
|-----------------|-------------|-------|
| CONST-WARN-001 | ENG-WARN-001 | Former root shorthand |
| CONST-TEST-001 | TEST-BEHAVIOR-001 | Former root shorthand |
| CONST-DOC-001 | DOC-SYNC-001 | Former root shorthand |
| CONST-SEC-001 | SEC-INPUT-001 | Former root shorthand |
| IP-INVENTION-001 | IP-INVENTION-001 | Orphan reference fixed in v4.1 |

---

## Changelog

| Version | Date | Notes |
|---------|------|-------|
| 1.0.0 | 2026-07-27 | Initial registry (implicit across modules) |
| 1.1.0 | 2026-07-27 | Hardened: single registry, alias ban, integrity IDs |

*End of RULE-REGISTRY.md*
