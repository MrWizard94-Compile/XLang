# M21 Design: narrow foreign ABI pilot (T-FFI)

**Status:** Design only — **blocked on threat model v3 acceptance + human implement go**  
**Date:** 2026-08-04  
**Decision record:** [ADR-025](ADR-025-m21-foreign-abi-pilot.md)  
**Threat:** [THREAT_MODEL-v3-FOREIGN-ABI.md](THREAT_MODEL-v3-FOREIGN-ABI.md)  

---

## 1. Purpose

Define the **smallest** typed foreign call surface beyond pure M8/M14 host
weaves — without C header ingestion.

## 2. Proposed source form (future)

```aether
foreign weave c_strlen [borrow s: Text] -> Whole from "libc" symbol "strlen"
```

## 3. Non-goals (v1)

Header parse, bindgen, callbacks into Aether, threads, async.

## 4. Stop conditions

- libloading without explicit path grant  
- void* as ambient escape  
- Claiming memory safety of foreign code  

## 5. Implementation

**None in this delivery.** Code requires ADR-025 Accepted **and** explicit
human “implement FFI” instruction after threat review.

---

*End of DESIGN-M21-FOREIGN-ABI-PILOT.md*
