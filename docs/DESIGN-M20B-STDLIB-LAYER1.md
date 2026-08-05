# M20b Design: stdlib layer 1 (pure Whole / Truth / Text)

**Status:** Accepted design for ADR-029 — implemented in package 0.27.0  
**Date:** 2026-08-04  
**Depends on:** M20 layer 0 (ADR-024), M11 modules  
**Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`, `RND-INVAR-001`

---

## 1. Purpose

Layer 0 shipped three Whole helpers. Layer 1 deepens the **offline pure** stdlib
without host I/O, registry, or M2 resource ownership in lib units.

## 2. Core claim

> Stdlib layer 1 is still a single offline multi-unit project under `stdlib/` of
> pure `export weave` helpers for Whole, Truth, and Text. Consumers use M11
> `import unit`. No registry; no ambient host.

## 3. Surface

| Module | Export weaves |
| --- | --- |
| `whole.ae` | layer 0: `double`, `inc`, `is_zero`; **layer 1:** `dec`, `abs`, `max`, `min`, `clamp_nonneg` |
| `truth.ae` | `invert`, `both` |
| `text.ae` | `text_len`, `text_empty` |

## 4. Layout

```text
stdlib/
  aether.project.json   # main + whole + truth + text libs
  main.ae               # demo entry (exit 42 via double 21)
  whole.ae
  truth.ae
  text.ae
  whole_test.ae         # aether test pure suite (exit 0)
```

## 5. Non-goals

- Host I/O stdlib wrappers (needs M14 grants in test/run — separate ADR)  
- Arena/Buffer helpers in lib units (arena is main-only M2)  
- Registry packaging  
- Policy B / FFI  

## 6. Package

**0.27.0** with dual-compare project build + `aether test` on stdlib tests.

---

*End of DESIGN-M20B-STDLIB-LAYER1.md*
