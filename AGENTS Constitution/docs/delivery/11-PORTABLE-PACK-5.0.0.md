# Pack release 5.0.0 — Universal portable solidification

**Status:** Released as pack law  
**Date:** 2026-07-28  

## What changed

* `VERSION` = 5.0.0  
* `PACK.md` — identity + `GOV-PORT-001`–`006`  
* `ADOPT.md` — move/copy/point projects  
* `templates/PROJECT-POINTER.template.md`  
* Tools path-independent; no desktop folder-name coupling  
* `verify-pack` enforces no absolute host paths in binding law  
* `tools/write-checksums.ps1` for movable release integrity  

## How to move

1. Copy or move entire `<pack-root>` folder.  
2. Update project pointers (relative paths).  
3. `pwsh -File tools/verify-pack.ps1` → exit 0.  

## How to adopt

See `ADOPT.md`.
