# M1 Design: Owned Values, Ephemeral Loans, and Bounded Arenas

**Status:** Accepted design direction; Aether 0.6 implements the bounded M2
subset recorded in [ADR-004](ADR-004-aeth-v6-bounded-resources.md)
**Date:** 2026-07-28; implementation record updated 2026-07-31
**Decision scope:** This remains the design foundation for future Aether
resource work. Aether 0.6 adds a deliberately narrower executable M2 subset;
its exact source and artifact contract is [AETHER_0.6.md](AETHER_0.6.md).
**Related:** [ADR-003](ADR-003-value-resource-semantics.md),
[M1 validation matrix](M1-VALIDATION-MATRIX.md),
[resource-model research](research/04-value-resource-models.md),
[roadmap](ROADMAP.md), and [current 0.5 specification](AETHER_0.5.md).

## Accepted direction

Human approval on 2026-07-31 accepted this resource direction before M2
implementation began:

> **Aether should use owned values, ephemeral non-escaping loans, and explicit
> bounded arena capabilities.** Copyable values duplicate by ordinary use;
> owner values move only through `move`; read loans use `borrow`; mutable
> resource capability loans use `access`; dynamic storage comes only from a
> named fixed-capacity arena; allocation reports a closed outcome instead of
> trapping or selecting an ambient allocator; and destruction runs no user
> code.

The model is deliberately narrower than a general reference/lifetime system.
It gives an AI author a small, visible state machine that the source checker,
typed semantic IR, AETH verifier, VM, and seed can each prove independently.

## Aether 0.6 implementation boundary

The accepted direction is broader than the first executable increment. Aether
0.6 delivers one arena, `Whole`/`Truth` buffers, `access`, closed terminal
`choose` outcomes, a typed resource semantic plan, AETH v6 validation, VM
accounting, and seed byte-identity proof. It does **not** claim every proposed
surface below is already executable.

| Design item | Aether 0.6 status |
| --- | --- |
| Named bounded arena, no ambient allocator, Copy-only buffer elements | Implemented. |
| Source-visible `borrow`, `move`, and ephemeral `access` for resource operations | Implemented within the closed M2 grammar. |
| Typed semantic resource facts before bytecode emission | Implemented as `SemanticResourcePlan`, recording region, element, owner/borrow place, and destination. |
| Allocation, append, and lookup total outcomes | Implemented as immediate bright/dim `choose` conditions; outcomes are not values. |
| Failure atomicity for allocation/append/lookup | Implemented and behavior-tested. |
| Buffer owner result / generic outcome propagation | Deliberately deferred; 0.6 rejects Buffer weave results. |
| General `revise` of a resource owner, generic effects, arbitrary references, individual reclamation | Deliberately deferred. |

The definitive 0.6 contract takes precedence over illustrative proposed
spellings in this design document. Future work may extend the design only with
a versioned specification, verifier/seed proof, and ADR update.

## 1. Existing 0.5 boundary

Aether 0.5 already distinguishes `Whole`/`Truth` from unique `Text`, `Bytes`,
and nominal records. The source validator requires explicit `borrow` or `move`
for the latter and tracks a binding as moved or live. `bind mutable` permits a
same-type `revise` while the binding is live. Records remain immutable,
non-recursive, primitive-field aggregates.

That is a useful ownership-shaped surface, but it is not yet the M1 model:

- `borrow` currently compiles as an ordinary AETH load for existing values. In
  the Rust VM that load clones a `RuntimeValue`; Aether 0.5 does **not** promise
  a general zero-copy runtime-reference implementation.
- Text and byte transformations can allocate bounded Rust runtime storage; they
  have no source-visible allocator or allocation-failure outcome.
- The checker and verifier track whole local move state only. They do not carry
  region provenance, loan scope, exclusive access, conditional transfer, or
  dynamic-aggregate state.
- AETH v4/v5 contain no arena or dynamic-aggregate representation. Existing
  artifacts stay valid exactly as specified in [AETHER_0.5.md](AETHER_0.5.md).

The proposal therefore adds a versioned future semantic layer. It must not be
used to claim that Aether 0.5 already has explicit allocation, true runtime
borrows, generalized mutable references, or deterministic dynamic-resource
outcomes.

## 2. Goals and exclusions

### Goals

1. Make every new dynamic allocation name its resource authority.
2. Make ownership transfer, borrowing, mutation, destruction, and failure
   statically inspectable at each public function boundary.
