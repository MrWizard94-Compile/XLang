# ADR-091: M35h — native executable link via host C toolchain

**Status:** Accepted — implemented  
**Date:** 2026-08-11  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`  
**Law fork:** F-NATIVE (reaffirmed 2026-08-11)  
**Depends on:** ADR-059, ADR-076, ADR-087  

## Context

M35g emits objects via LLVM tools. Operators need a product path that links a
runnable native executable and dual-runs against the VM when a host C toolchain
is present.

## Decision

1. **`lower_verified_aeth_to_native_exe(bytecode, path, verify_exit)`:**  
   AETH → C → host `cc`/`clang`/`gcc` link → optional run dual-compare.  
2. **CLI:** `aether compile <src> --output <exe> --native-exe`.  
3. **Fail closed** without host C toolchain (`AE-NATIVE-004`).  
4. When `verify_exit`, native exit must match VM (`AE-NATIVE-005`).  
5. Tracker: `native_exe_link_product() == true`.  

## Honesty

- Host toolchain required; not a hermetic linker.  
- Pure Total subset only (same as M35a–c).  

## Links

- ADR-037, ADR-087, M35A-VALIDATION-MATRIX  

---

*End of ADR-091.*
