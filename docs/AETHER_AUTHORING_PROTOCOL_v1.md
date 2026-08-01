# Aether structural authoring protocol v1

**Status:** Implemented M3 contract for Aether 0.6 tooling
**Protocol version:** `aether.edit/v1`
**AST schema version:** `aether.ast/v1`
**Diagnostic version:** `aether.diagnostic/v1`

## Purpose and boundary

This protocol gives AI and human tools a deterministic structural view of an
accepted Aether program and a small way to propose changes. It does not add
Aether syntax, change AETH, replace the formatter, execute code, write files by
itself, or grant any network/model/host authority.

The Rust bootstrap parses and validates source, then produces canonical text
and the `aether.ast/v1` document. The CLI validates an accepted edit again on
the seed-hosted product compiler before writing canonical source to the
caller-supplied output path. Artifact verification therefore remains unchanged.

All protocol input and output stays local to the CLI invocation. `structure`
emits a document to stdout; `apply-edit` reads explicitly named local files and
writes only its explicit output path after validation. The protocol has no
model, network, or application persistence integration.

## AST document

The canonical schema is
[`schemas/aether-ast-v1.schema.json`](../schemas/aether-ast-v1.schema.json).
It contains these required root properties:

| Property | Meaning |
| --- | --- |
| `schema` | Exactly `aether.ast/v1`. |
| `language` | `Aether` and the source-language version that produced the document. |
| `canonicalSource` | Formatter-owned LF source used as the document revision. |
| `program` | The complete semantic program tree. |

Every emitted syntax node includes `id`, `kind`, and `span`. A span is a
one-based canonical source position with `line` and `column`. IDs use a
document-local path such as `weave:main/body/0/value/left`; declaration IDs are
`record:<name>` and `weave:<name>`. IDs are regenerated after an accepted edit.
They must never be cached across a different `canonicalSource`.

The document represents all currently accepted Aether 0.6 source constructs,
including records, ownership modes, arenas, buffers, resource outcomes, and
the shallow expression/atom families. Its source spans are rebuilt after
canonical formatting, so CRLF input and optional final newlines cannot cause a
different structural location for the same program.

## Diagnostic envelope

The canonical schema is
[`schemas/aether-diagnostic-v1.schema.json`](../schemas/aether-diagnostic-v1.schema.json).
Compiler diagnostics use this envelope:

```json
{
  "schema": "aether.diagnostic/v1",
  "code": "AE-TYPE-001",
  "span": { "line": 4, "column": 3 },
  "message": "..."
}
```

The stable v1 categories are:

| Code | Meaning |
| --- | --- |
| `AE-SOURCE-001` | Empty or size-limited source input. |
| `AE-SYNTAX-001` | Lexical, grammar, indentation, or canonical-source violation. |
| `AE-NAME-001` | Invalid, duplicate, missing, or unresolved name. |
| `AE-TYPE-001` | Value, parameter, result, or operation type violation. |
| `AE-OWNERSHIP-001` | Move, borrow, access, mutability, or binding-state violation. |
| `AE-RESOURCE-001` | Arena, Buffer, access, or closed resource-outcome violation. |
| `AE-SEMANTIC-001` | Other validated source semantic violation. |
| `AE-EDIT-001` | Malformed edit JSON or unrecognized typed payload. |
| `AE-EDIT-002` | Unsupported protocol or schema version. |
| `AE-EDIT-003` | Stale canonical base source. |
| `AE-EDIT-004` | Missing, invalid, or disallowed structural target. |
| `AE-EDIT-005` | Invalid operation ordering or declaration kind. |
| `AE-EDIT-006` | Edit input exceeded a fixed byte, operation, node, or nesting limit. |

The code is stable for automation. The message is intentionally human-readable
and may become more specific in compatible releases. The span identifies the
structured construct or its source-validation result; protocol-only failures
use line 1, column 1 because they have no Aether source span.

## Edit request

The canonical schema is
[`schemas/aether-edit-v1.schema.json`](../schemas/aether-edit-v1.schema.json).
An edit is a JSON object with no extension fields in v1:

```json
{
  "protocol": "aether.edit/v1",
  "schema": "aether.ast/v1",
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
        "body": [
          {
            "kind": "Yield",
            "value": {
              "kind": "Atom",
              "atom": { "kind": "Whole", "value": 7 }
            }
          }
        ]
      }
    }
  ]
}
```

`baseSource` must exactly equal the current document's canonical source. The
implementation canonicalizes the source presented by the caller before that
comparison. A mismatch is `AE-EDIT-003`; no operation is applied and no source
is written.

Operations run in array order, with at most 32 operations in one request:

| Operation | Target | Effect |
| --- | --- | --- |
| `replace` | Existing `record:<name>` or `weave:<name>` | Replaces the matching declaration. The payload name and kind must match the target. |
| `insertAfter` | Existing top-level declaration; `world` only for a record | Inserts a typed record or weave in legal source order. |
| `delete` | Existing `record:<name>` or `weave:<name>` | Removes that declaration. Ordinary source validation decides whether the remaining program is valid. |

The `declaration` payload is an editable AST node. Strip only `id` and `span`
from an emitted declaration before sending it back: `kind` remains on every
typed node, including `RecordField` and `Parameter`. IDs and spans are output
provenance, never client-controlled source locations. All payload fields are
allow-listed; unknown fields, duplicate JSON object keys, wrong JSON types,
unknown node kinds, oversized input, excessive nesting, and invalid byte
literals are rejected deterministically.

The protocol does not support textual range replacement, JSON Pointer paths,
or direct statement/expression targets in v1. A tool that needs to change a
statement replaces the enclosing weave. That intentionally keeps every new
source shape on the existing parser/type/ownership/resource validator.

## Validation sequence

1. Read bounded UTF-8 JSON and reject duplicate or unknown fields.
2. Check the protocol/schema constants and exact canonical base source.
3. Resolve each allowed top-level target and construct typed AST payloads.
4. Render formatter-owned canonical Aether source.
5. Reparse and validate it with the bootstrap compiler.
6. In the CLI, seed-compile it and verify the returned AETH artifact before the
   canonical source is written to the requested output path.
7. Return the new canonical source and regenerated `aether.ast/v1` document.

Steps 1–5 are a pure core operation: they neither persist nor execute code.
Steps 6–7 are the user-facing authority boundary. A failed request leaves the
caller's document unchanged.

## Local CLI commands

```powershell
# Emit the canonical semantic document to stdout.
cargo run -p aether-cli -- structure (Resolve-Path .\examples\welcome.ae)

# Apply a local edit document only after structural, bootstrap, seed, and AETH verification succeeds.
cargo run -p aether-cli -- apply-edit (Resolve-Path .\examples\welcome.ae) .\edit.json --output .\target\welcome.edited.ae
```

No active desktop application is part of this protocol. A future interface must
call these versioned core/CLI contracts and receive its own explicit design and
security decision before it becomes a product surface.

## Compatibility policy

`aether.ast/v1`, `aether.edit/v1`, and `aether.diagnostic/v1` reject an
unknown major version. Additive or breaking wire changes require a new
explicitly versioned contract, schema, tests, documentation update, and ADR if
the authoring trust boundary changes.

This v1 contract is tooling metadata for Aether 0.6. It does not change the
Aether source grammar, AETH artifact version, Seed Profile, or claimed invalid
source diagnostic parity of the seed.
