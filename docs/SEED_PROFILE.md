# Aether Seed Profile

Status: normative seed-emission profile for package **0.36.0** (canonical 0.11
surface plus documented later bounded semantics), 2026-08-08.

This document defines the **Seed Profile** implemented by `seed/aether_seed.ae`.
It covers the documented canonical Aether 0.11 source surface and the later
seed-emitted product cases explicitly listed below. Product `compile` uses this
profile via the seed artifact. Product bytecode is forge-first (BARP Phase 2 /
ADR-044) without a bootstrap validate pre-gate; multi-module product emit does
not dual-compare-gate or bootstrap-parse for AST (ADR-045/047); structural-edit
accept is product seed (ADR-048). Product failures use bounded `AE-SEED-*`
codes (ADR-046/048 Phase 3a–3b). The product path can rebuild the seed artifact
from `seed/aether_seed.ae` byte-identically (independence proof; bootstrap
remains the recovery rebuild path). Rust bootstrap remains for seed recovery
rebuild, `check` AST full diagnostics, and dual-compare **proofs**. A Seed
Profile claim is not a claim of full bootstrap diagnostic parity for invalid input.
`compile_with_seed` never invokes bootstrap (ADR-051; empty placeholder `Program`
only); prefer `compile_product_bytecode` for emit-only. Product preflight codes
`AE-SEED-001`–`011` (ADR-046/050/052) are host/product fail-closed helpers, not full
seed diagnostic parity. CLI `aether check --product` validates the seed product
path without bootstrap AST (default `check` remains full diagnostics).
`format --product` / `project format --product` LF-normalize and product-accept
without bootstrap AST rewrite (default format remains full bootstrap canonical;
ADR-053/054). `structure --product` emits `aether.product-structure/v1` without
bootstrap AST (ADR-054). Project verify validates lib units via product seed
probes (ADR-052). Host-facing product diagnostics are [`product_diagnostics`]
with `AE-SEED-001`–`012` (ADR-055); raw `import unit` is `AE-SEED-012` — multi-module
product path is host elaborate + seed emit (ADR-056; seed does not elaborate
multi-file natively). LSP diagnostics are product-primary (ADR-058); hover/definition
are product-surface (ADR-066). **ADR-064–069:** default CLI check/format/structure/
project format, LSP format, product structural weave/statement/record ops, and seed
rebuild use the product seed path; bootstrap is recovery (`--bootstrap`), dual-compare
oracle, and residual nested body-list structural AST only. **ADR-094:** seed SPEAKs
structured `AETHER_SEED_ERROR` for empty source (AE-SEED-005, origin `seed-speak`);
full SPEAK conformance matrix remains residual.

Here, *canonical* means the Aether 0.11 grammar and formatting constraints in
[AETHER_0.11.md](AETHER_0.11.md): shallow prefix expressions, exact indentation,
root-only bindings, bounded immutable records, closed bounded-resource forms,
dual-layout tables, bounded terminal effect forms, M5/M15 arithmetic `comptime
bind`, structured nurseries, and capability-closed `host weave` declarations.
The profile does not silently expand that language surface.

## Claim

`seed/aether_seed.ae` is an Aether-written compiler that:

1. Accepts complete canonical Aether 0.11 source as `Text`.
2. Parses statements and expressions itself (no host parser callback).
3. Emits a complete AETH **v11** artifact for source without task frames and
    AETH **v12** for valid M19e task source, with resource-capacity header,
    optional record table, shape table, function metadata, table/nursery
    opcodes, and `HOST_CALL` through ordinary `Bytes` operations.
4. Exposes the forge ABI `weave compile [borrow source: Text] -> Bytes`.
5. Rebuilds its own source byte-for-byte under `aether forge`.
6. Compiles every documented canonical statement, shallow expression, literal,
   ownership mode, and multi-weave program (including forward calls to weaves
   declared later), matching bootstrap output byte-for-byte.
7. Accepts CRLF or LF line endings, including a valid final source line without
   a line terminator; canonical emission is independent of host newline style.
8. Decodes the Aether text escapes `\\`, `\"`, `\n`, `\r`, and `\t` before
   recording UTF-8 byte lengths.
9. Parses bounded immutable record declarations, constructors, and explicit
   borrowed field projections, plus `arena`, Whole/Truth `buffer`, `access`,
   `count`, and closed allocation/append/lookup outcomes; it matches bootstrap
   v10 output byte-for-byte for the documented canonical corpus.
10. Parses `raises Whole`, terminal `raise`, `forward call`, and one-line
    terminal `handle call ... into success otherwise error into code`; it emits
    `RAISE` (53), `FORWARD_CALL` (54), and `HANDLE_CALL` (55) byte-for-byte
    like the bootstrap.
