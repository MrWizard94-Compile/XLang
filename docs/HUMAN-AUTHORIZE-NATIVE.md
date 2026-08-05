# Human authorization checklist — F-NATIVE (native backend law fork)

**Status:** Decision aid — **not authorized** (no product native backend)  
**Date:** 2026-08-05  

**Law:** [ADR-037](ADR-037-f-native-law-fork.md), [DESIGN-LAW-FORK-F-NATIVE.md](DESIGN-LAW-FORK-F-NATIVE.md), [THREAT_MODEL-v4-NATIVE-BACKEND.md](THREAT_MODEL-v4-NATIVE-BACKEND.md)  
**Constitution:** Agents **must not** implement a native/LLVM product path until
section 3 is completed by a human in writing.

---

## 1. Why this exists

Project law and CLM-010 currently forbid product transpile and keep execution on
the Aether VM. Opening F-NATIVE rewrites Level-4 law. That is a **human**
sovereignty decision, not an agent default.

## 2. Residual risk (must accept)

| ID | Risk | Accept? |
| --- | --- | --- |
| N1 | Native code is not VM-sandboxed; process compromise possible | ☐ |
| N2 | Codegen may diverge from VM semantics without dual-run discipline | ☐ |
| N3 | Extra supply chain (codegen toolchain) | ☐ |
| N4 | Seed may never emit native; honesty required | ☐ |
| N5 | Source→C/Rust/JS remains **forbidden**; only verified AETH→native allowed | ☐ |

## 3. Law-fork authorization phrase

A human must write **exactly** (or clearly equivalent):

> **I authorize Aether law fork F-NATIVE** under ADR-037 / threat model v4 residual risk acceptance: optional native lower of **verified AETH only**, VM remains default and reference, no Aether-source transpile to other languages.

Until that phrase appears, agents must not:

- add LLVM/cranelift product dependencies for emit  
- ship `compile --native` (or equivalent)  
- rewrite CLM-010 / AGENTS invariants for native  

## 4. After §3 — still not free-form implement

Authorization opens the fork. A **vertical ADR** (M35-class) + matrix is still
required before product code (`CONST-DEP-001`).

## 5. Explicit non-authorization

“Continue”, “do law forks”, or “implement everything” **without** the §3 phrase
are **not** F-NATIVE implement authorization.

---

*End of HUMAN-AUTHORIZE-NATIVE.md*
