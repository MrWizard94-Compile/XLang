# Delivery Report — BARP root forward-call unknown-target seed-SPEAK pilot

**Date:** 2026-08-14
**Status:** Implementation and release verification complete
**Summary:** ADR-120 extends the bounded `AE-SEED-011` direct seed witness to
canonical root `forward call target` forms under a literal erroring-Whole
header marker without turning the seed into an M4 header, effect, or statement
validator.
**Scope:** ADR-120 direct-seed root forward-call unknown-target diagnostic authority reduction
**Rule IDs:** CONST-COMPLETE-001, CONST-DEP-001, CONST-DONE-001, CONST-GATE-001, DOC-SYNC-001, ENG-WARN-001, RND-INVAR-001, SEC-INPUT-001, TEST-BEHAVIOR-001

## Implemented behavior

The checked-in seed ordinary direct-call scan now has a separate state for a
trimmed top-level `weave ` line containing the literal
`-> Whole raises Whole:` marker. Only in that state, it accepts a trimmed root
line beginning exactly `forward call `. It extracts one opaque target ending at
the next space or exact end of line. If the existing top-level weave-header scan
finds no match, direct forge emits exactly one `AE-SEED-011` seed-SPEAK packet
with schema `aether.seed-error/v1`, position `1:1`, blank Bytes, and the
established unknown-call message. Product forge preserves that packet and origin.

Later-declared matching `raises Whole` targets remain valid, byte-identical
between seed and bootstrap, verified, and executable for argument-bearing and
zero-argument forward calls. An erroring-weave Text literal, a total-caller
forward, and an incomplete prefix remain outside the scan. A missing `world`
retains its higher-priority `AE-SEED-006` result.

## Delivery contents

| Artifact | Purpose |
|---|---|
| `seed/aether_seed.ae` and `seed/aether_seed.aeth` | Separate erroring-Whole state, exact forward prefix recognition, and rebuilt checked-in seed |
| `crates/xlang-core/src/lib.rs` | Explicit ADR-120 tracker and bounded direct-call contract documentation |
| `crates/xlang-core/tests/seed_self_host.rs` | Red/green direct, complete packet contract, product-origin, valid identity/run, literal, total-caller, incomplete-prefix, and priority tests |
| ADR-120 and BARP matrix | Decision plus direct, valid, boundary, priority, and product evidence |
| Current program, claims, roadmap, and progress docs | Current maturity ordering and residual status |

## Verification evidence

| Check | Current result |
|---|---|
| Test-driven regression | PASS — prior seed emitted no direct packet for canonical argument-bearing root forward-call unknown target |
| Direct seed forge | PASS — argument-bearing and zero-argument erroring-Whole root forward forms emit one blank-Bytes `AE-SEED-011` seed-SPEAK packet with schema and fixed position |
| Product merge | PASS — product compilation preserves the `AE-SEED-011`/`seed-speak` packet |
| Valid source boundary | PASS — later-declared argument-bearing and zero-argument `raises Whole` forward targets verify, match bootstrap, and run |
| False-positive boundary | PASS — erroring-weave forward-shaped Text literal and a total-caller forward emit no pilot packet |
| Malformed-prefix boundary | PASS — incomplete `forward call` does not trigger this witness |
| Priority boundary | PASS — missing world supersedes the lower-priority root forward-call witness |
| BARP tracker | PASS — ADR-120 registers while `seed_speak_emit_conformance_complete()` remains false |
| Full Constitution/release gate | PASS — 2026-08-14 release gate: pack integrity, 347-document/1,358-link integrity, formatting, deny-warning Clippy, full workspace suite, seed self-host corpus, 32 top-level seed≡bootstrap comparisons, four-way seed identity, and preview package/consumer verification |

## Security, performance, and honesty boundary

The change reuses the existing bounded source-line pass, literal prefix, opaque
target extraction, and top-level header scan. It adds one fixed header-marker
state only; it reads no new input, allocates no new runtime state, and adds no
VM, host, filesystem, process, network, shell, model, or guest authority.
Full M4 header parsing, terminality, target effect/result/signature, arguments,
resource boundary, and ownership analysis remain with the full compiler.

## Section 0 status

Completeness, dependency order, intended-behavior tests, documentation,
security scope, performance reasoning, version fidelity, deterministic seed
rebuild, and review packaging are present. The 2026-08-14 release gate passed
with zero warnings, the complete suite, four-way seed identity
`5CF25004F6C8CD9CB6AE12AADD1A3FED8EE2512A3549E63D615F255EC9584CAA`,
the 432-file technical-preview package plus `SHA-256SUMS`, and independent
consumer verification.

## Repeat verification

- `cargo test -p aether-core --test seed_self_host barp_adr120_seed_speaks_canonical_root_forward_unknown_calls`
- `cargo test -p aether-core --test seed_self_host barp_adr072_product_seed_error_packet_abi`
- `pwsh -NoProfile -File .\\tools\\aether-gate.ps1 -Mode release`

## Residuals

Broader `AE-SEED-011` and `AE-SEED-013` families, full seed SPEAK
conformance, and seed-native multi-file elaboration remain separate scoped
work. Preserve the verifier-first artifact rule, dual-compare proof, and
no-ambient-authority boundary.

---

*End of DELIVERY_REPORT-2026-08-14-BARP-ROOT-FORWARD-UNKNOWN-CALL-SPEAK.md*
