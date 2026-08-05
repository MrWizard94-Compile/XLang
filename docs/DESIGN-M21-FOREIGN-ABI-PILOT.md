# M21 Design: narrow foreign ABI pilot (T-FFI)

**Status:** Accepted design — **implemented pilot in package 0.31.0** after human
authorize (ADR-025 residual risk)  
**Date:** 2026-08-04  
**Decision record:** [ADR-025](ADR-025-m21-foreign-abi-pilot.md)  
**Threat:** [THREAT_MODEL-v3-FOREIGN-ABI.md](THREAT_MODEL-v3-FOREIGN-ABI.md)  
**Authorize:** [HUMAN-AUTHORIZE-FFI.md](HUMAN-AUTHORIZE-FFI.md)  

---

## 1. Purpose

Smallest typed foreign call surface beyond pure M8/M14 host weaves — without C
header ingestion.

## 2. Source form (pilot)

```aether
foreign weave whole_inc_f [value: Whole] -> Whole from "pilot" symbol "aether_whole_inc"
```

- `from "key"` is a **logical** library key (not a path).  
- `symbol "name"` is a pinned C symbol.  
- Pilot ABI: 0..=4 owned `Whole` parameters, `Whole` result.

## 3. Host grant

```text
aether run out.aeth --grant-lib pilot=/absolute/or/relative/file.dll
```

- KEY must match `from "key"`.  
- PATH must be an **existing library file** (canonicalize; no PATH search).  
- Missing grant → `AE-FFI-003`.  

## 4. Compile / seed honesty

- Bootstrap and seed both parse/emit foreign weaves.  
- Product default `compile` is seed-hosted; dual-compare is proven for the
  documented foreign corpus (`examples/foreign-pilot.ae`, `examples/foreign-sum.ae`).  
- `compile --bootstrap` remains for seed rebuild and diagnostic authority.  

## 5. Runtime

1. Verify AETH.  
2. On `HOST_CALL` to a foreign-encoded host function, require library grant.  
3. `libloading` load + symbol resolve + C call (scoped `unsafe` in `ffi` module).  

## 6. Non-goals (v1)

Header parse, bindgen, callbacks into Aether, threads, async, Text/Bytes C ABI.

## 7. Stop conditions

- libloading without explicit path grant  
- void* ambient escape  
- Claiming memory safety of foreign code  

---

*End of DESIGN-M21-FOREIGN-ABI-PILOT.md*