3. Keep loans lexical and non-escaping so users and tools do not need hidden
   global lifetime inference.
4. Give the verifier enough information to reject malformed or confused
   resource artifacts before VM execution.
5. Preserve the AETH-only, verify-before-run/write, no-host-capability, and
   seed-hosted-product-compile invariants.
6. Start with a collection small enough for byte-identity proof rather than a
   general heap, recursive type system, or concurrency model.

### Explicit exclusions for M1/M2

- No general references, pointers, user-visible addresses, or lifetime syntax.
- No hidden default/global allocator and no fallback allocator.
- No individual `free`, allocator cloning, arbitrary allocator replacement, or
  user-defined destruction callbacks.
- No recursive/cyclic aggregates, dynamic aggregates inside records, owned
  aggregate elements, closures, async suspension, tasks, or cross-thread loans.
- No change to existing Text/Bytes allocation behavior in the Aether 0.5
  compatibility surface.
- No host exposure of `Arena`, `Buffer`, loan, or allocation-outcome values
  through the primitive-only invoke or forge ABI.

## 3. Core vocabulary and typed semantic IR

The following is the proposed semantic vocabulary. The source spellings below
are proposed M2 spellings; they are intentionally not accepted by the current
0.5 parser until the approved implementation lands.

### 3.1 Value categories

| Category | Initial members | Ordinary name use | Ownership consequence |
| --- | --- | --- | --- |
| `Copy` | `Whole`, `Truth` | Reads a duplicate. | No source move state. |
| `Owner` | `Text`, `Bytes`, existing records, future `Buffer` | Requires `borrow` or `move`. | Exactly one owner binding is responsible for logical destruction. |
| `Capability` | Future `Arena` | May be used only through `access`; not copied, moved, returned, or stored in M2. | One root owner controls a bounded region. |
| `Outcome` | Future `Allocation[T]`, `Append[T]`, and `Lookup[T]` | Must be handled or propagated through a declared outcome path. | Never silently becomes a value, trap, or host exception. |

`Text`, `Bytes`, and records remain source-level `Owner` values. Their current
0.5 runtime copying behavior remains a versioned compatibility fact, not a
license to use them as silently copyable values in new semantics.

### 3.2 Proposed semantic types

```text
CopyType      κ ::= Whole | Truth
Region        ρ ::= lexical arena identity
Type          τ ::= Whole | Truth | Text | Bytes | Record(r)
                   | Arena(ρ) | Buffer(κ, ρ) | Allocation(τ) | Append(τ) | Lookup(κ)
ParameterMode μ ::= own | borrow | access
```

`Buffer(κ, ρ)` is a homogeneous, fixed-capacity dynamic aggregate allocated
from `Arena(ρ)`. M2 permits only `Whole` or `Truth` elements. `ρ` is part of
the semantic type, even if the eventual user spelling uses the arena's source
name rather than a separately written type parameter.

Every public weave that consumes, borrows, or returns `Buffer(κ, ρ)` names the
matching arena capability in its signature. The compiler does not infer an
unnamed region relationship across a weave, artifact, or host boundary.

Illustrative proposed spelling only:

```aether
weave append [
  access memory: Arena,
  move values: Buffer[Whole @ memory],
  value: Whole
] -> Append[Buffer[Whole @ memory]]:
  ...
```

The grammatical details of brackets, result syntax, and pattern syntax remain
an M2 implementation concern. The semantic names `access`, `allocated`, and
`exhausted`, the region-carrying type rule, and the rules below are the M1
decision surface.

### 3.3 Typed semantic IR operations

The bootstrap and seed must lower accepted M2 source into an explicit typed
semantic IR before bytecode emission. It must represent resource intent; a raw
AST with a Boolean moved bit is insufficient.

| IR form | Required type/state information | Meaning |
| --- | --- | --- |
| `copy p` | `p: Copy` | Read a duplicate; `p` remains live. |
| `borrow p` | `p: Owner`, live | Create an ephemeral read loan for the enclosing operation. |
| `move p` | `p: Owner`, live | Reserve whole ownership transfer; source becomes moved at successful commit. |
| `access a` | `a: Arena(ρ)`, live | Create an exclusive ephemeral capability loan for the enclosing operation. |
| `bind p = e` | destination absent, `e: τ` | Install a new owner/copy value. |
| `revise p = e` | `p` mutable and live, `e: τ` | Evaluate first, then atomically replace and logically destroy prior value. |
| `allocate a, n` | `access a: Arena(ρ)` | Reserve bounded storage and yield `Allocation[Buffer(κ, ρ)]`. |
| `destroy p` | `p: Owner`, live | Compiler-inserted logical destruction; never invokes user code. |
| `match allocation` | `Allocation[τ]` | Forces `allocated(value)` or `exhausted` handling/declared propagation. |

