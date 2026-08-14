# Delivery Report: Aether M8 host ABI pilot

**Status:** Verified Aether 0.11 language implementation; not a published or
signed public release
**Date:** 2026-08-04
**Scope:** Aether 0.11 source, AETH v11, verifier, VM pure host fixture, seed
compiler, local authoring v6, and CLI proof surface
**Rule IDs:** CONST-GATE-001, CONST-DONE-001, ENG-WARN-001,
TEST-BEHAVIOR-001, SEC-INPUT-001, DOC-SYNC-001, DOC-ADR-001,
RND-INVAR-001

## Delivered boundary

Aether 0.11 adds a capability-closed pure host ABI:

```aether
host weave whole_inc [value: Whole] -> Whole
host weave text_extent [borrow message: Text] -> Whole

weave main [] -> Whole:
  bind mutable n <- 41
  bind after <- call whole_inc n
  bind message <- "Aether"
  bind len <- call text_extent borrow message
  yield sum after len
```

Calls emit `HOST_CALL` (65) in AETH v11. Product `aether run` installs only
`whole_inc` and `text_extent`. Missing services fail closed (`AE-HOST-003`).
No C headers, libloading, or ambient guest I/O.

Evidence design: [DESIGN-M8-HOST-ABI-PILOT.md](DESIGN-M8-HOST-ABI-PILOT.md),
[ADR-011](ADR-011-m8-host-abi-pilot.md),
[M8-VALIDATION-MATRIX.md](M8-VALIDATION-MATRIX.md),
[AETHER_0.11.md](../Current%20state/AETHER_0.11.md).

## Quality-gate record

| Evidence | Result |
| --- | --- |
| Pack integrity | Pass (5.0.1) |
| fmt / Clippy `-D warnings` | Pass |
| `cargo test -p aether-core --lib` | Pass: 59 tests |
| Semantic models m4–m7 | Pass |
| `cargo test -p aether-cli` | Pass: 2 |
| Host seed dual-compare + shipped examples | Pass (seed_self_host host filters) |
| `host-pilot.ae` seed ≡ bootstrap | SHA-256 `A25F434BD5E94F4351DE1E519728BF9E2080B38E43130E9390645FB80BD23693`, exit **48** |
| Seed bootstrap ≡ self-forge | SHA-256 `6AC3C46B890029B646267E93C9FDA5CDD34F9E061D7BC0419D34E0DF6734B254` |

## Honest limits

- Not a public/signed release
- Pure host fixture only — no file/process/network/shell guest authority
- No C ABI, headers, or dynamic libraries
- Seed has emission parity, not full invalid-source diagnostic parity

## Next

M9 project/tooling evolution remains deferred/research-gated.
