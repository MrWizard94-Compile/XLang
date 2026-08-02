# SOP Phase 6 — Engineering Plan

**SOP:** SOP-PROD-001 Phase 6  
**Date:** 2026-07-28  
**Status:** Complete (milestones delivered)  

## Milestones

| ID | Milestone | DoD | Status |
|----|-----------|-----|--------|
| M1 | Short root SOUL + gate unified | `CONST-GATE-001` in root only | **Done** |
| M2 | Module tree per Modularize.md | All paths exist | **Done** |
| M3 | Rule IDs + registry | `GOV-REG-001` | **Done** |
| M4 | Module manifests + index | `MOD-*-001` | **Done** |
| M5 | Integrity tooling | `verify-pack` exit 0 | **Done** |
| M6 | Harden (aliases, stop-ship, overrides) | v4.1.0 | **Done** |
| M7 | Meta self-audit | `self-audit` exit 0 | **Done** |
| M8 | SOP run artifacts | `docs/**` this folder | **In progress** |

## Dependencies

M1 → M2 → M3/M4 → M5 → M6 → M7 → M8  

## Risks

| Risk | Mitigation |
|------|------------|
| Module drift | One home + registry |
| Dual-location drift | **Desktop is sole SoT going forward** |
| False TODO scans | Self-audit distinguishes prohibition prose |
| Scope creep into monolith | Amendment: expand modules, not root |

## Required decisions (human)

* Active pack tree is source of truth for the amend (`GOV-SYNC-001`).  
* Other mirrors are out of scope unless deliberately promoted.
