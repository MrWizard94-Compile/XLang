# Delivery Report: Aether M7 structured nursery concurrency

**Status:** Verified Aether 0.10 language implementation; not a published or
signed public release
**Date:** 2026-08-03
**Scope:** Aether 0.10 source, AETH v10, verifier, VM, seed compiler, local
authoring v5, and CLI proof surface
**Rule IDs:** CONST-GATE-001, CONST-DONE-001, ENG-WARN-001,
TEST-BEHAVIOR-001, SEC-INPUT-001, DOC-SYNC-001, DOC-ADR-001,
RND-INVAR-001

## Delivered boundary

Aether 0.10 adds a lexical structured nursery:

```aether
together:
  spawn call left into a
  spawn call right into b
```

Spawns run cooperatively in source order. First child `Error[Whole]` cancels
remaining unstarted spawns and re-raises. No OS threads, detached tasks, or
parallel speedup claims. Nursery weaves keep the M4 clean resource boundary;
`main` remains total.

Evidence design: [DESIGN-M7-STRUCTURED-CONCURRENCY.md](DESIGN-M7-STRUCTURED-CONCURRENCY.md),
[ADR-010](ADR-010-m7-structured-concurrency.md),
[M7-VALIDATION-MATRIX.md](M7-VALIDATION-MATRIX.md),
[AETHER_0.10.md](AETHER_0.10.md).

## Quality-gate record

| Evidence | Result |
| --- | --- |
| Pack integrity | Pass (5.0.1) |
| fmt / Clippy `-D warnings` | Pass |
| `cargo test -p aether-core --lib` | Pass: 55 tests |
| Semantic models m4/m5/m6/m7 | Pass: 10 + 4 + 3 + 4 |
| `cargo test -p aether-core --test seed_self_host` | Pass: 9 tests (rebuild ~3,050 s) |
| `cargo test -p aether-cli` | Pass: 2 |
| `nursery-total.ae` seed ≡ bootstrap | SHA-256 `E608DE449FD3ADD8F9C2B5DC39C5CBDD5A0C64232610737915DA80F4F23643F7`, exit 7 |
| `nursery-cancel.ae` seed ≡ bootstrap | SHA-256 `C89815A1CCABFCC2F1BBF589D19359121B5F47495C2E130119DF8430A4DBFB9B`, exit 9 |
| Seed bootstrap ≡ self-forge | SHA-256 `713DC5850F030177AD7D46CF619D13A205BC7BA1928B9A8617C88C2036C8045A` |

## Honest limits

- Not a public/signed release
- Cooperative scheduling only — no parallelism claim
- No nested nurseries, timeouts, task handles, or resourceful tasks
- Seed has emission parity, not full invalid-source diagnostic parity

## Next

M8 foreign/host interface pilot remains research-gated.
