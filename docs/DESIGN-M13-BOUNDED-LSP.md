# M13 Design: bounded offline Language Server Protocol (T-LSP)

**Status:** M13a implemented (package 0.17.0); M13b optional  
**Date:** 2026-08-04  
**Decision record:** [ADR-017](ADR-017-m13-bounded-lsp.md)  
**Validation record:** [M13 validation matrix](M13-VALIDATION-MATRIX.md)  
**Portfolio:** [ADR-014](ADR-014-post-m10-track-portfolio.md) track **T-LSP**  
**Depends on:** diagnostics codes/spans, `structure` / authoring v7, optional project open  
**Threat model:** [THREAT_MODEL-TECHNICAL-PREVIEW.md](THREAT_MODEL-TECHNICAL-PREVIEW.md) (no host I/O expansion for guest)  
**Related Rule IDs:** `DOC-ADR-001`, `CONST-DEP-001`, `SEC-INPUT-001`, `DOC-SYNC-001`, `TEST-BEHAVIOR-001`

---

## 1. Purpose and boundary

Editors (VS Code, Neovim, etc.) expect LSP for diagnostics, hover, and
navigation. Aether must not grow a **second product compiler** inside an LSP
process that silently writes files or bypasses seed validation.

**M13** specifies a **bounded, offline** language server that is a thin client of
existing `aether-core` APIs:

| May do | Must not do |
| --- | --- |
| Publish diagnostics from bootstrap `compile_source` / partial parse | Become default product bytecode authority |
| Provide document symbols from structure | Auto-write without explicit client `workspace/applyEdit` |
| Hover / definition for weaves, records, shapes, imports (best-effort) | Network, registry, model integration |
| Format via `format_source` / `documentFormatting` | Ambient filesystem walk outside opened docs + optional project root |
| Optional `textDocument/codeAction` → produce `aether.edit/v7` for client | Apply edits without client round-trip |

---

## 2. Core claim

> An offline `aether lsp` (stdio JSON-RPC) provides diagnostics, symbols,
> format, and limited navigation for open Aether documents by calling the same
> bootstrap authoring/compile APIs as the CLI. It never writes product artifacts
> or applies structural edits except by returning WorkspaceEdits / edit JSON for
> the client to confirm. Seed compile remains the product path for persisted
> `apply-edit` and `compile`.

**Falsifiers:** LSP writes `.ae` without client apply; LSP emits AETH as product
compile; silent network; diagnostics from a divergent semantic model.

---

## 3. Architecture

```text
┌─────────────┐  stdio JSON-RPC   ┌──────────────────┐
│ Editor      │◄────────────────►│ aether lsp        │
│ (trusted UI)│                   │ (host process)   │
└─────────────┘                   │  - open docs map │
                                  │  - project opt.  │
                                  └────────┬─────────┘
                                           │ pure API calls
                                  ┌────────▼─────────┐
                                  │ aether-core      │
                                  │ compile_source   │
                                  │ structural_doc   │
                                  │ format_source    │
                                  │ apply_structural │
                                  │   _edit (pure)   │
                                  └──────────────────┘
```

**Authority:** Human operates the editor. LSP process is trusted like CLI.
Guest AETH capability model is unchanged.

---

## 4. Design decisions

### D1 — Transport and binary

| Item | Choice |
| --- | --- |
| Binary | Same `aether` CLI: `aether lsp` (stdio) |
| Protocol | LSP 3.17 subset over JSON-RPC |
| Mode | Single-threaded event loop (M13); async optional later |

### D2 — Capability set (M13 pilot)

| LSP method | Support | Backend |
| --- | --- | --- |
| `initialize` / `shutdown` / `exit` | Yes | — |
| `textDocument/didOpen|didChange|didClose` | Yes | In-memory docs |
| `textDocument/publishDiagnostics` | Yes | `compile_source` + `diagnostic_json` mapping to LSP Diagnostic |
| `textDocument/documentSymbol` | Yes | `structural_document_json` → symbols |
| `textDocument/hover` | Yes | Name under cursor → weave/record/shape/param summary |
| `textDocument/definition` | Yes | Weave/record/shape declaration span (single-file first) |
| `textDocument/formatting` | Yes | `format_source` full document |
| `textDocument/rangeFormatting` | **No** (M13) | — |
| `textDocument/completion` | **Minimal** (keywords + in-file weaves) or defer | Prefer defer if schedule tight |
| `textDocument/rename` | **No** | — |
| `workspace/executeCommand` | Optional: `aether.applyEditV7` returns edit only | Client applies |
| `workspace/didChangeWatchedFiles` | Optional | Refresh project |

