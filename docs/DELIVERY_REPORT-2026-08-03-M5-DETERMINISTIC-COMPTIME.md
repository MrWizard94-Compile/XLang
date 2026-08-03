# Delivery Report: Aether M5 deterministic compile-time evaluation

**Status:** Verified Aether 0.8 language implementation; not a published or
signed public release
**Date:** 2026-08-03
**Scope:** Aether 0.8 source, AETH v8, verifier, VM, seed compiler, local
authoring v3, and CLI proof surface
**Rule IDs:** CONST-GATE-001, CONST-DONE-001, ENG-WARN-001,
TEST-BEHAVIOR-001, SEC-INPUT-001, DOC-SYNC-001, DOC-ADR-001,
RND-INVAR-001

## Delivered boundary

Aether 0.8 adds exactly one explicit, resource-bounded compile-time form:

```aether
world comptime_math

weave main [] -> Whole:
  comptime bind table_width <- product 16 8
  comptime bind header_size <- sum 12 4
  yield sum table_width header_size
```

`comptime bind` is root-only and immutable. The accepted evaluator grammar is
one literal signed-`Whole` operation among `sum`, `difference`, `product`,
`quotient`, and `remainder`. Each directive costs one fixed evaluation unit;
one source program may contain at most 1,024 directives. Arithmetic uses the
same checked signed-64-bit semantics as the VM. The result remains an ordinary
immutable runtime `Whole` local.

This milestone deliberately does **not** add named compile-time dependencies,
booleans, text, bytes, records, calls, loops, branches, type computation, code
generation, macros, generated source, host I/O, build scripts, or a
user-adjustable quota.

The executable contract is [AETHER_0.8.md](AETHER_0.8.md). The design and
decision record are
[DESIGN-M5-DETERMINISTIC-COMPTIME.md](DESIGN-M5-DETERMINISTIC-COMPTIME.md) and
[ADR-008](ADR-008-m5-deterministic-comptime.md). The explicit claims/evidence
map is [M5-VALIDATION-MATRIX.md](M5-VALIDATION-MATRIX.md).

## Artifact, seed, and authoring changes

- New compilation emits deterministic AETH v8. Verified v4, v5, v6, and v7
  inputs retain their existing interpretations.
- v8 preserves the v7 header layout (arena capacity, record table, effect tag)
  and adds `COMPTIME_WHOLE` (opcode 56) as a v8-only provenance push. The
  verifier rejects opcode 56 under v4–v7 and rejects truncated or unknown
  instruction forms before run or forge write.
- The Aether-written seed compiler parses, evaluates, emits, and self-forges
  the complete bounded M5 surface. The checked-in `seed/aether_seed.aeth` is
  v8.
- `aether.ast/v3`, `aether.edit/v3`, and `aether.diagnostic/v3` require
  `Bind.stage` as `runtime` or `comptime` and surface
  `AE-COMPTIME-001` through `AE-COMPTIME-003`. v1/v2 remain historical and are
  not silently reinterpreted.
- `examples/comptime.ae` provides the shipped source example for all five
  literal operations and signed results.

## Final quality-gate record

All commands below exited zero on the final source tree (2026-08-03).

| Command / evidence | Result |
| --- | --- |
| `pwsh -File "../../AGENTS Constitution/tools/verify-pack.ps1"` | Pass: `GOV-INT-001`, constitution pack 5.0.1. |
| `cargo fmt --all -- --check` | Pass. |
| `cargo clippy -p aether-core -p aether-cli -- -D warnings` | Pass: zero warnings. |
| `cargo test -p aether-core` | Pass: 48 core tests, 10 M4 semantic-model tests, 4 M5 semantic-model tests, and 7 seed/self-host tests. Seed self-host alone took about 1,661 seconds; full package about 1,665 seconds. |
| `cargo test -p aether-cli` | Pass: 2 CLI behavior tests. |
| JSON parse of all v3 schemas | Pass: AST, edit, and diagnostic schema files parse as JSON. |
| `aether check/compile/run examples/comptime.ae` | Pass: default seed path and bootstrap path produced identical artifacts; run exited 150. |
| Comptime bootstrap/seed comparison | Pass: both artifacts SHA-256 `9F8F9068D7736938F07762E3F16D6718A2AA2F91C3C7A490E1DD672E13F3AD56`. |
| CLI seed bootstrap + forge | Pass: checked-in, bootstrap, and self-forged seed artifacts SHA-256 `35EFCAD81C066C7C4E28416785A6938234AEFD9E1B5E5CC2709FE3EDD546DE4D`. |
| `cargo build --release -p aether-cli` | Pass: optimized Windows CLI binary built. |
| `target\release\aether.exe version` and release compile/run | Pass: reports Aether 0.8.0; release binary seed-compiles `comptime.ae` and exits 150. |

## Changed-file manifest

| Area | Purpose |
| --- | --- |
| `crates/xlang-core/src/lib.rs` | Implements the 0.8 grammar, M5 evaluation budget, AETH v8 emitter/parser/verifier, VM provenance opcode, hostile-artifact checks, and diagnostics. |
| `crates/xlang-core/src/authoring.rs` | Implements stage-aware `aether.ast/v3`, `aether.edit/v3`, and `aether.diagnostic/v3` serialization and validation. |
| `crates/xlang-core/tests/m5_comptime_semantics.rs` | Adds the pure bounded evaluator model and budget/overflow tests. |
| `crates/xlang-core/tests/seed_self_host.rs`, `seed/aether_seed.ae`, `seed/aether_seed.aeth` | Extends self-host proof and ships the rebuilt v8 seed compiler. |
| `apps/xlang-cli/` and Cargo manifests/lockfile | Advances the CLI/core version to 0.8.0 and labels the v3 authoring contracts. |
| `schemas/aether-*-v3.schema.json` | Publishes strict local authoring wire contracts for stage-bearing binds and comptime diagnostics. |
| `examples/comptime.ae` | Supplies the shipped M5 behavior example. |
| `docs/AETHER_0.8.md`, ADR-008, M5 design/matrix | Defines the released syntax, artifact rules, decisions, exclusions, and evidence. |
| Product/governance docs | Synchronizes README, manifest, architecture, seed profile, roadmap, claims, research, landscape, and project AGENTS pointer to the 0.8 boundary. |
| This report | Preserves scope, verification, known limits, and release-status evidence. |

## Honest limits and release status

The optimized `aether.exe` was built, inspected, and exercised locally, but no
installer, code signing, changelog/release archive, public hosting, or external
release publication was produced. Therefore this delivery is a verified language
milestone and local release-binary proof, **not** a claim that Aether 0.8 is a
finished public release.

Seed source-emission parity is proven for the documented Seed Profile, shipped
examples, complete prior canonical corpus, M2 resource corpus, bounded M4 error
corpus, and bounded M5 comptime corpus. Rust remains the invalid-source
diagnostic authority; the seed is not claimed to have complete diagnostic
parity.

The pre-existing untracked constitution-copy directories and zip
(`AGENTS Constitution/`, `.grok/AGENTS Constitution/`,
`AGENTS Constitution.zip`) remain outside this delivery and were not staged.

## Next action

M6 remains research-gated: specify generic shapes and data-layout experiments
only after explicit design, validation matrix, and human approval. Do not
expand M5 into general compile-time execution without a new ADR.
