# Delivery Report

**Title:** Retire active Aether Studio and make Aether CLI-only
**Date:** 2026-08-01
**One-sentence summary:** Removed the active Tauri/React workbench and its
dependencies while retaining and proving the seed-hosted Aether language/CLI
toolchain.

## Scope

- Removed all 75 tracked files under `apps/xlang-studio/`, including the Tauri
  host, React frontend, package lock, icons, tests, and configuration.
- Removed the active Studio agent, generated `aether-studio.exe`, and the
  Studio-only Windows installer bundle.
- Removed the Studio Cargo workspace member and regenerated `Cargo.lock`;
  `aether-core` and `aether-cli` are now the only workspace packages.
- Updated the product contract, architecture, structural-authoring docs,
  roadmap/claim materials, audit, and agent kit to describe the CLI-only
  product boundary.
- Added [ADR-006](ADR-006-retire-aether-studio.md), preserving the decision,
  scope, alternatives, and Git-history reversibility.
- Kept `legacy/`, including `legacy/aether-genesis-ai-studio`, as reference
  material only; it is not active source or a build input.

## Rule ID self-audit

| Rule ID | Status | Notes |
|---------|--------|-------|
| CONST-GATE-001 | Pass | All applicable Section 0 checks completed before delivery. |
| CONST-DONE-001 | Pass | CLI-only product builds, tests, documents, and launches without extra scaffolding. |
| CONST-COMPLETE-001 | Pass | App source, workspace wiring, lockfile dependencies, generated installers, docs, and agent role were retired together. |
| CONST-DEP-001 | Pass | Cargo workspace and lockfile were updated in the same delivery; no deleted package reference remains. |
| ENG-WARN-001 | Pass | `cargo fmt --check`, Clippy `-D warnings`, and final `git diff --check` are clean. |
| TEST-BEHAVIOR-001 | Pass | CLI 2, core 38, and seed self-host 5 intended-behavior tests passed. |
| DOC-SYNC-001 | Pass | Current docs, architecture, claims, gates, and agent kit match the CLI-only implementation; relative-link audit passed. |
| SEC-INPUT-001 | Pass | Reviewed explicit CLI local-file I/O and removed the active desktop/model/network integration surface. |
| DEP-MIN-001 | Pass | Removed Tauri, Reqwest, frontend, and model-review dependency closure from the active lockfile. |
| REV-PACK-001 | Pass | ADR, manifest, exact commands, results, risks, and commit message are included here. |

## Modules loaded

- Pack `AGENTS.md`, `SOP.md`, Definition of Done, Engineering, Testing, and
  Documentation standards.
- Refactoring, Version Control, Dependencies, Security, Delivery,
  Review Packaging, Institutional Memory, Novel R&D, and IP/Invention modules.

## MANIFEST

See [MANIFEST.md](../../MANIFEST.md) and
[ADR-006](ADR-006-retire-aether-studio.md).

## How to verify

```powershell
Set-Location C:\WPAI\Software\XLang

pwsh -File "../../AGENTS Constitution/tools/verify-pack.ps1"
cargo fmt --all -- --check
cargo test -p aether-core -p aether-cli
cargo clippy -p aether-core -p aether-cli -- -D warnings

$welcome = (Resolve-Path .\examples\welcome.ae).Path
cargo run -p aether-cli -- check $welcome
cargo run -p aether-cli -- compile $welcome --output .\target\welcome.retirement.aeth
cargo run -p aether-cli -- run .\target\welcome.retirement.aeth

cargo run -p aether-cli -- compile .\seed\aether_seed.ae --output .\target\aether_seed.retirement.bootstrap.aeth --bootstrap
cargo run -p aether-cli -- forge .\target\aether_seed.retirement.bootstrap.aeth .\seed\aether_seed.ae --output .\target\aether_seed.retirement.forged.aeth
Get-FileHash .\target\aether_seed.retirement.bootstrap.aeth, .\target\aether_seed.retirement.forged.aeth

cargo build --release -p aether-cli
.\target\release\aether.exe version
cargo metadata --offline --no-deps --format-version 1
```

Observed results: pack verifier passed; format and Clippy were clean; 2 CLI,
38 core, and 5 seed self-host tests passed (the seed self-host suite took
1384.65 seconds); `welcome.ae` ran with exit value 73; both seed artifacts had
SHA-256 `F440AD30DAEFA9FAC4ACDAC1EBC2B6F5A99EDF4EBD0A3FD8C4C3952C7F78F1F3`;
the release `aether.exe` reported Aether 0.6.0 and had SHA-256
`8E6D9FC8BA6C9A73A135CD45E0379223DDB6DA49593C4E5E4AB3AAB05E7916D3`.

## Suggested commit message(s)

```text
refactor(product): retire Aether Studio and keep CLI toolchain
```

## Risks / trade-offs

- Aether no longer ships an active GUI, WebView persistence layer, installer,
  or model-review feature.
- A future interface must start with a new explicit product/authority decision;
  it may not revive legacy code or bypass core/CLI validation.
- The former implementation remains recoverable in Git history, while legacy
  intake remains non-production reference material.

## Next actions for human (priority order)

1. Continue the approved M4 typed errors/effects research/design package when
   ready; the language and CLI baseline are clean and verified.
2. If a future interface becomes useful, decide its user/problem boundary first
   and create a new ADR before adding dependencies or code.

## Multi-agent coordination note

N/A — this delivery was completed by one agent; no parallel file ownership or
partial-package reconciliation was needed.

## Section 0 self-audit log

1. Confirmed the deletion scope before removal and preserved CLI, seed, and legacy references.
2. Confirmed Cargo metadata contains only `aether-core` and `aether-cli`.
3. Confirmed retired source, installer artifacts, agent, and lockfile dependencies are absent.
4. Ran pack integrity, formatting, full core/CLI/seed tests, Clippy, live CLI, and release-binary gates.
5. Checked active-reference scope, documentation links, diff whitespace, and no active model/network claims.
6. Preserved existing language behavior; only a stale documentation comment changed in core source.
7. Updated ADR, manifest, audit, roadmap/claims, architecture, and agent kit in the same delivery.
8. No secrets, external publication, or multi-agent coordination were involved.
