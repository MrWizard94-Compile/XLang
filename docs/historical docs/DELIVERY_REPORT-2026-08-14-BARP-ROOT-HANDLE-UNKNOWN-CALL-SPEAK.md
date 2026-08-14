# Delivery Report — BARP root handle-call unknown-target seed-SPEAK pilot

**Date:** 2026-08-14
**Status:** Implementation and release verification complete
**Summary:** ADR-119 extends the bounded `AE-SEED-011` direct seed witness to
canonical root `handle call target ... into success otherwise error into code`
forms without turning the seed into an M4 effect or statement validator.
**Scope:** ADR-119 direct-seed root handle-call unknown-target diagnostic authority reduction
**Rule IDs:** CONST-COMPLETE-001, CONST-DEP-001, CONST-DONE-001, CONST-GATE-001, DOC-SYNC-001, ENG-WARN-001, RND-INVAR-001, SEC-INPUT-001, TEST-BEHAVIOR-001

## Implemented behavior

The checked-in seed ordinary-Whole direct-call scan now accepts a trimmed root
line beginning exactly `handle call `. It extracts one opaque target ending at
the first following ASCII space, and recognizes the witness only when a later
literal ` into ` and a later literal ` otherwise error into ` leave a nonempty
destination suffix. If the existing top-level weave-header scan finds no match,
direct forge emits exactly one `AE-SEED-011` seed-SPEAK packet with blank Bytes
and the established unknown-call message. Product forge preserves that packet
and origin.

Later-declared matching `raises Whole` targets remain valid, byte-identical
between seed and bootstrap, verified, and executable for argument-bearing and
zero-argument handle calls. A handle-call-shaped Text literal remains outside
the scan. A missing `world` retains its higher-priority `AE-SEED-006` result,
and an incomplete handle tail does not trigger the bounded witness.

## Delivery contents

| Artifact | Purpose |
|---|---|
| `seed/aether_seed.ae` and `seed/aether_seed.aeth` | Fixed prefix plus two-delimiter recognition and rebuilt checked-in seed |
| `crates/xlang-core/src/lib.rs` | Explicit ADR-119 tracker and bounded M4 call-contract documentation |
| `crates/xlang-core/tests/seed_self_host.rs` | Red/green direct, cardinality, blank-result, product-origin, valid identity/run, literal, incomplete-tail, and priority tests |
| ADR-119 and BARP matrix | Decision plus direct, valid, boundary, priority, and product evidence |
| Current program, claims, roadmap, and progress docs | Current maturity ordering and residual status |

## Verification evidence

| Check | Current result |
|---|---|
| Test-driven regression | PASS — prior seed emitted no direct packet for canonical argument-bearing root handle-call unknown target |
| Direct seed forge | PASS — argument-bearing and zero-argument complete-tail root handle forms emit one blank-Bytes `AE-SEED-011` seed-SPEAK packet |
| Product merge | PASS — product compilation preserves the `AE-SEED-011`/`seed-speak` packet |
| Valid source boundary | PASS — later-declared argument-bearing and zero-argument `raises Whole` handle targets verify, match bootstrap, and run |
| False-positive boundary | PASS — handle-call-shaped Text literal emits no seed packet |
| Malformed-tail boundary | PASS — incomplete `handle call` tail does not trigger this witness |
| Priority boundary | PASS — missing world supersedes the lower-priority root handle-call witness |
| BARP tracker | PASS — ADR-119 registers while `seed_speak_emit_conformance_complete()` remains false |
| Full Constitution/release gate | PASS — 2026-08-14 release gate: pack integrity, 345-document/1,344-link integrity, formatting, deny-warning Clippy, full workspace suite, seed self-host corpus, 32 top-level seed≡bootstrap comparisons, four-way seed identity, and preview package/consumer verification |

## Security, performance, and honesty boundary

The change reuses the existing bounded source-line pass, literal prefixes,
delimiter search, and top-level header scan. It reads no new input, allocates no
new runtime state, and adds no VM, host, filesystem, process, network, shell,
model, or guest authority. Target names and tail text remain opaque. M4 effect
eligibility, result type, argument modes/types/count, destination distinctness
and mutability, terminality, ownership, and resource analysis remain with the
full compiler.

## Section 0 status

Completeness, dependency order, intended-behavior tests, documentation,
security scope, performance reasoning, version fidelity, deterministic seed
rebuild, and review packaging are present. The 2026-08-14 release gate passed
with zero warnings, the complete suite, four-way seed identity
`21B15C463D7DBB0D91C43012D6DB85BC11C97CCCCA89FBCC679845D439A1F31F`,
the 430-file technical-preview package plus `SHA-256SUMS`, and independent
consumer verification.

## Repeat verification

- `cargo test -p aether-core --test seed_self_host barp_adr119_seed_speaks_canonical_root_handle_unknown_calls`
- `cargo test -p aether-core --test seed_self_host barp_adr072_product_seed_error_packet_abi`
- `pwsh -NoProfile -File .\\tools\\aether-gate.ps1 -Mode release`

## Residuals

Broader `AE-SEED-011` and `AE-SEED-013` families, full seed SPEAK
conformance, and seed-native multi-file elaboration remain separate scoped
work. Preserve the verifier-first artifact rule, dual-compare proof, and
no-ambient-authority boundary.

---

*End of DELIVERY_REPORT-2026-08-14-BARP-ROOT-HANDLE-UNKNOWN-CALL-SPEAK.md*
