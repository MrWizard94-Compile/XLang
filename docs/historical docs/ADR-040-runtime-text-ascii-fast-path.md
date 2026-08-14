# ADR-040: cached ASCII provenance for VM Text (RTP-001)

**Status:** Implemented — package **0.34.0**
**Accepted:** 2026-08-08
**Decision makers:** Human-authorized language development; AGENTS Constitution
**Related:** [AETHER_0.34.md](AETHER_0.34.md),
[RTP-001 validation matrix](RTP-001-VALIDATION-MATRIX.md)

## Context

The seed compiler is an Aether program whose source is ASCII and which performs
many text scalar operations while parsing/emitting source. The VM previously
implemented each `measure`, `glyph`, `cut`, and `seek` through scalar iteration
over `String`, even when a byte index is provably the same index. The complete
debug self-host gate was therefore too slow to be a practical routine release
check, despite correctness and release-profile proof being intact.

The constitution requires measured performance work, preserved behavior,
zero-warning validation, and an honest statement of scope. The selected solution
must not create a second artifact representation, weaken UTF-8 validation, or
special-case the seed compiler.

## Decision

Adopt a private `RuntimeText { value: String, ascii: bool }` representation in
the VM.

- Determine `ascii` when text enters the VM; combine it with logical AND on
  `join`.
- Use direct byte indexing only when `ascii` is true.
- Retain scalar iterator behavior for non-ASCII strings.
- Keep the cache runtime-only: no AETH instruction, artifact field, public API,
  source syntax, or forge ABI changes.
- Add an explicit unit test for direct and fallback helper behavior in addition
  to the existing observable Unicode primitive test.
- Retain a checked-in PowerShell measurement harness that records a bounded,
  exact release self-host test distribution outside the source tree's tracked
  artifacts.

## Alternatives evaluated

1. **Decode and cache a VM instruction plan.** A narrow experiment did not
   produce a material improvement for the exact self-host workload, so it was
   removed rather than retained as complexity without evidence.
2. **Special-case self-revising Text/Bytes aggregation.** The experiment was
   not beneficial on the measured workload and was removed.
3. **Make Unicode operations byte-indexed.** Rejected: it violates Aether's
   stated Unicode scalar contracts.
4. **Optimize only the seed compiler source.** Rejected: it would hide the
   runtime cause and give ordinary guest programs inconsistent behavior.

The Windows performance-recorder policy on the measured host did not permit
system profiling, so the retained harness uses direct, repeatable test-process
timing and reports its machine/toolchain context.

## Consequences

Positive:

- ASCII-heavy programs avoid repeated scalar scans without changing observable
  behavior.
- The full debug self-host suite is practical again and remains part of normal
  full validation.
- The optimization is local to VM text values and requires no seed rebuild.

Costs and limits:

- Each live runtime `Text` carries one boolean.
- Non-ASCII scalar operations retain their prior traversal complexity.
- A Unicode-origin slice that happens to contain only ASCII may retain the
  fallback bit conservatively; this is correct but may miss a later fast path.
- The measured result is not a promise for user programs or different machines.

## Acceptance record

The package-0.34 exact release self-host benchmark recorded 10,469 ms, 10,751
ms, and 10,442 ms, compared with the immediate package-0.33 baseline of
129,533 ms, 128,680 ms, and 129,198 ms. Debug and release workspace suites,
Clippy with warnings denied, formatting, product-path checks, and seed identity
all remain required; benchmark success alone is not acceptance.

*End of ADR-040-runtime-text-ascii-fast-path.md*