11. Parses root-only immutable M5/M15 `comptime bind` with one `sum`,
    `difference`, `product`, `quotient`, or `remainder` operation over Whole
    literals or prior same-weave comptime names, evaluates it with checked
    Aether arithmetic, and emits `COMPTIME_WHOLE` (56) byte-for-byte like the
    bootstrap.
12. Parses `shape` declarations and `table Shape layout rows|columns` with
    closed allocate/store/load; emits table opcodes (57–61) byte-for-byte like
    the bootstrap for the documented M6 corpus.
13. Parses lexical `together:` nurseries with `spawn call ... into` lines;
    emits nursery opcodes (62–64) byte-for-byte like the bootstrap for the
    documented M7 corpus.
14. Parses body-less `host weave` declarations and emits AETH v11 host function
    entries (`kind=host`, empty code) plus `HOST_CALL` (65) for host targets
    byte-for-byte like the bootstrap for the documented M8 host-pilot corpus.
    The seed itself does not call host weaves.
15. Parses body-less `foreign weave` declarations (M21 Whole-only pilot) and
    emits host-kind function entries whose AETH names use the encoded foreign
    marker (`\x1eF\x1e` + library + `\x1e` + symbol + `\x1e` + user name),
    matching bootstrap byte-for-byte for `examples/foreign-pilot.ae` and
    `examples/foreign-sum.ae`. Runtime load still requires operator
    `--grant-lib`; the seed does not load libraries.
16. Accepts multi-weave total `arena N` declarations (M19d) and sums capacities
    into the AETH header, matching bootstrap for `examples/spawn-arena.ae`.
17. For M23 pure comptime calls, the seed interprets raw
    `comptime bind name <- call weave args...` under the D2a body subset
    (Whole `bind` / `revise` / terminal `yield`, prior total Whole guest
    callees, literals or prior comptime args), folds to `COMPTIME_WHOLE` (56),
    and dual-compares with bootstrap for `examples/comptime-calls.ae`. Product
    emission is forge-first (no bootstrap pre-validate; ADR-044); no
    materialization rewrite is applied
    (`seed_interprets_m23_comptime_calls_natively` is true; ADR-043 Phase 1).
18. Parses `task weave` and `checkpoint`, emits v12 task flags,
    `frame_arena_capacity`, and `TASK_CHECKPOINT` (67), preserves verifier-safe
    task loop back edges, and computes the exact main-plus-largest-task-nursery
    capacity header. The active-cancel, capacity, loop, and forward-task corpus
    matches bootstrap byte-for-byte.

Evidence lives in `crates/xlang-core/tests/seed_self_host.rs` and the checked-in
artifact `seed/aether_seed.aeth`.

Package 0.34's RTP-001 Text cache is VM-internal and deliberately outside this
profile: it changes neither seed source nor artifact bytes. The full seed
identity proof remains required after the runtime change.

Package 0.35's PKG-001 project/workspace lock workflow is host-local tooling
outside the profile. It does not alter Aether source parsing, seed source,
emitted AETH bytes, forge inputs, or the required seed identity proof.

Package 0.36's M19e task support is inside this profile. The checked-in seed
artifact remains a v11 compiler program, but it emits v12 when it parses valid
task source. Its v12 byte identity with the Rust bootstrap is required just as
for the prior seed surface.

## Required shape

A Seed Profile program must contain:

- One `world` line (name is accepted but not used by the seed emitter).
- Zero or more `record` and `shape` declarations after `world` and before the
  first weave. Each record has one to 64 primitive fields in declaration order.
  Shapes are Whole-only products (1–8 fields).
- One or more weaves declared at indentation level zero.
- Exactly one runnable main:

      weave main [] -> Whole:

The forge-facing compiler shape used by the seed itself remains:

      weave compile [borrow source: Text] -> Bytes:

Input programs are not required to declare `compile`. Weave indices in the
emitted artifact follow declaration order (0-based). Every weave body is
compiled from source, including `main`.

## Locals and parameters

- Parameters and locals may use ordinary lowercase names. The seed assigns slots
  in declaration order and resolves names through a per-weave name map.
- The compile parameter name `source` remains slot `0` when present.
- `vN` names still work when bound/declared that way (the seed source itself uses
  them heavily).
- Parameter lists may include multiple entries separated by commas. Optional
  `borrow` ownership is accepted for owners, `access` is accepted only for an
  `Arena` parameter, and owned is the default. A Buffer parameter requires
  exactly one access Arena parameter in the same weave.
- Result types are `Text`, `Whole`, `Truth`, `Bytes`, or a declared record name.
  `Arena`, access loans, `BufferWhole`, and `BufferTruth` are not result types.
