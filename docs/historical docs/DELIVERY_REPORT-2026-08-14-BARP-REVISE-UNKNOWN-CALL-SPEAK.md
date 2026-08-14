# Delivery Report — BARP root revise-call unknown-target seed-SPEAK pilot

**Date:** 2026-08-14
**Status:** Implementation and release verification complete
**Summary:** ADR-117 extends the bounded `AE-SEED-011` direct seed witness to
canonical root `revise name <- call target` forms without turning the seed into
a statement parser or semantic analyzer.
**Scope:** ADR-117 direct-seed root revise-call unknown-target diagnostic authority reduction
**Rule IDs:** CONST-COMPLETE-001, CONST-DEP-001, CONST-DONE-001, CONST-GATE-001, DOC-SYNC-001, ENG-WARN-001, RND-INVAR-001, SEC-INPUT-001, TEST-BEHAVIOR-001

## Delivered behavior

The checked-in seed ordinary-Whole direct-call scan now accepts a trimmed root
line beginning `revise ` and containing the existing ` <- call ` separator.
It extracts one opaque target ending at the next space or exact end of line. If
the existing top-level weave-header scan finds no match, direct forge emits
exactly one `AE-SEED-011` seed-SPEAK packet with blank Bytes and the established
unknown-call message. Product forge preserves that packet and origin.

Later-declared matching revise-call targets remain valid, byte-identical between
seed and bootstrap, verified, and executable for both argument-bearing and
zero-argument tails. Call-shaped Text literals remain outside the scan; missing
`world` remains the higher-priority `AE-SEED-006` result. Bind and root-yield
call behavior is unchanged.

## Delivered package

| Artifact | Purpose |
|---|---|
| `seed/aether_seed.ae` and `seed/aether_seed.aeth` | Exact root-revise prefix recognition and rebuilt checked-in seed |
| `crates/xlang-core/src/lib.rs` | Explicit ADR-117 tracker and bounded-call contract documentation |
| `crates/xlang-core/tests/seed_self_host.rs` | Red/green direct, cardinality, blank-result, product-origin, valid identity/run, literal-boundary, and priority tests |
| ADR-117 and BARP matrix | Decision plus direct/valid/boundary/priority evidence |
| Current program/claims/roadmap/progress | Current maturity ordering and residual status |

## Verification evidence

| Check | Result |
|---|---|
| Test-driven regression | PASS — prior seed emitted no direct packet for canonical argument-bearing root revise-call unknown target |
| Direct seed forge | PASS — argument-bearing and zero-argument revise forms emit one blank-Bytes `AE-SEED-011` seed-SPEAK packet |
| Product merge | PASS — product compilation preserves the `AE-SEED-011`/`seed-speak` packet |
| Valid source boundary | PASS — later-declared argument-bearing and zero-argument revise-call targets verify, match bootstrap, and run |
| False-positive boundary | PASS — revise-call-shaped Text literal emits no seed packet |
| Priority boundary | PASS — missing world supersedes the lower-priority revise-call witness |
| Full Constitution/release gate | PASS — 2026-08-14 release gate: pack integrity, 341-document/1,320-link integrity, formatting, deny-warning Clippy, full workspace suite, seed self-host corpus, 32 top-level seed≡bootstrap comparisons, four-way seed identity, and preview package/consumer verification |

## Security, performance, and honesty boundary

The change reuses the existing bounded source-line pass, literal separator, and
top-level header scan. It reads no new input, allocates no new runtime state,
and adds no VM, host, filesystem, process, network, shell, model, or guest
authority. Target names remain opaque text. Destination validation, signature,
type, effect, and ownership analysis remain with the full compiler.

## Section 0 status

Completeness, dependency order, targeted behavioral tests, documentation,
security scope, performance reasoning, version fidelity, deterministic seed
rebuild, and review packaging are present. The 2026-08-14 release gate passed
with zero warnings, the complete suite, four-way seed identity
`7F0878AA0E86162A8E6D4C911853A9037DB5A2DFD036646682C69A0F8ADE21A5`,
technical-preview packaging, and independent consumer verification.

## Repeat verification

- `cargo test -p aether-core --test seed_self_host barp_adr117_seed_speaks_canonical_revise_unknown_calls`
- `cargo test -p aether-core seed_speak_pilot_covers_lexical_structural_and_task_preflights --lib`
- `pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode release`

## Residuals

Broader `AE-SEED-011` and `AE-SEED-013` families, full seed SPEAK conformance,
and seed-native multi-file elaboration remain separate scoped work. Preserve the
verifier-first artifact rule, dual-compare proof, and no-ambient-authority
boundary.

---

*End of DELIVERY_REPORT-2026-08-14-BARP-REVISE-UNKNOWN-CALL-SPEAK.md*
