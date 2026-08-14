# Delivery Report: Aether 0.6 bounded M2 resources

**Status:** Verified implementation evidence; not a packaged desktop release
**Date:** 2026-07-31
**Scope:** Aether 0.6 / AETH v6 closed bounded-resource core
**Rule IDs:** CONST-GATE-001, ENG-WARN-001, TEST-BEHAVIOR-001,
SEC-INPUT-001, DOC-SYNC-001, RND-INVAR-001

## Delivered boundary

Aether 0.6 adds one fixed-capacity `arena` in `main`, `buffer Whole` and
`buffer Truth` owners, ephemeral `access` authority, and closed terminal
outcomes for allocation, append, and lookup. Bootstrap source validation creates
the typed `SemanticResourcePlan`; AETH v6 lowering consumes it; the verifier
tracks exact buffer provenance; and the VM preserves owners and arena accounting
on every dim outcome.

The seed compiler is the default CLI and Studio compilation path. It emits the
same AETH v6 bytes as bootstrap for all shipped examples, the prior canonical
surface corpus, and the documented M2 resource corpus.

The exact language contract is [AETHER_0.6.md](AETHER_0.6.md). The decision and
deliberate limits are [ADR-004](ADR-004-aeth-v6-bounded-resources.md).

## Seed reproducibility evidence

Bootstrap compilation of `seed/aether_seed.ae`, self-forging that artifact
against the same source, and the promoted checked-in artifact are byte-identical:

```text
F440AD30DAEFA9FAC4ACDAC1EBC2B6F5A99EDF4EBD0A3FD8C4C3952C7F78F1F3
```

The corresponding commands were:

```powershell
cargo run -p aether-cli -- compile .\seed\aether_seed.ae --output .\target\aether_seed.m2fix.aeth --bootstrap
cargo run -p aether-cli -- forge .\target\aether_seed.m2fix.aeth .\seed\aether_seed.ae --output .\target\aether_seed.m2fix.forged.aeth
Get-FileHash .\seed\aether_seed.aeth, .\target\aether_seed.m2fix.aeth, .\target\aether_seed.m2fix.forged.aeth -Algorithm SHA256
```

An initial full seed gate exposed a real parser ambiguity: ordinary
`append move bytes value` was incorrectly classified as the resource form when
it lacked `into destination`. The seed now requires that delimiter before
emitting the resource opcode. The formerly failing shipped-example and complete
canonical-surface seed tests were rerun in isolation, then the entire core suite
was rerun successfully.

## Behavior and hostile-input evidence

The canonical M2 examples prove the documented outcomes through both bootstrap
and default seed-hosted compilation:

| Example | Required exit value |
| --- | ---: |
| `arena-buffer.ae` | 7 |
| `arena-exhausted.ae` | -1 |
| `arena-full.ae` | -2 |
| `arena-lookup-fallback.ae` | 99 |
| `arena-truth-buffer.ae` | 1 |
| `arena-access-weave.ae` | 1 |

Core tests also reject malformed resource source, resource-owner `revise`,
invalid result boundaries, unallocated borrow, bad resource artifacts, invalid
destinations, and forged placeholder substitution for a moved owner.

## Final quality-gate record

All commands below exited zero after the final source refactor:

| Command | Result |
| --- | --- |
| `cargo fmt --all -- --check` | Pass |
| `cargo test -p aether-core` | Pass: 28 unit + 5 seed self-host/repro tests (23m 40s) |
| `cargo test -p aether-cli` | Pass: 1 test |
| `cargo test -p aether-studio` | Pass: 3 tests, including seed-hosted M2 Studio compile/run |
| `cargo clippy -p aether-core -p aether-cli -- -D warnings` | Pass, zero warnings |
| `cargo clippy -p aether-studio -- -D warnings` | Pass, zero warnings |
| `npm run lint` | Pass: ESLint `--max-warnings=0` |
| `npm test` | Pass: 3 frontend tests |
| `npm run build` | Pass: TypeScript and Vite production build |
| `aether check/compile/run examples/welcome.ae` | Pass: default seed compile; runtime exit 73 |
| `pwsh -File "../../AGENTS Constitution/tools/verify-pack.ps1"` | Pass: `GOV-INT-001` |

## Honest limits and release status

This delivery does not claim first-class resource outcomes, outcome propagation,
Buffer weave results, resource-owner `revise`, generic effects, general
references, individual arena reclamation, concurrency, FFI, a structural-edit
protocol, or full invalid-source diagnostic parity.

No Windows Tauri bundle/package inspection was performed in this language-core
delivery, so this report does not make a desktop-release claim. Release work
must additionally meet the `MANIFEST.md` bundle, package-inspection, and
optional-live-Ollama requirements when applicable.
