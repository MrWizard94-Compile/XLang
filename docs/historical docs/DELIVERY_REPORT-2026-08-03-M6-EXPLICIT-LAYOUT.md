# Delivery Report: Aether M6 explicit layout shapes

**Status:** Verified Aether 0.9 language implementation; not a published or
signed public release
**Date:** 2026-08-03
**Scope:** Aether 0.9 source, AETH v9, verifier, VM, seed compiler, local
authoring v4, and CLI proof surface
**Rule IDs:** CONST-GATE-001, CONST-DONE-001, ENG-WARN-001,
TEST-BEHAVIOR-001, SEC-INPUT-001, DOC-SYNC-001, DOC-ADR-001,
RND-INVAR-001

## Delivered boundary

Aether 0.9 adds author-visible multi-field layout for one collection form:

```aether
shape particle:
  mass Whole
  charge Whole

weave main [] -> Whole:
  bind memory <- arena 4096
  bind mutable parts <- table particle layout columns
  bind mutable sample <- 0
  choose allocate access memory move parts 2 into parts:
    choose store move parts 0 mass 10 into parts:
      choose load borrow parts 0 mass into sample:
        yield sample
      otherwise:
        yield -4
    otherwise:
      yield -3
  otherwise:
    yield -1
```

Shapes are Whole-only products (1–8 fields). Tables are arena-backed owners with
explicit `layout rows` or `layout columns`. Capacity is `1..=1024`. Closed
`allocate` / `store` / `load` outcomes follow M2 discipline. Rows and columns
share one logical model; physical addresses differ by design.

This milestone deliberately does **not** add generic shape parameters,
non-Whole fields, nested shapes, automatic layout conversion, table weave
results, host packing ABI, or whole-program specialization.

The executable contract is [AETHER_0.9.md](AETHER_0.9.md). Design and decision
records are
[DESIGN-M6-EXPLICIT-LAYOUT-SHAPES.md](DESIGN-M6-EXPLICIT-LAYOUT-SHAPES.md) and
[ADR-009](ADR-009-m6-explicit-layout-shapes.md). Evidence map:
[M6-VALIDATION-MATRIX.md](M6-VALIDATION-MATRIX.md).

## Final quality-gate record

| Command / evidence | Result |
| --- | --- |
| Pack integrity `GOV-INT-001` | Pass (constitution 5.0.1) |
| `cargo fmt --all -- --check` | Pass |
| `cargo clippy -p aether-core -p aether-cli -- -D warnings` | Pass |
| `cargo test -p aether-core --lib` | Pass: 52 tests |
| Semantic models m4/m5/m6 | Pass: 10 + 4 + 3 |
| `cargo test -p aether-core --test seed_self_host` | Pass: 8 tests (~2,894 s, dominated by multi-generation seed forge) |
| `cargo test -p aether-cli` | Pass: 2 |
| Layout example seed ≡ bootstrap | SHA-256 `D1F0F8845A6FA733ADC04AF2F7899D509F9AA36DAFD73D808DF86DBFAF9D0C82`, exit 10 |
| Seed bootstrap ≡ self-forge | SHA-256 `A46C9488FAD929EE9FF478965A478F54EE526B36FDF402CB70772F9A805CAAAB` |
| Layout harness (capacity 256, 32 runs) | columns ≈ 155.8 ms, rows ≈ 189.5 ms, shared exit 21 — scoped columns win on this interpreter workload |

## Honest limits

- Not a public/signed release package.
- Seed claims emission parity, not full invalid-source diagnostic parity.
- Performance claim is harness-local and interpreter-specific; it is not a
  universal SoA superiority claim.
- General generics and automatic layout rewriting remain research-gated.

## Next action

M7 (structured concurrency) remains research-gated and requires its own design,
ADR, and human approval before product code.
