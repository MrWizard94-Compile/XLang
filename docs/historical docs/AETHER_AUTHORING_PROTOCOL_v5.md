# Aether structural authoring protocol v5

**Status:** Implemented Aether 0.10 / M7 contract

**Protocol version:** `aether.edit/v5`

**AST schema version:** `aether.ast/v5`

**Diagnostic version:** `aether.diagnostic/v5`

## Purpose

Version 5 extends the local structural authoring contract for structured
nurseries without adding host capabilities.

## Additions

Schemas:
[aether-ast-v5.schema.json](../../schemas/aether-ast-v5.schema.json),
[aether-edit-v5.schema.json](../../schemas/aether-edit-v5.schema.json),
[aether-diagnostic-v5.schema.json](../../schemas/aether-diagnostic-v5.schema.json).

| Addition | Meaning |
| --- | --- |
| `Together` | Lexical nursery statement node |
| `Spawn` | `spawn call` line with weave, arguments, destination |
| `AE-TASK-001..003` | Nursery shape, spawn, and boundary diagnostics |

v1–v4 remain historical and are rejected rather than upgraded.

## Preserved rules

Exact `baseSource` stale guard, top-level typed ops, required `Bind.stage`,
formatter reparse, seed compile before write, no host authority.
