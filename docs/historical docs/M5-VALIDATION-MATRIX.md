# M5 deterministic compile-time evaluation validation matrix

**Status:** Implementation gate for Aether 0.8 / M5 — satisfied by
[DELIVERY_REPORT-2026-08-03-M5-DETERMINISTIC-COMPTIME.md](DELIVERY_REPORT-2026-08-03-M5-DETERMINISTIC-COMPTIME.md)

**Date:** 2026-08-01

**Design:** [M5 deterministic bounded compile-time evaluation](DESIGN-M5-DETERMINISTIC-COMPTIME.md)

## Invariants under test

| Invariant | Semantic-kernel evidence | Required product evidence |
| --- | --- | --- |
| M5-INV-001 explicit stage | Pure evaluator accepts only explicit immutable directives. | Parser, formatter, and v3 authoring round-trip `comptime bind`; ordinary `bind` remains runtime. |
| M5-INV-002 small pure subset | Kernel accepts only five literal Whole operations. | Source rejects names, mutable bindings, non-Whole operations, calls, values, and non-root placement. |
| M5-INV-003 fixed budget | Kernel rejects directive 1,025. | Bootstrap counts root statements deterministically and reports `AE-COMPTIME-003`. |
| M5-INV-004 checked arithmetic | Kernel distinguishes success, overflow, and zero-divisor cases. | Bootstrap diagnostics and seed-valid corpus agree with VM Whole semantics. |
| M5-INV-005 no host authority | Kernel is pure and receives only parsed literals. | No new dependency, CLI authority, Forge ABI, VM host bridge, or I/O capability is added. |
| M5-INV-006 v8 provenance | Kernel outcome maps to a stage-bearing instruction. | Decoder/verifier accept `COMPTIME_WHOLE` only in v8 and reject it in v4-v7. |
| M5-INV-007 seed parity | Canonical M5 corpus is deterministic. | Seed/bootstrap byte identity, self-host rebuild, forge second generation, and shipped example proof pass. |
| M5-INV-008 structural clarity | Binding stage has exactly two values. | `aether.ast/v3`, `aether.edit/v3`, and `aether.diagnostic/v3` reject v1/v2 or malformed stage payloads. |

## Required corpus

| Area | Positive cases | Negative / hostile cases |
| --- | --- | --- |
| Source and formatting | Each of five arithmetic operations; canonical source and CRLF normalization. | Missing `bind`, mutable directive, non-root directive, wrong arity, variable operand, text/Truth operation. |
| Arithmetic | Signed inputs, negative results, quotient/remainder. | Add/subtract/multiply/divide/remainder overflow and zero divisor. |
| Budget | Exactly 1,024 directives. | Directive 1,025 is rejected before seed invocation. |
| AETH | Deterministic v8 `COMPTIME_WHOLE` emission. | Opcode 56 under v4-v7, truncated immediate, unknown opcode, malformed v8 header. |
| Verifier and VM | Provenance instruction behaves as a normal Whole stack push. | It cannot create a host exit, side effect, owner, resource, or untyped stack value. |
| Seed/Profile | M5 example and canonical corpus match bootstrap byte-for-byte. | Seed claims no invalid-source diagnostic parity; bootstrap remains the source diagnostic authority. |
| Authoring | v3 structure exposes `stage: "comptime"`; a v3 edit writes canonical source then seed-compiles it. | Missing/unknown stage, malformed v3 payload, stale source, and v1/v2 protocol/schema are rejected. |
| Compatibility | Existing 0.7 source emits v8 with unchanged runtime behavior; v4-v7 artifacts continue to verify/run. | Version reinterpretation or a v8 opcode in an older artifact rejects before execution or Forge write. |

## Acceptance evidence

The completed delivery record must include the exact constitution gate commands,
test counts, seed/bootstrap and Forge hashes, CLI `check`/`structure`/
`compile`/`run` results for the M5 example, artifact-version inspection, and
the known limits above. Passing a constant-folding happy path alone is not M5
acceptance.
