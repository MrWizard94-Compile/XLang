# M22 Design: cross-package unit import (workspace packages)

**Status:** Accepted design for implementable ADR-026  
**Date:** 2026-08-04  
**Depends on:** M11 modules, M18 workspace  

---

## 1. Purpose

Allow a package’s units to import **lib units from another workspace package**
declared in `depends_on`, still offline and path-confined.

## 2. Core claim

> `import unit "<unit.ae>" from package <name> as <alias>` resolves `<unit.ae>`
> under the named workspace package root when building via workspace-aware
> elaboration. Paths stay under that package root. No network.

## 3. Syntax

```aether
import unit "lib/math.ae" from package util as math
```

Same-project imports remain:

```aether
import unit "lib/math.ae" as math
```

## 4. Resolution

1. Workspace document provides `name → package root` map.  
2. Import with `from package P` requires `P` in consumer’s `depends_on` (or
   self).  
3. Unit path validated with existing unit path grammar under package root.  
4. Target must be a **lib** unit of that package’s project document.  
5. Mangle identity: `package__unitpath__weave` (stable, no collision).

## 5. CLI

```text
aether workspace build <workspace.json> --package <name> --output <artifact.aeth>
```

Loads workspace, resolves package roots, elaborates consumer main cone with
cross-package imports, dual-compares seed≡bootstrap, writes seed artifact.

## 6. Non-goals

Registry names, version solve, importing main units, absolute paths.

## 7. Package pin

**0.24.0** (with M20) or **0.25.0** if split.

---

*End of DESIGN-M22-CROSS-PACKAGE-IMPORT.md*
