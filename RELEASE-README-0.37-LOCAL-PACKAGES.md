# Aether 0.37.0 Local Technical Preview

This offline Windows CLI package contains Aether 0.37.0. Aether parses only
Aether source, emits deterministic AETH bytecode, verifies it before execution,
and runs it in the Aether VM. Default compilation uses the checked-in
Aether-written seed compiler; bootstrap flags remain recovery/oracle tools.

Package 0.37 adds M25 local source packages. A complete locked project can be
packed into a transparent `aether.package/v1` directory bundle, verified,
published to an explicit local cache, and installed as a normal locked workspace
project. M25 is not a remote package manager, dependency resolver, signature
system, or guest capability.

## Quick start

Open PowerShell in this package directory:

```powershell
.\aether.exe version
.\aether.exe compile .\examples\welcome.ae --output .\welcome.aeth
.\aether.exe run .\welcome.aeth
```

The successful VM command reports `Aether 0.37.0 exited with 73`.

## Local package example

The shipped source fixture is already project-locked. Every output path is
explicit and must not already exist for `pack` or `install`.

```powershell
.\aether.exe pkg pack .\examples\package-publish\source\aether.project.json --output .\local-math.bundle
.\aether.exe pkg verify .\local-math.bundle
.\aether.exe pkg publish .\local-math.bundle --cache .\local-package-cache
.\aether.exe pkg verify-cache .\local-package-cache
.\aether.exe pkg install --cache .\local-package-cache --name local_math --version 1.0.0 --output .\workspace\local_math
.\aether.exe project verify .\workspace\local_math\aether.project.json
```

Add the installed directory deliberately to an `aether.workspace/v1` file and
declare the normal M22 `depends_on` relationship before another package imports
it. M25 never rewrites a workspace or resolves a package name automatically.

## Verify this preview

```powershell
pwsh -NoProfile -ExecutionPolicy Bypass -File .\verify-preview.ps1
```

The verifier checks SHA-256SUMS and release metadata, starts the executable,
validates seed/VM and project/workspace behavior, and exercises pack, verify,
idempotent publish, cache verification, both install paths, and installed
project verification in a temporary local directory.

## Security and limits

M25 treats selected local project, bundle, cache, and output paths as untrusted.
It rejects malformed metadata, unsafe paths, symlinks, nonregular files,
unlisted bundle payload, stale/incomplete locks, altered digests, oversized
inputs, conflicting cache identity, and existing pack/install targets. The cache
provides local content integrity, not publisher authentication. Do not trust a
bundle merely because it verifies if you do not trust the local source that
produced it.

This preview is `UNLICENSED`. It is not a public distribution, support promise,
or license grant. It has no ambient guest network, shell, process, model, or
file authority; host I/O remains operator-granted.

## Documentation

- [Aether 0.37 contract](docs/Current%20state/AETHER_0.37.md)
- [M25 design](docs/historical%20docs/DESIGN-M25-LOCAL-PACKAGE-PUBLICATION.md)
- [M25 ADR](docs/historical%20docs/ADR-107-m25-local-package-publication.md)
- [M25 validation matrix](docs/Current%20state/M25-VALIDATION-MATRIX.md)
- [M25 threat model](docs/Current%20state/THREAT_MODEL-0.37-LOCAL-PACKAGES.md)
- [Release notes](docs/Current%20state/RELEASE_NOTES-0.37-LOCAL-PACKAGES.md)

*Local preview package entry point for Aether 0.37.0.*
