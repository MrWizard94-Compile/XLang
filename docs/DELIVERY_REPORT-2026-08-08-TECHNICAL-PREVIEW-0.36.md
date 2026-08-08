# Delivery Report — Aether 0.36.0 Local Technical Preview Stabilization

**Title:** Version-derived local technical-preview package and release gate

**Date:** 2026-08-08

**Status:** Complete local-preview increment; no public release, commit, tag,
push, or license grant

**One-sentence summary:** Aether 0.36 now has a reproducible local package that
derives its version from the CLI manifest, verifies complete package integrity,
and proves current v11/v12 behavior from the staged consumer path.

## Scope

### In scope

- Replace stale 0.18/0.12 preview scripts with a 0.36 version-derived packager
  and consumer verifier.
- Add `aether-gate.ps1 -Mode release` as the release-level Constitution gate.
- Stage a local `dist/aether-0.36.0-tp` package with release metadata,
  SHA-256SUMS, current docs, seed, schemas, examples, and stdlib.
- Verify the release binary, v11/v12 examples, v8 structural authoring,
  project/workspace integrity, and rejected package/project tampering.
- Synchronize local-preview release notes, changelog, threat model, roadmap,
  manifest, root README, and historical 0.12 pointers.

### Out of scope

- New guest syntax, AETH semantics, task behavior, seed language, or host
  authority.
- Public release, source-control commit, tag, push, publication, licensing, or
  a claim of 1.0 readiness.
- New foreign ABI or native-backend scope.

## Local artifact evidence

| Item | Value |
| --- | --- |
| Package root | `dist/aether-0.36.0-tp/` |
| Package metadata | `aether.preview/v1`, Aether 0.36.0, language 0.11, AETH v11/v12, authoring v8, `UNLICENSED` |
| Staged file count | 308 files plus `SHA-256SUMS` |
| Release binary | `aether.exe`, 2,093,056 bytes |
| Binary SHA-256 | `B6AF0A911D8BB1C287DD5C7CF497E2EB5323B455555FC3F37B69A6E8D4D25B62` |
| Seed artifact | `seed/aether_seed.aeth`, 32,839 bytes |
| Seed SHA-256 | `DF4BBF08F33AF49BFE010E330373580C68B0F263057876E1D9A1D024EEC8CE0A` |
| Rust compiler | `rustc 1.96.0 (ac68faa20 2026-05-25)` |

`SHA-256SUMS` excludes only itself. The consumer verifier validates every listed
hash and also rejects any file not listed, closing the valid-prefix integrity
gap in the former preview verifier.

## Delivery manifest

| Path | Purpose |
| --- | --- |
| `tools/package-preview.ps1` | Derives package version, confines replacement to `dist/`, stages current contents, metadata, and checksums. |
| `tools/verify-preview.ps1` | Verifies exact package integrity and exercises staged product behavior without a source checkout. |
| `tools/aether-gate.ps1` | Adds `release` mode with workspace quality checks, staging, consumer verification, and tamper rejection. |
| `RELEASE-README-0.36-TECHNICAL-PREVIEW.md` | Portable package-root introduction, trust boundary, quick start, and license status. |
| `docs/RELEASE_NOTES-0.36-TECHNICAL-PREVIEW.md` | Current local-preview release notes, upgrade guidance, and known limits. |
| `docs/CHANGELOG-0.36.md` | User-facing deltas from the historical 0.12 preview to 0.36. |
| `docs/THREAT_MODEL-0.36-TECHNICAL-PREVIEW.md` | Current local-preview trust boundary covering grants, FFI pilot, package integrity, and v12 task frames. |
| `MANIFEST.md`, `README.md`, `docs/ROADMAP.md`, `docs/PROGRESS_REPORT-FULL-PROJECT.md` | Current release procedure, local-only channel boundary, and next-scope status. |
| `AUDIT_REPORT.md`, historical 0.12 release/threat documents | Correct current pointers while preserving dated records. |
| This report | Review package, evidence, risks, and Rule ID self-audit. |

## Verification evidence

