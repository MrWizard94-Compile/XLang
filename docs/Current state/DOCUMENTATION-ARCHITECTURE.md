# Documentation architecture

## Status

Current as of 2026-08-14. This document defines the repository documentation
topology; it does not alter Aether language, AETH, seed, VM, or capability
semantics.

## Decision

The former flat `docs/` collection is divided by authority and freshness:

| Path | Contents | Normative use |
|---|---|---|
| [`docs/Current state/`](./) | Current contracts, active security model, current status, active roadmaps, and active validation matrices. | The default documentation source after the root entry documents. |
| [`docs/historical docs/`](../historical%20docs/README.md) | Dated ADRs, delivery/audit reports, superseded version documents, prior matrices, and research. | Evidence and provenance only; never infer a live product contract without confirming it in Current state. |

The root [docs index](../README.md) is the sole navigation gateway. No stale
compatibility copies or redirect files are retained at the old paths, so each
document has one canonical repository location.

## Integrity requirements

Every local Markdown link must resolve after a move. The repository owns that
check in `tools/verify-doc-links.ps1`, with behavioral fixtures in
`tools/test-doc-links.ps1`. `tools/aether-gate.ps1` runs both checks before the
formatting and product gates, and package verification requires the current
release documents at their canonical locations.

Run the checks from the repository root:

```powershell
pwsh -NoProfile -File .\\tools\\test-doc-links.ps1
pwsh -NoProfile -File .\\tools\\verify-doc-links.ps1 -RepositoryRoot .
```

## Maintenance rule

When a document changes category, move the canonical file, update its local
links, update both collection indexes when discoverability changes, and run the
link checks. Do not recreate an old-path copy solely to preserve a link; repair
the link at every in-repository consumer instead.
