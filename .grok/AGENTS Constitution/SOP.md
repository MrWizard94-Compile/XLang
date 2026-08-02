# Standard Operating Procedure: Project-to-Product Delivery

**Document ID:** SOP-PROD-001  
**Version:** 2.1.0  
**Effective Date:** 2026-07-27  
**Owner:** Project Lead  
**Approved By:** Management / Product Owner  
**Status:** Binding (Level 2 — Operating Law)  
**Document ID / Module ID:** MOD-SOP-PROD-001  
**Supersedes:** WORKFLOW.md; SOP 1.x–2.0.x  

This SOP is **process law**. Quality law remains **[AGENTS.md](AGENTS.md)** (Section 0 gate, non-negotiable principles) plus applicable modules. This document does not restate quality rules — it **references Rule IDs** from [RULE-REGISTRY.md](RULE-REGISTRY.md).

---

## Purpose

Define the standard process for researching reference projects, designing a new system, engineering the solution, auditing and hardening the codebase, and delivering the final product—so execution is consistent and quality-controlled.

## Scope

Applies to all teams and contributors in this workspace (product, design, engineering, QA, security, delivery). Covers research through final delivery and handoff. Post-launch support is out of scope unless separately assigned.

## References

- Empire constitution: [AGENTS.md](AGENTS.md) (Level 1)  
- Module tree: `constitution/`, `standards/`, `operations/`, `collaboration/`, `specialist/`, `templates/`  
- Secure SDLC: requirements, planning, design, implementation, testing/deployment, maintenance as connected phases  
- Project product specs: project-local `docs/` (Level 4)  

## Definitions

| Term | Meaning |
|------|---------|
| **Reference projects** | External projects studied for product direction |
| **Core components** | Building blocks: UI, domain logic, data, integrations, security |
| **Hardening** | Reducing risk via security, reliability, validation before release |
| **Final audit** | Last formal review of code, docs, and release readiness before delivery |
| **Rule ID** | Stable identifier (e.g. `ENG-WARN-001`) for audit and self-check |

## Roles and Responsibilities

| Role | Responsibility |
|------|----------------|
| **Project Lead** | Scope, priorities, approvals, final delivery |
| **Product Owner** | User value, feature priorities, acceptance criteria |
| **Architect / Lead Engineer** | Research → technical design and engineering direction |
| **Engineers** | Implement approved plan; fix issues |
| **QA / Reviewer** | Functionality, fixes, release readiness |
| **Security Reviewer** | Vulnerabilities, access, hardening |

For AI contributors under AGENTS.md: AI owns implementation detail; human is sovereign director (`CONST-CONTRACT-001`).

---

## Quality Gates (Rule IDs)

**SOP-PHASE-001** — Phases 1–10 below are the binding order for project-to-product work unless the human explicitly truncates scope.  
**SOP-GATE-001** — Every implementation increment and final delivery must satisfy at least:

| Rule ID | Meaning | Canonical home |
|---------|---------|----------------|
| `CONST-GATE-001` | Pre-delivery Section 0 checklist | AGENTS.md |
| `CONST-DONE-001` | Definition of Done | constitution/03-DEFINITION-OF-DONE.md |
| `CONST-COMPLETE-001` | No partial delivery | constitution/00-CORE-LAW.md |
| `CONST-DEP-001` | Dependency-first | constitution/00-CORE-LAW.md |
| `ENG-WARN-001` | Zero warnings | standards/ENGINEERING.md |
| `TEST-BEHAVIOR-001` | Tests vs intended behavior | standards/TESTING.md |
| `DOC-SYNC-001` | Docs match code | standards/DOCUMENTATION.md |
| `SEC-INPUT-001` | Input validation (when applicable) | standards/SECURITY.md |
| `REV-PACK-001` | Human review packaging | collaboration/REVIEW-PACKAGING.md |
| `REL-PACKAGE-001` | Release packaging (when shipping) | operations/RELEASES.md |

Load modules per AGENTS.md applicability matrix. Rule catalog: [RULE-REGISTRY.md](RULE-REGISTRY.md).

---

## Procedure

### 1. Research 10 Well-Known Projects

1.1 Identify 10 projects relevant to product direction.  
1.2 Record purpose, target users, key features, platform, strengths.  
1.3 Document relevance to this workspace.  
1.4 Store findings in a shared research log (`docs/research/`).

### 2. Decompose Projects to Core Components

2.1 Break each project into comparable categories.  
2.2 At minimum: UI, workflow logic, data model, integrations, permissions, analytics, admin.  
2.3 Note visible dependencies between components.  
2.4 Use a standard format for direct comparison.

### 3. Study Each Project in Depth

