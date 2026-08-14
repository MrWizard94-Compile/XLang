# M9 Design: offline project metadata and tooling pilot

**Status:** Accepted implementation design for Aether 0.12 / M9

**Date:** 2026-08-04

**Decision record:** [ADR-012](ADR-012-m9-project-tooling.md)

**Validation record:** [M9 validation matrix](M9-VALIDATION-MATRIX.md)

## Purpose and boundary

M9 proves **reproducible, offline project identity** for Aether without a package
registry, network dependency fetch, or LSP server. It adds:

1. A versioned **project document** (`aether.project/v1`) listing local units.
2. An optional **content lock** of SHA-256 digests for offline integrity.
3. CLI **`project verify`** that validates schema, path bounds, lock hashes,
   and seed-compiles every unit.
4. CLI **`format`** that emits formatter-canonical source (already law for
   authoring) to an explicit path or stdout.

It does **not** implement multi-package graphs, version ranges, remote
download, or a language server. Those remain future decisions.

## Threat model

| Concern | Rule |
| --- | --- |
| Path escape | Unit paths must be relative, non-absolute, and stay under the project root after normalization (no `..` escape). |
| Network | No network access; verify is fully local. |
| Authority | CLI still only reads caller-selected files and writes explicit outputs. |
| Tamper | Lock digests detect modified unit sources before compile. |
| Capability | Guest AETH still has no file/process/network/shell authority. |

## Core claim and invariants

| ID | Invariant |
| --- | --- |
| M9-INV-001 | Project documents declare `schema: "aether.project/v1"`, `name`, `version`, and a non-empty `units` array. |
| M9-INV-002 | Each unit has a relative `path` ending in `.ae` and a `role` of `main` or `lib`. Exactly one `main` unit is required. |
| M9-INV-003 | Resolved unit paths remain inside the project root (directory containing the project file). |
| M9-INV-004 | When `lock.units` is present, every unit path has a matching SHA-256 of the exact file bytes; mismatch fails closed. |
| M9-INV-005 | `project verify` seed-compiles each unit and verifies the AETH artifact without writing unless asked. |
| M9-INV-006 | `format` produces the same canonical text as the bootstrap formatter and does not execute guest code. |
| M9-INV-007 | No package registry, URL dependency, or model/network call is introduced. |

## Project document surface

```json
{
  "schema": "aether.project/v1",
  "name": "host_pilot_project",
  "version": "0.1.0",
  "units": [
    { "path": "main.ae", "role": "main" }
  ],
  "lock": {
    "units": [
      { "path": "main.ae", "sha256": "<hex>" }
    ]
  }
}
```

Schema file: `schemas/aether-project-v1.schema.json`.

## CLI

```text
aether format <source-file> [--output <source-file>]
aether project verify <project-file> [--output-dir <dir>]
```

- `format` without `--output` prints canonical source to stdout.
- `project verify` reports unit digests and compile results; with
  `--output-dir` writes `<stem>.aeth` for each unit after verification.

Diagnostics (tooling, not Aether source codes):

| Code | Meaning |
| --- | --- |
| `AE-PROJECT-001` | Schema/structure invalid. |
| `AE-PROJECT-002` | Path escape or illegal unit path. |
| `AE-PROJECT-003` | Lock digest mismatch or missing lock entry. |
| `AE-PROJECT-004` | Unit compile/verify failure. |

## Language / artifact versions

M9 is **toolchain** evolution. Language surface remains Aether **0.11** / AETH
**v11**. Product package version becomes **0.12.0** to mark the tooling
milestone. No new AETH opcodes.

## Explicit non-goals

- Full LSP/IDE language server
- Multi-package dependency graphs and semver ranges
- Online registries or automatic upgrade/download
- Signed releases / installers (release proposal only)

## Release workflow proposal (documented, not a public ship)

1. `cargo fmt --check`, Clippy `-D warnings`, core/CLI/seed tests
2. `aether project verify` on each shipped project fixture
3. `cargo build --release -p aether-cli`
4. Inspect and launch release binary locally
5. Record hashes in a delivery report

## Stop conditions

Do not claim a package ecosystem if network fetch or ambient path authority is
required. Do not claim LSP completeness from format + project verify alone.