The IR also records a local resource annotation, `alloc ρ`, on every operation
that may consume capacity. It is a concrete verifier-facing resource fact, not
a general inferred effect system. M4 may later generalize effects only if it
preserves the explicit M1 allocation boundary.

## 4. Binding state machine

For each root binding, the semantic checker tracks one of the following states:

| State | Meaning | Legal next events |
| --- | --- | --- |
| `Live(τ)` | The binding owns or contains a usable value. | `copy` if Copy, `borrow` if Owner, `move` if Owner, `access` if Arena, `revise` if mutable, or destruction. |
| `ReadLoan(τ)` | A temporary `borrow` is active during one enclosing operation. | Read-only use within that operation; loan ends before the next statement/control-flow edge. |
| `AccessLoan(Arena(ρ))` | A temporary exclusive arena access is active during one enclosing operation. | Allocation/arena operations only; loan ends before the next statement/control-flow edge. |
| `ReservedForRevise(τ)` | The RHS of a `revise` is being evaluated. | Borrow target only; moving or revising target is rejected. |
| `Moved(τ)` | Ownership has transferred. | No read, borrow, access, revise, or second move. |
| `Destroyed(τ)` | Compiler-internal terminal state at scope exit. | No source use. |

Loans are semantic, not general runtime values. They cannot be bound, returned,
yielded, stored in a record/buffer, captured, placed on a persistent operand
stack, or cross a jump target. This makes loan duration syntactically bounded
and verifier-checkable.

### 4.1 State transitions and failure behavior

| Operation | Preconditions | Success state | Allocation-failure state | Rejected case |
| --- | --- | --- | --- | --- |
| ordinary `name` | `Live(Copy)` | remains live | n/a | ordinary use of an Owner/Capability |
| `borrow name` | `Live(Owner)` | temporary read loan, then live | n/a | borrow moved value or let loan escape |
| `move name` | `Live(Owner)` | source becomes moved after receiver commits | source stays live if a fallible receiver reports failure | move a loan/capability/already-moved source |
| `access arena` | `Live(Arena(ρ))` | temporary exclusive access loan, then live | arena remains live, capacity unchanged on failed allocation | access non-arena, overlap access, or let access escape |
| `bind` | destination absent | destination live | source operands follow their operation contract | conditional/nested binding outside approved scope rule |
| `revise name = expr` | mutable `Live(τ)` | RHS completes, old value destroyed, destination live with replacement | old value remains live and unchanged | move/revise target inside RHS or type mismatch |
| dynamic allocation | live access loan, request within encoded limit | `allocated(value)` and arena capacity decreases | `exhausted`; no owner operand consumed and no capacity changed | unnamed/default arena or ignored outcome |
| scope end | live owner/capability | compiler emits logical destruction | n/a | a live borrowed/accessed value at boundary |

The proposed commit rule is important: a fallible operation must prepare its
result before it consumes `move` operands or arena capacity. If it returns
`exhausted`, every caller owner remains usable and the arena has the same
remaining capacity. This **failure atomicity** avoids a lost owner or partially
mutated resource state after an expected resource failure.

### 4.2 Control-flow joins

At every control-flow join, an owner is `Live` only if it is live on every
incoming path. If any incoming path moved it, the joined state is `Moved` and a
later use is rejected. This is the conservative behavior Aether 0.5 already
uses for whole bindings, made explicit for the future semantic IR.

Read/access loans may not reach a join at all. They must close before a branch,
loop back-edge, jump target, or `yield`.

## 5. Mutation and deterministic logical destruction

### 5.1 `bind mutable` and `revise`

`bind mutable` describes a replaceable binding, not interior mutability of the
value held by that binding. Existing records stay immutable. The first dynamic
aggregate slice uses consuming/replacing operations rather than element
references or arbitrary in-place user mutation.

For `revise name = expression`:

1. mark `name` as `ReservedForRevise`;
2. evaluate the RHS left-to-right;
3. permit only `borrow name` from the target during that evaluation;
4. if the RHS succeeds, logically destroy the old value and install the new
   same-type value atomically; and
5. if it reports allocation failure, leave the old binding live and unchanged.

