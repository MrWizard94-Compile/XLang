# Changelog — Aether 0.37.0 Local Technical Preview

## 0.37.0 — M25 local package publication

### Added

- `aether.package/v1` transparent directory bundles containing generated
  metadata, a locked project manifest, and exactly declared Aether source units.
- `aether pkg pack`, `verify`, `publish`, `install`, and `verify-cache`.
- Domain-separated bundle content identity, individual raw-file SHA-256 values,
  strict path/tree/regular-file checks, bounded source/cache sizes, staged
  writes, and collision-preserving cache publication.
- Core evidence for deterministic pack, two independent workspace consumers,
  hostile metadata/filesystem inputs, cache collision, and output preservation.
- Quick/full gate and portable preview verifier M25 lifecycle coverage.

### Changed

- `aether-core` and `aether-cli` package versions advance from 0.36.0 to 0.37.0.
- The current contract is [AETHER_0.37.md](AETHER_0.37.md); 0.36 remains the
  historical M19e task-frame contract.

### Unchanged

- Aether 0.11 source grammar and semantics.
- AETH v11/v12 behavior and v4–v12 compatibility input support.
- Seed artifact, product compile authority, bootstrap recovery role, authoring
  v8, project/workspace schemas, host grants, and F-NATIVE/F-REGISTRY boundaries.

### Not added

- Remote registry, URL fetch, dependency resolver/ranges, automatic workspace
  editing, archive payload, package scripts/assets, signing/publisher identity,
  or guest-visible package authority.

*End of CHANGELOG-0.37.md*
