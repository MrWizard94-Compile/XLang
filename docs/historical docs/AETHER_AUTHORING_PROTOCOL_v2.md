# Aether structural authoring protocol v2

**Status:** Historical Aether 0.7 / M4 contract; the current authoring
contract is [AETHER_AUTHORING_PROTOCOL_v3.md](AETHER_AUTHORING_PROTOCOL_v3.md).
**Protocol version:** `aether.edit/v2`
**AST schema version:** `aether.ast/v2`
**Diagnostic version:** `aether.diagnostic/v2`

## Purpose and trust boundary

Version 2 is the local, deterministic structural contract for Aether 0.7. It
adds the bounded `Error[Whole]` effect surface to the M3 structural authoring
contract without adding any host capability. It does not execute code, write
files by itself, use a model or network, bypass formatting, or replace AETH
verification.

`aether structure` parses and validates source, then emits formatter-canonical
text plus `aether.ast/v2`. `aether apply-edit` accepts a bounded v2 request,
revalidates its formatter-owned source with the Rust bootstrap, then
seed-compiles and verifies the result before it writes to the explicitly named
output path. The seed remains the default product compiler; the bootstrap
remains the diagnostic and seed-rebuild authority.

## Version 2 additions

The schemas are
[`aether-ast-v2.schema.json`](../../schemas/aether-ast-v2.schema.json),
[`aether-edit-v2.schema.json`](../../schemas/aether-edit-v2.schema.json), and
[`aether-diagnostic-v2.schema.json`](../../schemas/aether-diagnostic-v2.schema.json).
They retain v1 definitions for unchanged values and expressions, but emit and
accept this explicit weave property:

| Property | Allowed values | Meaning |
| --- | --- | --- |
| `effect` | `Total`, `ErrorWhole` | Whether the weave is total or declares the bounded `raises Whole` effect. |

The AST has three new statement kinds:

| Kind | Required payload | Source form |
| --- | --- | --- |
| `Raise` | `code` atom | `raise code` |
| `Forward` | `weave`, copy `arguments` | `forward call weave arguments` |
| `Handle` | `weave`, copy `arguments`, `successDestination`, `errorDestination` | `handle call weave arguments into success otherwise error into code` |

`Handle` is intentionally a terminal linear eliminator in M4: a normal result
is written to `successDestination` and yielded; an error code is written to
`errorDestination` and yielded. Both destinations are distinct mutable root
`Whole` bindings. This removes client-authored branch blocks from the first
effect contract and keeps the two exits explicit and verifiable.

`ErrorWhole` weaves use ordinary owned `Whole`/`Truth` copy parameters and
return `Whole`; `main` stays `Total`. Effect control cannot cross live owners,
loans, arenas, buffers, or resource outcomes. The full normative behavior is
in [AETHER_0.7.md](AETHER_0.7.md) and
[DESIGN-M4-TYPED-ERROR-EFFECTS.md](DESIGN-M4-TYPED-ERROR-EFFECTS.md).

## AST document

Every emitted node includes formatter-derived `id`, `kind`, and one-based
canonical `span`; do not send `id` or `span` back in an edit payload. The root
properties are unchanged from v1:

| Property | Meaning |
| --- | --- |
| `schema` | Exactly `aether.ast/v2`. |
| `language` | `Aether` and the language version that generated the document. |
| `canonicalSource` | Formatter-owned LF source used as the exact edit revision. |
| `program` | Complete typed program tree, including each weave `effect`. |

IDs are document-local and are regenerated after every accepted edit. Clients
must base an edit on the exact `canonicalSource`, never a stale node ID.

## Diagnostic envelope

The v2 envelope is still `{ schema, code, span, message }`. Existing M3 codes
remain stable. M4 adds these categories:

| Code | Meaning |
| --- | --- |
| `AE-EFFECT-001` | An effect declaration or entry boundary is invalid. |
| `AE-EFFECT-002` | An effect target, terminal form, or result relationship is invalid. |
| `AE-EFFECT-003` | An effect attempts to cross an ownership or resource boundary. |
| `AE-EFFECT-004` | An effect code or destination is not an allowed live copy value. |

The code and span are automation-facing. Messages can become more specific
within the v2 contract. Protocol-only validation still uses line 1, column 1.

## Edit request

```json
{
  "protocol": "aether.edit/v2",
  "schema": "aether.ast/v2",
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

The permitted operations are unchanged: at most 32 ordered top-level
`replace`, `insertAfter`, or `delete` operations against `world`,
`record:<name>`, or `weave:<name>` as the v2 schema allows. The complete source
is revalidated after each edit request; statement-level pointers and textual
range patches remain out of scope.

The editable M4 payload uses the same `effect`, `Raise`, `Forward`, and
`Handle` fields as emitted structure, minus generated `id` and `span`. All
fields are allow-listed; duplicate JSON keys, unknown fields, stale sources,
wrong kinds, unbounded input, and invalid typed nodes fail before any output is
written.

## Validation and compatibility

1. Read bounded UTF-8 JSON and reject duplicate or unknown fields.
2. Require the v2 protocol/schema pair and exact formatter-canonical base.
3. Construct only the allowed typed top-level declarations.
4. Render canonical source, bootstrap-parse and validate it.
5. Seed-compile and verify AETH before the CLI writes the requested output.
6. Return new canonical source and a regenerated `aether.ast/v2` document.

Steps 1–4 are pure core work. Step 5 is the explicit CLI write boundary. A
failure returns no candidate source or artifact write.

v1 documents remain historical M3 artifacts and are rejected by the v2
endpoint rather than guessed or silently upgraded. A future wire change needs
a new versioned schema, tests, docs, and an ADR if it changes the authoring
trust boundary.

## Local CLI commands

```powershell
cargo run -p aether-cli -- structure (Resolve-Path .\examples\error-effect.ae)
cargo run -p aether-cli -- apply-edit (Resolve-Path .\examples\welcome.ae) .\edit.json --output .\target\welcome.edited.ae
```

No desktop application is part of this contract. Any future interface must use
these explicit local core/CLI boundaries or define an equally constrained new
boundary.
