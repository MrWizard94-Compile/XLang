# Law fork F-NATIVE — optional native / machine-code backend

**Status:** Design decision package — **authorized** 2026-08-10; product pilot ADR-059  
**Date:** 2026-08-05  
**Decision records:** [ADR-037](ADR-037-f-native-law-fork.md), [HUMAN-AUTHORIZE-NATIVE.md](HUMAN-AUTHORIZE-NATIVE.md)  
**Threat draft:** [THREAT_MODEL-v4-NATIVE-BACKEND.md](THREAT_MODEL-v4-NATIVE-BACKEND.md)  
**Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `SEC-INPUT-001`, `RND-INVAR-001`, `CONST-CONTRACT-001`

---

## 1. Purpose

Decide whether Aether may ever emit or run **native machine code** in addition to
(or instead of) the Aether VM, under what authority boundaries, and how that
interacts with CLM-010 (“source is not translated to another language”).

This document is **not** an implement go. Product code requires:

1. Human residual-risk acceptance via [HUMAN-AUTHORIZE-NATIVE.md](HUMAN-AUTHORIZE-NATIVE.md)  
2. ADR-037 status flip to implementable  
3. A vertical-slice design (e.g. M35) with matrix + dual-compare plan  

## 2. Default law today (if fork is rejected)

| Rule | Default |
| --- | --- |
| Execution | Verified AETH on the **Aether VM** only |
| Lowering | Source → AETH only (seed or bootstrap) |
| CLM-010 | No transpile of Aether source to C/Rust/JS/LLVM IR as a product compiler |
| Performance path | VM improvements, optional future **AETH-level** JIT *if* separately ADRed and still not “source transpile” |

Rejecting F-NATIVE keeps the cleanest verify-before-run story and the strongest
local-first “one artifact format” claim.

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

## 6. First implementable slice (only after authorize)

**Not in this package.** Candidate later vertical (M35-class):

1. `aether compile --native-object` (name TBD) from **verified** AETH only  
2. Whole-only pure programs (no host I/O, no foreign) dual-run VM vs native exit code  
3. Explicit non-goals: no FFI, no nursery, no grant I/O in v1 native slice  

## 7. Stop conditions

- Implementing native without HUMAN-AUTHORIZE-NATIVE phrase  
- Marketing “Aether compiles to LLVM” without “verified AETH intermediate”  
- Dropping VM as reference semantics  

---

*End of DESIGN-LAW-FORK-F-NATIVE.md*