- Nested blocks may `revise` existing locals but must not introduce bindings.
- `bind` / `bind mutable` establish runtime locals; `revise` replaces a live
  local. M5/M15 `comptime bind` establishes one immutable checked `Whole` local
  under the fixed 1,024-directive budget. M23 pure calls are evaluated natively
  by the seed under the D2a body subset and folded to `COMPTIME_WHOLE`.
- Hex `bytes "ff…"` literals decode to raw bytes. Text literals decode the five
  defined escapes (`\\`, `\"`, `\n`, `\r`, `\t`) and record **byte** length of
  UTF-8 content after decoding (not scalar count).
- One total weave may declare `arena N`; M19d sums every declared capacity into
  the v11 resource-plan header. `buffer Whole` and `buffer Truth` establish
  unallocated owner placeholders. Resource owner replacement uses only closed
  outcomes, never `revise`.

## Statements

Supported forms:

| Form | Notes |
| --- | --- |
| `bind name <- expression` | Immutable local |
| `bind mutable name <- expression` | Mutable local |
| `comptime bind name <- op left right` | Root-only immutable M5/M15 Whole result; operands are literals or prior comptime names |
| `comptime bind name <- call weave args...` | M23 product form; seed interprets D2a callee body and emits `COMPTIME_WHOLE` |
| `revise name <- expression` | Same-type replacement |
| `speak expression` | Expression must be `Text` |
| `release name` | Root-only logical destruction of a live unique/resource owner; emits `RELEASE` (66) |
| `yield expression` | Root-only; result type must match the weave |
| `raise whole-atom` | Root-only terminal exit from a `raises Whole` weave |
| `forward call weave args...` | Root-only terminal propagation from a `raises Whole` weave |
| `handle call weave args... into success otherwise error into code` | Root-only terminal discharge in a total `Whole` weave |
| `choose expression:` / `otherwise:` | Condition is `Truth` |
| `while expression:` | Condition is `Truth` |

Indentation is exactly two spaces per level. Control-flow jump targets are
patched with `poke32` after structured blocks close.

## Expressions

Expressions are shallow prefix forms. Operands are atoms (literals or
`borrow`/`move` names). Intermediate results must be bound before reuse.

Supported operations (by seed emitter opcode mapping):

- Unary: `not`, `measure`, `render`, `extent`, `encode`, `decode`, `number`,
  `pack16`, `pack32`, `pack64`, `count borrow buffer`
- Binary: `sum`, `difference`, `product`, `less`, `same`, `join`, `glyph`,
  `quotient`, `remainder`, `fuse`, `append`, `octet`, `unpack16`, `unpack32`
- Ternary: `cut`, `slice`, `seek`, `poke`, `poke32`
- Records: `make record-name fields...` constructs every declared field in
  order; `field borrow record-binding field-name` projects a cloned immutable
  field.
- Call: `call weave_name args...` emits `OP_CALL` (21), the callee's declaration
  index as `u16`, and argument count as `u8`. A first pass records every weave
  name and result type so forward calls are allowed. Result type is the callee
  weave result.
- Atoms: decimal `Whole` literals (optional leading `-`), `bright` / `dim`,
  text literals, `bytes "hex..."`, ordinary names for copyable values, and
  `borrow` / `move` of owner locals and parameters, plus `access` of a live
  Arena only in a resource operation or access-parameter call.
- Resources: `allocate access arena move buffer count into buffer`,
  `append move buffer value into buffer`, and `at borrow buffer index into
  target`. Each is emitted only from a terminal resource `choose`, has an
  explicit `otherwise` branch, and lowers to direct v6/v7/v8 replacement/update
  instructions.
- Effects: `raise whole`, `forward call weave args...`, and `handle call weave
  args... into success otherwise error into code`. The seed emits direct v7
  two-exit metadata and branch targets; M4's source/type/ownership restrictions
  are verified by the product artifact before output is accepted.
- Comptime: the seed directly accepts one M5/M15 signed-`Whole` binary operation
  over literals or prior same-weave comptime names, and M23 pure `call` forms
  under the D2a body subset (seed-native evaluation; BARP Phase 1). There are
  no comptime loops, recursion, text/bytes evaluation, effects, resources, host
  calls, or source-configurable fuel; at most 1,024 directives occur in one
  source program.

## Multi-weave emission

The seed keeps a function-table accumulator. On each new weave declaration and
at end of source it flushes the previous weave record:

- name length and ASCII name
- parameter count, then `(type, mode)` pairs
- result type
- effect tag (`0` total, `1` `Error[Whole]`)
- local count, then `(type, mutable)` pairs (parameters occupy the leading slots)
- code length and instruction bytes

