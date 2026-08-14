# Aether 0.37 Toolchain Contract (M25 local package publication)

**Status:** Current workspace package contract
**Language surface:** Aether **0.11**, unchanged
**Artifact output:** AETH **v11** for non-task source and **v12** for valid M19e task-frame source, unchanged
**Authoring:** `aether.ast/v8`, `aether.edit/v8`, `aether.diagnostic/v8`, unchanged
**Decision:** [ADR-107](ADR-107-m25-local-package-publication.md)
**Design:** [M25 local package publication](DESIGN-M25-LOCAL-PACKAGE-PUBLICATION.md)
**Validation:** [M25 validation matrix](M25-VALIDATION-MATRIX.md)
**Security review:** [0.37 local-package threat model](THREAT_MODEL-0.37-LOCAL-PACKAGES.md)
**Post-contract diagnostic maturity:** [ADR-108](ADR-108-barp-seed-speak-whole-text-yield-pilot.md), [ADR-109](ADR-109-barp-seed-speak-truth-choose-yield-pilot.md), [ADR-110](ADR-110-barp-seed-speak-unknown-call-pilot.md), [ADR-111](ADR-111-barp-seed-speak-root-yield-unknown-call-pilot.md), [ADR-112](ADR-112-barp-seed-speak-less-choose-yield-pilot.md), [ADR-113](ADR-113-barp-seed-speak-literal-truth-choose-yield-pilot.md), [ADR-114](ADR-114-barp-seed-speak-unary-literal-truth-choose-yield-pilot.md), and [ADR-115](ADR-115-barp-seed-speak-whole-truth-yield-pilot.md) update the checked-in seed's bounded direct-forge diagnostic behavior; none is an M25 protocol or version change.

## What changed

Package 0.37 adds a closed, local-only lifecycle for publishing one locked
`aether.project/v1` project as reusable Aether source:

```text
aether pkg pack <aether.project.json> --output <bundle-dir>
aether pkg verify <bundle-dir>
aether pkg publish <bundle-dir> --cache <cache-dir>
aether pkg install <bundle-dir> --output <package-dir>
aether pkg install --cache <cache-dir> --name <name> --version <version> --output <package-dir>
aether pkg verify-cache <cache-dir>
```

The package protocol is `aether.package/v1`. A transparent bundle contains only
its generated `aether.package.json`, a locked project manifest, and exactly the
project units declared by that manifest:

```text
<bundle>/
  aether.package.json
  project/
    aether.project.json
    <declared unit paths only>
```

M25 itself is toolchain-only. It does not change Aether source syntax, AETH
bytes, the seed compiler interface, verifier rules, VM semantics,
workspace/project schemas, or guest capabilities. Separately, post-contract
BARP diagnostic pilots can update the checked-in seed without changing M25's
package protocol or these language/runtime boundaries.

## Identity and acceptance

`pkg pack` requires a complete existing project lock and validates the source
through the ordinary product project verifier. It snapshots the manifest and
declared unit bytes before and after that validation; a changed snapshot fails
instead of creating a mixed bundle.

Each bundle binds the raw project-manifest SHA-256, a sorted complete
path-and-raw-file SHA-256 list, and a domain-separated SHA-256 over every path
length/path byte sequence and raw file length/file byte sequence.

`pkg verify` derives the allowed tree from the parsed project document, rejects
all unlisted bundle payload, checks every path and regular file, verifies both
digest layers, then re-runs locked-project verification. A successful report
names the exact `name@version`, content SHA-256, file count, and source bytes.

## Local cache and workspace reuse

`pkg publish` writes only to an explicit cache in this deterministic layout:

```text
<cache>/packages/<name>/<version>/
```

It is idempotent only when the existing verified entry has the same identity and
content digest. A different bundle for the same cache identity fails closed and
is never overwritten.

`pkg install` copies only the verified `project/` tree to an explicit, absent
directory. The output is an ordinary locked project root; users add it to an
ordinary `aether.workspace/v1` document and use existing M22 `depends_on`
authorization for cross-package imports. M25 does not edit workspaces or infer
dependencies.

## Safety bounds and write discipline

Metadata is limited to 1,000,000 bytes; a bundle holds at most 257 bound files
(one manifest plus up to 256 units); each bound file is limited to 1,000,000
bytes; total bound project bytes are limited to 16 MiB; and a package cache is
limited to 256 package identities. Package paths use an ASCII forward-slash
grammar and are confined to the bundle/project root. Symlinks, traversal,
nonregular files, unsafe names, malformed JSON, incomplete locks, altered
digests, and unlisted bundle paths reject before acceptance.

Every mutating command has an explicit destination. Pack, publish, and install
stage bounded bytes beside their destination, verify the staged result, and use
rename finalization. Pack/install never replace an existing output directory;
publication never replaces a conflicting cached identity.

## Compatibility

| Interface | 0.37 behavior |
| --- | --- |
| Existing Aether source | Unchanged; package operations do not compile or reinterpret guest source. |
| AETH v4–v12 inputs | Unchanged verifier/VM compatibility behavior. |
| Existing project/workspace files | Unchanged schemas and commands; package use remains an explicit workspace step. |
| Seed artifact | M25 itself has no seed interface or bytecode surface; the current checked-in seed also includes separately documented BARP diagnostic pilots. |
| F-REGISTRY | Separate law-fork functionality; M25 performs no fetch, URL handling, signature trust, or resolver action. |

## Explicit non-goals

M25 does not add a network registry, URLs, dependency ranges or version solver,
automatic workspace mutation, archive format, package assets, executable
payloads, package scripts, package signing/authentication, a guest package API,
ambient host capability, or a claim that a cache directory is a trusted remote
distribution channel. The local cache provides exact integrity and collision
protection, not publisher identity or compromise recovery.

## Local verification

```powershell
cargo run -p aether-cli -- project verify .\examples\package-publish\source\aether.project.json
cargo run -p aether-cli -- pkg pack .\examples\package-publish\source\aether.project.json --output .\target\local-math.bundle
cargo run -p aether-cli -- pkg verify .\target\local-math.bundle
cargo run -p aether-cli -- pkg publish .\target\local-math.bundle --cache .\target\aether-package-cache
cargo run -p aether-cli -- pkg install --cache .\target\aether-package-cache --name local_math --version 1.0.0 --output .\target\local-math
cargo run -p aether-cli -- project verify .\target\local-math\aether.project.json
pwsh -File .\tools\aether-gate.ps1 -Mode full
```

The core tests additionally prove two independent workspace consumers can build
and execute against one installed package. Full delivery evidence is recorded in
[the M25 delivery report](DELIVERY_REPORT-2026-08-11-M25-LOCAL-PACKAGE-PUBLICATION.md).

*End of AETHER_0.37.md*
