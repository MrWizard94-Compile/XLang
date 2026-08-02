# M6 Design: explicit shape and layout experiment

**Status:** Research-gated proposal for Aether 0.8 / M6
**Decision record:** Pending ADR
**Validation record:** Pending

## Purpose

This document captures the first bounded research spike for M6: an explicit
shape/layout experiment that stays inside Aether's current verifier-first and
capability-constrained model. The goal is not to add a general generic system or
an automatic layout optimizer. The goal is to test whether a small, explicit
layout contract can be expressed and proven without weakening Aether's existing
ownership, verification, or seed-parity guarantees.

## Scope and non-goals

The M6 spike is intentionally narrow:

- it studies one explicit layout contract for a constrained data shape;
- it requires semantic-equivalence and ABI clarity before any feature can enter
  the product path;
- it does not introduce automatic layout rewriting, native codegen, C ABI
  expansion, or general generic specialization; and
- it does not relax verify-before-run/write or the current no-host-capability
  boundary.

The current Aether 0.8 contract remains the only implemented product surface.
M6 work must remain research-gated until it has a specification, tests,
benchmark protocol, and human approval.

## Proposed experiment

The spike should define one small, explicit layout choice over a bounded record
collection, such as:

- an explicit layout annotation over a record or a small aggregate shape;
- a shape-aware selection rule that stays visible in source and in the semantic
  model; and
- a proof obligation that a layout choice preserves observable semantics and the
  existing resource/effect boundaries.

The experiment should be limited to a single explicit layout form, one proof
corpus, and one benchmark workload. The design should answer four questions:

1. Can the language express the shape choice in source without introducing hidden
   lowering or implicit layout rewriting?
2. Can the verifier and semantic model preserve the same observable behavior for
   equivalent programs under the chosen layout?
3. Can the seed compiler and the bootstrap produce equivalent output for the
   documented research slice?
4. Does the proposed layout contract improve a measurable workload without
   violating the current Aether law and capability boundaries?

## Proposed invariants

The spike must preserve the following invariants:

- AETH remains the only executable artifact format.
- The verifier remains the authority for artifact validity before execution.
- Host capability leakage remains prohibited.
- Existing M2/M4/M5 semantics remain unchanged unless a new ADR explicitly
  approves a broader change.
- Any layout rule must be explicit, testable, and explainable in source.
- Any performance claim requires a reproducible benchmark and counterexamples.

## Evaluation plan

The research spike should produce:

1. a minimal source grammar and semantic model for the explicit layout choice;
2. a small equivalence corpus that demonstrates preserved semantics under the
   chosen layout;
3. a benchmark harness with workload, baseline, and variance reporting; and
4. a stop condition if the design requires hidden layout inference,
   automatic rewrite, or a new host ABI.

## Acceptance gate before product work

M6 may advance to implementation only if it satisfies the roadmap acceptance
criteria:

- explicit layout/ABI rules are documented;
- semantic-equivalence tests pass;
- the benchmark protocol is reproducible and not misleading;
- the design does not weaken the current verifier or capability boundaries; and
- the seed/compiler proof path remains intact for the documented slice.

## Relation to the roadmap

This document is a bounded research proposal for the M6 milestone described in
[ROADMAP.md](ROADMAP.md). It does not claim that Aether 0.8 implements generic
shape folding, SoA lowering, or a native backend. It only establishes the
conditions under which a future, explicitly approved M6 increment may be
considered.
