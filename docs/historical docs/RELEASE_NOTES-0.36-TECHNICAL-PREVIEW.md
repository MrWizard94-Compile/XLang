# Aether 0.36.0 — Local Technical Preview Release Notes

**Channel:** Local folder plus `SHA-256SUMS`; not a public release

**Date:** 2026-08-08

**Product contract:** [MANIFEST.md](../../MANIFEST.md)

**Threat model:** [Technical Preview 0.36 threat model](THREAT_MODEL-0.36-TECHNICAL-PREVIEW.md)

**License status:** `UNLICENSED`. No distribution or public-use rights are
granted by this preview package.

## Purpose

This release refreshes the local technical-preview channel around the current
Aether 0.36.0 contract. It is a verifier-first, offline CLI toolchain for
deterministic AI-primary authorship. The package is intended for local review,
reproducible evaluation, and further gated development; it is not a 1.0,
multi-tenant, sandboxed-native-code, or public-distribution release.

## Current package contents

```text
dist/aether-0.36.0-tp/
  aether.exe
  README.md
  MANIFEST.md
  RELEASE-METADATA.json
  SHA-256SUMS
  verify-preview.ps1
  seed/
  schemas/
  examples/
  stdlib/
  docs/
```

`SHA-256SUMS` covers every package file except itself. The consumer verifier
also rejects an unexpected unlisted file, preventing a valid prefix of hashes
from being mistaken for whole-package integrity.

## Highlights since the original 0.12 preview

- Offline multi-unit projects, language modules, cross-package workspace
  imports, local locks, test runners/reports, and a bounded offline LSP.
- Grant-mediated host I/O with deny-by-default roots and environment names;
  no ambient guest filesystem, process, shell, or network authority.
- Pure Whole-only stdlib layers, a deliberately narrow foreign ABI pilot under
  an explicit native-library grant, and deterministic pure comptime helpers.
- AETH v12 M19e task frames: explicit `task weave` / `checkpoint`, verified
  safe-point cancellation, private pre-admitted lanes, deterministic teardown,
  and v11 compatibility for non-task source.
- AI-authoring v8: explicit `Weave.task`, typed `Checkpoint`, canonical source
  revalidation, and seed compilation before an edit can write output.

The exact current surface and deliberate non-goals are in
[AETHER_0.36.md](AETHER_0.36.md) and [MANIFEST.md](../../MANIFEST.md).

## Upgrade guidance

Existing AETH v4–v11 artifacts retain their verified historical meanings.
Existing source that does not use `task weave` remains an AETH v11 program.
Tools consuming structural authoring should update from historical v7 contracts
to the current v8 schemas before emitting a task/checkpoint request. No source
file is silently upgraded to a task frame and no output is rewritten without an
explicit CLI output path.

The workspace lock surface is optional but strict once present: use `project
lock` and `workspace lock` without `--write` to inspect a candidate, then pass
`--write` only when the operator intentionally updates the local lock.

## Verify the package

From the package root:

```powershell
pwsh -NoProfile -File .\verify-preview.ps1
```

Expected terminal line:

```text
PREVIEW VERIFY PASS
```

The verifier establishes the following local evidence:

1. Each checksum is valid and every staged file is listed exactly once.
2. `aether.exe version` matches the signed-by-checksum package metadata.
3. The v11 welcome, pure host, and v12 task-frame examples compile and return
   their documented program exits.
4. `structure` reports `aether.ast/v8` with a task weave and checkpoint.
5. The project/workspace fixtures verify and a path-escape project fails closed.

Developers rebuilding the local preview should run the release gate:

```powershell
pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode release
```

This includes full source/seed evidence, the release build, staging, consumer
verification, and a negative integrity probe. It does not publish, tag, commit,
or transmit the resulting `dist/` folder.

## Known limits and residual risks

- The foreign ABI pilot loads an operator-granted native library; it is not a
  sandbox and is unsuitable for hostile libraries.
- Host I/O is only as safe as the roots/environment names an operator grants to
  a guest program. Network, shell, process, and ambient filesystem authority
  remain absent.
- M19e is cooperative and checkpoint-bound; it does not provide task handles,
  timeouts, manual cancellation, arbitrary preemption, nested task nurseries,
  or parallel execution.
- The bootstrap remains the invalid-source diagnostic authority. Full seed
  diagnostic parity is not claimed.
- `UNLICENSED` is a release-distribution boundary. A human decision is required
  before any public channel, license grant, tag, or publication.

## Related evidence

- [M19e implementation report](DELIVERY_REPORT-2026-08-08-M19E-T-RX-IMPLEMENTATION.md)
- [M19e validation matrix](../Current%20state/M19E-VALIDATION-MATRIX.md)
- [0.36 local-preview stabilization report](DELIVERY_REPORT-2026-08-08-TECHNICAL-PREVIEW-0.36.md)
- [Full project progress report](../Current%20state/PROGRESS_REPORT-FULL-PROJECT.md)

*End of local technical-preview release notes.*
