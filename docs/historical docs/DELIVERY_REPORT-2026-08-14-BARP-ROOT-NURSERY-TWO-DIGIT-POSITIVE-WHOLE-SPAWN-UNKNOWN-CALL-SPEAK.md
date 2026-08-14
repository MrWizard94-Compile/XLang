# Delivery Report — BARP root nursery positive two-digit Whole spawn unknown-target seed-SPEAK pilot

**Date:** 2026-08-14
**Status:** Implementation and release verification complete
**Summary:** ADR-123 extends the bounded `AE-SEED-011` nursery witness from one digit to one exact positive two-digit decimal Whole (`10`–`99`) without turning the seed into an M7 parser, task checker, or scheduler.
**Scope:** ADR-123 direct-seed root-nursery positive two-digit Whole spawn unknown-target diagnostic authority reduction
**Rule IDs:** CONST-COMPLETE-001, CONST-DEP-001, CONST-DONE-001, CONST-GATE-001, DOC-SYNC-001, ENG-WARN-001, RND-INVAR-001, SEC-INPUT-001, TEST-BEHAVIOR-001

**Later scoped extension:** ADR-124 independently adds only the immediate exact
`bright` Truth argument shape; it does not broaden this ADR's positive
two-digit-Whole decision.

## Implemented behavior

After ADR-121's zero-argument and ADR-122's one-digit branches decline a source, the checked-in seed recognizes a literal ordinary total-Whole root `together:` only when its next physical line is exactly four-space `spawn call target digits into destination`. `digits` must be exactly two ASCII decimal characters, with the first in `1`–`9` and the second in `0`–`9`; the next byte must begin literal ` into ` and a destination suffix must remain.

An absent top-level declaration then emits exactly one blank-Bytes `AE-SEED-011` seed-SPEAK packet with schema `aether.seed-error/v1`, origin `seed-speak`, fixed position `1:1`, and the established unknown-call message. Product forge preserves the packet and origin. A later-declared checkpointed task with one `Whole` parameter remains valid, byte-identical between seed and bootstrap, verified, and executable with its two-digit argument value.

## Delivery contents

| Artifact | Purpose |
|---|---|
| `seed/aether_seed.ae` and `seed/aether_seed.aeth` | Exact positive-two-digit branch after ADR-122 one-digit recognition and rebuilt checked-in seed |
| `crates/xlang-core/src/lib.rs` | Explicit ADR-123 tracker and bounded two-digit nursery-call contract documentation |
| `crates/xlang-core/tests/seed_self_host.rs` | Red/green direct, packet, product-origin, valid identity/run, lexical/caller-state/delimiter/priority tests |
| ADR-123 and BARP matrix | Decision plus direct, lexical endpoint, valid, boundary, priority, and product evidence |
| Current program, claims, roadmap, and progress docs | Current maturity ordering and residual status |

## Verification evidence

| Check | Current result |
|---|---|
| Test-driven regression | PASS — the ADR-122 seed emitted no direct packet for a canonical two-digit root-nursery spawn unknown target |
| Direct seed forge | PASS — literal `42` child emits one blank-Bytes `AE-SEED-011` seed-SPEAK packet with schema and fixed position |
| Product merge | PASS — product compilation preserves the `AE-SEED-011` / `seed-speak` packet |
| Lexical endpoints | PASS — `10` and `99` reach the bounded witness; leading-zero literals retain literal priority |
| Valid source boundary | PASS — later-declared checkpointed one-Whole task target verifies, matches bootstrap, and exits `42` |
| False-positive boundary | PASS — three-digit, signed, `dim`/name/general-Truth, multi-argument, delayed, descendant, and non-task-target sources emit no pilot packet |
| Delimiter boundary | PASS — a missing destination suffix does not trigger this witness |
| Caller/priority boundary | PASS — erroring parent emits no pilot packet; missing world supersedes the lower-priority witness |
| BARP tracker | PASS — ADR-123 registers while `seed_speak_emit_conformance_complete()` remains false |
| Seed Profile variant boundary | PASS — ADR-123 adds no local binding and preserves the established `v132` unused-local self-host variant probe |
| Full Constitution/release gate | PASS — 2026-08-14: pack v5.0.1, 353 Markdown files / 1,395 local links, zero-warning workspace build and test corpus, four-way seed identity `C993C6F5BB0967BB900E574B7B0671A6419243CA2993951EFACEE67594796AAD`, and a 438-file technical-preview package plus `SHA-256SUMS` with independent consumer verification |

## Security, performance, and honesty boundary

The change reuses the existing bounded source-line pass, literal prefix, opaque target extraction, declaration-header scan, and a dead Whole scratch slot. It reads no new input, allocates no new runtime state, and adds no VM, host, filesystem, process, network, shell, model, or guest authority. General M7 argument parsing and validation, complete Whole syntax/range handling, task identity, destination, effect/result, signature, nursery policy, resource boundary, scheduling, and ownership remain with the full compiler.

## Section 0 status

The targeted red/green test, rebuilt seed, focused tracker evidence, and full Constitution/release gate pass. Documentation, security scope, performance reasoning, version fidelity, four-way deterministic artifact identity, technical-preview packaging, and independent consumer verification are synchronized. The delivery satisfies its bounded Section 0 gate without claiming full seed-SPEAK parity.

## Repeat verification

- `cargo test -p aether-core --test seed_self_host barp_adr123_seed_speaks_canonical_root_nursery_two_digit_positive_whole_spawn_unknown_calls`
- `cargo test -p aether-core --test seed_self_host barp_adr072_product_seed_error_packet_abi`
- `pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode release`

## Residuals

Zero/one-digit handling remains ADR-121/122; signed, leading-zero, three-or-more digit, `dim`, names, general Truth, general atom, and multi-argument spawn calls; non-immediate/nested nursery children; broader `AE-SEED-011` and `AE-SEED-013` families; full seed SPEAK conformance; and seed-native multi-file elaboration remain separate scoped work. ADR-124 separately admits only exact `bright`. Preserve the verifier-first artifact rule, dual-compare proof, and no-ambient-authority boundary.

---

*End of DELIVERY_REPORT-2026-08-14-BARP-ROOT-NURSERY-TWO-DIGIT-POSITIVE-WHOLE-SPAWN-UNKNOWN-CALL-SPEAK.md*
