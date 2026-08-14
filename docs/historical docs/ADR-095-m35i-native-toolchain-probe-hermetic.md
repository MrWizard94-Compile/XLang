# ADR-095: M35i — native toolchain probe + hermetic env

**Status:** Accepted — implemented  
**Date:** 2026-08-11  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`  
**Law fork:** F-NATIVE (reaffirmed 2026-08-11)  
**Depends on:** ADR-091  

## Context

M35h links executables via ambient host `cc`/`clang`. Operators need discovery of
tools, default target triple, and a hermetic fail-closed mode when explicit tool
paths are required.

## Decision

1. **`probe_native_toolchain()`** reports cc/clang/llc presence, host OS/arch,
   optional target triple, and hermetic flag.  
2. **Env overrides:** `AETHER_CC`, `AETHER_CLANG`, `AETHER_LLC`,
   `AETHER_NATIVE_TARGET`.  
3. **Hermetic:** `AETHER_NATIVE_HERMETIC=1` requires a usable C compiler
   (`AE-NATIVE-006` via `require_hermetic_native_toolchain`).  
4. **CLI:** `aether native probe`.  
5. Trackers: `native_toolchain_probe_product()`, `native_hermetic_toolchain_env()`.  

## Honesty

- Not a full cross-compile matrix or bundled toolchain.  
- Best-effort PATH discovery when env overrides are unset.  

## Links

- ADR-037, ADR-091, M35A-VALIDATION-MATRIX  

---

*End of ADR-095.*