`revise name = move name` is therefore invalid. The current verifier already
rejects a revised moved local; M2 must move the same rejection into source
semantic validation and preserve it in the verifier.

### 5.2 Destruction rule

Logical destruction occurs exactly when:

1. a live owner is replaced by a successful `revise`;
2. a live owner reaches the end of its lexical owner scope after any yielded
   result has transferred; or
3. an aggregate owner is logically destroyed, recursively destroying its
   logically-owned contents.

`move` itself does not destroy the value; it transfers the destruction duty to
the receiving owner. M2 does not expose an explicit source `destroy` operation.
The compiler inserts it at defined state transitions, and the verifier checks
that no logical destruction can consume a moved or borrowed value.

There are no user destructors, finalizers, callbacks, implicit host calls, or
I/O during destruction. This preserves determinism and prevents a resource
rule from smuggling in an untyped effect or host capability.

## 6. Arena and allocation contract

### 6.1 Arena identity and capacity

An `Arena(ρ)` is an opaque, non-copyable, non-transferable root capability. It
has a named lexical identity `ρ` and a fixed logical capacity declared in the
artifact's resource plan. The proposed M2 initial policy is deliberately
small: one arena per invocation, with a maximum of **1,000,000 logical bytes**
including buffer metadata and element storage. This matches the existing
Aether 0.5 upper bound for Text, Bytes, and aggregate runtime payloads.

Arena creation is an explicit resource declaration, not an invisible ordinary
allocation expression. The future artifact must declare the requested logical
capacity; the verifier validates the encoded limit; the VM attempts to reserve
the bounded backing store before guest execution; and failure to admit the
resource plan prevents guest execution rather than partially starting a weave.

The exact surface spelling for this declaration is an M2 parser decision. Its
semantic facts are fixed by this proposal: named arena, fixed capacity,
versioned artifact representation, no ambient default, and no host authority.

### 6.2 Allocation outcome

Every operation that needs new arena storage yields a closed allocation outcome:

```text
Allocation[T] ::= allocated(T) | exhausted
```

The caller must immediately handle it or propagate it through a function whose
signature explicitly declares that same allocation outcome. It may not be
discarded, coerced to a nullable value, converted to a VM panic, or routed to a
host exception. M2 can introduce the narrow source matching syntax necessary
to make that rule usable; M4 may later unify it with a general typed error/effect
model without changing its observable contract.

Fixed-capacity append has its own closed, owner-preserving outcome:

```text
Append[T] ::= appended(T) | full(T)
```

`append` receives a moved buffer and returns that buffer in both alternatives.
It needs no arena capacity after construction, so `full(buffer)` is distinct
from allocation exhaustion and cannot lose the caller's only buffer owner.

Read-only indexed observation uses a third closed outcome:

```text
Lookup[κ] ::= found(κ) | absent
```

An out-of-range observation is therefore data-level absence, not a VM fault.
`Lookup` carries a Copy element only and never carries an owner or loan.

### 6.3 Storage reclamation

M2 uses arena reclamation, not individual free:

- logical destruction makes an owner unavailable immediately;
- backing storage remains reserved to its arena until the arena's lexical owner
  ends; and
- M2 exposes no `reset` or individual deallocation operation.

This is intentionally conservative. It makes capacity use deterministic and
avoids double-free/use-after-free states, at the cost of retention until the
arena ends. A future manual reset, reusable arena, or individual deallocation
proposal requires a separate ADR, escape proof, verifier state, seed proof, and
failure-path test suite.

### 6.4 Resource and security consequences

- No dynamic operation may choose or obtain a host allocator, file, process,
  socket, shell, model, or arbitrary pointer.
- Capacity must be checked with overflow-safe arithmetic before reservation.
- The VM must use fallible reservation and report resource admission failure;
  it must not panic on an allocation request.
- A malformed artifact may not name a missing arena, exceed its declared
  capacity, use an arena outside its declared function/region relation, or
  encode an invalid outcome transition.
- The fixed M2 budget is a safety boundary, not a performance promise. Any
  future larger/default budget needs measured hardware and denial-of-service
  analysis.

## 7. Dynamic aggregate boundary

M2's one representative aggregate is a homogeneous `Buffer[κ @ ρ]`, where
`κ` is initially `Whole` or `Truth` only. It demonstrates actual dynamic
storage and ownership without importing recursive objects, nested owners,
partial moves, or generalized generics.

