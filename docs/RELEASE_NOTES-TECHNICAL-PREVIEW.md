# Aether 0.12.0 — Technical Preview Release Notes

> **Historical 0.12 preview record.** The current local preview contract and
> package procedure are [AETHER_0.37.md](AETHER_0.37.md),
> [RELEASE_NOTES-0.37-LOCAL-PACKAGES.md](RELEASE_NOTES-0.37-LOCAL-PACKAGES.md),
> and [THREAT_MODEL-0.37-LOCAL-PACKAGES.md](THREAT_MODEL-0.37-LOCAL-PACKAGES.md).
> This document is retained as a dated distribution record, not current
> package instructions.

**Channel:** local folder + SHA-256SUMS (not a public GitHub Release)  
**Date:** 2026-08-04  
**Threat model:** [THREAT_MODEL-TECHNICAL-PREVIEW.md](THREAT_MODEL-TECHNICAL-PREVIEW.md)  
**Contract:** [MANIFEST.md](../MANIFEST.md)

## What this is

A **local-first** compiler CLI for Aether: parse Aether source, emit verified
**AETH** bytecode (default **v11** via the seed compiler), and run it in the
Aether VM. Offline project verify/format is included as a pilot.

This is a **technical preview**, not 1.0.

## Install (from package layout)

```text
dist/aether-0.12.0-tp/
  aether.exe
  SHA-256SUMS
  RELEASE_NOTES-TECHNICAL-PREVIEW.md
  CHANGELOG-0.12.md
  seed/aether_seed.aeth
  schemas/
  examples/
  verify-preview.ps1
```

1. Verify checksums: `Get-FileHash` vs `SHA-256SUMS`, or run `verify-preview.ps1`.  
2. Put `aether.exe` on your PATH or invoke with full path.  
3. Keep `seed/` next to the binary only if you rebuild from monorepo; the product
   binary embeds the seed artifact for offline compile.

## Quick start

```powershell
.\aether.exe version
.\aether.exe compile .\examples\welcome.ae --output .\welcome.aeth
.\aether.exe run .\welcome.aeth
.\aether.exe compile .\examples\host-pilot.ae --output .\host-pilot.aeth
.\aether.exe run .\host-pilot.aeth
# prints: Aether 0.12.0 exited with 48
.\aether.exe project verify .\examples\project\aether.project.json
```

## CLI surface

| Command | Purpose |
| --- | --- |
| `version` | Print product version |
| `check` | Parse / validate (bootstrap AST path) |
| `structure` | Emit `aether.ast/v6` JSON |
| `apply-edit` | Apply `aether.edit/v6` then seed-compile before write |
| `format` | Canonical format to stdout or `--output` |
| `project verify` | Offline project integrity + seed-compile units |
| `compile` | Seed-hosted compile (default); `--bootstrap` for rebuild path |
| `forge` | Host ABI compile using a compiler artifact |
| `run` | Verify then execute AETH |

## Trust model (summary)

- **Host CLI** is trusted by the operator who launched it.  
- **Guest AETH** is untrusted: no file/process/network/shell authority.  
- Pure host fixtures only: `whole_inc`, `text_extent`.  
- Writes only to explicit output paths.  
- No network registry, no model integration.

See the full [threat model](THREAT_MODEL-TECHNICAL-PREVIEW.md).

## Process exit codes (host CLI)

| Situation | Process exit |
| --- | --- |
| Success (including successful `run` of a program) | `0` |
| Usage / validation / compile / verify failure | non-zero |

Note: the **program** exit code from `main` is printed as
`Aether 0.12.0 exited with <n>` and is **not** always the process exit code.

## Non-claims

- Not full seed diagnostic parity for invalid source.  
- Not C/FFI, ambient I/O, package registry, or full LSP.  
- Not multi-unit project graphs (next track after preview).  
- Not a guarantee of production-grade multi-tenant security.

## Verify the package

From the package root:

```powershell
pwsh -File .\verify-preview.ps1
```

Expected: version 0.12.0, welcome compile+run, host-pilot program exit 48,
project verify pass, checksums match.

## Build from monorepo (developers)

```powershell
cargo build --release -p aether-cli
pwsh -File .\tools\package-preview.ps1
pwsh -File .\dist\aether-0.12.0-tp\verify-preview.ps1
```
