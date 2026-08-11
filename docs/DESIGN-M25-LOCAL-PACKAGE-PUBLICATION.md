# M25 Design: Offline Local Package Publication

**Status:** Implemented in package 0.37.0
**Date:** 2026-08-11
**Decision record:** [ADR-107](ADR-107-m25-local-package-publication.md)
**Validation:** [M25 validation matrix](M25-VALIDATION-MATRIX.md)
**Depends on:** M18 workspace confinement, M22 cross-package imports, PKG-001
locked projects/workspaces
**Rule IDs:** `CONST-DEP-001`, `DOC-ADR-001`, `DOC-SYNC-001`,
`SEC-INPUT-001`, `TEST-BEHAVIOR-001`, `ENG-WARN-001`, `RND-INVAR-001`

---

## 1. Scope

M25 adds a fully local publication path for a locked `aether.project/v1`
project. It produces a transparent directory bundle, verifies the exact source
and manifest bytes it carries, publishes that bundle into an explicit local
cache, and materializes a verified cache entry as a standard workspace package
directory.

The feature is a toolchain-only increment. Aether source, AETH, the seed
compiler, verifier, VM, guest authority, project schema, workspace schema, and
M22 import grammar remain unchanged.

## 2. Commands

```text
aether pkg pack <aether.project.json> --output <bundle-dir>
aether pkg verify <bundle-dir>
aether pkg publish <bundle-dir> --cache <cache-dir>
aether pkg install <bundle-dir> --output <package-dir>
aether pkg install --cache <cache-dir> --name <name> --version <version> --output <package-dir>
aether pkg verify-cache <cache-dir>
```

Every mutating form names its destination explicitly. `pack` and `install`
reject an existing output path. `publish` targets only the deterministic cache
entry for the verified package identity and is idempotent only when that entry
has the same verified content digest.

## 3. Portable directory-bundle contract

A bundle root contains exactly the generated metadata plus the project
manifest and declared source units:

```text
<bundle>/
  aether.package.json
  project/
    aether.project.json
    <every project unit path>
```

`aether.package.json` has strict `aether.package/v1` shape:

```json
{
  "schema": "aether.package/v1",
  "name": "math_pkg",
  "version": "0.1.0",
  "project": "project/aether.project.json",
  "project_sha256": "<64 lowercase hexadecimal characters>",
  "content_sha256": "<64 lowercase hexadecimal characters>",
  "files": [
    { "path": "project/aether.project.json", "sha256": "<64 lowercase hexadecimal characters>" },
    { "path": "project/lib/math.ae", "sha256": "<64 lowercase hexadecimal characters>" },
    { "path": "project/main.ae", "sha256": "<64 lowercase hexadecimal characters>" }
  ]
}
```

The list is lexically sorted and must be exactly the project manifest followed
by the unit paths declared by that parsed project document. Arbitrary files,
directories, source globs, archive metadata, package dependencies, URLs,
signatures, and executable artifacts are outside M25.

`content_sha256` is SHA-256 over the domain-separated, sorted sequence
`aether.package/v1\0`, then every file path length, UTF-8 path bytes, raw file
length, and raw file bytes. It binds both the list and the exact package
content; individual file digests make a failure explainable.

## 4. Pack and verify algorithm

`pkg pack` accepts only a regular file named `aether.project.json`. It:

1. Canonicalizes the source project root and rejects source manifest or unit
   symlinks, path escapes, non-UTF-8 project metadata, nonregular declared
   units, and incomplete project locks. Extra source-tree files are not copied
   into or authenticated by a bundle.
2. Parses and verifies the locked project through the ordinary product project
   pipeline before copying any material.
3. Snapshots the raw project manifest and every declared unit before and after
   verification. A changed snapshot fails instead of emitting a mixed bundle.
4. Builds the strict metadata and writes a staging directory beside the named
   output, copies only the snapshot files, verifies the staging bundle, then
   atomically renames it into the previously absent output path.

`pkg verify` repeats strict metadata parsing, exact expected-file derivation,
path and regular-file checks, every individual digest, the content digest, and
ordinary locked-project verification. A bundle cannot pass by naming a correct
manifest while substituting a source unit.

## 5. Cache and installation

`pkg publish` verifies its source bundle before and after copying it through a
staging directory into this deterministic local-only layout:

```text
<cache>/packages/<name>/<version>/
```

The name and version use a restricted ASCII path grammar. An existing entry
with a different content digest fails closed and is never overwritten. An
identical verified entry is a successful no-op.

`pkg install` accepts either a verified bundle directory or a cache identity.
It copies only the bundle's `project/` tree to the named, absent output
directory, verifies the copied project bytes against the bundle, and leaves a
normal `aether.project.json` package root. Users add that directory to an
ordinary `aether.workspace/v1` document; M22's declared `depends_on` graph
continues to authorize cross-package imports.

`pkg verify-cache` enumerates only bounded, grammar-valid cache package entries
and runs the same bundle verification on each one. It does not resolve
versions, fetch, or trust remote state.

## 6. Invariants

| ID | Invariant |
| --- | --- |
| M25-INV-001 | A package source project carries a complete PKG-001 project lock before pack. |
| M25-INV-002 | Bundle files are exactly the project manifest plus declared unit files, with no unlisted payload. |
| M25-INV-003 | Both file digests and a domain-separated content digest bind the portable bundle. |
| M25-INV-004 | All input paths are canonicalized and confined; symlinks and traversal fail closed. |
| M25-INV-005 | Output directories are explicit, absent, staged, verified, and never silently overwritten. |
| M25-INV-006 | Cache publication is local and deterministic; conflicting identity/content pairs fail closed. |
| M25-INV-007 | Installed output is a normal locked project consumable by M18/M22 workspaces. |
| M25-INV-008 | No registry, network, resolver, ambient guest capability, Aether syntax, AETH revision, seed behavior, verifier, or VM behavior is added. |

## 7. Bounds and failure behavior

The metadata document is capped at 1 MiB, a bundle contains at most 257 files
(one project manifest plus the 256-unit project limit), every packaged file is
capped at 1 MiB, and total package source bytes are capped at 16 MiB. Failure
cleans the staging directory where the platform permits cleanup and never
replaces an existing requested output or cache entry.

The package verifier reports closed `AE-PACKAGE-*` diagnostics for malformed
metadata, unsafe paths, integrity mismatch, and output/cache conflicts. Nested
project verification remains visible in the package error context rather than
being silently reclassified as a valid package.

## 8. Rejected alternatives

| Alternative | Decision | Reason |
| --- | --- | --- |
| Remote registry or URL dependencies | Rejected | M25 is local publication; remote trust is separate F-REGISTRY scope. |
| Opaque archive format | Rejected | Transparent directory bundles make byte identity, review, and hostile-file testing direct. |
| Package source globbing | Rejected | Exact project unit enumeration prevents hidden payload and traversal ambiguity. |
| Auto-inject package paths into workspaces | Rejected | Workspace mutation and dependency authorization remain explicit user-controlled steps. |
| Overwrite existing cache/output directories | Rejected | A conflicting package identity must fail closed; accidental replacement loses provenance. |
| Version solver or ranges | Rejected | M25 installs one named exact local version and does not widen dependency semantics. |

## 9. Evidence plan

The implementation proves deterministic repeated packing; source/project lock,
manifest, unit, content-digest, unlisted-bundle-file, traversal, symlink,
oversize, collision, and output-preservation negatives; direct and cache
installation; a locked installed project; two independent workspace consumers
of one installed package; product seed compilation; and the full Aether gate.

---

*End of DESIGN-M25-LOCAL-PACKAGE-PUBLICATION.md.*
