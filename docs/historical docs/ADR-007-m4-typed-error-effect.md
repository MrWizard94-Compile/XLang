# ADR-007: bounded typed abortive error effect

**Status:** Accepted and implemented in Aether 0.7 / M4

**Date:** 2026-08-01

**Decision makers:** WPAI product direction; user authorization to continue
language development and make necessary design decisions

**Related Rule IDs:** RND-INVAR-001, RND-CORE-001, RND-DOC-001,
IP-INVENTION-001, SEC-INPUT-001, TEST-BEHAVIOR-001, DOC-SYNC-001,
DOC-ADR-001, CONST-GATE-001

## Context

Aether 0.6 deliberately kept allocation outcomes closed and terminal because
first-class propagation requires a typed error/effect model. It has neither an
effect annotation nor a hidden exception path. The next roadmap milestone must
therefore prove one inspectable failure capability before compile-time work,
colorless blocking, cancellation, or general result unions are considered.

Koka demonstrates why effect information belongs in a function type and why a
handler can discharge an effect. OCaml demonstrates both forwarding to an outer
handler and the difficulty of continuation linearity around resources. Aether
does not adopt either complete system: its VM, seed proof boundary, explicit
resource model, and AI-first structural tooling require a far smaller initial
kernel.

## Decision

Aether 0.7 M4 implements exactly one abortive typed error effect:

```text
Error[Whole]
```

The source spelling is `raises Whole`, with terminal `raise`, `forward
call`, and `handle call ... otherwise error` forms. It has these properties:

1. The effect is visible in the weave signature; no inference or implicit
   exception propagation exists.
2. `raise` aborts the current frame with a `Whole` code and has no resumption.
3. `forward` is a terminal, statically declared propagation route.
4. `handle` is a terminal two-exit call that writes a normal value or error
   code to its designated mutable `Whole` root binding and immediately yields
   that binding.
5. Erroring function boundaries are copy-only and cannot cross an active owner,
   loan, arena, buffer, or M2 resource outcome.
6. AETH v7 encodes a function effect tag and explicit effect instructions;
   v4/v5/v6 remain immutable compatibility formats.
7. `main`, primitive host invocation, and forge `compile` remain total, so no
   unhandled guest error becomes a host exception or capability.
8. Source structure, edit payloads, and diagnostics version to v2 rather than
   silently changing their existing M3 v1 contracts.

The complete grammar, verifier/VM contract, source examples, proof obligations,
and stop conditions are [the M4 design](DESIGN-M4-TYPED-ERROR-EFFECTS.md).

## Consequences

### Positive

- A failure path is available for a real handled/forwarded/rejected experiment
  without introducing an ambient exception mechanism.
- The effect signature, source operation, AETH tag, verifier state, and VM exit
  have one small one-to-one model suitable for deterministic seed proof.
- No continuations or dynamic handler lookup are available to duplicate or
  strand a resource; M2 remains a hard safety boundary.
- AI tools receive explicit syntax, terminal destinations, and stable v2
  diagnostics instead of guessing whether a call can fail.

### Costs and deliberate limits

- Only `Whole` error codes, `Whole` erroring results, and copy-only
  `Whole`/`Truth` parameters are admitted.
- A program cannot use M2 resources and M4 effect control in the same weave.
- Erroring operations must be terminal; there is no local recovery-and-continue
  pattern, typed result union, or generic handler.
- The initial feature does not provide cancellation, async behavior, external
  authority, or a general effects language.

## Alternatives rejected

| Alternative | Reason rejected for M4 |
| --- | --- |
| Hidden VM/host exceptions | Violates capability transparency, makes artifact validation incomplete, and cannot be represented in M3 structure. |
| First-class generic `Result` values first | Couples result unions, pattern matching, generic variance, resource joins, and propagation before the effect core is proved. |
| General algebraic effects and resumptions | Requires continuation lifetime/linearity rules and interacts directly with future concurrency and resource destruction. |
| Inferred/open effect rows | Adds inference and diagnostic ambiguity before the source/AETH/seed representation is stable. |
| Error payloads carrying owners/resources | Requires cleanup and cross-frame ownership proof that M4 intentionally defers. |
| Adding errors to AETH v6 | Reinterprets a released artifact version and breaks the verifier compatibility boundary. |

## Implementation gate

This decision required a complete vertical slice rather than a parser-only,
VM-only, or bootstrap-only feature. The implemented increment is accepted only
with the [M4 validation matrix](M4-VALIDATION-MATRIX.md), seed/bootstrap byte
identity, synchronized documentation, zero-warning gates, and the full
constitution gate. It does not claim invalid-source diagnostic parity for the
seed or authority for a broader effects system.

## Links

- [M4 design and invariants](DESIGN-M4-TYPED-ERROR-EFFECTS.md)
- [M4 validation matrix](M4-VALIDATION-MATRIX.md)
- [Roadmap](../Current%20state/ROADMAP.md)
- [ADR-003: owned values and bounded arenas](ADR-003-value-resource-semantics.md)
- [ADR-004: AETH v6 resources](ADR-004-aeth-v6-bounded-resources.md)
- [Aether 0.7 contract](AETHER_0.7.md)
