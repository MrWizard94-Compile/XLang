# SOP Phase 3 — Deep-Dive Patterns

**SOP:** SOP-PROD-001 Phase 3  
**Date:** 2026-07-28  
**Status:** Complete  

## Adopt

| Pattern | Source | Implementation |
|---------|--------|----------------|
| Short root | AGENTS.md culture | SOUL in `AGENTS.md` |
| Selective pillars | AWS WA | Applicability matrix |
| Checkable rules | Rust API Guidelines | `RULE-REGISTRY.md` |
| Topic directories | K8s / Kernel | Module folders |
| Process ≠ policy | ISO SOP | `SOP.md` only process |
| Amendment rigor | IETF | Complete-file replace |
| Review packaging | Google | `REV-PACK-001` |
| Automated gate | CI culture | `tools/verify-pack.ps1` |

## Avoid

| Friction | Mitigation |
|----------|------------|
| Monoliths | Modular load |
| Duplicate rules | One canonical home |
| No machine check | `GOV-INT-001` |
| Process mixed into quality law | Level 1 vs 2 split |
| Alias Rule IDs | `GOV-REG-003` |
| Multi-agent thrash | `AI-COORD-*` |

## Design insight

> One constitution, one SOP, many narrowly scoped modules; Rule IDs; manifests; integrity tooling; unified Section 0 gate in root only.
