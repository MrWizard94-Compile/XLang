# M6 validation matrix for explicit shape and layout research

**Status:** Research-gated validation plan for Aether 0.8 / M6

**Date:** 2026-08-02

**Design:** [DESIGN-M6-SHAPE-LAYOUT-EXPERIMENT.md](DESIGN-M6-SHAPE-LAYOUT-EXPERIMENT.md)

## Invariants under test

| Invariant | What must be shown | Required product evidence |
| --- | --- | --- |
| M6-INV-001 explicitness | The layout choice is visible in source and in the semantic model. | The parser/formatter and semantic kernel expose the rule without hidden lowering or implicit rewrite. |
| M6-INV-002 semantics preservation | Equivalent programs preserve observable behavior under the chosen layout rule. | A validation corpus proves the same runtime-visible outcome for equivalent shapes. |
| M6-INV-003 ABI clarity | The layout rule has a documented and testable ABI boundary. | The design note and tests define the input/output contract and reject ambiguous layout assumptions. |
| M6-INV-004 bounded scope | The spike remains a single explicit layout form and one benchmark workload. | The implementation and test corpus do not silently broaden to generic specialization or a native backend. |
| M6-INV-005 capability safety | The research spike does not introduce host authority or a capability leak. | No ambient allocator, no host I/O bridge, no foreign-code execution, and no verifier weakening are introduced. |
| M6-INV-006 seed parity | The documented research slice remains compatible with the seed/bootstrap proof model. | Seed and bootstrap produce matching output for the research slice; the existing self-host proof remains intact. |
| M6-INV-007 performance evidence | Any performance claim is reproducible and not overstated. | The benchmark harness includes baseline, workload, variance, and counterexamples. |
| M6-INV-008 roadmap discipline | The spike remains a research proposal until it passes the acceptance gate. | The work is explicitly marked as research-gated and requires a new ADR plus proof before becoming product work. |

## Required corpus

| Area | Positive cases | Negative / hostile cases |
| --- | --- | --- |
| Source and syntax | One explicit layout annotation over a bounded record or aggregate shape. | Hidden layout inference, ambiguous layout selection, or source forms that rely on implicit rewrite. |
| Semantics | Two equivalent programs that differ only by the explicit layout choice but preserve observable behavior. | A layout rule that changes behavior, ownership flow, or effect visibility. |
| ABI and verifier | An explicit layout choice that can be validated and decoded by the existing artifact rules. | A layout choice that requires a new host ABI, unsupported artifact metadata, or verifier bypass. |
| Benchmarking | One reproducible workload with baseline and variance reporting. | Performance claims without baseline, workload definition, or counterexamples. |
| Seed and bootstrap | The research slice compiles and verifies through the existing seed/compiler proof path. | A proof path that depends on unverified or non-deterministic lowering. |

## Acceptance evidence

The evidence package for M6 must include:

1. a bounded source corpus for the explicit layout rule;
2. semantic-equivalence tests for the chosen layout shape;
3. an ABI/verification test that shows the rule remains explicit and bounded;
4. a benchmark harness with baseline and variance data; and
5. the seed/bootstrap proof results for the documented research slice.

Passing a single happy-path example is not M6 acceptance. The milestone remains research-gated until the above evidence is recorded and reviewed.
