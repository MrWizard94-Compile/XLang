# ADR-008: deterministic bounded compile-time evaluation

**Status:** Accepted and implemented in Aether 0.8 / M5

**Date:** 2026-08-01

**Decision makers:** WPAI product direction; user authorization to continue
language development and make necessary design decisions

**Related Rule IDs:** RND-INVAR-001, RND-CORE-001, RND-DOC-001,
IP-INVENTION-001, SEC-INPUT-001, TEST-BEHAVIOR-001, DOC-SYNC-001,
DOC-ADR-001, CONST-GATE-001

## Context

The original Aether brief calls for compile-time execution instead of macros.
M4 deliberately completed the first explicit effect boundary before this work,
because a compile-time evaluator must not become an ambient authority or an
unbounded error route. The current language has a seed-hosted product compile
path, a verifier-centered AETH VM, and AI-facing structural contracts; any M5
design must preserve those foundations.

An unrestricted `comptime { ... }` block would immediately require a semantic
model for loops, calls, ownership, effects, dynamic values, resource budgets,
diagnostics, artifact provenance, and a matching Aether-written evaluator. It
would be a feature pile-on rather than a falsifiable first experiment.

## Decision

Aether 0.8 implements M5's first complete vertical slice as explicit,
root-only `comptime bind` statements containing one literal `Whole` arithmetic
operation. The allowed operations are `sum`, `difference`, `product`,
`quotient`, and `remainder`; all operands are signed decimal Whole literals.

The result is evaluated during compilation under a fixed program-wide budget of
1,024 directives. It lowers to AETH v8 `COMPTIME_WHOLE` (opcode 56), followed
by ordinary local storage. The instruction preserves stage provenance while
retaining normal runtime `Whole` semantics.

The complete grammar, invariants, errors, seed requirements, and stop
conditions are in [the M5 design](DESIGN-M5-DETERMINISTIC-COMPTIME.md).

## Consequences

### Positive

- The language gains a real, deterministic compile-time evaluator without a
  macro processor, a generated-text escape hatch, or host capability.
- Every accepted evaluation has a constant, inspectable work bound and shares
  VM checked-arithmetic behavior.
- AETH records the stage explicitly, so binary artifacts remain auditable.
- AI tooling receives an explicit stage field in a versioned structural schema
  rather than attempting to infer author intent from arithmetic syntax.
- The seed can prove the feature using ordinary Aether operations and the
  existing forge boundary.

### Costs and deliberate limits

- No named compile-time dependencies, booleans, text, bytes, records, calls,
  loops, branches, type computation, code generation, or build scripting are
  admitted in M5.
- A `comptime` result remains a normal runtime local; M5 does not introduce a
  new constant storage class or erase the binding from AETH.
- The provenance marker is an auditable compiler claim, not a cryptographic
  source attestation.
- The fixed budget is intentionally conservative and cannot be raised from
  source.

## Alternatives rejected

| Alternative | Reason rejected for M5 |
| --- | --- |
| Textual macros or generated source | Breaks canonical source/AST tooling and creates a second unchecked language. |
| General `comptime` blocks now | Requires unproven control-flow, call, resource, effect, and termination rules. |
| Host-evaluated scripts or build hooks | Violates local artifact capability confinement and reproducibility. |
| Automatic, invisible constant folding | Cannot express author intent or preserve artifact provenance. |
| User-settable evaluation quotas | Lets generated input evade bounded compiler resource policy. |
| Reusing AETH v7 | Would reinterpret an existing artifact version rather than adding an independently verifiable v8 format. |

## Implementation gate

This decision is complete only with bootstrap parser/formatter/validator,
AETH v8 encoder/decoder/verifier/VM, seed emission, v3 authoring contracts,
valid and hostile corpus tests, byte-identical seed proof, documentation sync,
and constitutional quality gates. It does not authorize broader compile-time
features without a new decision record.

## Links

- [M5 design and invariants](DESIGN-M5-DETERMINISTIC-COMPTIME.md)
- [M5 validation matrix](M5-VALIDATION-MATRIX.md)
- [Roadmap](ROADMAP.md)
- [M4 ADR](ADR-007-m4-typed-error-effect.md)
- [Aether research evidence](research/03-synthesis-and-evidence.md)
