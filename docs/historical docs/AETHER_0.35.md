# Aether 0.35 Toolchain Contract (PKG-001 offline workspace locks)

**Status:** Historical package contract — superseded by
[AETHER_0.36.md](AETHER_0.36.md). Package 0.35 kept language **0.11** source
forms and AETH **v11** unchanged.
**Classification:** Offline package-integrity tooling increment, not a
language-surface milestone
**Decision:** [ADR-041](ADR-041-pkg-001-offline-workspace-locks.md)
**Design:** [PKG-001 workspace locks](DESIGN-PKG-001-OFFLINE-WORKSPACE-LOCKS.md)
**Validation:** [PKG-001 validation matrix](PKG-001-VALIDATION-MATRIX.md)

## What changed

Package 0.35 closes the deferred M18 workspace-integrity gap with optional,
fully local workspace locks. It adds two explicit CLI refresh commands:

```text
aether project lock <project-file> [--write]
aether workspace lock <workspace-file> [--write]
```

Without `--write`, each command prints a deterministic pretty JSON document and
does not modify disk. With `--write`, it replaces only the named manifest after
the ordinary local verification required for that refresh.

`project lock` verifies an unlocked view of the declared project, derives a
SHA-256 entry for every unit, and can repair a structurally valid but stale
project lock. `workspace lock` requires every nested package project to already
have a complete project lock, verifies all packages in deterministic dependency
order, then records one identity entry per package:

```json
{
  "name": "app",
  "path": "app",
  "project_name": "app_pkg",
  "project_version": "0.1.0",
  "project_sha256": "<64 lowercase hexadecimal characters>"
}
```

`project_sha256` binds the raw bytes of that package's
`aether.project.json`. The nested project lock binds raw source-unit bytes.
Together, a complete workspace lock detects package-manifest identity drift and
locked source drift without a registry or network.

## Verification behavior

An unlocked `aether.workspace/v1` document retains the M18/M22 behavior. When
the optional `lock` field is present, it must list every declared package exactly
once with the same package name and path. `aether workspace verify` then:

1. Canonicalizes package roots and the project manifest itself, rejecting a
   manifest symlink that leaves the package root.
2. Checks the lock's raw manifest digest and parsed project name/version.
3. Requires and verifies the nested complete project lock, including every unit
   digest and ordinary seed/artifact verification.

Lock-document errors and observed workspace-identity mismatches use
`AE-WORKSPACE-005`. Existing nested project verification failures remain in the
`AE-WORKSPACE-004` workspace envelope and retain the underlying project code.

For a locked workspace, `aether workspace build <workspace> --package <name>
--output <artifact>` performs workspace verification before elaboration and
before writing the artifact. A stale declared lock therefore cannot be bypassed
by building directly. This preflight verifies the full workspace; it is an
integrity guarantee, not an incremental-build performance claim.

## Safe local workflow

Refresh project locks first, then the workspace lock:

```powershell
aether project lock .\packages\util\aether.project.json --write
aether project lock .\packages\app\aether.project.json --write
aether workspace lock .\aether.workspace.json --write
aether workspace verify .\aether.workspace.json
```

The checked-in `examples/workspace/` fixture follows this workflow and remains
the M22 `app → util` exit-42 example.

## Compatibility and non-goals

- Existing unlocked `aether.workspace/v1` documents remain valid in package
  0.35. Older strict v1 parsers reject the new optional field rather than
  silently ignoring a lock.
- Aether source syntax, the 0.11 language surface, AETH v4–v11 verification,
  new AETH v11 emission, the seed artifact, forge ABI, authoring v7, VM
  semantics, and guest capabilities do not change.
- M19e's accepted active-frame cancellation design is a future AETH v12 / task
  frame proposal only; it does not add `task`, `checkpoint`, or any changed
  nursery behavior to package 0.35.
- PKG-001 does not add a registry, URLs, network fetch, semver/version solving,
  package archives, signatures, vendor cache, automatic multi-file lock writes,
  native compilation, or guest package authority.
- A workspace lock is a local reproducibility and integrity mechanism, not a
  package publication or third-party trust system.

## Evidence boundary

The package includes intended-behavior and hostile tests for stale lock repair,
complete entry coverage, malformed lock data, project-manifest drift, nested
source-unit drift, explicit write authority, locked build preflight, and the
checked-in M22 workspace fixture. Final command-level evidence and known limits
are recorded in the PKG-001 delivery report.

*End of AETHER_0.35.md*
