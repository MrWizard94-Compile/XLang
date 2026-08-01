# M4 typed error/effect validation matrix

**Status:** Implemented Aether 0.7 matrix. Final release-gate commands remain
the required acceptance evidence; no broader effect claim is implied.

**Date:** 2026-08-01

**Design:** [M4 typed abortive error effect](DESIGN-M4-TYPED-ERROR-EFFECTS.md)

## Invariants under test

| Invariant | Semantic-kernel evidence | Required product evidence |
| --- | --- | --- |
| M4-INV-001 visible route | `handled_forwarded_and_rejected_routes_are_distinct` | Source rejects ordinary effectful calls; `examples/error-effect.ae` proves signature/raise/forward/handle syntax. |
| M4-INV-002 abortive control | `raise_has_no_continuation_to_resume` | VM returns `ErrorWhole`; no continuation, panic, or host exception path exists. |
| M4-INV-003 total callers handle | `total_weave_rejects_unhandled_error_call` | Source and authoring tests emit v2 effect diagnostics and retain total `main`. |
| M4-INV-004 declared forwarding | `forward_requires_matching_declared_effect` | Source, emitter, verifier, and seed byte-identity tests reject incompatible target metadata. |
| M4-INV-005 clean boundary | `effect_boundary_rejects_owner_loan_and_resource_state` | Source negatives and verifier clean-boundary checks reject owner/resource crossings. |
| M4-INV-006 verified v7 | `terminal_handle_requires_whole_result_on_both_exits` | Decoder/verifier reject old-version instructions, metadata conflicts, invalid slots, targets, and stacks. |
| M4-INV-007 total host boundary | `unhandled_error_cannot_be_main_exit` | Run, forge, and invoke verification keep entry points total and reject hostile error-entry artifacts before execution. |

## Semantic-kernel scope

`crates/xlang-core/tests/m4_error_effect_semantics.rs` is a pure, bounded
reference model. It has no parser, VM, AETH writer, host I/O, network access,
or production compiler path. It executes the core two-exit operational model
and validates static route/boundary rules. Its role remains to isolate the
semantic claim from the production implementation.

| Case | Expected result |
| --- | --- |
| A declared leaf raises a `Whole` code and a terminal handler selects its error destination. | Handled error becomes an ordinary result. |
| A declared wrapper forwards a leaf. | Normal and error exits remain distinguishable and the wrapper declares the effect. |
| A total weave uses an ordinary call to an erroring leaf. | Static rejection. |
| A total `main` is declared erroring or reaches an error exit. | Static rejection. |
| A boundary has a unique owner, borrow/access loan, arena, buffer, or M2 outcome. | Static rejection. |
| A terminal handler or erroring signature cannot preserve a `Whole` result. | Static rejection before evaluation. |

## Required product corpus

The following rows are implemented product scope and remain mandatory release
evidence.

| Area | Positive cases | Negative / hostile cases |
| --- | --- | --- |
| Parser and formatter | `raises Whole`, `raise`, `forward`, and one-line terminal `handle` canonical source; CRLF/LF formatting. | Duplicate/unsupported effect annotation, non-Whole payload, malformed handle destinations, non-root/nonterminal placement, or nested effect control. |
| Static semantics | Handled, forwarded, and total normal paths; `Whole` signature/result/destination typing. | Ordinary call to erroring callee, missing/extra declaration, wrong destination, wrong effect target, erroring `main`. |
| Ownership and M2 | Copy-only effect boundaries after all owners are consumed. | Live Text/Bytes/record, borrow/access loan, Arena/Buffer/resource outcome at an effect boundary; any M2/M4 mix in one weave. |
| AETH encoder/decoder | Deterministic v7 metadata and all three effect instructions. | v4/v5/v6 effect tags/instructions, unknown tags/opcodes, corrupt function index/count/slot/target. |
| Verifier | Valid terminal raise/forward/handle stack and branch states. | Nonempty abortive stack, uninitialized handler local, join after terminal effect op, mismatched callee tag/result, forged live resource state. |
| VM | Normal result, handled error, and forwarded error behavior. | No `panic`, host exception, host capability, or invalid entry error escape. |
| Seed/Profile | Canonical M4 fixtures byte-identical to bootstrap; seed rebuild and forge self-reproduction in v7. | Seed cannot claim invalid-source diagnostic parity; bootstrap remains diagnostic authority. |
| Authoring | `aether.ast/v2`, `aether.edit/v2`, and `aether.diagnostic/v2` round-trip every M4 construct. | v1 reinterprets v2 nodes, stale/malformed edit, unknown field, wrong effect node/type. |
| Compatibility | Existing 0.6 source compiles as v7 with unchanged behavior; v4/v5/v6 artifacts verify/run as before. | Earlier/unknown artifacts and any version-reinterpretation attempt reject before run/write. |

## Release evidence

The release delivery record must include exact commands/results, corpus hashes,
AETH v7 fixture inspection, seed/bootstrap hash proof, and verifier hostile-
input evidence. M4 cannot claim general effects, resource/effect composition,
cancellation, or full invalid-source seed diagnostic parity.
