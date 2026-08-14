# ADR-079: M35e — product native object emit via host cc

**Status:** Accepted — implemented  
**Date:** 2026-08-10  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`  
**Law fork:** F-NATIVE (reaffirmed 2026-08-10)  
**Depends on:** ADR-059, ADR-073, ADR-076  

## Context

M35d dual-exec is best-effort when no C toolchain exists. Operators need a
**product** path that emits a native object file from verified AETH, failing
closed without a host compiler (not a silent no-op).

## Decision

1. **`lower_verified_aeth_to_native_object(bytecode, path)`:** verify AETH → C →
   `cc -c -o path`.  
2. **CLI:** `aether compile <src> --output <file.o> --native-object`.  
3. **Fail closed** without host `cc`/`clang`/`gcc` (`AE-NATIVE-004`).  
4. **Not LLVM IR**, not a full linker product path.  
5. Tracker: `native_object_emit_product() == true`.  

## Links

- ADR-037, ADR-076, M35A-VALIDATION-MATRIX  

---

*End of ADR-079.*
