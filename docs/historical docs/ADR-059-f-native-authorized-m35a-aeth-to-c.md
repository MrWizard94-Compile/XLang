# ADR-059: F-NATIVE authorized — M35a verified AETH → C pure pilot

**Status:** Accepted — implemented (bounded pilot)
**Date:** 2026-08-10
**Related Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`
**Law fork:** [ADR-037](ADR-037-f-native-law-fork.md), [HUMAN-AUTHORIZE-NATIVE.md](../Current%20state/HUMAN-AUTHORIZE-NATIVE.md)
**Threat:** [THREAT_MODEL-v4-NATIVE-BACKEND.md](../Current%20state/THREAT_MODEL-v4-NATIVE-BACKEND.md) (binding for this path)

## Context

Human authorized F-NATIVE (2026-08-10). First vertical slice after fork open.

## Decision

1. **Fork open:** optional native lower of **verified AETH only**; VM remains default
   and reference semantics; **no** Aether-source transpile to C/Rust/JS.
2. **M35a pilot:** `lower_verified_aeth_to_c` emits ISO C for a **pure**
   restricted AETH subset (Whole arithmetic, speak/yield, total main; no host I/O,
   foreign, nursery, resources).
3. **CLI:** `aether compile --native-c --output <file.c>` product-compiles source
   (seed), verifies AETH, then lowers.
4. **Dual-run:** tests compare VM exit code to expectations; optional host `cc`
   of emitted C is not required for matrix green on Windows without a C toolchain.
5. Trackers: `f_native_authorized() == true`, `native_aeth_to_c_pilot() == true`.

## Honesty

- Not LLVM/object emit; C is an intermediate deploy aid under F-NATIVE shape.
- Seed does not emit C.
- Native is not memory-safe by claim.

## Links

- DESIGN-LAW-FORK-F-NATIVE §6, ADR-037

---

*End of ADR-059.*