| Rule | M2 consequence |
| --- | --- |
| Capacity is fixed at construction. | The requested element count and element size are validated before one bounded arena reservation. |
| Elements are Copy-only. | Moving an element, sharing a mutable element, and partial-move tracking are out of scope. |
| Buffer has region provenance. | Its semantic type contains its allocating arena identity. |
| Buffer is an Owner. | It transfers only via `move`; ordinary name use requires a defined non-owning operation. |
| No dynamic field embedding. | A 0.5 record cannot contain a Buffer, and M2 adds no new dynamic record form. |
| No manual free/reset. | Backing storage remains in its arena until arena destruction. |
| No host crossing. | Primitive-only invoke/forge ABI remains unchanged. |

The proposed M2 collection contract is fixed at this semantic level:

| Operation | Inputs | Result | Resource rule |
| --- | --- | --- | --- |
| `allocate` | `access Arena(ρ)`, Copy element kind, bounded capacity | `Allocation[Buffer(κ, ρ)]` | Reserves once; `exhausted` changes no owner/capacity state. |
| `append` | `move Buffer(κ, ρ)`, `κ` | `Append[Buffer(κ, ρ)]` | Does not allocate; returns `appended(next)` or `full(original)`. |
| `count` | `borrow Buffer(κ, ρ)` | `Whole` | Read-only, no allocation. |
| `at` | `borrow Buffer(κ, ρ)`, `Whole` index | `Lookup[κ]` | Read-only, no allocation; `absent` for out-of-range index. |

The first collection API must be deliberately small and total over its declared
state. It contains a fallible fixed-capacity construction operation,
owner-consuming `append`, a read-only count operation, and one read-only
element-observation operation for the two Copy element types. The observation
operation returns `Lookup[κ]`. `append` returns
`Append[Buffer]`: `appended(next)` after a valid insertion or `full(original)`
without allocation. It must not lose the buffer, allocate from another region,
or raise a host exception.

## 8. Escape and public-boundary rules

1. A `borrow` or `access` loan cannot be returned, yielded, stored, captured,
   or carried across a control-flow boundary.
2. A `Buffer[κ @ ρ]` can appear in a weave signature only when that signature
   also names a matching `access` arena relation for `ρ`.
3. A buffer cannot outlive the lexical root arena from which it was allocated.
   Returning a buffer from a helper is allowed only when the caller supplied
   the named arena relation; returning one tied to a local arena is rejected.
4. M2 keeps `Arena` root-owned and non-transferable. It cannot be a record
   field, buffer element, weave result, host invoke value, or forge value.
5. No current or future AETH artifact gains authority merely by holding an
   arena. Arena memory is VM-private bounded guest storage.

These rules replace hidden outlives inference with an audit-friendly fact: a
dynamic value visibly names the arena capability that must still be valid.

## 9. Compiler, artifact, VM, seed, and tool implications

| Layer | Required M2 work after approval | M1 guardrail |
| --- | --- | --- |
| Parser / AST | Add only the approved arena, buffer, `access`, and outcome forms. | Do not reinterpret existing 0.5 `borrow` or Text/Bytes syntax. |
| Source semantic checker | Replace Boolean-only move reasoning with typed owner/loan/region state where new forms occur. | Reject escape, overlap, source-level revise-self-move, omitted arena, and unhandled outcome. |
| Typed semantic IR | Carry category, parameter mode, region identity, state transition, `alloc ρ`, and outcome branch. | No raw AST-to-bytecode shortcut for new forms. |
| AETH format | Introduce a new artifact version and new typed resource metadata/instructions. | Never reinterpret v4/v5 bytes or weaken existing verifier behavior. |
| Verifier | Validate resource plan, region/type relations, owner state, loans, operand stack outcomes, joins, and capacity arithmetic. | Reject before execution; source compiler is not trusted authority. |
| VM | Pre-admit fixed backing storage, execute deterministic capacity checks, preserve failure atomicity, and logically destroy safely. | No ambient allocator fallback, panic path, user destructor, or host capability. |
| Forge / invoke ABI | Keep primitive-only boundary. | No Arena/Buffer/outcome crossing or new host I/O authority. |
| Seed compiler | Parse/lower/emit canonical M2 source byte-identically with bootstrap. | Do not claim invalid-source diagnostic parity; bootstrap remains diagnostic authority. |
| Studio / structural tooling | Surface capability and outcome states as explicit semantics once M3 contract exists. | Keep source/model data local; no model becomes compiler authority. |

The new artifact version number and byte encoding are not chosen in M1. They
must be assigned in the M2 format ADR after a complete compatibility inventory.
The non-negotiable rule is that v4/v5 remain unmodified and a verifier must
understand every new resource instruction before it can run it.