3.1 Review user flows, architecture clues, feature behavior.  
3.2 Identify strengths, weaknesses, friction, unique design choices.  
3.3 Capture cross-project patterns.  
3.4 Record insights for new system design.

### 4. Design a New System

4.1 Synthesize strongest features into a coherent product.  
4.2 Define vision, target users, core use cases.  
4.3 Prioritized feature set.  
4.4 Initial user journeys and system/trust boundaries.  
4.5 Confirm design direction before full documentation.

### 5. Write Foundational Documentation

5.1 Core docs for implementation (PRD, functional, architecture, data model, API, security, test strategy).  
5.2 Consistency with approved design (`DOC-SYNC-001`, ADRs via `templates/ADR.template.md`).  
5.3 Review/signoff before engineering (or incremental milestone approval for green-lit slices).

### 6. Plan Engineering

6.1 Convert docs into implementation plan.  
6.2 Milestones, dependencies, delivery phases.  
6.3 Definition of done per milestone (`CONST-DONE-001`).  
6.4 Risks, unknowns, required decisions.  
6.5 Plan approval before coding (or green-light per milestone).

### 7. Implement Engineering Plan

7.1 Build in prioritized increments.  
7.2 Core data model and primary workflows first.  
7.3 Auth, integrations, reporting, admin in planned phases.  
7.4 Every increment must satisfy: `ENG-WARN-001`, `TEST-BEHAVIOR-001`, `DOC-SYNC-001`, `CONST-DONE-001`, `CONST-GATE-001`.  
7.5 Track milestones; update stakeholders.  
7.6 Load `standards/SECURITY.md` for any untrusted input or auth work (`SEC-INPUT-001`).

### 8. Audit Codebase and Apply Fixes

8.1 Functional defects, security issues, architecture drift.  
8.2 Automated tests, linting, dependency checks, security scans.  
8.3 Triage by severity.  
8.4 Fix and re-test.  
8.5 Document audit results (`templates/AUDIT.template.md`).

### 9. Harden Codebase

9.1 Reduce attack surface (`SEC-*`).  
9.2 Validation, error handling, logging, observability (`OBSERVABILITY.md`).  
9.3 Authorization, secrets, deployment controls (`SEC-SECRET-001`).  
9.4 Performance and reliability weak points (`PERFORMANCE.md`).  
9.5 Confirm release-ready stability.

### 10. Final Audit, Fix Pass, and Delivery

10.1 Final review of code, docs, release package.  
10.2 Critical findings resolved or formally accepted.  
10.3 Release readiness, rollback, handoff materials (`REL-PACKAGE-001`, `REV-PACK-001`).  
10.4 Deliver to approved channel.  
10.5 Archive documentation; record completion.  
10.6 Delivery report: `templates/DELIVERY-REPORT.template.md` with Rule ID self-audit.

---

## Inputs and Outputs

### Inputs

- Project goals and scope  
- Research sources  
- Stakeholder requirements  
- Technical constraints  
- Security and compliance requirements  

### Outputs

- Research summary  
- Component decomposition matrix  
- Deep-dive notes  
- System design proposal  
- Foundational documentation set  
- Engineering plan  
- Implemented codebase  
- Audit and hardening reports  
- Final release package (MANIFEST + delivery report)

---

## Quality Controls

- Major steps produce written outputs.  
- Reviews before phase advance (or explicit green-light for a milestone).  
- Security and quality checks required before delivery.  
- Changes traceable to findings, decisions, or requirements.  
- **Rule IDs** (not vague “follow AGENTS”) bind each increment.  
- **Git:** commit after discrete changes (`VCS-ATOMIC-001`); push at end of work turn when directed.  

---

## Project baseline (optional Level 4)

Individual projects may maintain a short resumption table (complete vs next) in local docs. That is **project law**, not SOP text.

---

## Revision History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-07-27 | Initial SOP (as WORKFLOW.md content). |
| 1.0.1 | 2026-07-27 | Renamed to SOP.md; cleaned structure; linked AGENTS.md; early project baseline. |
| 1.0.2 | 2026-07-27 | Linked modular AGENTS modules/ (v3 flat layout). |
| 2.0.0 | 2026-07-27 | Level-2 process law under v4 four-level constitution; Rule ID gates; template references. |
| 2.0.1 | 2026-07-27 | Universal human-workspace pack: remove project-specific baseline notes; supersede WORKFLOW.md. |
| 2.1.0 | 2026-07-27 | Hardened: SOP-PHASE-001 / SOP-GATE-001; registry linkage; pack integrity alignment. |

## Approval

- **Prepared By:** Project Lead  
- **Reviewed By:** Architect / Lead Engineer  
- **Approved By:** Product Owner / Management  

---

*End of SOP-PROD-001*
