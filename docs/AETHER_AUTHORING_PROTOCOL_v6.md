# Aether structural authoring protocol v6

**Status:** Implemented Aether 0.11 language / 0.12 toolchain contract

**Protocol version:** `aether.edit/v6`

**AST schema version:** `aether.ast/v6`

**Diagnostic version:** `aether.diagnostic/v6`

## Purpose

Version 6 extends the local structural authoring contract for host weaves (M8)
while preserving prior stage/shape/effect nodes. M9 project documents are a
separate schema (`aether.project/v1`), not part of the AST edit protocol.

## Schemas

- [aether-ast-v6.schema.json](../schemas/aether-ast-v6.schema.json)
- [aether-edit-v6.schema.json](../schemas/aether-edit-v6.schema.json)
- [aether-diagnostic-v6.schema.json](../schemas/aether-diagnostic-v6.schema.json)
- [aether-project-v1.schema.json](../schemas/aether-project-v1.schema.json) (project tooling)

## Additions beyond v5

| Addition | Meaning |
| --- | --- |
| `HostWeave` | Body-less total host service signature |
| `AE-HOST-001..003` | Host form, signature, and missing-service diagnostics |

v1–v5 remain historical and are rejected rather than upgraded.

## Preserved rules

Exact `baseSource` stale guard, top-level typed operations, required
`Bind.stage`, formatter reparse, seed compile before write, no host network or
model authority.
