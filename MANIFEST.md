# Aether Production Manifest

## Contract

Aether **0.11.0** accepts Aether source, returns a canonical AST from the Rust
bootstrap for tooling, and **emits AETH v11 bytecode primarily through the
Aether-written seed compiler** (forge ABI). The `aether` CLI is the product
interface for that seed-hosted compile path. Verified AETH
v4/v5/v6/v7/v8/v9/v10 artifacts remain accepted compatibility inputs with their
original meanings. Source is never translated to an existing language.

## Scope Boundary

This manifest is the executable Aether 0.11 product contract. It intentionally
does not promote long-range research directions to implemented behavior. The
AI-first systems-language direction, evidence policy, and staged dependencies
are [docs/NORTH_STAR.md](docs/NORTH_STAR.md),
[docs/CORE_CLAIMS.md](docs/CORE_CLAIMS.md), and [docs/ROADMAP.md](docs/ROADMAP.md).
The accepted M1 direction is
[docs/DESIGN-M1-VALUE-RESOURCE-SEMANTICS.md](docs/DESIGN-M1-VALUE-RESOURCE-SEMANTICS.md).
The bounded executable M2 decision is
[docs/ADR-004-aeth-v6-bounded-resources.md](docs/ADR-004-aeth-v6-bounded-resources.md).
General generics, C-header interop, libloading, ambient guest I/O, and a native
backend are not Aether 0.11 surface area. M4 implements the bounded, testable
`Error[Whole]` capability in
[docs/DESIGN-M4-TYPED-ERROR-EFFECTS.md](docs/DESIGN-M4-TYPED-ERROR-EFFECTS.md)
and [docs/AETHER_0.7.md](docs/AETHER_0.7.md). M5 implements bounded explicit
literal compile-time `Whole` evaluation in
[docs/DESIGN-M5-DETERMINISTIC-COMPTIME.md](docs/DESIGN-M5-DETERMINISTIC-COMPTIME.md)
and [docs/AETHER_0.8.md](docs/AETHER_0.8.md). M6 implements explicit layout
shapes and dual-layout tables in
[docs/DESIGN-M6-EXPLICIT-LAYOUT-SHAPES.md](docs/DESIGN-M6-EXPLICIT-LAYOUT-SHAPES.md)
and [docs/AETHER_0.9.md](docs/AETHER_0.9.md). M7 implements structured nurseries
in [docs/AETHER_0.10.md](docs/AETHER_0.10.md). M8 implements the capability-closed
pure host ABI pilot in [docs/AETHER_0.11.md](docs/AETHER_0.11.md). Structural
authoring is local tooling metadata, not a host-capability expansion; its
current contract is
[docs/AETHER_AUTHORING_PROTOCOL_v5.md](docs/AETHER_AUTHORING_PROTOCOL_v5.md)
with v6 node surface for host weaves.

## Implemented Language Boundary

Aether 0.11.0 includes the 0.4/0.5 scalar and byte surface plus immutable nominal
records declared after `world` and before weaves. Record fields are bounded to
the primitive `Text`, `Whole`, `Truth`, and `Bytes` types; records cannot nest.
`make` constructs in declaration order and `field borrow` projects a cloned
immutable field. Records are unique values with explicit `borrow`/`move`, may
cross internal weave calls, and remain outside the primitive-only host invoke
ABI. Root bindings receive fixed slots; nested blocks may revise but cannot
introduce bindings.

The bounded M2 surface adds exactly one positive `arena` root binding in
`main`, `buffer Whole` / `buffer Truth` owner placeholders, and operation-scoped
`access`. `allocate`, resource `append`, and `at` appear only as terminal
closed `choose` conditions with explicit bright/dim branches. Allocation and
append restore the same moved mutable Buffer owner; lookup updates its mutable
copy destination only when bright. Resource owners cannot be revised or weave
results, and no resource value crosses the primitive host or forge ABI.

Bounded facilities include `encode`, `decode`, `extent`, `octet`, `slice`,
`fuse`, `append`, `seek`, `number`, `pack16`, `pack32`, `pack64`, `unpack16`,
`unpack32`, `poke`, and `poke32`.

Source is UTF-8. Names and keywords are lowercase ASCII. Canonical formatting
uses LF, exact two-space indentation, no tabs, and no trailing whitespace. CRLF
input is accepted by bootstrap formatters and by the seed line scanner; a valid
final source line need not end in a terminal LF.

M4 adds one explicit abortive `Error[Whole]` effect. A non-`main` weave may
write `raises Whole` only when it returns `Whole` and accepts ordinary owned
`Whole`/`Truth` copy parameters. `raise`, `forward call`, and one-line terminal
`handle call ... into success otherwise error into code` are the only M4 control
forms. The handler writes and yields its selected distinct mutable root `Whole`
destination. An ordinary call cannot invoke an erroring weave; M4 control cannot
cross live owners, loans, arenas, buffers, or M2 outcomes.

M5 adds root-only immutable `comptime bind` for exactly one `sum`,
`difference`, `product`, `quotient`, or `remainder` operation over literal
`Whole` operands. Every
directive has checked signed-64-bit arithmetic and one fixed evaluation unit;
one source program may contain at most 1,024 directives. M5 accepts no names,
calls, loops, text, bytes, records, resources, effects, macros, generated
source, host I/O, or user-adjustable budget. Its result remains an ordinary
immutable `Whole` local.

M6 adds root-level `shape` declarations of 1–8 `Whole` fields and
`table Shape layout rows|columns` arena-backed owners with closed
`allocate` / `store` / `load` outcomes. Capacity is `1..=1024`. Physical layout
is author-selected only; rows and columns share one logical model. Tables cannot
be weave results or host/forge values.

