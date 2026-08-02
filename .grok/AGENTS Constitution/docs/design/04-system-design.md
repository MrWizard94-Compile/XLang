# SOP Phase 4 — System Design Proposal

**SOP:** SOP-PROD-001 Phase 4  
**Date:** 2026-07-28  
**Status:** Complete (implemented as pack v4.1.0)  

## Vision

A governed AGENTS Constitution pack that any project can adopt without reading a 40-section monolith, without weakening zero-warning / complete-delivery law.

## Target users

1. Human director (sovereign, high cognitive load)  
2. AI coding agents (must load selectively, self-audit)  
3. Multi-agent teams (coordination without file thrash)  

## Core use cases

| Priority | Use case |
|----------|----------|
| P0 | Always-load SOUL + quality standards for any code change |
| P0 | Section 0 gate before human presentation |
| P0 | Process path research → deliver via SOP |
| P1 | Conditional modules (security, release, novel R&D, multi-agent) |
| P1 | Pack integrity verification and meta self-audit |
| P2 | Templates for MANIFEST, ADR, audit, delivery, overrides |

## Architecture (four levels)

| Level | Artifact | Role |
|-------|----------|------|
| 1 | `AGENTS.md` + `constitution/` | Immutable quality law |
| 2 | `SOP.md` | Project-to-product workflow |
| 3 | standards / operations / collaboration / specialist | Execution law by category |
| 4 | Consumer project docs | What is being built |

## Trust boundaries

* Lower levels never override higher (`CONST-AUTH-001`).  
* Human instruction cannot silently waive gate/completeness/zero-warnings without `PROJECT-OVERRIDE`.  
* Gate text lives only in root (`CONST-ONEHOME-001` / `GOV-INT-006`).  

## Prioritized feature set (built)

1. Short root SOUL + 15-point gate  
2. Modular tree per `Modularize.md`  
3. Rule registry + module index  
4. Integrity + self-audit tools  
5. Templates + SOP Rule ID gates  

**Design confirmed by implementation** — see pack root.
