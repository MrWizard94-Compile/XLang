# Delivery Report: Aether M9 offline project tooling pilot

**Status:** Verified Aether 0.12 toolchain pilot; language surface remains 0.11 /
AETH v11; not a published or signed public release
**Date:** 2026-08-04
**Scope:** `aether.project/v1`, CLI `project verify`, CLI `format`, schema,
tests, and release-gate proposal
**Rule IDs:** CONST-GATE-001, CONST-DONE-001, ENG-WARN-001,
TEST-BEHAVIOR-001, SEC-INPUT-001, DOC-SYNC-001, DOC-ADR-001

## Delivered boundary

```text
aether format <source-file> [--output <source-file>]
aether project verify <project-file> [--output-dir <dir>]
```

Project documents list relative `.ae` units, require exactly one `main` unit,
optionally lock SHA-256 digests, confine paths under the project root, and
seed-compile every unit offline.

Fixture: `examples/project/aether.project.json` + `main.ae`.

## Quality-gate record

| Evidence | Result |
| --- | --- |
| Pack integrity | Pass (5.0.1) |
| fmt / Clippy `-D warnings` | Pass |
| `cargo test -p aether-core --lib` | Pass: 61 tests (includes project module) |
| `cargo test -p aether-cli` | Pass: 2 |
| `aether project verify examples/project/aether.project.json` | Pass: project_demo@0.1.0, 1 unit |
| `aether format examples/project/main.ae` | Pass: canonical source |
| New dependency | `sha2 = 0.10.9` (exact pin) for lock digests |

## Honest limits

- No LSP/IDE language server
- No package registry or network dependency resolution
- No multi-package graphs or semver ranges
- Not a public signed release

## Next

Roadmap language milestones M0–M9 pilots are complete within their stated
bounds. Further work requires new human-scoped decisions (expanded packages,
LSP, C interop, native backends, public release packaging).