The final artifact is `AETH` + version `11` or `12` + arena capacity (`u32`
little endian) + bounded record schema table + function table. Record type descriptors
retain tag `5` plus a record identifier; v6+ use Arena/Buffer type tags,
access parameter mode, and the closed resource instruction payloads; v7 adds
the effect tag and three explicit effect instructions; v8 adds
`COMPTIME_WHOLE`; v9 adds shape metadata and table opcodes; v10 adds nursery
opcodes; v11 adds function host-kind metadata, `HOST_CALL`, and `RELEASE`; v12
adds task flags, per-function frame capacity, and `TASK_CHECKPOINT`.
This replaces the Stage 3 fixed two-weave (`compile` + synthetic `main`)
emitter.

## Explicit non-goals

Seed parity does not silently expand Aether 0.36 language rules. In particular, nested
expression trees, nested binding introduction, nested record fields, record
mutation, host record invocation, first-class resource outcomes, Buffer weave
results, resource-owner `revise`, OS-thread parallelism, automatic layout
rewrite, and general generics remain outside the language grammar rather than
Seed Profile exclusions. The Seed Profile compiler does **not** claim support
for:

- Host I/O, networking, or model access
- Full Aether diagnostic fidelity (invalid Seed Profile input may fail late or
  produce a rejectable artifact; the bootstrap compiler remains the complete
  diagnostic authority for invalid Aether input)
- M23 callees beyond the D2a body subset (nested calls, choose/while, effects,
  resources, host/foreign targets) — bootstrap rejects; seed must dual-compare
  only the proven corpus
- General async/parallel tasks, task handles, timeout or manual cancellation,
  nested task nurseries, arbitrary preemption, task external effects, or guest
  cancellation handlers beyond the M19e closed task/checkpoint subset
- Future Aether language extensions until they meet the same byte-identity proof

## Reproducibility procedure

From the repository root:

```powershell
cargo run -p aether-cli -- compile .\seed\aether_seed.ae --output .\target\aether_seed.aeth --bootstrap
cargo run -p aether-cli -- forge .\target\aether_seed.aeth .\seed\aether_seed.ae --output .\target\aether_seed.forged.aeth
Get-FileHash .\target\aether_seed.aeth, .\target\aether_seed.forged.aeth, .\seed\aether_seed.aeth
```

All three SHA-256 digests must match. The regression tests also forge:

1. A nearby source variant (different verified artifact — not a fixed payload).
2. A multi-weave program with `call` (byte identity with bootstrap + run).
3. A forward-call program (callee after caller) and a CRLF multi-weave source.
4. Every shipped `examples/*.ae` file seed-compiles byte-identically to bootstrap
   (`compile_with_seed`).
5. A prior canonical-surface corpus covering every statement, expression,
   ownership mode, literal mode, record operation, and final-line termination
   behavior.
6. The canonical M2 arena/buffer corpus: Whole allocation/append/lookup,
   allocation exhaustion, append full, lookup fallback, Truth elements, and an
   access-bound helper weave.
7. The M4 error and normal-exit fixtures, including `examples/error-effect.ae`.
8. The M5 literal and M15 name-chain comptime fixtures, including
   `examples/comptime.ae` and `examples/comptime-chain.ae`.
9. The M6 layout fixture, including `examples/layout-table.ae`.
10. The M7 nursery fixtures, including `examples/nursery-total.ae` and
    `examples/nursery-cancel.ae`.
11. The M21 foreign fixtures (`examples/foreign-pilot.ae`,
    `examples/foreign-sum.ae`) dual-compare and fail closed without a library grant.
12. The M19d spawn-arena fixture (`examples/spawn-arena.ae`) dual-compares with
    header capacity equal to the sum of arena declarations.
13. The M23 pure-call fixture (`examples/comptime-calls.ae`) is forged from raw
    call source by the seed (no materialization), dual-compares with bootstrap,
    and exits 512.
14. The M19e active-frame fixtures (`examples/active-cancel.ae`,
    `examples/task-frame-capacity.ae`, and `examples/task-loop.ae`) plus a
    forward-declared task fixture dual-compare as v12 and run with their
    documented exits/capacities.

## Authority

- Current toolchain delta: [AETHER_0.36.md](AETHER_0.36.md)
- Historical base language: [AETHER_0.11.md](AETHER_0.11.md)
- Host forge ABI: [FORGE_CONTRACT.md](FORGE_CONTRACT.md)
- Architecture: [ARCHITECTURE.md](ARCHITECTURE.md)
- Product gate: [../MANIFEST.md](../MANIFEST.md)