## 10. Required counterexamples and negative cases

The following are proposed M2 semantic examples. They illustrate rejected
program shapes; they are not valid Aether 0.5 source samples.

### N1 — owner copied without an explicit operation

```aether
bind first <- make_label ...
bind second <- first
```

Reject: `first` is an Owner. The author must use a specified copy-producing
operation or transfer it using `move first`.

### N2 — borrowed value escapes

```aether
weave leak [borrow item: Buffer[Whole @ memory]] -> Buffer[Whole @ memory]:
  yield borrow item
```

Reject: a read loan cannot become an owner result.

### N3 — allocator capability escapes

```aether
weave expose [access memory: Arena] -> Arena:
  yield access memory
```

Reject: `access` is an ephemeral operation capability, not a first-class value.

### N4 — local region escapes

M2 permits one invocation-root arena, so local arena declaration is rejected
outright in that first slice. This example records the general escape rule that
must also hold if a later, separately approved extension introduces local
arena scopes:

```aether
weave make [] -> Buffer[Whole @ local]:
  bind local <- arena 64
  bind result <- allocate access local capacity 8
  yield move result
```

Reject: result storage would outlive its local arena.

### N5 — revise moves its own target

```aether
bind mutable values <- ...
revise values <- move values
```

Reject: the target is reserved for replacement and may be borrowed, but not
moved, while its replacement expression is evaluated.

### N6 — allocation outcome disappears

```aether
allocate access memory capacity 4096
yield 0
```

Reject: `Allocation[Buffer]` must be matched or explicitly propagated.

### N7 — unsupported owned element

```aether
allocate access memory Buffer[Text] capacity 4
```

Reject in M2: `Buffer` elements are Copy-only (`Whole` or `Truth`).

### N8 — malformed artifact resource path

An artifact encodes an allocation instruction referencing an undeclared arena,
or joins a path that has different buffer/arena ownership state.

Reject before VM execution: the verifier must reject the artifact regardless of
whether source tooling ever emitted it.

## 11. Alternatives considered

| Alternative | Benefit | Rejection reason |
| --- | --- | --- |
| Full borrow checker with reference/lifetime types | Maximum expressiveness. | Too much implicit/inferred state before Aether has stable dynamic/resource primitives; poor M2 proof size. |
| Unique owners plus a global heap | Easy collection API. | Hides allocation authority, capacity, and policy; violates the accepted explicit-allocation direction. |
| Manual explicit free immediately | Potential early reuse. | Introduces alias, double-free, and failure-path complexity before a bounded safety core exists. |
| Reference counting / garbage collection as the base model | Familiar sharing ergonomics. | Makes destruction/cost less visible and does not meet the explicit resource goal. |
| Pass arena by ownership on every allocation | Strong linearity. | Makes ordinary composition unnecessarily disruptive and still needs a temporary capability protocol. |
| `borrow` arena with hidden mutable accounting | Surface simplicity. | Conceals resource mutation; `access` makes it visible while staying non-escaping. |
| Proposed owned values + ephemeral loans + bounded arenas | Compact explicit state, bounded resource behavior, verifier feasibility. | Intentionally restricts sharing, recursive data, reuse, and generic expressiveness until separately proven. |

## 12. Approval and implementation gate

Human approval of this model authorizes an M2 implementation proposal, not an
unbounded language rewrite. Before any new form enters default product compile,
the implementation must deliver all of the following as one complete increment:

1. an accepted M2 syntax/format ADR and synchronized language/architecture/
   manifest documentation;
2. source checker, typed semantic IR, new-version encoder/decoder, verifier,
   VM, forge boundary review, seed compiler, and Studio contract updates where
   applicable;
3. the complete test matrix in
   [M1-VALIDATION-MATRIX.md](M1-VALIDATION-MATRIX.md), including source and
   hostile-artifact negative cases;
4. canonical M2 corpus byte identity between bootstrap and seed, plus updated
   seed self-host/rebuild/forge proof;
5. warning-free format, test, lint, Clippy, documentation-link, and pack
   integrity gates; and
6. an evidence report that states the actual capacity policy, any measured
   resource behavior, and remaining intentional limits without overclaiming.

If the human rejects any core rule — especially `access`, fixed capacity,
allocation outcomes, no individual free, or non-escaping loans — this document
and ADR-003 must be revised before M2 begins. No implementation may silently
choose a different semantic model.
