# ADR-083: M35f — verified AETH → LLVM IR text product path

**Status:** Accepted — implemented  
**Date:** 2026-08-10  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`  
**Law fork:** F-NATIVE (reaffirmed 2026-08-10)  
**Depends on:** ADR-059, ADR-073, ADR-079  

## Context

M35e emits native objects via host `cc`. Roadmap M35f asked for LLVM IR as a
portable intermediate without requiring libLLVM in the Aether process.

## Decision

1. **`lower_verified_aeth_to_llvm_ir`:** hand-written LLVM IR text for the pure
   Total subset (Whole locals/arithmetic, SPEAK/Text, multi-weave CALL).  
2. **CLI:** `aether compile <src> --output <file.ll> --native-llvm-ir`.  
3. **No libLLVM** dependency; optional `llc`/`clang` remains operator-side.  
4. Tracker: `native_llvm_ir_emit_product() == true`.  

## Honesty

- Not a full LLVM object/linker product path.  
- IR targets a generic `x86_64-unknown-linux-gnu` triple for text portability.  

## Links

- ADR-037, ADR-079, M35A-VALIDATION-MATRIX  

---

*End of ADR-083.*
