# Aether Atlas Threat Model

## Scope

Atlas is an offline, single-operator demonstration of a bounded journal
recovery path. It protects the boundary that Aether can actually enforce:
verified AETH execution, explicit host grants, bounded guest resources, and
deterministic source/artifact identity. It does not claim to be a production
database, network service, cryptographic ledger, or durable storage engine.

## Assets and trust boundaries

| Asset | Boundary | Control |
| --- | --- | --- |
| Aether source and locked manifests | Repository/workspace input | Offline project and workspace verification plus committed locks. |
| AETH artifact | Compiler-to-VM boundary | The VM parses and verifies the artifact before it can run. The evidence script proves a modified artifact is refused. |
| Journal text | Explicit read grant | operator reads only its constant atlas.journal path through a caller-selected, jail-enforced grant-read root. |
| Receipt text | Explicit write grant | operator writes only its constant atlas.receipt path through grant-write; no directory traversal or input-derived path exists. |
| In-memory replay state | Guest resource boundary | A 512-byte arena controls the three table/buffer allocations; resource outcomes are handled explicitly. |
| Cancellation witness | M19e task-frame boundary | The separate fault persona is a v12 artifact with verifier-approved checkpoints and an erroring sibling. |

## Input handling

The journal is untrusted until the protocol parser accepts it. The accepted
frame has a fixed 62-scalar grammar and exact line positions. The parser:

1. checks the header, line separators, command text, field widths, and commit
   count;
2. validates each decimal glyph before accepting its decoded event;
3. uses fixed arithmetic folding rather than number, so malformed text does
   not become a numeric-conversion runtime fault; and
4. returns -100 for every rejected frame, which policy refuses to promote.

The operator currently recognizes the same canonical frame before issuing its
accepted receipt. It intentionally does not attempt to be a second,
general-purpose parser: the primary parser is the pure multi-package protocol
component.

## Deliberate non-goals and residual risk

- The rolling fingerprints are deterministic integrity identifiers, **not**
  cryptographic hashes, signatures, or authentication.
- Aether host I/O can read/write only granted local roots, but operator
  permissions still depend on the human supplying safe roots.
- There is no fsync, atomic rename, crash recovery, multi-process locking, or
  persistent database protocol.
- Error[Whole] is terminal and task cancellation has no guest cleanup hook.
  The fault persona proves the documented cancellation behavior; it does not
  claim transactional rollback.
- Current product compilation does not compose a host-ABI package with
  cross-package imports. Atlas isolates the adapter rather than weakening the
  verifier or adding host-side application logic.
- Inputs remain bounded by the Aether runtime Text limits. The accepted Atlas
  grammar is much stricter: exactly 62 ASCII scalars.

## Operational rule

Never run the operator with a broad or sensitive grant root merely because the
example itself uses fixed filenames. Use a dedicated input directory and a
fresh dedicated output directory, as the verification script does.
