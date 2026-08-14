# Delivery Report — RTP-001 cached ASCII VM Text fast path

**Date:** 2026-08-08
**Package:** Aether **0.34.0**
**Decision:** [ADR-040](ADR-040-runtime-text-ascii-fast-path.md)
**Contract:** [AETHER_0.34.md](AETHER_0.34.md)
**Matrix:** [RTP-001 validation matrix](RTP-001-VALIDATION-MATRIX.md)

## Delivered

- Private VM `RuntimeText` representation with cached ASCII provenance.
- Scalar-contract-preserving ASCII paths for `measure`, `glyph`, `cut`, and
  `seek`; Unicode remains scalar-aware.
- Correct cache propagation through bytecode constants, host values, invocation
  values, `decode`, `render`, `join`, output, equality, and size accounting.
- An intended-behavior unit test for ASCII and Unicode offsets.
- `tools/measure-seed-self-host.ps1`, a bounded local release timing harness
  that builds once, invokes the exact test directly, captures errors, enforces a
  timeout, and writes a JSON distribution under `target/`.

No source grammar, AETH bytes, verifier acceptance rules, seed source/artifact,
host grant, capability, or authoring-protocol behavior changed.

## Measured result

The harness measures only
`seed_profile_compiler_rebuilds_itself_and_a_distinct_valid_variant` after
`cargo test --release --no-run`; build time is excluded.

| Local same-host sample set | Samples (ms) | Median (ms) |
| --- | --- | --- |
| Pre-RTP-001 package-0.33 baseline | 129533, 128680, 129198 | 129198 |
| Package-0.34 final run | 10469, 10751, 10442 | 10469 |

Final-run context: Windows NT 10.0.26200.0; Intel64 Family 6 Model 158
Stepping 13; 8 logical processors; Cargo/Rustc 1.96.0. The final release test
binary SHA-256 was
`19D626517E3340F6B709719A5022C1C316105F34A3922EA74EC931D435E28330`.

This is a local regression-gate improvement, not a public competitive benchmark,
overall Aether VM speed claim, user-workload guarantee, or Unicode performance
claim. The raw JSON report is intentionally ignored with other build outputs;
the command and raw samples above make the evidence reproducible.

## Verification

| Gate | Result |
| --- | --- |
| `cargo fmt --all -- --check` | Passed |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed with no warnings |
| `cargo test --workspace` | Passed: 17 CLI, 85 core, 10 M4, 4 M5, 3 M6, 4 M7, and 17 seed/self-host tests |
| `cargo test --release --workspace` | Passed: same complete suite; exact release self-host group completed in 9.32 s inside the suite |
| `pwsh -NoProfile -File tools/measure-seed-self-host.ps1` | Passed: 10,469 / 10,751 / 10,442 ms |
| Product CLI | `version`, `check`, M23 seed/bootstrap hash equality, and run exit 512 passed in release mode |
| `pwsh -NoProfile -File tools/aether-gate.ps1 -Mode full -SkipPack` | Passed: full debug core/CLI suites, 29 example dual-compares, host exit 48, project/workspace checks, and bootstrap ≡ forged ≡ checked-in seed identity |

The gate's pack verifier was intentionally skipped because the active
post-cleanup workspace has no `verify-pack.ps1` to invoke. This is not a claim
that a separate distribution-pack audit ran; all active source, runtime, seed,
and product-path checks above did run.

The previous M23 delivery report accurately recorded its then-current debug-gate
limitation. RTP-001 supersedes that performance status without altering M23
language semantics or its bootstrap-materialized seed boundary.

## Residual limits and next evidence

- Non-ASCII scalar operations are intentionally not claimed faster.
- The benchmark has one host/toolchain profile; cross-machine measurements need
  a new evidence package.
- The source still has no JIT/native backend, compiler cache, or general runtime
  optimization program.
- A corrected repository-wide link scan reports 29 inactive historical/template
  links under `.grok` and the frozen `dist/aether-0.12.0-tp` package. The active
  0.34 contract set passes local-link validation; this increment does not restore
  deleted governance files or rewrite frozen distribution history.
- No public release artifact, commit, or push is implied by this working-tree
  delivery; those remain explicit human actions.

*End of RTP-001 delivery report.*
