# Human ordered backlog (2026-08-05)

**Authority:** Human session order under AGENTS Constitution  
**Date:** 2026-08-05  

| # | Track | Status |
| --- | --- | --- |
| **1** | **T-CT ADR** (pure comptime calls) | **Implemented in package 0.33**; bootstrap-materialized seed-emission bridge proven |
| **2** | Offline package polish | **Implemented in package 0.35** as PKG-001: explicit project/workspace locks, local package identity pins, and locked-build preflight; no registry |
| **3** | Ownership + destroy + mid-frame cancel | **Implemented in package 0.36**: [M19e](DESIGN-M19E-T-RX-ACTIVE-FRAME-CANCEL.md) / [ADR-042](ADR-042-m19e-active-frame-cancel.md) / [matrix](M19E-VALIDATION-MATRIX.md); bounded v12 task/checkpoint scope only |
| **4** | Human law forks F-NATIVE / F-REGISTRY | Decision packages ready; blocked until §3 authorize phrases |

## Active gate

Offline package polish is complete within current M18/M22 bounds. M23's vertical
slice (bootstrap + seed-emission bridge + dual-compare + package 0.33) and
PKG-001 (package 0.35) and M19e (package 0.36) are complete. M19e's full v12
vertical gate covers parser/semantic model, verifier, resumable VM frames,
private lanes/destruction, seed, authoring, hostile artifacts, compatibility,
and warning-free evidence. It must not be relabeled as a general async or task
handle system.

The next implementation candidate requires a new scoped ADR and matrix; further
offline package/stdlib polish remains the nearest non-law-fork direction.

Law forks are **not** next for code without:

- [HUMAN-AUTHORIZE-NATIVE.md](HUMAN-AUTHORIZE-NATIVE.md) and/or  
- [HUMAN-AUTHORIZE-REGISTRY.md](HUMAN-AUTHORIZE-REGISTRY.md)  

---

*End of backlog.*
