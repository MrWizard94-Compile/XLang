# M20 Design: stdlib layer 0 (pure modules)

**Status:** Accepted design for implementable ADR-024  
**Date:** 2026-08-04  
**Decision record:** [ADR-024](ADR-024-m20-stdlib-layer0.md)  
**Depends on:** M11 modules, M17 test, M18 workspace  

---

## 1. Purpose

Ship a **minimal offline standard library** of pure Aether modules (no host I/O)
as ordinary project units, proven by `project build` dual-compare and `aether test`.

## 2. Core claim

> Layer 0 stdlib is a local multi-unit project of pure `export weave` helpers
> (Whole arithmetic/predicates). Consumers import units via M11 inside one
> project root. No registry; no ambient host.

## 3. Surface (layer 0)

| Module | Weaves (export) |
| --- | --- |
| `stdlib/whole.ae` | `double`, `inc`, `clamp_nonneg` (illustrative pure Whole helpers) |

## 4. Layout

```text
stdlib/
  aether.project.json
  whole.ae
  tests/   # optional *_test via consumer project or workspace app
examples/stdlib-demo/   # main imports stdlib units vendored or path-linked
```

Pilot: ship stdlib as `stdlib/` project; demo project copies import paths under
one root **or** (with M22) imports from package.

M20 without M22: demo is a single project that includes stdlib units as `lib`
paths (`stdlib/whole.ae` relative units).

## 5. Non-goals

Host I/O wrappers, collections requiring M2 in lib units (M11 lib bans
records/shapes/host; buffer ops need arena in main), network fetch.

## 6. Package pin

**0.24.0** when shipped with demo + tests.

---

*End of DESIGN-M20-STDLIB-LAYER0.md*
