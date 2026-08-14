# Delivery Report — BARP root nursery single-digit Whole spawn unknown-target seed-SPEAK pilot

**Date:** 2026-08-14
**Status:** Implementation and release verification complete
**Summary:** ADR-122 extends the bounded `AE-SEED-011` nursery witness from zero arguments to one exact single-digit decimal Whole argument without turning the seed into an M7 parser, task checker, or scheduler.
**Scope:** ADR-122 direct-seed root-nursery single-digit Whole spawn unknown-target diagnostic authority reduction
**Rule IDs:** CONST-COMPLETE-001, CONST-DEP-001, CONST-DONE-001, CONST-GATE-001, DOC-SYNC-001, ENG-WARN-001, RND-INVAR-001, SEC-INPUT-001, TEST-BEHAVIOR-001

## Implemented behavior

After ADR-121's zero-argument branch declines a source, the checked-in seed recognizes a literal ordinary total-Whole root `together:` only when its next physical line is exactly four-space `spawn call target digit into destination`. The target's first following space must precede one ASCII decimal digit, and the next byte must begin literal ` into ` with a destination suffix. An absent top-level declaration then emits exactly one blank-Bytes `AE-SEED-011` seed-SPEAK packet with schema `aether.seed-error/v1`, origin `seed-speak`, fixed position `1:1`, and the established unknown-call message. Product forge preserves the packet and origin.

A later-declared checkpointed task with one `Whole` parameter remains valid, byte-identical between seed and bootstrap, verified, and executable with its argument value.

## Delivery contents

| Artifact | Purpose |
|---|---|
| `seed/aether_seed.ae` and `seed/aether_seed.aeth` | Exact single-digit branch after ADR-121 zero-argument recognition and rebuilt checked-in seed |
| `crates/xlang-core/src/lib.rs` | Explicit ADR-122 tracker and bounded single-digit nursery-call contract documentation |
| `crates/xlang-core/tests/seed_self_host.rs` | Red/green direct, packet, product-origin, valid identity/run, lexical/caller-state/delimiter/priority tests |
| ADR-122 and BARP matrix | Decision plus direct, valid, boundary, priority, and product evidence |
| Current program, claims, roadmap, and progress docs | Current maturity ordering and residual status |

## Verification evidence

| Check | Current result |
|---|---|
| Test-driven regression | PASS — prior seed emitted no direct packet for a canonical one-digit root-nursery spawn unknown target |
| Direct seed forge | PASS — literal `3` child emits one blank-Bytes `AE-SEED-011` seed-SPEAK packet with schema and fixed position |
| Product merge | PASS — product compilation preserves the `AE-SEED-011` / `seed-speak` packet |
| Valid source boundary | PASS — later-declared checkpointed one-Whole task target verifies, matches bootstrap, and exits `3` |
| False-positive boundary | PASS — multi-digit, negative, Truth, multi-argument, delayed, and descendant children emit no pilot packet |
| Delimiter boundary | PASS — a missing destination suffix does not trigger this witness |
| Caller/priority boundary | PASS — erroring parent emits no pilot packet; missing world supersedes the lower-priority witness |
| BARP tracker | PASS — ADR-122 registers while `seed_speak_emit_conformance_complete()` remains false |
| Seed Profile variant boundary | PASS — ADR-122 adds no local binding and preserves the established `v132` unused-local self-host variant probe |
| Full Constitution/release gate | PASS — 2026-08-14 release gate: pack integrity, 351-document/1,382-link integrity, formatting, deny-warning Clippy, full workspace suite, 41-test seed self-host corpus, 32 top-level seed≡bootstrap comparisons, four-way seed identity, and preview package/consumer verification |

## Security, performance, and honesty boundary

The change reuses the existing bounded source-line pass, literal prefix, opaque target extraction, declaration-header scan, and a dead Whole scratch slot. It reads no new input, allocates no new runtime state, and adds no VM, host, filesystem, process, network, shell, model, or guest authority. General M7 argument parsing and validation, task identity, destination, effect/result, signature, nursery policy, resource boundary, scheduling, and ownership remain with the full compiler.

## Section 0 status

Completeness, dependency order, intended-behavior tests, documentation, security scope, performance reasoning, version fidelity, deterministic seed rebuild, and review packaging are present. The 2026-08-14 release gate passed with zero warnings, the complete suite, four-way seed identity `F72DA6E65D06158729D3F4DD5E2689D698EA1E23B97F0F3F83B54BFE987D61C4`, the 436-file technical-preview package plus `SHA-256SUMS`, and independent consumer verification.

## Repeat verification

- `cargo test -p aether-core --test seed_self_host barp_adr122_seed_speaks_canonical_root_nursery_single_digit_whole_spawn_unknown_calls`
- `cargo test -p aether-core --test seed_self_host barp_adr072_product_seed_error_packet_abi`
- `pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode release`

## Residuals

Multi-digit, signed, general atom, and multi-argument spawn calls; non-immediate/nested nursery children; broader `AE-SEED-011` and `AE-SEED-013` families; full seed SPEAK conformance; and seed-native multi-file elaboration remain separate scoped work. Preserve the verifier-first artifact rule, dual-compare proof, and no-ambient-authority boundary.

---

*End of DELIVERY_REPORT-2026-08-14-BARP-ROOT-NURSERY-SINGLE-DIGIT-WHOLE-SPAWN-UNKNOWN-CALL-SPEAK.md*
