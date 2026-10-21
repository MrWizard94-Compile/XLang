# ADR-133: Honest default check

**Status:** Accepted — implemented<br>
**Date:** 2026-10-21<br>
**Related Rule IDs:** DOC-ADR-001, DOC-SYNC-001, ENG-WARN-001, TEST-BEHAVIOR-001, SEC-INPUT-001<br>
**Depends on:** ADR-044, ADR-046, ADR-051, ADR-064<br>
**Design:** [BARP](../Current%20state/DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md), [Seed profile](../Current%20state/SEED_PROFILE.md)

## Context

Default `aether check` and `aether compile` forge with the seed and treat a
verified artifact as success (ADR-044, ADR-064). The seed compiler is not a
complete checker:

- An unknown statement such as `bad thing` is skipped. Check prints `check
  passed` and the program runs.
- An undefined name often dies inside the seed VM. The host then reports
  `AE-SEED-008` with `unpack16 index is invalid` instead of the source name.
- `aether check --bootstrap` already reports both faults in plain language
  (`unknown Aether statement`, `value … has not been bound`) at a source line
  and column.

Rewriting the seed's statement walker until it matches the bootstrap validator
would be the long-term BARP direction. It would not, by itself, prove that
every program the validator rejects is rejected. The acceptance rule for this
change is that proof: ordinary default check and compile must not accept a
program [`compile_source`](../../crates/xlang-core/src/lib.rs) rejects, and the
text a person sees must name a file line without a raw VM unpack.

## Decision

1. **Seed bytecode stays the artifact.** When both accept, `check` and
   `compile` still return [`compile_product_bytecode`](../../crates/xlang-core/src/lib.rs)
   bytes. Bootstrap does not emit.
2. **Single-file rejection oracle.** [`compile_product_checked`](../../crates/xlang-core/src/lib.rs)
   runs the seed forge and `compile_source`. Success requires both. Forge still
   runs first (`product_path_forges_before_bootstrap_validate` stays true).
3. **What the user reads.**
   - Bootstrap rejects and the seed result is an opaque forge/verify failure
     (`AE-SEED-001`, `AE-SEED-002`, `AE-SEED-008`, `unpack16` / `unpack32`,
     `index is invalid`, `artifact error`, or `unreachable`): report the
     bootstrap diagnostic, which already carries line and column.
   - Bootstrap rejects and the seed result is a stable host preflight
     (`AE-SEED-003`–`007`, `012`–`017`) whose text is not one of those leaks:
     keep that preflight. A `line N` hint in the text becomes the span.
   - Only the seed fails, and the failure is opaque: report
     `internal compiler error` at a line and column. The VM sentence is not
     copied.
   - Only the seed fails with a stable preflight: keep the preflight.
   - CLI `check` and `compile` prefix the source path, so the line sits next
     to a file.
4. **Out of this oracle.** `aether.seed-modules/v1` catalogs,
   `aether.seed-bundle/v1`–`v3` bundles, and `aether.multi-source/v1` envelopes
   stay on the seed route. The bootstrap validator does not parse those
   frames. `compile_product_bytecode`, `product_diagnostics`, and
   `compile_with_seed` stay the classified forge ABI, including `AE-SEED-008`
   for tools that already consume it. LSP product diagnostics are unchanged.
   `check --bootstrap` remains the AST dump.
5. **Tracker.** `product_single_file_check_fail_closed() == true`.
   `product_cli_check_without_bootstrap()` stays true: success is still seed
   emission, not a bootstrap AST check.

## Consequences

### Positive

- Junk statements and unbound names fail default check and compile with a
  source location.
- A program the bootstrap validator rejects cannot be emitted by single-file
  default compile.
- Accepted programs, including the seed source, shipped examples, and resource
  `choose` yields, keep their seed bytes.

### Costs

- Ordinary check and compile parse twice.
- The seed checker itself is unchanged, so this is not diagnostic parity.
- Project, workspace, and catalog builds are outside the oracle.

### Honesty

Host preflight text can still say `product seed path failed` and can still
point at `aether check` for a fuller AST dump. Opaque `AE-SEED-008` remains on
the forge ABI. Neither is a claim that the seed names every fault.

## Links

- ADR-044, ADR-046, ADR-051, ADR-064, DESIGN-BARP-001, SEED_PROFILE
- Corpus: `crates/xlang-core/tests/honest_default_check.rs`

---

*End of ADR-133.*
