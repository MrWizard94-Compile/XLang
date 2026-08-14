# Aether 0.37.0 — Local Source Packages Release Notes

**Channel:** Local technical preview only
**Date:** 2026-08-11
**Package:** `aether-core` / `aether-cli` 0.37.0
**Language / artifact / authoring:** Aether 0.11; AETH v11/v12 unchanged; `aether.ast/edit/diagnostic` v8 unchanged
**Contract:** [AETHER_0.37.md](AETHER_0.37.md)
**Security review:** [M25 threat model](THREAT_MODEL-0.37-LOCAL-PACKAGES.md)
**Delivery evidence:** [M25 delivery report](../historical%20docs/DELIVERY_REPORT-2026-08-11-M25-LOCAL-PACKAGE-PUBLICATION.md) — full and release gates passed 2026-08-11

## Highlights

- Adds M25 `aether pkg pack`, `verify`, `publish`, `install`, and
  `verify-cache` for transparent local source packages.
- Requires complete project locks before packing and binds raw manifest/source
  bytes with individual SHA-256 values plus a domain-separated content digest.
- Publishes only to an explicit deterministic local cache identity and rejects
  same-identity different-content collisions.
- Installs only to an explicit absent directory, yielding a normal locked
  project ready for explicit M18/M22 workspace composition.
- Extends the standard gate and portable preview verifier with real package
  lifecycle evidence.

## Upgrade notes from 0.36

No existing Aether source, AETH artifact, project/workspace schema, seed
artifact, VM behavior, authoring protocol, host grant, F-NATIVE flow, or
F-REGISTRY command needs migration. The version change is a toolchain-only M25
addition. Existing 0.36 task-frame semantics remain exactly as documented in
[AETHER_0.36.md](../historical%20docs/AETHER_0.36.md).

To use M25, first ensure a project carries a complete lock:

```powershell
.\aether.exe project lock .\my-project\aether.project.json --write
.\aether.exe pkg pack .\my-project\aether.project.json --output .\my-project.bundle
```

`pkg pack` and `pkg install` reject an existing output directory rather than
overwriting it. Keep a cache dedicated to M25: `pkg verify-cache` intentionally
rejects unrecognized entries in its cache root.

## Boundaries and non-goals

M25 is not remote distribution. It does not fetch URLs, resolve ranges, search
for a package, mutate workspaces, run package scripts, accept arbitrary assets
or archives, authenticate publishers, sign bundles, or widen guest authority.
The exact local cache is an integrity mechanism, not a trust/provenance system.

## Verification

```powershell
pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode full
pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode release
```

The release gate creates a version-derived `dist\aether-0.37.0-tp` folder and
runs its standalone verifier. It does not commit, push, tag, or publish.

*End of RELEASE_NOTES-0.37-LOCAL-PACKAGES.md*
