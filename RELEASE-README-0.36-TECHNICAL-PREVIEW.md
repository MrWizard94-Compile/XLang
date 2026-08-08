# Aether 0.36.0 Local Technical Preview

**Channel:** local folder with `SHA-256SUMS`; not a public release

**License:** `UNLICENSED`. This package does not grant redistribution or use
rights. Obtain written authorization from Wizard Productions AI Studio before
any distribution outside the approved local channel.

## What this package is

This is an offline Windows CLI package for Aether 0.36.0. It parses Aether
source, default-compiles through the embedded Aether-written seed compiler,
verifies every AETH artifact before execution, and runs it in the Aether VM.
Source without task frames emits AETH v11; the bounded M19e task-frame subset
emits AETH v12. It is a technical preview, not a 1.0 release or a claim of
general systems-language completeness.

The package contains the checked-in seed artifact, schemas, examples, stdlib,
current contract documents, and a consumer verifier. It contains no network
client, registry, model integration, desktop workbench, native backend, or
ambient guest shell/process/network authority.

## Verify before use

From this package directory:

```powershell
pwsh -NoProfile -File .\verify-preview.ps1
```

The verifier checks exact package membership and SHA-256 hashes, release
metadata, the CLI version, v11 and v12 example behavior, structural authoring
v8, project/workspace integrity, and a rejected path-escape project.

## Quick start

```powershell
.\aether.exe version

.\aether.exe compile .\examples\welcome.ae --output .\welcome.aeth
.\aether.exe run .\welcome.aeth

.\aether.exe compile .\examples\active-cancel.ae --output .\active-cancel.aeth
.\aether.exe run .\active-cancel.aeth

.\aether.exe project verify .\examples\project\aether.project.json
.\aether.exe workspace verify .\examples\workspace\aether.workspace.json
```

`welcome.ae` prints `Aether` and its program exits with 73. `active-cancel.ae`
exercises the bounded v12 task-frame cancellation path and exits with 9. A
successful `run` command reports the guest program exit in its output while the
host CLI process exits successfully.

## Trust boundary

Treat source, project/workspace JSON, structural edit JSON, and AETH artifacts
as untrusted input. The CLI validates and verifies them before use. Default
`run` installs only pure fixtures. Grant-backed host I/O requires explicit
operator `--grant-read`, `--grant-write`, or `--grant-env` flags. The optional
Whole-only foreign pilot requires an explicit `--grant-lib KEY=PATH`; a granted
native library is **not sandboxed** and has the same class of risk as other
operator-approved native code.

## Compatibility and limits

- AETH v4–v11 artifacts remain accepted with their historical semantics.
- Non-task source remains on AETH v11; task source uses the closed AETH v12
  subset only.
- Authoring consumers must use `aether.ast/v8`, `aether.edit/v8`, and
  `aether.diagnostic/v8` for the current task/checkpoint surface.
- There are no task handles, timeouts, manual cancellation, arbitrary
  preemption, parallel execution, general FFI, native/LLVM backend, package
  registry, or ambient guest authority.

## Read next

- [Executable contract](MANIFEST.md)
- [Aether 0.36 contract](docs/AETHER_0.36.md)
- [Release notes](docs/RELEASE_NOTES-0.36-TECHNICAL-PREVIEW.md)
- [Changelog](docs/CHANGELOG-0.36.md)
- [Current technical-preview threat model](docs/THREAT_MODEL-0.36-TECHNICAL-PREVIEW.md)
- [AI authoring protocol v8](docs/AETHER_AUTHORING_PROTOCOL_v8.md)

*Local preview package entry point for Aether 0.36.0.*
