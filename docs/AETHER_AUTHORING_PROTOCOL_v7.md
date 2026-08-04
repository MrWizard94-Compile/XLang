# Aether structural authoring protocol v7

**Status:** Implemented (package **0.16.0**)  
**Protocol:** `aether.edit/v7`  
**AST:** `aether.ast/v7`  
**Diagnostics:** `aether.diagnostic/v7`

## Purpose

v7 preserves v6 top-level declaration edits and adds **statement-level** body
edits (M12 / ADR-016).

## New operations

| op | Fields |
| --- | --- |
| `replaceStatement` | `path`, `statement` |
| `insertStatementAfter` | `path`, `statement` |
| `insertStatementAt` | `list`, `index`, `statement` |
| `deleteStatement` | `path` |

### Paths

- `weave:main/body/1`
- `weave:main/body/0/whenBright/0` (Choose)
- `weave:main/body/0/body/0` (While)

Not allowed: expression/atom paths, Together spawn micro-edits.

## Preserved rules

Exact `baseSource`, reparse, seed compile before CLI write, no network/model.

## Schemas

- `schemas/aether-ast-v7.schema.json`
- `schemas/aether-edit-v7.schema.json`
- `schemas/aether-diagnostic-v7.schema.json`

v6 clients must upgrade; product `apply-edit` accepts **v7 only**.
