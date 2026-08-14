# Aether structural authoring protocol v4

**Status:** Implemented Aether 0.9 / M6 contract

**Protocol version:** `aether.edit/v4`

**AST schema version:** `aether.ast/v4`

**Diagnostic version:** `aether.diagnostic/v4`

## Purpose and authority boundary

Version 4 is Aether's local, deterministic structural authoring contract for
Aether 0.9. It preserves v3 binding-stage provenance and adds explicit shape
declarations plus dual-layout table expressions without adding a host
capability, model connection, or direct file authority.

`aether structure` emits formatter-canonical text plus an `aether.ast/v4`
document. `aether apply-edit` accepts a bounded v4 request only when its
`baseSource` exactly matches the formatter-owned source; it constructs a typed
candidate, revalidates with the Rust bootstrap, seed-compiles and verifies it,
and only then lets the CLI write to the caller's explicit output path.

## Version 4 additions

Schemas:
[aether-ast-v4.schema.json](../../schemas/aether-ast-v4.schema.json),
[aether-edit-v4.schema.json](../../schemas/aether-edit-v4.schema.json), and
[aether-diagnostic-v4.schema.json](../../schemas/aether-diagnostic-v4.schema.json).

| Addition | Meaning |
| --- | --- |
| `Shape` / `ShapeField` | Declared Whole-only layout product |
| `Table` expression | `shape` name plus `layout` of `rows` or `columns` |
| Resource store/load nodes | Closed table cell outcomes |
| Targets `shape:name` | Top-level insert/replace/delete for shapes |
| Diagnostics `AE-LAYOUT-001..003` | Layout/shape/capacity failures |

v1–v3 remain historical. They are rejected rather than silently upgraded.

## Preserved rules from v3

- Exact canonical `baseSource` stale guard
- Top-level typed operations only (record/shape/weave)
- Required `Bind.stage` of `runtime` or `comptime`
- Formatter-owned reparse and seed compile before CLI write
- No host file, process, network, model, or shell authority

## Explicit non-goals

Fine-grained body edits, automatic layout conversion, generic shape parameters,
and host ABI packing remain out of scope.
