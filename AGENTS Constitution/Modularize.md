# Modularize — Design Spec for the AGENTS Constitution

**Status:** Design history only (advisory) — live pack is **v5.0.1 LOCKED**  
**Date:** 2026-07-28  
**Scope:** Reusable empire-wide `AGENTS Constitution` folder  
**Not binding law** — see `LOCK.md` (`GOV-LOCK-002`)

---

## Problem

Right now, `AGENTS.md` is doing several jobs at once:

* constitution  
* coding standard  
* delivery checklist  
* security policy  
* release policy  
* multi-agent coordination guide  
* project lifecycle handbook  
* invention/IP policy  
* human–AI collaboration contract  

That breadth gives it authority, but it also makes it heavy to load, harder to maintain, and easier for important rules to get buried. The SOP already demonstrates the cleaner pattern: one document owns process, while `AGENTS.md` owns quality law.

---

## The best modular model

Keep one short root `AGENTS.md` as the constitutional entry point, then move domain-specific rules into modules.

```
AGENTS Constitution/
│
├── AGENTS.md
├── SOP.md
│
├── constitution/
│   ├── 00-CORE-LAW.md
│   ├── 01-AUTHORITY-AND-PRECEDENCE.md
│   ├── 02-HUMAN-AI-CONTRACT.md
│   └── 03-DEFINITION-OF-DONE.md
│
├── standards/
│   ├── ENGINEERING.md
│   ├── TESTING.md
│   ├── SECURITY.md
│   ├── DOCUMENTATION.md
│   ├── PERFORMANCE.md
│   ├── DEPENDENCIES.md
│   └── OBSERVABILITY.md
│
├── operations/
│   ├── DELIVERY.md
│   ├── VERSION-CONTROL.md
│   ├── RELEASES.md
│   ├── REFACTORING.md
│   ├── MIGRATIONS.md
│   └── PROJECT-LIFECYCLE.md
│
├── collaboration/
│   ├── MULTI-AGENT.md
│   ├── CONTEXT-SWITCHING.md
│   ├── REVIEW-PACKAGING.md
│   └── INSTITUTIONAL-MEMORY.md
│
├── specialist/
│   ├── NOVEL-RND.md
│   ├── IP-AND-INVENTION.md
│   ├── NETWORKING.md
│   ├── LOW-LEVEL-SAFETY.md
│   └── CONSTRAINED-HARDWARE.md
│
└── templates/
    ├── MANIFEST.template.md
    ├── ADR.template.md
    ├── AUDIT.template.md
    ├── DELIVERY-REPORT.template.md
    └── PROJECT-OVERRIDE.template.md
```

---

## What the new root `AGENTS.md` should contain

The root should remain authoritative, but much shorter—perhaps **150–300 lines** rather than the full constitution.

It should contain only:

1. Core identity  
2. Authority hierarchy  
3. Non-negotiable principles  
4. Mandatory pre-delivery gate  
5. Definition of done  
6. Module-loading rules  
7. Conflict resolution  
8. Constitutional amendment process  
9. Links to all modules  

The current document’s strongest material already supports this. Its core identity, dependency-first rule, zero-warning requirement, testing philosophy, and Section 0 gate are clearly constitutional.

The rest is mostly statutory or operational law.

---

## A useful hierarchy

| Level | Document class | Owns |
|-------|----------------|------|
| **1 — Constitution** | `AGENTS.md` + `constitution/` | Immutable cross-project law |
| **2 — Operating Law** | `SOP.md` | Project-to-product workflow |
| **3 — Standards modules** | `standards/`, `operations/`, `collaboration/`, `specialist/` | How categories of work execute |
| **4 — Project Law** | Local AGENTS.md, architecture, PRDs, ADRs, task plans | What is being built |

### Precedence

```
Safety and legal constraints
        ↓
Current human instruction
  (cannot silently waive core quality gate / safety)
        ↓
Root AGENTS.md + constitution/
        ↓
SOP.md
        ↓
Applicable standards modules
        ↓
Project-local instructions
        ↓
Task plans
```

Be careful with “human instruction first”: protect core quality obligations (completeness, zero-warnings, Section 0) immediately under safety/legality so convenience cannot void the gate.

---

## Mandatory versus conditional modules

Not every AI needs to read every module for every task.

### Always loaded

* `AGENTS.md`  
* `SOP.md`  
* `constitution/03-DEFINITION-OF-DONE.md`  
* `standards/ENGINEERING.md`  
* `standards/TESTING.md`  
* `standards/DOCUMENTATION.md`  

### Load when applicable

| Signal | Module |
|--------|--------|
| security work | `SECURITY.md` |
| novel architecture | `NOVEL-RND.md` |
| networking | `NETWORKING.md` |
| release task | `RELEASES.md` |
| migration or port | `MIGRATIONS.md` |
| multiple agents | `MULTI-AGENT.md` |
| invention-related work | `IP-AND-INVENTION.md` |
| constrained systems | `CONSTRAINED-HARDWARE.md` |

That reduces context load without weakening enforcement.

---

## The most important design rule

