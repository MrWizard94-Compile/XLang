# Threat Model v4 — optional native backend (F-NATIVE)

**Status:** Draft for law-fork decision — **not binding product law** until F-NATIVE authorized  
**Date:** 2026-08-05  
**Depends on:** [DESIGN-LAW-FORK-F-NATIVE.md](DESIGN-LAW-FORK-F-NATIVE.md), [HUMAN-AUTHORIZE-NATIVE.md](HUMAN-AUTHORIZE-NATIVE.md)  
**Related Rule IDs:** `SEC-INPUT-001`, `CONST-DEP-001`

## 1. Purpose

Extend technical-preview trust boundaries if Aether may lower **verified AETH**
to native machine code. This does **not** authorize implementation by itself.

## 2. Assets (added)

| Asset | Why |
| --- | --- |
| Native object / executable output | Can run outside VM sandbox assumptions |
| Codegen backend (LLVM/cranelift/etc.) | Supply chain + correctness |
| Dual-run corpus | Detects VM vs native semantic drift |

## 3. Actors

| Actor | Trust |
| --- | --- |
| Operator | Chooses `--native` (or equivalent) opt-in |
| Host CLI / native backend | Trusted by operator who launched it |
| Verified AETH | Untrusted *content*; must verify before lower |
| Generated native code | **Hostile if codegen wrong or AETH hostile** — process-level risk |

## 4. Controls (required if implemented)

1. Verify AETH before any native lower.  
2. Opt-in CLI only; default remains VM.  
3. Dual-run corpus (exit codes / pure Whole programs) before product claims.  
4. No claim that native inherits VM isolation.  
5. No Aether-source→C/Rust product path.  

## 5. Explicit non-goals

OS sandbox for native, formal codegen proof, 1.0 “safe native” marketing.

---

*End of THREAT_MODEL-v4-NATIVE-BACKEND.md*
