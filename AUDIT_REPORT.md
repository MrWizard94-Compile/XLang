# Aether Migration Audit

Date: 2026-08-03 (Aether 0.8: Stage 7 bounded resources, M3/M4/M5 authoring,
typed-error baseline, and deterministic literal comptime; active Studio
retirement)

## Legacy Intake

| Area | Result | Aether use |
| --- | --- | --- |
| `legacy/xlang-v1-prototype` | Experimental Rust implementations with known compile failures. | Preserve source spans, diagnostic discipline, and failure cases as reference; do not compile or emit its syntax. |
| `legacy/xlang-v2-snapshot` | A former parser and type checker with a focused frontend test suite. | Preserve parser, semantic-check, and test-design lessons; do not promote its grammar or token stream. |
| `legacy/aether-genesis-ai-studio` | Historical AI Studio material with stale cloud metadata and JavaScript evaluation behavior. | Preserve as historical material; do not evaluate generated JavaScript or use cloud AI. |

## Production Boundary

The production compiler accepts Aether 0.8.0 source. Default CLI compilation
uses the Aether-written seed compiler; the Rust bootstrap remains the canonical
AST and invalid-source diagnostic authority and is selected explicitly with
`--bootstrap`. Both paths produce deterministic AETH v8, which is verified
before the VM runs it. Verified AETH v4/v5/v6/v7 remain compatibility inputs.
The command line is the sole active product interface.

Stage 7 completes canonical Aether 0.6 source-emission parity in the
Aether-written seed compiler. It preserves every prior statement and shallow
expression family, named locals and parameters, ownership modes, multi-weave
calls, bounded text/binary primitives (`seek`, `number`, `pack*`, `unpack*`,
`poke*`), canonical text escapes, and LF/CRLF input with or without a final
terminator. It adds bounded immutable nominal records: primitive fields only,
construction in declaration order, explicit borrowed field projection, internal
weave transport, and structural equality. Record-free programs remain byte-stable
AETH v4; record-bearing programs emit verified AETH v5 (and later versions).
Legacy C-shaped, Rust-shaped, V1, V2, and pre-v4 AETH input remains intentionally
rejected. The repository does not transpile Aether to C, Rust, JavaScript, LLVM,
or another target language.

M3/M4/M5 add local `aether.ast/v3`, `aether.edit/v3`, and
`aether.diagnostic/v3` tooling contracts. They are not another Aether grammar
or artifact format: every accepted edit is formatter-canonicalized and
bootstrap-validated, and the CLI seed-compiles it before source is written to
the caller-supplied output path. The protocol accepts only typed top-level
record/weave insert/replace/delete operations against an exact canonical base
source. Every `Bind` node must state `stage: "runtime"` or `stage: "comptime"`.
v1 and v2 remain historical contracts and are not silently reinterpreted.

`aether forge` verifies a compiler artifact, requires
`compile [borrow source: Text] -> Bytes`, gives it the source text, verifies the
returned artifact bytes, and writes only a verified result. The v3 structures
expose the bounded M4 `Error[Whole]` effect, its terminal `raise`, `forward`,
and one-line `handle` statements, and M5 stage provenance without creating a
hidden host exception route or macro expansion path.

## Strategic Design Boundary

This audit records the implemented Aether 0.8 migration/product boundary. The
separate AI-first language direction is evidence-gated rather than represented
as completed feature work: [docs/NORTH_STAR.md](docs/NORTH_STAR.md),
[docs/CORE_CLAIMS.md](docs/CORE_CLAIMS.md), and [docs/ROADMAP.md](docs/ROADMAP.md).
General effects/resumptions, broader compile-time execution, structured
concurrency, data-layout/generic work, C interop, and fine-grained
arbitrary-node structural AI edits are future research/design items. M3/M4/M5's
implemented bounded protocols must not be reported as arbitrary text editing,
general metaprogramming, or a host-capability expansion, and must not weaken
AETH-only execution, verification, seed proof, or local-first authority.

## Seed-Profile Self-Hosting Boundary

`seed/aether_seed.ae` is source in Aether. It parses the complete documented
canonical Aether 0.8 surface and emits v8 through ordinary language
operations. The regression tests prove bootstrap, first forge, and second forge
match the checked-in artifact; a distinct source variant produces a different
verified artifact; and multi-weave + `call`, records, every shipped example,
the documented M2 corpus, the bounded M4 error-effect corpus, and the bounded
M5 comptime corpus match bootstrap.

This is canonical Aether 0.8 source-emission self-hosting. Rust remains the
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
corpus, including the record, M2, M4, and M5 surfaces, match bootstrap
byte-for-byte under seed compile.

## M3 Structural Authoring Contract

The bootstrap exports semantic structure only after canonical formatting, so
line/column spans and node IDs refer to one local source revision. Strict JSON
validation rejects duplicate keys, unknown fields, unsupported versions,
malformed payloads, stale base source, and bounded-input violations. Core and
CLI contract tests prove a valid edit round-trip plus seed validation, while
malformed/stale requests return deterministic machine-readable diagnostics and
do not replace source. The current v3 contract is
[docs/AETHER_AUTHORING_PROTOCOL_v3.md](docs/AETHER_AUTHORING_PROTOCOL_v3.md);
v1 and v2 remain historical compatibility documentation.

## M5 Deterministic Compile-Time Evaluation

Aether 0.8 admits root-only immutable `comptime bind` for one literal
signed-`Whole` arithmetic operation under a fixed 1,024-directive budget.
Results lower to AETH v8 `COMPTIME_WHOLE` provenance and ordinary local
storage. The design, decision, validation matrix, and delivery evidence are
[docs/DESIGN-M5-DETERMINISTIC-COMPTIME.md](docs/DESIGN-M5-DETERMINISTIC-COMPTIME.md),
[docs/ADR-008-m5-deterministic-comptime.md](docs/ADR-008-m5-deterministic-comptime.md),
[docs/M5-VALIDATION-MATRIX.md](docs/M5-VALIDATION-MATRIX.md), and
[docs/DELIVERY_REPORT-2026-08-03-M5-DETERMINISTIC-COMPTIME.md](docs/DELIVERY_REPORT-2026-08-03-M5-DETERMINISTIC-COMPTIME.md).
