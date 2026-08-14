# Delivery Report — M23 pure comptime weave calls

**Date:** 2026-08-07
**Package:** Aether **0.33.0**
**Decision:** [ADR-039](ADR-039-m23-comptime-pure-calls.md)
**Contract:** [AETHER_0.33.md](AETHER_0.33.md)

## Shipped

- `comptime bind name <- call weave args...` for an earlier total guest helper
  with owned Whole parameters/result and the restricted M23 body subset.
- A bounded bootstrap evaluator with checked Whole arithmetic, exact source
  order, fixed existing directive budget, and no VM/host/resource/effect path.
- Folding to existing AETH v11 `COMPTIME_WHOLE` (56); verifier and VM contracts
  remain unchanged.
- The seed-emitted product bridge: bootstrap materializes validated M23 calls
  into equivalent literal M5 directives before forge; output is required to be
  byte-identical to bootstrap output.
- `examples/comptime-calls.ae`, bootstrap/seed parity tests, behavior negatives,
  and structural-authoring v7 round-trip coverage.

## Evidence

- P1–P4, N1–N9, and H1 in [M23-VALIDATION-MATRIX.md](../Current%20state/M23-VALIDATION-MATRIX.md).
- `crates/xlang-core/src/lib.rs` M23 evaluator and product materializer.
- `crates/xlang-core/tests/seed_self_host.rs` M23 seed-hosted byte-identity
  proof, plus the complete shipped-example corpus.

## Verification

- `cargo fmt --all -- --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo test --release --workspace` — passed. The suite includes 84 core unit
  tests, 17 seed/self-host tests, CLI tests, semantic suites, and the M23
  product-path proof. The seed self-rebuild proof completed in 129.95 seconds.
- Release CLI: `version`, `check`, `structure`, seed `compile`, and `run` for
  `examples/comptime-calls.ae` passed; the artifact exits 512.

The default debug `cargo test --workspace` run is not represented as a pass:
its existing full seed self-rebuild workload did not complete inside a 244-second
bounded run. The equivalent release-profile proof passed above. This is a
developer-gate performance limitation of the interpreted self-rebuild workload,
not a M23 semantic or seed-parity failure; it needs a dedicated performance
increment before a fast default-debug-gate claim.

## Honest boundary

The checked-in seed compiler does not independently execute raw M23 source.
M23 reaches seed emission only after bootstrap validation and deterministic
literal materialization. This preserves the seed-emitted product artifact while
making the bootstrap responsibility explicit. A future direct seed evaluator
would require a new design/ADR and a new proof claim.

## Not shipped

No recursion, mutual recursion, comptime `choose`/`while`, nested calls,
Text/Bytes/Truth evaluation, source generation, host/foreign calls, or
user-configurable fuel.

*End of M23 implementation delivery report.*
