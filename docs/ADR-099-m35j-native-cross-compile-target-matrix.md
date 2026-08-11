# ADR-099: M35j — native cross-compile target matrix

**Status:** Accepted — implemented  
**Date:** 2026-08-11  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`  
**Law fork:** F-NATIVE (reaffirmed 2026-08-11)  
**Depends on:** ADR-091, ADR-095  

## Context

M35i discovers host tools. Operators need a closed list of supported target
triples and a fail-closed path for out-of-matrix or failed cross links.

## Decision

1. **`native_supported_target_triples()`** closed matrix (Windows/Linux/macOS
   x86_64/aarch64 triples).  
2. **Operator path:** `aether compile <source> --output <exe> --native-exe
   --target <triple>` passes the selected triple to
   `lower_verified_aeth_to_native_exe_for_target`. `--target` is rejected
   unless `--native-exe` is selected.
3. **`lower_verified_aeth_to_native_exe_for_target`:**
   - unknown target → `AE-NATIVE-007`  
   - host triple → M35h dual-run path  
   - other matrix triple → clang/cc `--target` / `-target` best-effort,
     link-only; it is never executed on the host
   - an API request to dual-run a cross target fails with `AE-NATIVE-007`
     before it creates an output file or temporary lowering directory
4. Tracker: `native_cross_compile_target_matrix() == true`.

## Honesty

- Not a bundled sysroot or guaranteed cross success.  
- Fail closed when the host toolchain cannot produce the target.  
- Cross-link output is only an operator-requested artifact; successful linking
  is not evidence that another OS/architecture can execute it.

## Links

- ADR-091, ADR-095, M35A-VALIDATION-MATRIX  

---

*End of ADR-099.*
