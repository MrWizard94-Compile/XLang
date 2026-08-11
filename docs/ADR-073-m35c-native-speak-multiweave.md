# ADR-073: M35c — AETH→C SPEAK/Text + multi-weave pure helpers

**Status:** Accepted — implemented  
**Date:** 2026-08-10  
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `TEST-BEHAVIOR-001`, `SEC-INPUT-001`  
**Law fork:** F-NATIVE ([HUMAN-AUTHORIZE-NATIVE.md](HUMAN-AUTHORIZE-NATIVE.md); reaffirmed 2026-08-10)  
**Depends on:** ADR-059, ADR-062  

## Context

M35a/b lowered pure Whole yield and Whole locals. Showcase and control-flow
programs need `speak` Text and helper weaves (`call`). Roadmap M35c+ asked for
SPEAK/Text, multi-weave, and dual-run discipline.

## Decision

1. **SPEAK/Text:** verified AETH `PUSH_TEXT` + `SPEAK` lower to a C text stack
   (`tstack`) and `fputs`. SPEAK requires Text (matches VM).  
2. **Multi-weave:** pure **Total** guest weaves with **Whole** params/results;
   `CALL` lowers to static `aether_fn_<index>(...)`. One `main [] -> Whole` required.  
3. **Rejected still:** host, task, Error effect, non-Whole params/results/locals,
   nursery, foreign, resource opcodes.  
4. **Dual-run helper:** `native_dual_run_vm_exit` product-compiles, runs VM, and
   requires successful C lower (VM remains reference). Host `cc` dual-exec remains
   optional when a C toolchain is present (not required for matrix green).  
5. Trackers: `native_aeth_to_c_speak_multiweave_pilot() == true`.  

## Honesty

- Still ISO C intermediate, not LLVM/object.  
- Seed does not emit C.  
- Native is not memory-safe by claim.  

## Links

- ADR-037, ADR-059, ADR-062, M35A-VALIDATION-MATRIX  

---

*End of ADR-073.*
