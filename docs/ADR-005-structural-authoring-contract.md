# ADR-005: Versioned structural authoring contract

**Status:** Accepted
**Date:** 2026-07-31
**Decision makers:** WPAI product direction; implementation authorized by the human director
**Related Rule IDs:** DOC-ADR-001, DOC-SYNC-001, SEC-INPUT-001, TEST-BEHAVIOR-001, RND-INVAR-001, IP-INVENTION-001

## Context

Aether 0.6 already has a deterministic formatter, a bootstrap-owned AST, and
source diagnostics with line/column locations. Those pieces are useful to
people, but they are not yet a stable machine contract for an AI authoring
tool. Text patches can be stale, use line positions that change after
formatting, and can accidentally write source that has not passed Aether's
validator.

M3 requires an authoring boundary that is useful to generated code while
preserving current product law: Aether source remains the only source language,
the Rust bootstrap remains the diagnostic authority, default artifact emission
remains seed-hosted, and AETH verification remains mandatory before an artifact
can run or be written.

## Decision

Adopt two versioned, local-only JSON contracts:

1. `aether.ast/v1` is a machine-readable semantic AST document. It is produced
   only from a successfully parsed and canonicalized Aether 0.6 source file.
   Every emitted construct carries a document-local node ID and canonical
   line/column span.
2. `aether.edit/v1` is a small, validated structural-edit protocol. Its
   operations are `replace`, `insertAfter`, and `delete` over top-level record
   and weave declarations. `insertAfter` may target the `world` node only to
   insert the first record. A payload is typed JSON AST data, never a line or
   byte-range text patch.

An edit contains the complete canonical `baseSource`. The implementation
canonicalizes the current source and requires exact equality before applying an
operation. This is the v1 stale-edit guard: it has no hash-collision or
time-of-check/time-of-use ambiguity and makes the base visible to an AI and a
human reviewer.

After applying operations in order, the implementation renders canonical text
from the AST and reparses it through the ordinary bootstrap validator. The CLI
and Studio additionally invoke `compile_with_seed` before they write or persist
the accepted edit, so accepted user-facing edits remain on the seed-hosted,
verified AETH product path. The protocol never writes a file itself, executes
source, contacts a model, or grants an artifact any capability.

Diagnostics are exposed as `aether.diagnostic/v1` envelopes with a stable code,
one-based line/column span, and human message. Codes identify a stable category;
the message retains precise context without becoming a second schema.

The formal wire definitions are
[aether-ast-v1.schema.json](../schemas/aether-ast-v1.schema.json),
[aether-edit-v1.schema.json](../schemas/aether-edit-v1.schema.json),
[aether-diagnostic-v1.schema.json](../schemas/aether-diagnostic-v1.schema.json), and
[AETHER_AUTHORING_PROTOCOL_v1.md](AETHER_AUTHORING_PROTOCOL_v1.md).

## Consequences

AI tools can inspect semantic structure and make bounded, reviewable changes
without relying on line numbers. The first version intentionally does not edit
individual statements or expressions in place. Replacing a complete weave is
the smallest unit that lets the existing compiler re-establish name, type,
ownership, resource, and control-flow invariants in one pass.

The contract adds a small, audited JSON dependency to the bootstrap core for
strict parsing and deterministic serialization. It is pinned in the existing
lockfile and replaces a riskier custom JSON parser. The previous
"dependency-free bootstrap core" description is updated accordingly; the
compiler and VM retain no network, model, filesystem, or guest-capability
dependency.

Node IDs are stable only within a matching canonical base document. They are
not durable database identifiers and are regenerated after every accepted edit.
This avoids pretending that a renamed or structurally replaced construct has
the identity of its predecessor.

## Alternatives considered

| Option | Pros | Cons |
| --- | --- | --- |
| Formatter-only text edits | Minimal implementation work. | Does not identify semantic targets, cannot reject stale line patches safely, and fails the M3 evidence requirement. |
| Arbitrary JSON Patch over a serialized AST | Familiar generic tooling. | Couples safety to mutable JSON paths and makes source-language invariants difficult to explain or validate. |
| Full arbitrary-node editing in v1 | Maximum expressive power. | Broadens the trusted parser and invalid-state surface before there is evidence for smaller operations. |
| Top-level typed declarations with exact canonical base source (chosen) | Bounded, deterministic, visible to reviewers, and sufficient to prove insert/replace/delete behavior. | Fine-grained edits require a future versioned extension. |

## Links

* Related ADRs: [ADR-002](ADR-002-ai-first-design-foundation.md), [ADR-003](ADR-003-value-resource-semantics.md), [ADR-004](ADR-004-aeth-v6-bounded-resources.md).
* Related direction: [NORTH_STAR.md](NORTH_STAR.md), [CORE_CLAIMS.md](CORE_CLAIMS.md), [ROADMAP.md](ROADMAP.md).
* Related implementation: `crates/xlang-core/src/authoring.rs`, `apps/xlang-cli/src/main.rs`, `apps/xlang-studio/src-tauri/src/main.rs`.
