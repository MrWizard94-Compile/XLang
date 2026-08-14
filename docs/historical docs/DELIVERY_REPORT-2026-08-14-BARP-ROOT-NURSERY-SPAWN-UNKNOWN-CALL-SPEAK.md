# Delivery Report — BARP root nursery zero-argument spawn unknown-target seed-SPEAK pilot

**Date:** 2026-08-14
**Status:** Implementation and release verification complete
**Summary:** ADR-121 extends the bounded `AE-SEED-011` direct seed witness to a
literal total root nursery with one immediate zero-argument spawn child, without
turning the seed into an M7 parser, task checker, or scheduler.
**Scope:** ADR-121 direct-seed root-nursery zero-argument spawn unknown-target diagnostic authority reduction
**Rule IDs:** CONST-COMPLETE-001, CONST-DEP-001, CONST-DONE-001, CONST-GATE-001, DOC-SYNC-001, ENG-WARN-001, RND-INVAR-001, SEC-INPUT-001, TEST-BEHAVIOR-001

## Implemented behavior

The checked-in seed direct-call scan now records a prior-line state only for a
literal ordinary total-Whole root `together:` line. On the next physical source
line only, it accepts exactly four-space `spawn call target into destination`
with no intervening argument: the first space after the opaque target must be
the ` into ` delimiter, and a suffix must remain. When no canonical top-level
declaration header matches that target, direct forge emits exactly one
`AE-SEED-011` seed-SPEAK packet with schema `aether.seed-error/v1`, position
`1:1`, blank Bytes, and the established unknown-call message. Product forge
preserves the packet and origin.

A later-declared checkpointed task target remains valid, byte-identical between
seed and bootstrap, verified, and executable. A spawn-shaped Text literal, an
erroring parent nursery, incomplete delimiter forms, and missing `world` remain
outside or above the scanner.

## Delivery contents

| Artifact | Purpose |
|---|---|
| `seed/aether_seed.ae` and `seed/aether_seed.aeth` | One-line nursery parent state, exact immediate zero-argument spawn recognition, and rebuilt checked-in seed |
| `crates/xlang-core/src/lib.rs` | Explicit ADR-121 tracker and bounded nursery-call contract documentation |
| `crates/xlang-core/tests/seed_self_host.rs` | Red/green direct, complete packet contract, product-origin, valid identity/run, literal, caller-state, delimiter, and priority tests |
| ADR-121 and BARP matrix | Decision plus direct, valid, boundary, priority, and product evidence |
| Current program, claims, roadmap, and progress docs | Current maturity ordering and residual status |

## Verification evidence

| Check | Current result |
|---|---|
| Test-driven regression | PASS — prior seed emitted no direct packet for a canonical zero-argument root-nursery spawn unknown target |
| Direct seed forge | PASS — literal root nursery child emits one blank-Bytes `AE-SEED-011` seed-SPEAK packet with schema and fixed position |
| Product merge | PASS — product compilation preserves the `AE-SEED-011` / `seed-speak` packet |
| Valid source boundary | PASS — later-declared checkpointed task target verifies, matches bootstrap, and exits with the expected result |
| False-positive boundary | PASS — spawn-shaped Text literal, erroring parent, delayed/descendant child, and argument-bearing child emit no pilot packet |
| Delimiter boundary | PASS — missing `into` or destination suffix does not trigger this witness |
| Priority boundary | PASS — missing world supersedes the lower-priority root-nursery spawn witness |
| BARP tracker | PASS — ADR-121 registers while `seed_speak_emit_conformance_complete()` remains false |
| Seed Profile variant boundary | PASS — ADR-121 uses fresh `v133`, preserving the established `v132` unused-local self-host variant probe |
| Full Constitution/release gate | PASS — 2026-08-14 release gate: pack integrity, 349-document/1,371-link integrity, formatting, deny-warning Clippy, full workspace suite, seed self-host corpus, 32 top-level seed≡bootstrap comparisons, four-way seed identity, and preview package/consumer verification |

## Security, performance, and honesty boundary

The change reuses the existing bounded source-line pass, literal prefix, opaque
target extraction, and declaration-header scan. It adds one Whole line-state
flag only; it reads no new input, allocates no new runtime state, and adds no
VM, host, filesystem, process, network, shell, model, or guest authority.
M7 parsing, task identity, arguments, destination, effect/result/signature,
nursery policy, resource boundary, scheduling, and ownership analysis remain
with the full compiler.

## Section 0 status

Completeness, dependency order, intended-behavior tests, documentation,
security scope, performance reasoning, version fidelity, deterministic seed
rebuild, and review packaging are present. The 2026-08-14 release gate passed
with zero warnings, the complete suite, four-way seed identity
`29B9CE55B0844B5A07FA2279CBB989723AE635B869B3A03084E92C921197F898`,
the 434-file technical-preview package plus `SHA-256SUMS`, and independent
consumer verification.

## Repeat verification

- `cargo test -p aether-core --test seed_self_host barp_adr121_seed_speaks_canonical_root_nursery_zero_argument_spawn_unknown_calls`
- `cargo test -p aether-core --test seed_self_host barp_adr072_product_seed_error_packet_abi`
- `pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode release`

## Residuals

Argument-bearing spawn calls, non-immediate or nested nursery children, broader
`AE-SEED-011` and `AE-SEED-013` families, full seed SPEAK conformance, and
seed-native multi-file elaboration remain separate scoped work. Preserve the
verifier-first artifact rule, dual-compare proof, and no-ambient-authority
boundary.

---

*End of DELIVERY_REPORT-2026-08-14-BARP-ROOT-NURSERY-SPAWN-UNKNOWN-CALL-SPEAK.md*
