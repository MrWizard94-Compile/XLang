# Delivery Report — BARP root speak-call unknown-target seed-SPEAK pilot

**Date:** 2026-08-14
**Status:** Implementation and release verification complete
**Summary:** ADR-118 extends the bounded `AE-SEED-011` direct seed witness to
canonical root `speak call target` forms without turning the seed into an
expression parser or a Text type checker.
**Scope:** ADR-118 direct-seed root speak-call unknown-target diagnostic authority reduction
**Rule IDs:** CONST-COMPLETE-001, CONST-DEP-001, CONST-DONE-001, CONST-GATE-001, DOC-SYNC-001, ENG-WARN-001, RND-INVAR-001, SEC-INPUT-001, TEST-BEHAVIOR-001

## Implemented behavior

The checked-in seed ordinary-Whole direct-call scan now accepts a trimmed root
line beginning exactly `speak call `. It extracts one opaque target ending at
the next space or exact end of line. If the existing top-level weave-header scan
finds no match, direct forge emits exactly one `AE-SEED-011` seed-SPEAK packet
with blank Bytes and the established unknown-call message. Product forge
preserves that packet and origin.

Later-declared matching Text-result speak-call targets remain valid,
byte-identical between seed and bootstrap, verified, and executable for both
argument-bearing and zero-argument tails. Call-shaped Text literals remain
outside the scan; missing `world` remains the higher-priority `AE-SEED-006`
result. Bind, root-yield, and root-revise call behavior is unchanged.

## Delivery contents

| Artifact | Purpose |
|---|---|
| `seed/aether_seed.ae` and `seed/aether_seed.aeth` | Exact root-speak prefix recognition and rebuilt checked-in seed |
| `crates/xlang-core/src/lib.rs` | Explicit ADR-118 tracker and bounded-call contract documentation |
| `crates/xlang-core/tests/seed_self_host.rs` | Red/green direct, cardinality, blank-result, product-origin, valid identity/run, literal-boundary, and priority tests |
| ADR-118 and BARP matrix | Decision plus direct/valid/boundary/priority evidence |
| Current program/claims/roadmap/progress | Current maturity ordering and residual status |

## Verification evidence

| Check | Current result |
|---|---|
| Test-driven regression | PASS — prior seed emitted no direct packet for canonical argument-bearing root speak-call unknown target |
| Direct seed forge | PASS — argument-bearing and zero-argument root speak forms emit one blank-Bytes `AE-SEED-011` seed-SPEAK packet |
| Product merge | PASS — product compilation preserves the `AE-SEED-011`/`seed-speak` packet |
| Valid source boundary | PASS — later-declared argument-bearing and zero-argument Text-result speak-call targets verify, match bootstrap, and run |
| False-positive boundary | PASS — speak-call-shaped Text literal emits no seed packet |
| Priority boundary | PASS — missing world supersedes the lower-priority root speak-call witness |
| BARP tracker | PASS — ADR-118 registers while `seed_speak_emit_conformance_complete()` remains false |
| Full Constitution/release gate | PASS — 2026-08-14 release gate: pack integrity, 343-document/1,331-link integrity, formatting, deny-warning Clippy, full workspace suite, seed self-host corpus, 32 top-level seed≡bootstrap comparisons, four-way seed identity, and preview package/consumer verification |

## Security, performance, and honesty boundary

The change reuses the existing bounded source-line pass, literal prefix, and
top-level header scan. It reads no new input, allocates no new runtime state,
and adds no VM, host, filesystem, process, network, shell, model, or guest
authority. Target names remain opaque text. Text result validation, argument,
signature, type, effect, and ownership analysis remain with the full compiler.

## Section 0 status

Completeness, dependency order, targeted behavioral tests, documentation,
security scope, performance reasoning, version fidelity, deterministic seed
rebuild, and review packaging are present. The 2026-08-14 release gate passed
with zero warnings, the complete suite, four-way seed identity
`10121380A4747A67C2811C0350574894B1A9EE39F0B5807150007C57EFDE7CCB`,
the 428-file technical-preview package, and independent consumer verification.

## Repeat verification

- `cargo test -p aether-core --test seed_self_host barp_adr118_seed_speaks_canonical_root_speak_unknown_calls`
- `cargo test -p aether-core seed_speak_pilot_covers_lexical_structural_and_task_preflights --lib`
- `pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode release`

## Residuals

Broader `AE-SEED-011` and `AE-SEED-013` families, full seed SPEAK conformance,
and seed-native multi-file elaboration remain separate scoped work. Preserve the
verifier-first artifact rule, dual-compare proof, and no-ambient-authority
boundary.

---

*End of DELIVERY_REPORT-2026-08-14-BARP-ROOT-SPEAK-UNKNOWN-CALL-SPEAK.md*
