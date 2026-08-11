# M25 validation matrix — offline local package publication

**Status:** Implemented and release-verified in package 0.37.0; full and release gates green on 2026-08-11
**ADR:** [ADR-107](ADR-107-m25-local-package-publication.md)
**Design:** [M25 local package publication](DESIGN-M25-LOCAL-PACKAGE-PUBLICATION.md)

| ID | Case | Expected evidence |
| --- | --- | --- |
| M25-P1 | Pack a complete locked project twice | Same metadata/content digest and verified directory bundle shape |
| M25-P2 | Verify a fresh bundle | Exact project manifest and every declared source unit pass digest, content, and product verification |
| M25-P3 | Publish a verified bundle then repeat publish | First creates deterministic cache entry; identical second publish is a verified no-op |
| M25-P4 | Install from bundle and cache | Both produce a normal locked `aether.project.json` package root that verifies |
| M25-P5 | Two independent workspace consumers import one installed package | Both workspace builds use existing `depends_on` authorization and execute expected results |
| M25-N1 | Missing, stale, or incomplete source project lock | Pack fails before bundle output |
| M25-N2 | Manifest schema/field/name/version/path failure | Package verification fails closed `AE-PACKAGE-001` or `AE-PACKAGE-002` |
| M25-N3 | Tampered project manifest, source unit, file digest, or content digest | Package verification fails closed `AE-PACKAGE-003` |
| M25-N4 | Unlisted file, traversal path, symlink, nonregular file, or size-bound violation | Package verification/pack fails closed before acceptance |
| M25-N5 | Existing output directory | Pack/install preserve it and fail `AE-PACKAGE-004` |
| M25-N6 | Existing cache identity with different content | Publish preserves cached entry and fails `AE-PACKAGE-005` |
| M25-N7 | Cache enumeration malformed entry | `pkg verify-cache` fails closed; no package is reported verified |
| M25-N8 | Product/seed identity after package-tooling addition | No language/seed change; full gate remains green |

## Boundary

This matrix covers only transparent local source bundles and explicit cache
materialization. It does not claim remote registry fetch, resolver/version
ranges, signed package trust, package assets, executable payloads, automatic
workspace mutation, or guest-visible package authority.

## Implemented evidence

| Evidence | Matrix coverage |
| --- | --- |
| `package_pack_verify_publish_install_is_deterministic_and_workspace_consumable` | P1–P5: repeated identity, verify, idempotent publish, direct/cache install, and two independent M22 consumers exiting 22 and 24. |
| `package_rejects_stale_lock_tampering_unlisted_files_and_existing_output` | N1, N3–N5: stale lock, altered unit, unlisted bundle file, existing pack/install outputs, and oversize source. |
| `package_rejects_missing_and_incomplete_source_locks` | N1: absent and incomplete project locks. |
| `package_rejects_unsafe_metadata_and_nonregular_source_input` | N2–N4: schema/name/version/unknown-field/traversal metadata; altered project, file, and content digests; and nonregular source. |
| `package_rejects_symlinked_source_units_when_the_platform_allows_test_symlinks` | N4: real symlink source rejection on the release-test Windows host. |
| `package_rejects_cache_collision_and_malformed_cache_entry` | N6–N7: preserved collision and malformed cache entry. |
| `package_install_arguments_are_closed_and_complete` + `aether-gate` M25 step | CLI argument closure and real product CLI flow. |
| `aether-gate -Mode full` / `-Mode release` | N8 source/seed identity and independently staged consumer/package evidence. |

---

*End of M25-VALIDATION-MATRIX.md.*
