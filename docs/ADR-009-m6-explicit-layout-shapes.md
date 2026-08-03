# ADR-009: explicit layout shapes and dual-layout tables

**Status:** Accepted for Aether 0.9 / M6 implementation

**Date:** 2026-08-03

**Decision makers:** WPAI product direction; user authorization to continue
language development (M6) under AGENTS Constitution

**Related Rule IDs:** RND-INVAR-001, RND-CORE-001, RND-DOC-001,
DOC-ADR-001, DOC-SYNC-001, TEST-BEHAVIOR-001, SEC-INPUT-001,
CONST-GATE-001

## Context

Roadmap M6 requires an explicit-layout collection and a constrained shape
analysis prototype, with layout/ABI rules, semantic-equivalence tests, and a
reproducible performance methodology. M2 already provides one arena and
copy-element Whole/Truth buffers, but those buffers have no multi-field layout
vocabulary. Records are immutable single values, not collections. Automatic
AoS→SoA transformation would violate Aether's "data-oriented performance must
be proved" principle and the AI-first need for visible structure.

## Decision

Aether 0.9 implements M6 as:

1. **`shape` declarations** — fixed products of 1–8 `Whole` fields only.
2. **`table Shape layout rows|columns`** — arena-backed table owners with
   author-selected physical layout.
3. **Closed `allocate` / `store` / `load`** — M2-style outcomes; no ambient
   allocation; capacity `1..=1024`.
4. **Local shape analysis** — declared-shape resolution and field membership
   checks; same-shape means the same shape declaration.
5. **AETH v9** — shape table metadata plus table opcodes; v4–v8 unchanged.
6. **Authoring v4** — explicit shape/layout nodes; no silent v3 upgrade.
7. **Evidence** — dual-layout semantic equivalence corpus, hostile
   source/artifact tests, seed byte identity, and the published layout harness.

Full grammar, invariants, and stop conditions live in
[DESIGN-M6-EXPLICIT-LAYOUT-SHAPES.md](DESIGN-M6-EXPLICIT-LAYOUT-SHAPES.md).

## Consequences

### Positive

- Authors and AI tools can choose and audit layout without guessing optimizer
  behavior.
- Rows/columns share one logical model, enabling equivalence tests.
- Extends M2 resource discipline instead of inventing a second heap.
- Artifact metadata makes layout inspectable after compilation.

### Costs and deliberate limits

- No generic shape parameters, nested shapes, or non-Whole fields.
- No automatic layout conversion or whole-program specialization.
- Tables cannot be weave results or host/forge values.
- Interpreter wall-clock benefits may be small; claims must follow the harness.

## Alternatives rejected

| Alternative | Reason rejected for M6 |
| --- | --- |
| Automatic AoS→SoA rewrite | Hidden layout mutation; hard to audit; research stop condition. |
| Generic `Table[T]` type system now | Requires variance, inference, and seed-scale generics before a finite proof. |
| Record buffers only (no layout keyword) | Does not make physical layout explicit or dual-testable. |
| C ABI / native packing | Violates current AETH-only product law and host capability boundary. |
| Reusing AETH v8 without a version bump | Would reinterpret existing artifacts. |

## Implementation gate

Complete only with bootstrap + seed + verifier/VM + authoring v4 + dual-layout
equivalence tests + harness evidence + docs/manifest sync + constitutional
gates. Broader generics remain a new decision.
