# Delivery Report: Aether M3 structural authoring

**Status:** Verified implementation evidence; not a packaged desktop release
**Date:** 2026-07-31
**Scope:** Aether 0.6 tooling metadata; no source-grammar or AETH change
**Rule IDs:** CONST-GATE-001, ENG-WARN-001, TEST-BEHAVIOR-001,
SEC-INPUT-001, DOC-SYNC-001, RND-INVAR-001

## Delivered boundary

M3 establishes three local, versioned, machine-readable contracts:

- `aether.ast/v1` is deterministic semantic structure for formatter-canonical,
  bootstrap-validated Aether 0.6 source.
- `aether.edit/v1` is a bounded typed top-level Record/Weave
  `replace`/`insertAfter`/`delete` protocol tied to an exact canonical source
  revision.
- `aether.diagnostic/v1` provides stable diagnostic categories, one-based
  source spans, and human-readable messages.

The pure core operation never persists or executes code. It strictly rejects
unknown or duplicate JSON fields, invalid types, unsupported versions, stale
canonical source, invalid targets, and fixed byte/operation/node/depth limits.
It renders canonical source and revalidates with bootstrap. The local CLI and
Studio then use the seed-hosted product compiler and verified AETH artifact
before they write or persist a successful edit.

The exact protocol, schemas, limits, and compatibility policy are in
[AETHER_AUTHORING_PROTOCOL_v1.md](AETHER_AUTHORING_PROTOCOL_v1.md); the design
decision is [ADR-005](ADR-005-structural-authoring-contract.md).

## Behavior and security evidence

- The AST document is deterministic, rebases spans to canonical LF source, and
  covers the shipped scalar, record, and M2 arena/buffer corpus.
- A stale `baseSource` returns `AE-EDIT-003` without a replacement source.
- Duplicate JSON object keys are rejected at every nesting level instead of
  being silently overwritten by a decoder.
- An emitted declaration becomes a valid edit payload by removing only generated
  `id` and `span`; all typed `kind` fields, including `Parameter` and
  `RecordField`, remain part of the contract.
- Identity replacements for every shipped example pass through the edit parser,
  canonical formatter, bootstrap validator, and default seed compiler.
- Studio stores only its existing local source/model keys; structural operations
  add no cloud, model, network, file, process, or guest capability authority.

## Final quality-gate record

All commands below exited zero after the final protocol correction:

| Command | Result |
| --- | --- |
| `cargo fmt --all -- --check` | Pass |
| `cargo test -p aether-core` | Pass: 38 core tests + 5 seed self-host/reproducibility tests (22m 25s) |
| `cargo test -p aether-cli` | Pass: 2 tests |
| `cargo clippy -p aether-core -p aether-cli -- -D warnings` | Pass: zero warnings |
| `cargo test -p aether-studio` | Pass: 6 tests |
| `cargo clippy -p aether-studio -- -D warnings` | Pass: zero warnings |
| `npm run lint` | Pass: ESLint `--max-warnings=0` |
| `npm test` | Pass: 4 frontend tests |
| `npm run build` | Pass: TypeScript and Vite production build |
| `aether check/compile/run examples/welcome.ae` | Pass: default seed compile; verified artifact runs with exit 73 |
| `aether compile --bootstrap` + `aether forge` for the seed | Pass: checked-in, bootstrap, and forged artifacts share SHA-256 `F440AD30DAEFA9FAC4ACDAC1EBC2B6F5A99EDF4EBD0A3FD8C4C3952C7F78F1F3` |
| `aether structure examples/welcome.ae` | Pass: parsed `aether.ast/v1` semantic document |
| JSON-schema parse | Pass: AST, edit, and diagnostic schemas declare Draft 2020-12 metadata |
| `pwsh -File "../../AGENTS Constitution/tools/verify-pack.ps1"` | Pass: `GOV-INT-001` (pack 5.0.1) |

## Changed-file manifest

| Path | Purpose |
| --- | --- |
| `Cargo.lock` | Records the pinned JSON serialization dependencies for the core crate. |
| `crates/xlang-core/Cargo.toml` | Pins `serde` and `serde_json` for strict local protocol handling. |
| `crates/xlang-core/src/lib.rs` | Exposes the structural-authoring and diagnostic public APIs. |
| `crates/xlang-core/src/authoring.rs` | Implements the AST serializer, strict edit parser, validation, diagnostics, and behavior tests. |
| `apps/xlang-cli/src/main.rs` | Adds local `structure` and seed-validated `apply-edit` commands. |
| `apps/xlang-studio/src-tauri/src/main.rs` | Adds local structural commands and schema-shaped diagnostic responses. |
| `apps/xlang-studio/src/services/desktopApi.ts` | Types and invokes the local Studio authoring commands. |
| `apps/xlang-studio/src/App.tsx` | Adds Structure and Apply edit flows that update source only after success. |
| `apps/xlang-studio/src/styles.css` | Styles the bounded authoring controls and results. |
| `apps/xlang-studio/src/services/persistence.test.ts` | Proves the Studio persistence boundary remains local and limited. |
| `schemas/aether-ast-v1.schema.json` | Defines the semantic-document and editable payload wire shapes. |
| `schemas/aether-edit-v1.schema.json` | Defines the bounded structural-edit request. |
| `schemas/aether-diagnostic-v1.schema.json` | Defines the stable diagnostic envelope. |
| `docs/AETHER_AUTHORING_PROTOCOL_v1.md` | Specifies the protocol, operations, limits, and compatibility policy. |
| `docs/ADR-005-structural-authoring-contract.md` | Records the M3 decision, alternatives, and safety boundary. |
| `README.md` | Makes the implemented M3 contract discoverable and states its limits. |
| `MANIFEST.md` | Adds the executable structural-authoring boundary. |
| `AUDIT_REPORT.md` | Updates the current implementation audit and non-claims. |
| `docs/AETHER_0.6.md` | Distinguishes language semantics from M3 tooling metadata. |
| `docs/ARCHITECTURE.md` | Maps structural-authoring flow and authority boundaries. |
| `docs/CORE_CLAIMS.md` | Classifies M3 claims as proven rather than aspirational. |
| `docs/NORTH_STAR.md` | Connects AI-primary authorship goals to a verified narrow contract. |
| `docs/LANGUAGE_LANDSCAPE.md` | Updates comparative design positioning. |
| `docs/ROADMAP.md` | Marks M3 implemented and preserves dependency-gated future work. |
| `docs/ADR-002-ai-first-design-foundation.md` | Links the AI-first foundation to the realized M3 boundary. |
| `docs/research/02-component-decomposition.md` | Updates the component model with the structural protocol. |
| `docs/research/03-synthesis-and-evidence.md` | Records the M3 evidence and remaining research limits. |
| `docs/DELIVERY_REPORT-2026-07-31-M3-STRUCTURAL-AUTHORING.md` | Provides this durable verification and handoff record. |

## Honest limits and release status

M3 does not add source syntax, AETH fields/opcodes, seed language surface,
generic effects, concurrency, FFI, arbitrary statement/expression targets, or
universal syntax-error-proof AI generation. It does not claim full invalid-source
diagnostic parity from the seed.

No Windows Tauri bundle, installer, or package inspection was performed for this
language-tooling milestone. Therefore this report makes no desktop-release
claim. A release still requires the additional `MANIFEST.md` bundle,
package-inspection, and applicable live-Ollama checks.
