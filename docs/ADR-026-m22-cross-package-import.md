# ADR-026: cross-package unit import via workspace (M22)

**Status:** Accepted — **implemented in package 0.24.0**  
**Date:** 2026-08-04  

## Decision

1. Adopt DESIGN-M22 syntax `import unit "…" from package <name> as <alias>`.  
2. Resolve only through M18 workspace package roots + `depends_on`.  
3. Ship `workspace build --package` for product elaboration.  
4. Keep single-file compile rejecting module surface.

---

*End of ADR-026.*
