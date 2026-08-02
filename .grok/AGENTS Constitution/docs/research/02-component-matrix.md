# SOP Phase 2 — Component Decomposition Matrix

**SOP:** SOP-PROD-001 Phase 2  
**Date:** 2026-07-28  
**Status:** Complete  

Categories mapped for a **governance pack** (not a SaaS UI).

| Component | Cursor | AGENTS.md culture | Google Eng | Rust API | K8s | Kernel | IETF | ISO SOP | AWS WA | **This pack** |
|-----------|--------|-------------------|------------|----------|-----|--------|------|---------|--------|---------------|
| Entry UX | Many files | Root md | Handbook | Single guide | Docs home | Doc tree | RFC index | SOP cover | Framework | Short `AGENTS.md` |
| Workflow | Implicit | Implicit | Review | N/A | Contrib flow | Dev process | Standards track | Numbered steps | Review loops | `SOP.md` 1–10 |
| Data model | Free text | Free text | Principles | Numbered rules | Doc tree | Subsystems | RFC nos | Controlled IDs | Pillar Qs | Rule + Module IDs |
| Integrations | Per-repo | Per-repo | Org | Lang | Multi-SIG | Multi-subsys | Multi-WG | Multi-site | Multi-account | Single Desktop SoT |
| Authority | Weak | Weak | Org policy | Norms | SIG owners | Maintainers | IESG | Doc control | Boundaries | 4-level precedence |
| Health/analytics | None | None | Metrics | CI/clippy | CI | Digests | Errata | Audits | WA reviews | Self-audit / health |
| Admin/tooling | IDE | None | Internal | Clippy | Prow | Scripts | Datatracker | DMS | Console | `verify-pack` / `self-audit` |
| Security | Secrets risk | Same | Strong | Safe APIs | RBAC docs | Hardening | Sec ADs | Controls | SEC pillar | `SECURITY.md` |
| Testing | Ad hoc | Ad hoc | Required | Compile | Unit/e2e | Selftests | Interop | Procedure tests | Reviews | Pack integrity suite |

## Dependencies

```
Constitution (authority + gate)
  → SOP (process)
    → Standards (always + conditional)
      → Operations / Collaboration / Specialist
        → Templates + Tools
          → Level 4 project law (consumers)
```