M7 adds lexical `together:` nurseries with `spawn call ... into` lines
(1 through 8). Execution is cooperative and source-ordered. First child
`Error[Whole]` cancels remaining unstarted spawns and re-raises. Nursery
weaves keep the M4 clean resource boundary; `main` remains total.

M8 adds body-less `host weave` declarations for pure host services. Host
parameters are only owned/copy `Whole`/`Truth` or `borrow Text`/`borrow Bytes`;
results are only `Whole`/`Truth`/`Text`/`Bytes`. Ordinary `call` of a host weave
emits `HOST_CALL` (65). The product pure fixture installs only `whole_inc` and
`text_extent`; missing services fail closed. No guest file/process/network/shell
authority is granted.

Every 0.11 compilation emits AETH v11 with an arena-capacity header field, a
possibly empty bounded record table, a shape table, a function `effect_tag`, and
a function host-kind byte. v11 preserves prior verified forms and adds host
function entries plus `HOST_CALL` (65). AETH v4 through v10 remain accepted with
their original bytes and meanings; earlier and unknown versions are rejected.

## Structural authoring boundary

`aether.ast/v5` describes successfully parsed Aether 0.10 source after
formatter canonicalization. `aether.edit/v5` accepts only a matching complete
canonical `baseSource` and bounded typed top-level Record/Shape/Weave
`replace`/`insertAfter`/`delete` operations. The bootstrap revalidates the
formatter-owned result; the CLI seed-compiles it before writing source to the
requested output path. It cannot execute code, write a file by itself, contact an
AI/model service, change artifact bytes, or grant a guest capability.

Stable `aether.diagnostic/v5` code/span envelopes make source and edit failures
machine-readable, including `AE-EFFECT-001` through `AE-EFFECT-004`,
`AE-COMPTIME-001` through `AE-COMPTIME-003`, `AE-LAYOUT-001` through
`AE-LAYOUT-003`, and `AE-TASK-001` through `AE-TASK-003`. Every `Bind` node must
state `stage: "runtime"` or `stage: "comptime"`. The exact schemas, limits,
operation vocabulary, and compatibility policy are in
[docs/AETHER_AUTHORING_PROTOCOL_v5.md](docs/AETHER_AUTHORING_PROTOCOL_v5.md).

## Compile Path Boundary

| Path | Role |
|------|------|
| **Seed (default)** | `compile_with_seed` / CLI `compile` / CLI `apply-edit` validation — Aether-written compiler |
| **Bootstrap** | `compile_to_bytecode` / CLI `compile --bootstrap` / `check` AST — rebuild seed, diagnostics |
| **Forge** | Host ABI only: `compile [borrow source: Text] -> Bytes` |

The seed artifact is checked in at `seed/aether_seed.aeth` and embedded as
`SEED_COMPILER_ARTIFACT` for offline deterministic product builds.

## Seed-Profile Self Hosting

`seed/aether_seed.ae` parses the complete documented canonical Aether 0.10
surface: all statement and shallow expression forms, named locals/params,
`borrow`/`move`/`access`, multi-weave `call` including forward callees, hex bytes
literals, UTF-8 text constants with the five defined escapes, and LF/CRLF input
with or without a final line terminator. It also emits the bounded immutable
record declaration, constructor, and projection surface plus `arena`,
Whole/Truth buffers, closed resource outcomes, the v9 resource and shape
headers, the bounded M4 effect metadata/instructions, M5 literal `comptime bind`
/`COMPTIME_WHOLE`, and M6 shape/table dual-layout emission through ordinary
`Bytes` operations with no host parser callback.

Proofs in `crates/xlang-core/tests/seed_self_host.rs`:

1. Multi-generation self-host identity of the seed
2. Distinct source variant yields a different artifact
3. Multi-weave, forward-call, and CRLF fixtures match bootstrap
4. **All shipped `examples/*.ae` seed-compile byte-identically to bootstrap**
5. A complete prior canonical-surface corpus covering every statement,
   expression, ownership mode, literal mode, and accepted line termination
   matches bootstrap
6. The six documented M2 arena/buffer examples match bootstrap byte-for-byte,
   verify, and run with their expected outcomes
7. The handled-error and normal-erroring M4 fixtures match bootstrap byte-for-
   byte, verify, and run with their expected outcomes
8. The shipped M5 comptime fixture matches bootstrap byte-for-byte, verifies,
   and runs with its expected outcome
9. The shipped M6 layout-table fixture matches bootstrap byte-for-byte, verifies,
   and runs with its expected outcome
10. The shipped M7 nursery fixtures match bootstrap byte-for-byte, verify, and
   run with their expected outcomes

This is full canonical Aether 0.10 source-emission parity for the documented
surface, self-hosting of the seed, and seed-hosted compilation of the shipped
example corpus. It is **not** a claim of full invalid-source diagnostic parity or of parity for future language
features without the same proof.

## Product Interface Boundary

The CLI is the only active product interface. Aether has no active desktop,
WebView, model, or network integration. Its AI-first design is expressed through
the versioned structural contracts and deterministic diagnostics, not by giving
an AI service compilation or persistence authority. The retired workbench is
recorded in [docs/ADR-006-retire-aether-studio.md](docs/ADR-006-retire-aether-studio.md).

## Quality Gate

Rust format, Clippy `-D warnings`, core/CLI/seed tests, CLI seed-compile of
examples, and forge self-host hash check are required. A release also requires
`cargo build --release -p aether-cli`, forge contract verification, inspection
of the release binary, and a successful launch on the target Windows system.
