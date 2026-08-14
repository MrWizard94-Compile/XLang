# Delivery Report — BARP zero-argument unknown-call seed-SPEAK pilot

**Date:** 2026-08-14
**Status:** Implementation and release verification complete
**Summary:** ADR-116 extends the bounded AE-SEED-011 direct seed witness to
canonical zero-argument calls without turning the seed into a call parser.
**Scope:** ADR-116 direct-seed zero-argument unknown-call diagnostic authority reduction
**Rule IDs:** CONST-COMPLETE-001, CONST-DEP-001, CONST-DONE-001, CONST-GATE-001, DOC-SYNC-001, ENG-WARN-001, RND-INVAR-001, SEC-INPUT-001, TEST-BEHAVIOR-001

## Delivered behavior

The checked-in seed's ordinary-Whole direct-call scan now accepts an
end-of-line target in bind-call and root yield-call forms. If no matching
top-level declared weave header exists, direct forge emits exactly one
AE-SEED-011 seed-speak packet with blank Bytes and the established stable
unknown-call message. Product forge preserves that packet and origin.

Declared zero-argument weaves remain valid, byte-identical between seed and
bootstrap, and executable. Call-shaped Text literals remain outside the scan;
a missing world remains the higher-priority AE-SEED-006 result. The
argument-bearing ADR-110/111 forms are unchanged.

## Delivered package

| Artifact | Purpose |
|---|---|
| seed/aether_seed.ae and seed/aether_seed.aeth | End-of-line zero-argument target recognition and rebuilt checked-in seed |
| crates/xlang-core/src/lib.rs | Explicit ADR-116 tracker and bounded-call contract documentation |
| crates/xlang-core/tests/seed_self_host.rs | Red/green direct, cardinality, blank-result, product-origin, valid identity/run, literal-boundary, and priority tests |
| ADR-116 and BARP matrix | Decision plus direct/valid/boundary/priority evidence |
| Current program/claims/roadmap/progress | Current maturity ordering and residual status |

## Verification evidence

| Check | Result |
|---|---|
| Test-driven regression | PASS — prior seed emitted no direct packet for canonical zero-argument unknown bind |
| Direct seed forge | PASS — bind and root-yield forms emit one blank-Bytes AE-SEED-011 seed-SPEAK packet |
| Valid source boundary | PASS — later-declared zero-argument weave calls verify, match bootstrap, and run |
| False-positive boundary | PASS — call-shaped Text literal emits no seed packet |
| Priority boundary | PASS — missing world supersedes the lower-priority call witness |
| Full Constitution/release gate | PASS — 2026-08-14 release gate: pack integrity, 339-document/1,310-link integrity, formatting, deny-warning Clippy, full workspace suite, seed self-host corpus, 32 top-level seed≡bootstrap comparisons, four-way seed identity, and preview package/consumer verification |

## Security, performance, and honesty boundary

The change reuses the existing bounded source-line pass and adds only two
end-of-line branches. It reads no new input, allocates no new runtime state, and
adds no VM, host, filesystem, process, network, shell, model, or guest
authority. Target names remain opaque text for a pre-existing top-level-header
existence scan. General call parsing and semantic analysis remain with the full
compiler.

## Section 0 status

Completeness, dependency order, targeted behavioral tests, documentation,
security scope, performance reasoning, version fidelity, deterministic seed
rebuild, and review packaging are present. The 2026-08-14 release gate passed
with zero warnings, the complete suite, four-way seed identity
`E05584DCBD90A35C14BB99EE4472A4DCFF0A6AE4C06430F03CE1DE1E767E407F`,
technical-preview packaging, and independent consumer verification.

## Repeat verification

- cargo test -p aether-core --test seed_self_host barp_adr116_seed_speaks_canonical_zero_argument_unknown_calls
- cargo test -p aether-core seed_speak_pilot_covers_lexical_structural_and_task_preflights --lib
- pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode release

## Residuals

Broader AE-SEED-011 and AE-SEED-013 families, full seed SPEAK conformance, and
seed-native multi-file elaboration remain separate scoped work. Preserve the
verifier-first artifact rule, dual-compare proof, and no-ambient-authority
boundary.

---

*End of DELIVERY_REPORT-2026-08-14-BARP-ZERO-ARG-UNKNOWN-CALL-SPEAK.md*
