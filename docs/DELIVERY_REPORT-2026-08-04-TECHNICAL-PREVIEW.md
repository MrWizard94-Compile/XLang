# Delivery Report — Aether 0.12.0 Technical Preview

**Title:** Local technical preview package (TP-2)  
**Date:** 2026-08-04  
**One-sentence summary:** Offline Windows CLI package with checksums, threat model freeze, and consumer verification for Aether 0.12 / language surface 0.11.

## Scope

- **In:** P2 harden/threat model docs; P3 local `dist/aether-0.12.0-tp/` package; packaging and verify scripts; release notes and changelog.  
- **Out:** Git tag, GitHub Release, C/FFI, network registry, full LSP, multi-unit projects (P4.1 next).  
- **Channel:** Local folder + SHA-256SUMS only (P0 freeze).

## Rule ID self-audit

| Rule ID | Status | Notes |
| --- | --- | --- |
| CONST-GATE-001 | Pass | TP-1 gate script; preview consumer script green |
| CONST-DONE-001 | Pass | Package usable offline; notes + verify script included |
| CONST-COMPLETE-001 | Pass | No partial package; staging script complete |
| CONST-DEP-001 | Pass | No new language features; packaging only |
| ENG-WARN-001 | Pass | Release build succeeded; prior clippy gate green |
| TEST-BEHAVIOR-001 | Pass | Consumer verify: welcome, host-pilot 48, project, path escape |
| DOC-SYNC-001 | Pass | Threat model, notes, changelog, ROADMAP, AGENTS links |
| SEC-INPUT-001 | Pass | Threat model freeze; path-escape negative in verify |
| REV-PACK-001 | Pass | This delivery report + verification commands |
| REL-PACKAGE-001 | Pass | Binary + schemas + examples + seed pin + sums + notes |
| REL-DETERM-001 | Pass | SHA-256SUMS over package files |
| REL-HARDEN-001 | Pass | SOP 8–10 path; threat model + residual risks documented |
| SOP-PHASE-001 | Pass | Research–implement done earlier; audit→harden→deliver this arc |
| SOP-GATE-001 | Pass | Gate table observed for packaging increment |

## Modules loaded

Pack: AGENTS.md, SOP.md, DEFINITION-OF-DONE, ENGINEERING, TESTING, DOCUMENTATION, SECURITY (input), RELEASES, DELIVERY, REVIEW-PACKAGING.  
Project Level 4: AGENTS.md, MANIFEST, threat model, roadmap.

## MANIFEST

Product contract: [MANIFEST.md](../MANIFEST.md).  
Threat model: [THREAT_MODEL-TECHNICAL-PREVIEW.md](THREAT_MODEL-TECHNICAL-PREVIEW.md).  
Notes: [RELEASE_NOTES-TECHNICAL-PREVIEW.md](RELEASE_NOTES-TECHNICAL-PREVIEW.md).

## Package layout

```text
dist/aether-0.12.0-tp/
  aether.exe
  SHA-256SUMS
  RELEASE_NOTES-TECHNICAL-PREVIEW.md
  CHANGELOG-0.12.md
  THREAT_MODEL-TECHNICAL-PREVIEW.md
  MANIFEST.md
  seed/aether_seed.aeth
  schemas/
  examples/
  verify-preview.ps1
```

`dist/` is gitignored; rebuild with `tools/package-preview.ps1`.

## Build evidence

| Item | Value |
| --- | --- |
| Package version | 0.12.0 |
| Binary | `target/release/aether.exe` → staged as package `aether.exe` |
| Binary size | 1,290,240 bytes |
| Binary SHA-256 | `E6D958F111F820C9EAD7C1B49D6D6F4C5FA9A29D1EA289C5B314D97D54833FE7` |
| Seed SHA-256 | `6AC3C46B890029B646267E93C9FDA5CDD34F9E061D7BC0419D34E0DF6734B254` |
| rustc | 1.96.0 (ac68faa20 2026-05-25) |
| Package file count | 47 (+ SHA-256SUMS) |
| Consumer verify | `PREVIEW VERIFY PASS` |

## How to verify

```powershell
# From monorepo
pwsh -File .\tools\package-preview.ps1
pwsh -File .\dist\aether-0.12.0-tp\verify-preview.ps1

# Or from package root
pwsh -File .\verify-preview.ps1
```

Optional full seed forge identity (release-blocking confidence):

```powershell
pwsh -File .\tools\aether-gate.ps1 -Mode full
```

## Suggested commit message(s)

```text
docs(tp2): technical preview package scripts, threat model, delivery report

Add threat model freeze, changelog, release notes, package-preview and
verify-preview scripts, and SOP-10 delivery report for local dist/ TP.
```

## Risks / trade-offs

- Binary not committed (by design); operators must build or receive `dist/` out-of-band.  
- Full multi-generation seed rebuild not re-timed in this delivery; seed hash pin is recorded; `aether-gate -Mode full` remains the long-pole check.  
- Preview wording only — not 1.0.

## Next actions for human (priority order)

1. Accept technical preview wording and local package channel.  
2. Optionally archive/copy `dist/aether-0.12.0-tp/` to your preferred store.  
3. Green-light **P4.1** design for multi-unit offline projects (SOP design → ADR → matrix before code).  

## Multi-agent coordination note

Single-agent delivery under Constitution; no parallel package authority.

---

*SOP phase 10 delivery report (pack template fields).*
