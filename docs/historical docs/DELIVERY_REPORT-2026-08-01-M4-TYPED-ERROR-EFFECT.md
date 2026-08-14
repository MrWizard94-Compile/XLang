# Delivery Report: Aether M4 typed abortive error effect

**Status:** Verified Aether 0.7 language implementation; not a published or
signed public release
**Date:** 2026-08-01
**Scope:** Aether 0.7 source, AETH v7, verifier, VM, seed compiler, local
authoring v2, and CLI proof surface
**Rule IDs:** CONST-GATE-001, CONST-DONE-001, ENG-WARN-001,
TEST-BEHAVIOR-001, SEC-INPUT-001, DOC-SYNC-001, DOC-ADR-001,
RND-INVAR-001

## Delivered boundary

Aether 0.7 adds exactly one explicit, typed error capability:

```aether
weave leaf [value: Whole] -> Whole raises Whole:
  raise value

weave main [] -> Whole:
  bind mutable success <- 0
  bind mutable code <- 0
  handle call leaf 17 into success otherwise error into code
```

`Error[Whole]` is abortive, not resumptive. An erroring weave returns `Whole`
on its normal path or exits with one `Whole` code. A total caller can terminate
with a one-line `handle`; an erroring caller can terminate with `forward`.
Ordinary `call` cannot invoke an erroring weave, and `main` remains total.

This milestone deliberately does **not** add effect rows, inference, arbitrary
error payloads, resumption, exceptions, resource-carrying effects, cleanup
handlers, concurrency, or host exceptions. `raise`, `forward`, and `handle`
are permitted only at a clean root boundary with no live owner, loan, arena,
Buffer, or resource-outcome state.

The executable contract is [AETHER_0.7.md](AETHER_0.7.md). The design and
decision record are [DESIGN-M4-TYPED-ERROR-EFFECTS.md](DESIGN-M4-TYPED-ERROR-EFFECTS.md)
and [ADR-007](ADR-007-m4-typed-error-effect.md). The explicit claims/evidence
map is [M4-VALIDATION-MATRIX.md](M4-VALIDATION-MATRIX.md).

## Artifact, seed, and authoring changes

- New compilation emits deterministic AETH v7. Verified v4, v5, and v6 inputs
  retain their existing interpretations.
- v7 adds a per-weave effect tag and verified `RAISE`, `FORWARD_CALL`, and
  `HANDLE_CALL` instructions. The verifier rejects unknown effect tags,
  effectful `main`, error result types other than `Whole`, invalid effect calls,
  an effect instruction under a forged total signature, and v7 data presented
  as an older format.
- The Aether-written seed compiler parses, emits, and self-forges the complete
  bounded M4 surface. The checked-in `seed/aether_seed.aeth` is v7.
- `aether.ast/v2`, `aether.edit/v2`, and `aether.diagnostic/v2` expose the
  `effect`, `Raise`, `Forward`, and `Handle` structures to local AI tooling.
  v1 schemas/protocol documentation remain historical and are not silently
  reinterpreted.
- `examples/error-effect.ae` provides the shipped source example for raise,
  forward, and handle behavior.

## Final quality-gate record

All commands below exited zero on the final source tree.

| Command / evidence | Result |
| --- | --- |
| `pwsh -File "../../AGENTS Constitution/tools/verify-pack.ps1"` | Pass: `GOV-INT-001`, constitution pack 5.0.1. |
| `cargo fmt --all -- --check` | Pass. |
| `cargo clippy -p aether-core -p aether-cli -- -D warnings` | Pass: zero warnings. |
| `cargo test -p aether-core` | Pass: 43 core tests, 10 M4 semantic-model tests, and 6 seed/self-host tests. The full run took 1,785.3 seconds. |
| `cargo test -p aether-cli` | Pass: 2 CLI behavior tests. |
| JSON parse of all v2 schemas | Pass: AST, edit, and diagnostic schema files parse as JSON. |
| `aether check/compile/run examples/welcome.ae` | Pass: default seed path produced a verified artifact; it printed `Aether` and exited 73. |
| `aether check/compile/run examples/error-effect.ae` | Pass: default seed path produced a verified artifact and exited 17 through the terminal handler. |
| Error-effect bootstrap/seed comparison | Pass: both artifacts SHA-256 `EFBF0AB50C97F49B0111583D237E70F09D929B8443FE6EE6E959A096ABB591B5`. |
| CLI seed bootstrap + forge | Pass: bootstrap and self-forged seed artifacts SHA-256 `E8940AF4B3708FA1B0BF6E3FF761DBA8D47346CAD103171B891818876C3A0FAD`. |
| `cargo build --release -p aether-cli` | Pass: optimized Windows CLI binary built. |
| `target\release\aether.exe version` and release compile/run | Pass: reports Aether 0.7.0; release binary seed-compiles `error-effect.ae` and exits 17. |

## Changed-file manifest

| Area | Purpose |
| --- | --- |
| `crates/xlang-core/src/lib.rs` | Implements the 0.7 grammar, semantic validation, AETH v7 emitter/parser/verifier, VM exits, hostile-artifact checks, and diagnostics. |
| `crates/xlang-core/src/authoring.rs` | Implements effect-aware `aether.ast/v2`, `aether.edit/v2`, and `aether.diagnostic/v2` serialization and validation. |
| `crates/xlang-core/tests/m4_error_effect_semantics.rs` | Adds the pure bounded two-exit semantic model and boundary tests. |
| `crates/xlang-core/tests/seed_self_host.rs`, `seed/aether_seed.ae`, `seed/aether_seed.aeth` | Extends self-host proof and ships the rebuilt v7 seed compiler. |
| `apps/xlang-cli/` and Cargo manifests/lockfile | Advances the CLI/core version to 0.7.0 and labels the v2 authoring contracts. |
| `schemas/aether-*-v2.schema.json` | Publishes strict local authoring wire contracts for the new nodes and diagnostics. |
| `examples/error-effect.ae` | Supplies the shipped M4 behavior example. |
| `docs/AETHER_0.7.md`, ADR-007, M4 design/matrix | Defines the released syntax, artifact rules, decisions, exclusions, and evidence. |
| Product/governance docs | Synchronizes README, manifest, architecture, audit, seed profile, roadmap, claims, research, landscape, and project AGENTS pointer to the 0.7 boundary. |
| This report | Preserves scope, verification, known limits, and release-status evidence. |

## Honest limits and release status

The optimized `aether.exe` was built, inspected, and exercised locally, but no
installer, code signing, changelog/release archive, public hosting, or external
release publication was produced. Therefore this delivery is a verified language
milestone and local release-binary proof, **not** a claim that Aether 0.7 is a
finished public release.

Seed source-emission parity is proven for the documented Seed Profile, shipped
examples, complete prior canonical corpus, M2 resource corpus, and bounded M4
error corpus. Rust remains the invalid-source diagnostic authority; the seed is
not claimed to have complete diagnostic parity. The two pre-existing untracked
constitution-copy directories remain untouched and outside this delivery.

## Next action

M5 remains research-gated: specify a deterministic, bounded compile-time
execution subset before adding source syntax. It must preserve AETH-only
execution, verifier-first artifacts, local authority, and the clean M4 resource
boundary.
