# Aether Migration Audit

Date: 2026-08-01 (Stage 7 bounded-resource and M3 authoring-contract baseline; active Studio retirement)

## Legacy Intake

| Area | Result | Aether use |
| --- | --- | --- |
| `legacy/xlang-v1-prototype` | Experimental Rust implementations with known compile failures. | Preserve source spans, diagnostic discipline, and failure cases as reference; do not compile or emit its syntax. |
| `legacy/xlang-v2-snapshot` | A former parser and type checker with a focused frontend test suite. | Preserve parser, semantic-check, and test-design lessons; do not promote its grammar or token stream. |
| `legacy/aether-genesis-ai-studio` | Historical AI Studio material with stale cloud metadata and JavaScript evaluation behavior. | Preserve as historical material; do not evaluate generated JavaScript or use cloud AI. |

## Production Boundary

The production bootstrap compiler accepts Aether 0.6.0 source. It produces a
canonical AST and deterministic AETH v6 bytecode artifact, verifies that
artifact, and runs it in the Aether VM. Verified AETH v4/v5 remain compatibility
inputs. The command line is the sole active product interface.

Stage 7 completes canonical Aether 0.6 source-emission parity in the
Aether-written seed compiler. It preserves every prior statement and shallow
expression family, named locals and parameters, ownership modes, multi-weave
calls, bounded text/binary primitives (`seek`, `number`, `pack*`, `unpack*`,
`poke*`), canonical text escapes, and LF/CRLF input with or without a final
terminator. It adds bounded immutable nominal records: primitive fields only,
construction in declaration order, explicit borrowed field projection, internal
weave transport, and structural equality. Record-free programs remain byte-stable
AETH v4; record-bearing programs emit verified AETH v5. Legacy C-shaped,
Rust-shaped, V1, V2, and pre-v4 AETH input remains intentionally rejected. The
repository does not transpile Aether to C, Rust, JavaScript, LLVM, or another
target language.

M3 adds local `aether.ast/v1`, `aether.edit/v1`, and
`aether.diagnostic/v1` tooling contracts. They are not another Aether grammar
or artifact format: every accepted edit is formatter-canonicalized and
bootstrap-validated, and the CLI seed-compiles it before source is written to
the caller-supplied output path. The protocol accepts only typed top-level record/weave
insert/replace/delete operations against an exact canonical base source.

`aether forge` verifies a compiler artifact, requires
`compile [borrow source: Text] -> Bytes`, gives it the source text, verifies the
returned artifact bytes, and writes only a verified result.

## Strategic Design Boundary

This audit records the implemented Aether 0.6 migration/product boundary. The
separate AI-first language direction is evidence-gated rather than represented
as completed feature work: [docs/NORTH_STAR.md](docs/NORTH_STAR.md),
[docs/CORE_CLAIMS.md](docs/CORE_CLAIMS.md), and [docs/ROADMAP.md](docs/ROADMAP.md).
Explicit allocators, typed effects, structured concurrency, data-layout/generic
work, C interop, and fine-grained arbitrary-node structural AI edits are future
research/design items. M3's implemented bounded protocol must not be reported
as arbitrary text editing or a source-language feature, and it must not weaken AETH-only execution,
verification, seed proof, or local-first authority.

## Seed-Profile Self-Hosting Boundary

`seed/aether_seed.ae` is source in Aether. It parses the complete documented
canonical Aether 0.6 surface and emits v6 through ordinary language
operations. The regression tests prove bootstrap, first forge, and second forge
match the checked-in artifact; a distinct source variant produces a different
verified artifact; and multi-weave + `call`, records, every shipped example, and
a complete canonical-surface corpus match bootstrap.

This is canonical Aether 0.6 source-emission self-hosting. Rust remains the
bootstrap and invalid-source diagnostic authority; full diagnostic parity is not
claimed. Scope: [docs/SEED_PROFILE.md](docs/SEED_PROFILE.md).

## Retired Application Boundary

The active `apps/xlang-studio` Tauri/React workbench, its loopback model-review
integration, and its generated Windows installers were removed. They are not a
production build input or release requirement. The CLI keeps the same
seed-hosted compiler and structural-edit safety boundary without an application
or model authority. The decision and reversibility rationale are in
[docs/ADR-006-retire-aether-studio.md](docs/ADR-006-retire-aether-studio.md).

The historical `legacy/aether-genesis-ai-studio` material remains reference
only under the existing legacy rule; it is not active application source.

## Release Gate

A release requires passing Aether core and CLI tests (including the seed
self-host proof), Clippy with warnings denied, command-line compile/forge/run
checks, and a release CLI build. A successful package must have its binary
inspected and launched before it is reported as delivered. There is no frontend,
Tauri bundle, installer, or model-status requirement.


## Stage 7 Seed-Hosted Compile

Default CLI compilation uses the Aether-written seed artifact. Bootstrap
remains for seed rebuild, `check` AST, invalid-source diagnostics, and
dual-compare proofs. All shipped examples and the complete canonical-surface
corpus, including the record surface, match bootstrap byte-for-byte under seed
compile.

## M3 Structural Authoring Contract

The bootstrap exports semantic structure only after canonical formatting, so
line/column spans and node IDs refer to one local source revision. Strict JSON
validation rejects duplicate keys, unknown fields, unsupported versions,
malformed payloads, stale base source, and bounded-input violations. Core and
CLI contract tests prove a valid edit round-trip plus seed validation, while
malformed/stale requests return deterministic machine-readable diagnostics and
do not replace source. The full v1 contract is
[docs/AETHER_AUTHORING_PROTOCOL_v1.md](docs/AETHER_AUTHORING_PROTOCOL_v1.md).
