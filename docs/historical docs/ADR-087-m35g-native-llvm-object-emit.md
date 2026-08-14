# ADR-087: M35g — LLVM IR to native object via host toolchain

**Status:** Accepted — implemented  
**Date:** 2026-08-10  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`  
**Law fork:** F-NATIVE (reaffirmed 2026-08-10)  
**Depends on:** ADR-083  

## Context

M35f emits LLVM IR text. Roadmap M35g asked for object emit through LLVM tools
without embedding libLLVM in Aether.

## Decision

1. **`lower_verified_aeth_to_llvm_object`:** AETH → IR → host `clang -c` or
   `llc -filetype=obj`.  
2. **CLI:** `aether compile <src> --output <file.o> --native-llvm-object`.  
3. **Fail closed** without clang/llc (`AE-NATIVE-004`).  
4. Tracker: `native_llvm_object_emit_product() == true`.  

## Honesty

- No embedded libLLVM; host toolchain required.  
- Not a full linker/exe product path.  

## Links

- ADR-037, ADR-083, M35A-VALIDATION-MATRIX  

---

*End of ADR-087.*