| Command | Result |
| --- | --- |
| `pwsh -NoProfile -File .\tools\aether-gate.ps1 -Mode release` | **Pass** — Constitution pack, format, workspace Clippy with warnings denied, workspace tests, 32 seed≡bootstrap examples, host/project checks, seed forge identity, release build, staged consumer verification, and unlisted-file rejection. |
| `pwsh -NoProfile -File .\dist\aether-0.36.0-tp\verify-preview.ps1` | **Pass** — 308-file final package integrity; version; welcome 73; host pilot 48; active cancel 9; task capacity 3; task loop 3; v8 structure; project/workspace; path-escape rejection. |
| `git diff --check` | **Pass** after final documentation sync. |
| JSON schema parse | **Pass** for all checked-in schema documents. |

The release gate’s tamper subtest copies the staged package to a confined target
fixture, adds `UNLISTED-TAMPER-PROBE.txt`, and requires consumer verification to
fail. The fixture is removed in a `finally` block after the assertion.

## Constitution self-audit

| Rule / gate item | Status | Evidence |
| --- | --- | --- |
| `CONST-COMPLETE-001` / gate 1 | Pass | Package, verifier, metadata, docs, and release gate are complete; no placeholder behavior remains. |
| `CONST-DEP-001` / gate 2 | Pass | Release build, version metadata, package contents, and verifier are wired in dependency order. |
| `ENG-WARN-001` / gate 3 | Pass | `cargo fmt --check` and workspace/all-target Clippy with `-D warnings` pass. |
| `TEST-BEHAVIOR-001` / gate 4 | Pass | Workspace tests plus consumer behavior and negative tamper/path-escape tests pass. |
| `DOC-SYNC-001` / gate 5 | Pass | Current contract, release notes, changelog, threat model, roadmap, README, and historical pointers are synchronized. |
| `SEC-INPUT-001` / gate 6 | Pass | Confined package paths, exact checksums, unlisted-file rejection, and current capability/FFI risk documentation. No secrets were introduced. |
| Performance / gate 7 | Reviewed | Packaging is release-build-only; no VM hot-path or semantic performance claim changed. |
| Stack fidelity / gate 8 | Pass | Uses current Rust workspace, exact existing dependencies, PowerShell, and offline package flow. |
| `OPS-DEL-001` / gate 9 | Pass | `dist/` is generated/ignored; the source delivery contains no binary or temporary fixture. |
| Hardware / gate 10 | Pass | Native local release build and verifier ran on the target Windows workspace; no new service or hardware dependency. |
| Reproducibility / gate 11 | Pass | Version-derived staging, deterministic metadata/checksums, seed forge identity, and consumer verification pass. |
| `IP-INVENTION-001` / gate 12 | N/A to this packaging-only slice | No new language invention claim was introduced; existing claims/limits remain linked. |
| Multi-agent / gate 13 | N/A | Single-agent delivery; no parallel package authority. |
| `REV-PACK-001` / gate 14 | Pass | This report provides change manifest, commands, risks, next actions, and suggested commit. |
| Self-audit / gate 15 | Pass | This table records the completed release-gate evidence and deliberate boundaries. |

## Known risks and deliberate trade-offs

1. The package is local-only and `UNLICENSED`; a human must make the licensing
   and public-distribution decision.
2. The Whole-only foreign pilot loads an explicitly granted native library and
   is not sandboxed.
3. Grant-backed host I/O remains constrained by operator-selected roots/names;
   the product is not multi-tenant isolation.
4. M19e remains a checkpointed cooperative cancellation slice, not task
   handles/timeouts/manual cancellation/arbitrary preemption/parallelism.
5. Full invalid-source diagnostic parity for the seed compiler is still not
   claimed.

## Suggested commit message

```text
chore(release): harden Aether 0.36 local technical-preview packaging

Derive preview metadata from the CLI version, verify exact package membership,
exercise staged v11/v12 behavior, and add a release-level Constitution gate.
```

## Next actions for the human

1. Inspect or archive the local package at `dist/aether-0.36.0-tp/` if desired.
2. Decide whether to retain the local-only `UNLICENSED` channel or authorize a
   separate licensing/public-release scope.
3. Select one new ADR-backed language/design milestone; no additional feature
   is implicitly authorized by this stabilization increment.

*Release/stabilization handoff under `CONST-GATE-001`, `CONST-DONE-001`,
`REL-PACKAGE-001`, and `REV-PACK-001`.*
