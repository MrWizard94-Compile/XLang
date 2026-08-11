# Human authorization checklist — F-NATIVE (native backend law fork)

**Status:** **Authorized** 2026-08-10 (human §3 phrase accepted as clearly equivalent;
residual-risk bounds of the full §3 text remain **binding**)  
**Date:** 2026-08-05 (authorized 2026-08-10)  

**Law:** [ADR-037](ADR-037-f-native-law-fork.md), [ADR-059](ADR-059-f-native-authorized-m35a-aeth-to-c.md), [DESIGN-LAW-FORK-F-NATIVE.md](DESIGN-LAW-FORK-F-NATIVE.md), [THREAT_MODEL-v4-NATIVE-BACKEND.md](THREAT_MODEL-v4-NATIVE-BACKEND.md)  
**Constitution:** Product native paths require this authorization plus a vertical ADR + matrix.

---

## 1. Why this exists

Project law and CLM-010 currently forbid product transpile and keep execution on
the Aether VM. Opening F-NATIVE rewrites Level-4 law. That is a **human**
sovereignty decision, not an agent default.

## 2. Residual risk (accepted with authorization)

| ID | Risk | Accept? |
| --- | --- | --- |
| N1 | Native code is not VM-sandboxed; process compromise possible | ☑ |
| N2 | Codegen may diverge from VM semantics without dual-run discipline | ☑ |
| N3 | Extra supply chain (codegen toolchain) | ☑ |
| N4 | Seed may never emit native; honesty required | ☑ |
| N5 | Source→C/Rust/JS remains **forbidden**; only verified AETH→native allowed | ☑ |

## 3. Law-fork authorization phrase

A human must write **exactly** (or clearly equivalent):

> **I authorize Aether law fork F-NATIVE** under ADR-037 / threat model v4 residual risk acceptance: optional native lower of **verified AETH only**, VM remains default and reference, no Aether-source transpile to other languages.

**Recorded authorization (2026-08-10):** human wrote `I authorize Aether law fork F-NATIVE`
(clearly equivalent intent); full residual-risk bounds above are enforced as fork law.

**Reaffirmed (2026-08-10):** human again wrote `I authorize Aether law fork F-NATIVE`
when directing M35c–e work (ADR-073/076/079), including product native-object emit.

## 4. After §3 — still not free-form implement

Authorization opens the fork. Vertical ADR + matrix still required (`CONST-DEP-001`).
**M35a** (ADR-059): verified AETH → ISO C pure pilot is the first product slice.

## 5. Explicit non-authorization

“Continue”, “do law forks”, or “implement everything” **without** the §3 phrase
are **not** F-NATIVE implement authorization.

---

*End of HUMAN-AUTHORIZE-NATIVE.md*
