# Aether structural authoring protocol v3

**Status:** Implemented Aether 0.8 / M5 contract

**Protocol version:** `aether.edit/v3`

**AST schema version:** `aether.ast/v3`

**Diagnostic version:** `aether.diagnostic/v3`

## Purpose and authority boundary

Version 3 is Aether's local, deterministic structural authoring contract. It
adds explicit binding-stage provenance for M5 without adding a host capability,
an evaluator callback, a model connection, or direct file authority.

`aether structure` parses and validates source, then emits formatter-canonical
text plus an `aether.ast/v3` document. `aether apply-edit` accepts a bounded v3
request only when its `baseSource` exactly matches the formatter-owned source;
it constructs a typed candidate, revalidates it with the Rust bootstrap,
seed-compiles and verifies it, and only then lets the CLI write to the caller's
explicit output path.

The protocol does not execute code, write files by itself, contact a model or
network, bypass source validation, or replace AETH verification.

## Version 3 addition: explicit binding stage

The v3 schemas are
[aether-ast-v3.schema.json](../../schemas/aether-ast-v3.schema.json),
[aether-edit-v3.schema.json](../../schemas/aether-edit-v3.schema.json), and
[aether-diagnostic-v3.schema.json](../../schemas/aether-diagnostic-v3.schema.json).

Every emitted or editable `Bind` statement has this required property:

| Property | Allowed values | Meaning |
| --- | --- | --- |
| `stage` | `runtime`, `comptime` | Whether the binding uses ordinary runtime evaluation or explicit M5 literal compile-time evaluation. |

`stage: "comptime"` renders exactly as `comptime bind`. It is valid only when
the normal Aether 0.8 semantic rules accept a root-only, immutable, literal
`Whole` arithmetic directive. The JSON contract records stage intent; it does
not bypass the parser or grant a wider comptime language.

`stage: "runtime"` renders normal `bind` or `bind mutable` source. A mutable
binding with `stage: "comptime"`, an omitted stage, an unknown stage, or a
non-M5 expression is rejected before any candidate output is written.

## AST document

The root document is:

| Property | Meaning |
| --- | --- |
| `schema` | Exactly `aether.ast/v3`. |
| `language` | `Aether` and the language version that emitted the document. |
| `canonicalSource` | Formatter-owned LF source used as the exact edit revision. |
| `program` | The complete validated program tree, including `effect` and binding `stage`. |

Every emitted node contains formatter-derived `id`, `kind`, and one-based
canonical `span`. `id` and `span` are generated provenance and must not be
included in an edit declaration. IDs are document-local; clients must use the
exact complete `canonicalSource` rather than treating an ID as a revision key.

An editable M5 binding looks like this:

```json
{
  "kind": "Bind",
  "name": "table_width",
  "mutable": false,
  "stage": "comptime",
  "value": {
    "kind": "Binary",
    "operation": "product",
    "left": { "kind": "Whole", "value": 16 },
    "right": { "kind": "Whole", "value": 8 }
  }
}
```

The exact expression object is defined by the schema. The core parses it into
the same typed AST used by source, then applies Aether 0.8 semantic validation.

## Diagnostic envelope

The envelope remains `{ schema, code, span, message }`, now with
`schema: "aether.diagnostic/v3"`. Existing source, ownership, resource,
effect, and edit codes remain stable. M5 adds:

| Code | Meaning |
| --- | --- |
| `AE-COMPTIME-001` | Invalid placement or literal evaluator shape. |
| `AE-COMPTIME-002` | Compile-time Whole overflow or zero divisor. |
| `AE-COMPTIME-003` | Fixed comptime directive budget exceeded. |

Messages may become more specific within the v3 contract. Protocol-only
failures use line 1, column 1; source failures retain their canonical source
span.

## Edit request

```json
{
  "protocol": "aether.edit/v3",
  "schema": "aether.ast/v3",
  "baseSource": "world example\n\nweave main [] -> Whole:\n  yield 0\n",
  "operations": [
    {
      "op": "replace",
      "target": "weave:main",
      "declaration": {
        "kind": "Weave",
        "name": "main",
        "parameters": [],
        "result": "Whole",
        "effect": "Total",
        "body": [
          {
            "kind": "Bind",
            "name": "answer",
            "mutable": false,
            "stage": "comptime",
            "value": {
              "kind": "Binary",
              "operation": "sum",
              "left": { "kind": "Whole", "value": 40 },
              "right": { "kind": "Whole", "value": 2 }
            }
          },
          {
            "kind": "Yield",
            "value": { "kind": "Atom", "atom": { "kind": "Name", "name": "answer" } }
          }
        ]
      }
    }
  ]
}
```

At most 32 ordered top-level `replace`, `insertAfter`, or `delete` operations
are allowed against `world`, `record:<name>`, or `weave:<name>`. This is not a
general AST-patch language: arbitrary statement pointers, textual range
patches, and direct bytecode edits remain out of scope.

## Validation and compatibility

1. Read bounded UTF-8 JSON and reject duplicate keys and unknown fields.
2. Require the exact v3 protocol/schema pair and exact canonical base source.
3. Construct only allow-listed typed record and weave declarations.
4. Render canonical Aether source and bootstrap-parse/validate it.
5. Seed-compile and verify AETH before the CLI writes the requested output.
6. Return the canonical source and a regenerated `aether.ast/v3` document.

The core part is pure; source persistence is the explicit CLI boundary in step
5. v1 and v2 documents are historical and rejected by v3 rather than silently
upgraded. A later wire change requires a new schema version, tests, synced
documentation, and an ADR if it expands the authoring trust boundary.

## Local commands

```powershell
cargo run -p aether-cli -- structure (Resolve-Path .\examples\comptime.ae)
cargo run -p aether-cli -- apply-edit (Resolve-Path .\examples\welcome.ae) .\edit.json --output .\target\welcome.edited.ae
```

No desktop application is part of this contract. Any future interface must use
these same explicit local core and CLI authority boundaries, or define an
equally constrained replacement.
