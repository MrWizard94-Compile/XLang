# MANIFEST — SOP Final Delivery (Desktop Pack)

**Delivery title:** AGENTS Constitution pack v4.1.0 — SOP-complete meta-delivery  
**Date:** 2026-07-28  
**Location:** `<pack-root>`  
**Related Rule IDs:** CONST-GATE-001, CONST-DONE-001, SOP-PHASE-001, SOP-GATE-001, REV-PACK-001, REL-PACKAGE-001  

## Summary

One-sentence: Modular constitution pack completed through SOP phases 1–10 on pack-local only, with research through delivery artifacts and green integrity gates.

## Files (purpose)

### Law & process

| Path | Purpose | New/Modified |
|------|---------|--------------|
| `AGENTS.md` | Level 1 SOUL + gate | Existing (v4.1) |
| `SOP.md` | Level 2 process | Existing |
| `RULE-REGISTRY.md` | Rule ID authority | Existing |
| `MODULE-INDEX.md` | Module catalog | Existing |
| `INTEGRITY.md` | Pack integrity law | Existing |
| `README.md` | Start here | Existing |
| `Modularize.md` | Design history | Existing |

### Modules / templates / tools

| Path | Purpose |
|------|---------|
| `constitution/*` | Core law, authority, contract, DoD |
| `standards/*` | Engineering, testing, security, docs, perf, deps, observability |
| `operations/*` | Delivery, VCS, releases, refactor, migrations, lifecycle |
| `collaboration/*` | Multi-agent, context, review packaging, memory |
| `specialist/*` | Novel R&D, IP, networking, low-level, hardware |
| `templates/*` | MANIFEST, ADR, audit, delivery, override |
| `tools/verify-pack.ps1` | Integrity tests |
| `tools/self-audit.ps1` | Meta Section 0 |

### SOP run outputs (this delivery)

| Path | Purpose | New/Modified |
|------|---------|--------------|
| `docs/research/01-reference-projects.md` | Phase 1 | **New** |
| `docs/research/02-component-matrix.md` | Phase 2 | **New** |
| `docs/research/03-deep-dive-patterns.md` | Phase 3 | **New** |
| `docs/design/04-system-design.md` | Phase 4 | **New** |
| `docs/design/05-foundational-docs.md` | Phase 5 | **New** |
| `docs/plan/06-engineering-plan.md` | Phase 6 | **New** |
| `docs/plan/07-implementation-log.md` | Phase 7 | **New** |
| `docs/audit/08-audit-report.md` | Phase 8 | **New** |
| `docs/audit/08-sop-runtime-scorecard.md` | Phase 8 runtime | **New** |
| `docs/audit/09-hardening-report.md` | Phase 9 | **New** |
| `docs/delivery/10-MANIFEST.md` | Phase 10 manifest | **New** |
| `docs/delivery/10-DELIVERY-REPORT.md` | Phase 10 report | **New** |
| `docs/delivery/10-SOP-RUN-SUMMARY.md` | Phase scoreboard | **New** |

## Verification

```powershell
cd "<pack-root>"
pwsh -File tools\verify-pack.ps1
pwsh -File tools\self-audit.ps1
```

Expected: both exit `0`; `reports/SELF-AUDIT-latest.md` OVERALL PASS.

## Risks / trade-offs

* Desktop is sole ongoing source of truth; other mirrors may drift if edited.  
* SOP phases 1–3 for this product are retrospective research (references), not greenfield product research.

## Next actions for human

1. Review `docs/delivery/10-DELIVERY-REPORT.md`.  
2. Optionally archive or tag Desktop pack as v4.1.0-SOP-complete.  
3. Point projects at this Desktop folder (or a controlled copy you authorize).  