**Avoid turning modularization into fragmentation.**

Each rule should have **exactly one canonical home**.

Examples:

| Rule | Canonical home |
|------|----------------|
| Zero warnings | `standards/ENGINEERING.md` |
| Pre-delivery enforcement | root `AGENTS.md` |
| Release packaging | `operations/RELEASES.md` |
| Human review packaging | `collaboration/REVIEW-PACKAGING.md` |
| Research-to-delivery order | `SOP.md` |

Other documents may **link** to those rules (by Rule ID), but should not restate them in full.

Otherwise the modules will drift.

---

## Use rule identifiers

Every important rule should have a stable ID.

```
CONST-COMPLETE-001
CONST-AUTH-001
ENG-WARN-001
TEST-BEHAVIOR-001
SEC-INPUT-001
REL-PACKAGE-001
AI-COORD-001
IP-INVENTION-001
```

Then the SOP can say:

> Implementation must satisfy `ENG-WARN-001`, `TEST-BEHAVIOR-001`, `DOC-SYNC-001`, and `CONST-DONE-001`.

This is much stronger than repeatedly saying “follow AGENTS.md.”

Traceability for:

* audits  
* automated checks  
* delivery reports  
* project exceptions  
* AI self-audits  
* future tooling  

---

## Add module manifests

Each module should start with metadata:

```
Document: Engineering Standards
Module ID: MOD-ENG-001
Version: 1.1.0
Status: Binding
Authority: AGENTS.md
Applies When: Any source code is created or modified
Dependencies:
  - AGENTS.md
  - constitution/03-DEFINITION-OF-DONE.md
Overrides: None
```

This turns the folder into a governed system rather than a loose collection of Markdown files.

---

## Applicability matrix

| Task type | Required modules |
|-----------|------------------|
| Documentation only | Constitution, SOP, Documentation |
| Feature implementation | Engineering, Testing, Documentation, Security |
| Release | Delivery, Releases, Security, Review Packaging |
| Novel R&D | Engineering, Testing, Novel R&D, IP |
| Port or migration | Engineering, Testing, Migrations |
| Multi-agent work | Multi-Agent, Version Control, Review Packaging |

An AI entering the folder can immediately determine what it must read.

---

## What should stay together

Do **not** split the Section 0 checklist across multiple files.

That checklist is the enforcement engine of the constitution and should remain in the root document. Checklist items may link to detailed modules, but the gate itself stays unified.

Keep in root:

* core identity statement  
* no partial delivery  
* dependency-first  
* definition of done  
* authority hierarchy  
* conflict resolution  
* amendment rules  
* module applicability rules  

Everything else can move outward.

---

## Suggested first-pass decomposition

| Current sections | Destination |
|------------------|-------------|
| 0, 1, 18, 19, 40 | Root Constitution |
| 2, 4, 6, 8, 21, 22, 25, 33 | Engineering Standards |
| 3 | Testing |
| 5 | Research (within Engineering) |
| 7, 31 | Documentation and Memory |
| 9, 36 | Security |
| 10, 26 | Refactoring and Migration |
| 11, 29 | Human Collaboration and Handoff |
| 12, 13 | Project and Stack Applicability (Engineering) |
| 14 | SOP |
| 15 | Version Control |
| 16 | Observability |
| 17, 27 | Novel R&D and IP |
| 20, 37 | Resource and Cost Constraints |
| 23, 24 | Low-Level and Networking Safety |
| 28, 30, 38 | Multi-Agent and Context Switching |
| 32, 39 | Quality Metrics and Health |
| 34, 35 | Release and Project Lifecycle |

---

## Overall recommendation

Do modularize it, but preserve `AGENTS.md` as the sovereign entry point.

End state:

> **One constitution, one SOP, many narrowly scoped standards modules.**

* The constitution says what can never be violated.  
* The SOP says how work moves.  
* The modules say how specific categories of work must be executed.  
* Project documents say what is being built.  

That makes the reusable AGENTS Constitution folder easier to maintain, easier for an AI to load selectively, and more scalable across projects without weakening the original philosophy.

---

## Implementation status (this workspace)

| Spec item | Status |
|-----------|--------|
| Folder tree | Done |
| Short root AGENTS.md v5.0 | Done (universal portable pack) |
| SOP as Level 2 process law | Done (`SOP.md` v2.1; supersedes `WORKFLOW.md`) |
| Rule IDs | Done + **RULE-REGISTRY.md** |
| Module manifests | Done + **MODULE-INDEX.md** (`MOD-…-001`) |
| Applicability matrix | Done in root |
| Templates | Done |
| Pack integrity | Done — `INTEGRITY.md` + `tools/verify-pack.ps1` |
| Alias ban / one home | Done (`GOV-REG-003`, `CONST-ONEHOME-001`) |
| Pre-v4 monolith archive | `_archive_pre_v4/` |

**Verify:** `pwsh -File tools/verify-pack.ps1` → must PASS.

**Portability:** `PACK.md` / `ADOPT.md` / `VERSION` (5.0.0).

*End of Modularize.md*
