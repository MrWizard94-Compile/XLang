# Aether structural authoring protocol v8

**Status:** Implemented in package **0.36.0**
**Protocol:** `aether.edit/v8`
**AST:** `aether.ast/v8`
**Diagnostics:** `aether.diagnostic/v8`

## Purpose

v8 retains v7's bounded declaration and statement-level edit operations and
adds the complete M19e task-frame representation. Every `Weave` node now carries
the required Boolean `task` field, and `Checkpoint` is a typed statement node.
This is an intentional protocol break: product `apply-edit` accepts v8 only.

## M19e contract

- `task: true` represents the `task weave` source role; `task: false` is
  required for every ordinary, host, and foreign weave.
- `{ "kind": "Checkpoint" }` represents the standalone `checkpoint` statement.
- A structural edit is never authority to create an invalid task. The bootstrap
  reparses, reformats, performs the complete task/nursery/capacity analysis, and
  seed-compiles before the CLI writes the requested output file.
- The schema represents syntax and structural shape only. M19e eligibility,
  resource ownership, effect closure, checkpoint placement, and scheduler
  legality remain compiler semantic checks.

## Preserved bounded edit model

v8 preserves exact canonical `baseSource`, top-level `replace` / `insertAfter`
/ `delete`, and statement `replaceStatement` / `insertStatementAfter` /
`insertStatementAt` / `deleteStatement` operations. Paths remain typed weave
body indices, not expression paths or JSONPath. The CLI never contacts a model,
network, or host service while applying an edit.

## Schema files

- [aether-ast-v8.schema.json](../schemas/aether-ast-v8.schema.json)
- [aether-edit-v8.schema.json](../schemas/aether-edit-v8.schema.json)
- [aether-diagnostic-v8.schema.json](../schemas/aether-diagnostic-v8.schema.json)

v7 documents remain historical. Clients must request v8 and include the required
`task` field when supplying a full `Weave` declaration.

*End of AETHER_AUTHORING_PROTOCOL_v8.md*
