# M19d Design: resourceful-spawn precondition for Policy B

**Status:** Accepted **design direction** — **implementation blocked**  
**Date:** 2026-08-04  
**Decision record:** [ADR-034](ADR-034-m19d-resourceful-spawn-precondition.md)  
**Depends on:** M19c Policy B design (ADR-032), M2, M7  
**Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `RND-INVAR-001`

---

## 1. Purpose

Policy B cancel-destroy is meaningless until spawn callees can own resources
(or transferable owners). This design states the **minimum precondition slice**
without shipping product code.

## 2. Minimum future claim (not proven now)

> A later ADR may admit **one** of:

1. **Spawn-local unique owners only** (Text/Bytes) with cancel of unstarted
   spawns needing no destroy, and completed spawns ending owners at return; or  
2. **Main-arena access parameter on total callees** called only via ordinary
   `call` (not spawn) — does not unlock Policy B; or  
3. **Explicit spawn-scoped arena** (new M2 form) with cancel-time `release` of
   all still-live Buffer/Table/Arena locals in cancelled/aborted frames.

Option **3** is the real Policy B enabler. Options 1–2 do not justify free-on-
cancel product claims.

## 3. Stop conditions

- Implementing destroy without an admissible corpus  
- Free-on-raise for parent live owners  
- Resource spawn arguments without ownership rules  

## 4. Package

**No product package bump** for design-only M19d.

---

*End of DESIGN-M19D-RESOURCEFUL-SPAWN-PRECONDITION.md*
