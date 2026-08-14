# Aether documentation

This directory separates the current operating contract from retained evidence.
Start with the repository [README](../README.md), [MANIFEST](../MANIFEST.md),
and [project entry law](../AGENTS.md); then use one of the two collections below.

| Collection | Purpose | Use it for |
|---|---|---|
| [Current state](Current%20state/README.md) | Living product contract and active engineering direction. | Current behavior, supported toolchain contract, active security model, and the next accepted work. |
| [Historical docs](historical%20docs/README.md) | Dated ADRs, validation matrices, release evidence, superseded contracts, and research. | Provenance, prior decisions, and the evidence behind a current claim. |

## Classification policy

Current-state documents describe the product as it is presently supported or
the work that is presently authorized. Historical documents preserve their
original dated context. A historical document is never a current product
contract merely because it remains accurate in part.

No compatibility copies remain at the former flat `docs/` locations. Local
links are part of the repository contract and are checked by:

```powershell
pwsh -NoProfile -File .\\tools\\verify-doc-links.ps1 -RepositoryRoot .
```

The release gate also runs the verifier and its fixtures, so a packaging change
cannot silently ship a broken local documentation reference.
