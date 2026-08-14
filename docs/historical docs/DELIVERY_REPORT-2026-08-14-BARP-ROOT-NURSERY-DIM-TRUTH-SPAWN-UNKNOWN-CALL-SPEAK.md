# Delivery Report — BARP root nursery exact dim Truth spawn unknown-target seed-SPEAK pilot

**Date:** 2026-08-14
**Status:** Implementation and release verification complete
**Summary:** ADR-125 extends the bounded `AE-SEED-011` nursery witness with one exact `dim` `Truth` literal without turning the seed into an M7 parser, task checker, or scheduler.
**Scope:** ADR-125 direct-seed root-nursery exact dim Truth spawn unknown-target diagnostic authority reduction
**Rule IDs:** CONST-COMPLETE-001, CONST-DEP-001, CONST-DONE-001, CONST-GATE-001, DOC-SYNC-001, ENG-WARN-001, RND-INVAR-001, SEC-INPUT-001, TEST-BEHAVIOR-001

## Implemented behavior

After ADR-121's zero-argument, ADR-122's one-digit, ADR-123's positive
two-digit, and ADR-124's exact-`bright` branches decline a source, the
checked-in seed recognizes a literal ordinary total-Whole root `together:` only
when its next physical line is exactly four-space
`spawn call target dim into destination`. The opaque target must be nonempty,
the literal span after its first delimiter must be exactly `dim into `, and a
destination suffix must remain.

An absent top-level declaration then emits exactly one blank-Bytes `AE-SEED-011`
seed-SPEAK packet with schema `aether.seed-error/v1`, origin `seed-speak`, fixed
position `1:1`, and the established unknown-call message. Product forge
preserves the packet and origin. A later-declared checkpointed task with one
`Truth` parameter remains valid, byte-identical between seed and bootstrap,
verified, and executable with `dim`.

## Delivery contents

| Artifact | Purpose |
|---|---|
| `seed/aether_seed.ae` and `seed/aether_seed.aeth` | Exact `dim into ` branch after ADR-124 and rebuilt checked-in seed |
| `crates/xlang-core/src/lib.rs` | Explicit ADR-125 tracker and bounded exact-Truth nursery-call contract documentation |
| `crates/xlang-core/tests/seed_self_host.rs` | Red/green direct, packet, product-origin, valid identity/run, predecessor, lexical/signature/caller-state/delimiter/priority tests |
| ADR-125 and BARP matrix | Decision plus direct, valid, boundary, priority, and product evidence |
| Current program, claims, roadmap, and progress docs | Current maturity ordering and residual status |

## Verification evidence

| Check | Current result |
|---|---|
| Test-driven regression | PASS — the ADR-124 seed emitted no direct packet for a canonical dim root-nursery spawn unknown target |
| Direct seed forge | PASS — exact `dim` child emits one blank-Bytes `AE-SEED-011` seed-SPEAK packet with schema and fixed position |
| Product merge | PASS — product compilation preserves the `AE-SEED-011` / `seed-speak` packet |
| Valid source boundary | PASS — later-declared checkpointed one-Truth task target verifies, matches bootstrap, and exits `42` |
| Predecessor boundary | PASS — exact `bright` remains an ADR-124 `AE-SEED-011` witness |
| False-positive boundary | PASS — name, `not dim`, repeated-argument, missing-destination, delayed, descendant, non-task-target, wrong-parameter, and erroring-parent sources emit no ADR-125 packet; their existing seed failure, when any, retains its prior authority |
| Priority boundary | PASS — missing world emits `AE-SEED-006`, not `AE-SEED-011` |
| BARP tracker | PASS — ADR-125 registers while `seed_speak_emit_conformance_complete()` remains false |
| Seed Profile variant boundary | PASS — ADR-125 adds no local binding and preserves the established `v132` unused-local self-host variant probe |
| Full Constitution/release gate | PASS — 2026-08-14: pack v5.0.1, 357 Markdown files / 1,427 local links, zero-warning workspace build and test corpus, four-way seed identity `110D8FF718D7FF578903C75398CB465C6499F5CBF2995F7C5617B4802DEFD6DB`, and a 442-file technical-preview package plus `SHA-256SUMS` with independent consumer verification |

## Security, performance, and honesty boundary

The change reuses the existing bounded source-line pass, literal prefix, opaque
target extraction, declaration-header scan, and scratch state. It reads no new
input, allocates no new runtime state, and adds no VM, host, filesystem,
process, network, shell, model, or guest authority. General M7 expression and
argument parsing, complete Truth syntax/type handling, task identity,
destination, effect/result, signature, nursery policy, resource boundary,
scheduling, and ownership remain with the full compiler.

## Section 0 status

The targeted red/green test, rebuilt seed, focused tracker evidence, and full
Constitution/release gate pass. Documentation, security scope, performance
reasoning, version fidelity, and deterministic artifact evidence are
synchronized. This satisfies the bounded Section 0 delivery gate without
claiming full seed SPEAK parity.

## Repeat verification

- `cargo test -p aether-core --test seed_self_host barp_adr125_seed_speaks_canonical_root_nursery_dim_truth_spawn_unknown_calls`
- `cargo test -p aether-core --test seed_self_host barp_adr072_product_seed_error_packet_abi`
- `pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode release`

## Residuals

Zero/one/two-digit Whole handling remains ADR-121/122/123; exact `bright`
remains ADR-124; Truth names, unary/general Truth, and multi-argument spawn
calls; non-immediate/nested nursery children; broader `AE-SEED-011` and
`AE-SEED-013` families; full seed SPEAK conformance; and seed-native multi-file
elaboration remain separate scoped work. Preserve the verifier-first artifact
rule, dual-compare proof, and no-ambient-authority boundary.

**Later scoped extension:** ADR-126 separately recognizes only the immediate
exact empty-Text child `spawn call target "" into destination`; it does not
broaden this exact-dim delivery.

---

*End of DELIVERY_REPORT-2026-08-14-BARP-ROOT-NURSERY-DIM-TRUTH-SPAWN-UNKNOWN-CALL-SPEAK.md*
