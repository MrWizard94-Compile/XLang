# Law fork F-NATIVE — optional native / machine-code backend

**Status:** Design decision package — **authorized** 2026-08-10; bounded F-NATIVE implemented through M35j / ADR-059–099
**Date:** 2026-08-11
**Decision records:** [ADR-037](ADR-037-f-native-law-fork.md), [HUMAN-AUTHORIZE-NATIVE.md](HUMAN-AUTHORIZE-NATIVE.md), [ADR-059](ADR-059-f-native-authorized-m35a-aeth-to-c.md)–[ADR-099](ADR-099-m35j-native-cross-compile-target-matrix.md)
**Threat draft:** [THREAT_MODEL-v4-NATIVE-BACKEND.md](THREAT_MODEL-v4-NATIVE-BACKEND.md)  
**Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `SEC-INPUT-001`, `RND-INVAR-001`, `CONST-CONTRACT-001`

---

## 1. Purpose

Decide whether Aether may ever emit or run **native machine code** in addition to
(or instead of) the Aether VM, under what authority boundaries, and how that
interacts with CLM-010 (“source is not translated to another language”).

This document authorized the bounded M35a–j product work. Any **new** native
scope beyond that authorized boundary still requires:

1. Human residual-risk acceptance via [HUMAN-AUTHORIZE-NATIVE.md](HUMAN-AUTHORIZE-NATIVE.md)  
2. ADR-037 status flip to implementable  
3. A vertical-slice design (e.g. M35) with matrix + dual-compare plan  

## 2. Invariants retained by the accepted fork

| Rule | Default |
| --- | --- |
| Execution | Verified AETH on the **Aether VM** by default; native is explicit opt-in |
| Lowering | Source → AETH only (seed or bootstrap) |
| CLM-010 | No transpile of Aether source to C/Rust/JS/LLVM IR as a product compiler |
| Performance path | VM improvements, optional future **AETH-level** JIT *if* separately ADRed and still not “source transpile” |

The accepted fork retains the verify-before-run story: native lowering begins
only after verified AETH, and the VM remains the reference semantics.

## 3. If fork is accepted — allowed product shape

### 3.1 Preferred shape (recommended)

```text
Aether source ──► AETH (verify) ──► optional native codegen from AETH
                      │
                      └──► VM run (always remains valid)
```

| Property | Requirement |
| --- | --- |
| Source transpile | **Forbidden** — never “Aether → C/Rust/JS” as product |
| Input to native | **Only verified AETH** (same verifier as VM) |
| Output | Platform object/executable or in-memory code, versioned |
| Default product path | Still seed-hosted AETH; native is **opt-in** CLI mode |
| Dual authority | VM remains the reference semantics; native must match on corpus |
| Seed dual-compare | Native backend is **not** required to be seed-emitted; honesty: bootstrap/native tooling claim explicit |

### 3.2 Rejected shapes (even if fork is accepted)

- Product compiler that emits C/Rust/JS source as the primary artifact  
- Silent default native without operator opt-in  
- Skipping AETH verify before native lower  
- Claiming “memory safe native” without evidence  
- Replacing seed-hosted AETH as the sole product identity without DOC-SYNC  

### 3.3 JIT note

An **AETH bytecode JIT** that never materializes another language’s source may
be designable **without** full F-NATIVE if framed as VM performance. That still
needs its own ADR (unsafe/code cache/threat). F-NATIVE is required when the
product ships a **native backend / object emit / LLVM** story as a first-class
deploy path.

## 4. Law rewrites required on accept

| Artifact | Change |
| --- | --- |
| `AGENTS.md` invariants | Allow optional native lower of verified AETH; reaffirm no source transpile |
| `CORE_CLAIMS` CLM-010 | Refine: “no source-to-other-language product transpile”; AETH→native optional |
| `MANIFEST.md` | Opt-in native path; VM remains default |
| Threat model | Adopt threat v4 as binding for native slices |
| Portfolio ADR-014 | Unblock T-NATIVE track under F-NATIVE |

## 5. Residual risks (human must accept)

| ID | Risk |
| --- | --- |
| N1 | Native code can escape Aether’s VM sandbox model (process-level compromise) |
| N2 | Codegen bugs can diverge from VM semantics (need corpus dual-run) |
| N3 | Toolchain complexity (LLVM/cranelift/etc.) and supply-chain surface |
| N4 | Seed path may never emit native; bootstrap/native split honesty required |
| N5 | `unsafe` / platform ABIs expand beyond current deny-by-default culture |

## 6. First implementable slice (implemented historical boundary)

The original M35-class vertical is implemented and expanded through M35j:

1. `aether compile --native-object` lowers **verified** AETH only.
2. Whole-only pure programs dual-run VM versus native exit behavior.
3. The M35a–j portfolio adds verified AETH→C, object, LLVM IR/object, native
   executable, hermetic toolchain probing, and the closed `--native-exe --target`
   matrix without changing the VM-default product path.
4. M14 host I/O, M21 foreign, nursery/task, and resourceful programs remain
   outside the native lower subset unless a future scoped ADR proves them.

## 7. Stop conditions

- Implementing native scope beyond authorized M35a–j without a renewed named human authorization and scoped ADR
- Marketing “Aether compiles to LLVM” without “verified AETH intermediate”  
- Dropping VM as reference semantics  

---

*End of DESIGN-LAW-FORK-F-NATIVE.md*