### D3 — Diagnostics mapping

- On open/change (debounced **150–300 ms**): run bootstrap compile on buffer text.  
- Map `Diagnostic { code, span, message }` → LSP `Diagnostic` with `source: "aether"`.  
- Full success → clear diagnostics.  
- **No seed compile** on every keystroke (too slow); seed remains for CLI apply-edit/compile.  
- Document in UI: “editor diagnostics are bootstrap; product compile is seed.”

### D4 — Multi-file / modules

| Phase | Behavior |
| --- | --- |
| M13a | **Single-file** open documents only; `import unit` may show diagnostics that import requires project |
| M13b | Optional: open `aether.project.json` → workspace folders; resolve imports for hover/definition across units via project root |

M13 ship may deliver **M13a only** if schedule demands; matrix splits a/b.

### D5 — Path and workspace security

- Only documents opened by the client are analyzed, plus optional project root
  set by `initializationOptions.projectFile` or `rootUri`.  
- Refuse to follow imports outside project unit list (reuse M10 path jail).  
- No recursive workspace index of entire disk.

### D6 — Format and edits

- `formatting` returns full TextEdit from `format_source`.  
- Structural edits: server may **compute** `apply_structural_edit` result and
  return `WorkspaceEdit` or command payload; **never** write disk itself.  
- Persisted structural apply continues via CLI `apply-edit` (seed) or client
  save after format.

### D7 — VS Code extension (thin)

Optional companion (can be separate delivery):

- `aether` path setting  
- Launch `aether lsp`  
- Language id `aether`, `*.ae`  

Extension is **not** required for Done if `aether lsp` + matrix + manual client
test exist.

### D8 — Package pin

Suggested: **0.17.0** when M13a ships.

### D9 — Stop conditions

1. LSP becomes the only compile path for product AETH  
2. Background seed forge on every keystroke without user control  
3. Network features  
4. Silent disk writes  
5. Divergent semantic rules from `aether-core`  

---

## 5. Invariants

| ID | Invariant |
| --- | --- |
| M13-INV-001 | `aether lsp` speaks stdio LSP subset only offline |
| M13-INV-002 | Diagnostics from bootstrap compile_source |
| M13-INV-003 | No product AETH emission from LSP |
| M13-INV-004 | No disk write of `.ae`/`.aeth` by server process |
| M13-INV-005 | Import resolution respects project path jail when multi-file enabled |
| M13-INV-006 | Format uses format_source (canonical LF) |
| M13-INV-007 | Honest docs: bootstrap diagnostics ≠ seed product compile |

---

## 6. CLI surface

```text
aether lsp
```

Optional flags (M13):

```text
aether lsp [--log-file <path>] [--project <aether.project.json>]
```

No TCP listen in M13 (stdio only).

---

## 7. Implementation plan

### M13a (minimum Done)

1. JSON-RPC framing + initialize handshake  
2. Doc store + didOpen/Change/Close  
3. publishDiagnostics  
4. documentSymbol + formatting  
5. hover + definition (single-file)  
6. Tests: mock RPC or unit-test handlers  
7. Docs + delivery report  

### M13b (follow-on)

1. Project-aware import resolution  
2. Cross-file definition  
3. Optional VS Code extension  

---

## 8. Non-goals

| Non-goal | Later |
| --- | --- |
| Full completion / snippets marketplace | — |
| Debug Adapter Protocol | Separate |
| Remote LSP / multi-tenant | Blocked by local-first |
| Refactoring renames across packages | After T-PKG |

---

*End of DESIGN-M13-BOUNDED-LSP.md*
