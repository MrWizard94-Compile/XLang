# Aether 0.31 Toolchain Contract (M21 foreign ABI pilot)

**Status:** Current package — language **0.11** / AETH **v11** + M21 foreign pilot  
**Depends on:** ADR-025 human implement authorization, threat model v3 residual acceptance  

## Surface

```aether
foreign weave whole_inc_f [value: Whole] -> Whole from "pilot" symbol "aether_whole_inc"
```

- **Pilot only:** owned `Whole` parameters and `Whole` result (no Text pointers).  
- **Load:** host-side after AETH verify; not via seed compile path.  
- **Grant:** `aether run … --grant-lib pilot=C:\path\to\aether_ffi_pilot.dll`  
  (explicit file path; no PATH search).  
- **Compile:** `aether compile --bootstrap` for foreign programs (seed does not yet emit foreign).  

## Residual risk (honest)

Operator-granted native libraries run in the host process. Aether does **not**
sandbox foreign code and does **not** claim memory safety of the foreign library.

## Non-goals (v1)

C headers, bindgen, callbacks into Aether, Text/Bytes C pointers, seed dual-compare.

*End of AETHER_0.31.md*
