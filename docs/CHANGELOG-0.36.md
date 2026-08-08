# Changelog — Aether 0.36.0 Local Technical Preview

**Date:** 2026-08-08

**Scope:** User-facing delta from the original 0.12 local technical preview to
the current package. This is not a claim of 1.0 completeness or public-release
status.

## 0.36.0 — M19e active-frame cancellation and preview refresh

- Added bounded `task weave` and `checkpoint` source forms.
- Added AETH v12 task descriptors and `TASK_CHECKPOINT`, while preserving v11
  output and behavior for non-task source.
- Added deterministic source-order round-robin task frames, checkpoint-only
  cancellation, private pre-admitted resource lanes, reverse-slot teardown,
  and lane zeroization after an eligible companion failure.
- Added authoring v8 task/checkpoint nodes and schemas.
- Refreshed the local preview packager and consumer verifier to derive package
  version from the CLI manifest, enforce complete checksum membership, and
  exercise v12 task-frame behavior.

## 0.35.0 — PKG-001 offline workspace locks

- Added explicit project/workspace lock refresh commands and optional fully
  local package identity pins.
- Locked workspace builds verify nested project/unit locks before output.
- Added path-confinement and stale-lock negatives without a registry, URL,
  remote cache, or version solver.

## 0.34.0 — RTP-001 runtime Text fast path

- Added private cached ASCII provenance for scalar-equivalent VM Text paths.
- Preserved source syntax, AETH bytes/version, verifier behavior, seed output,
  and guest-visible Unicode semantics.

## 0.33.0 — M23 pure comptime weave calls

- Added one restricted pure total `Whole` helper call from `comptime bind`.
- Bootstrap materializes the bounded result before seed emission; byte identity
  remains required for the documented corpus.

## 0.31.0–0.32.0 — foreign pilot and resource/nursery policy slices

- Added a Whole-only foreign weave pilot under explicit `--grant-lib KEY=PATH`.
- Added multi-weave arena planning, resourceful total spawn callees, and
  bounded cooperative Policy B behavior.
- Native libraries remain operator-approved and unsandboxed; broader FFI is not
  implied.

## 0.24.0–0.30.0 — local tooling and stdlib depth

- Added stdlib layers 0/1, cross-package imports, project tests, optional host
  grants in tests, and structured JSON/JUnit test reports.
- Preserved offline-first package/project behavior and explicit write controls.

## 0.13.0–0.23.0 — multi-file and authoring/tooling growth

- Added multi-unit projects, language modules, bounded structural edit v7,
  offline LSP, grant-backed host I/O, comptime name chaining, resource/handle,
  standalone tests, and local workspaces.
- Structural authoring is now v8 in 0.36; v7 remains historical only.

## Compatibility and non-claims

- AETH v4–v11 inputs remain accepted with historical meanings.
- No general effects, parallelism, native/LLVM backend, package registry,
  general FFI safety, or ambient guest authority is introduced by this preview.
- The package stays local-only and `UNLICENSED`; publication requires a separate
  human-directed release and licensing decision.

*For exact semantics, read [AETHER_0.36.md](AETHER_0.36.md) and
[MANIFEST.md](../MANIFEST.md).*
