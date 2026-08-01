# Language Landscape

Date: 2026-07-28; current-contract update: 2026-07-31

## Scope

No finite survey can literally enumerate every programming language ever created.
This compact ledger decomposes influential language families into reusable
questions. It is a design input, not a claim that Aether implements every
feature listed here. The source-backed ten-system study, component matrix, and
evidence plan are [research/](research/).

## Feature Ledger

| Family and representatives | Keep | Reject or defer |
| --- | --- | --- |
| C, Pascal, Fortran, Ada | predictable layout, direct data representation, small runtime | undefined behavior as a language contract, textual preprocessor |
| C++, D, Zig | RAII-style deterministic cleanup, explicit resource APIs, compile-time evaluation | implicit conversions, macro systems, hidden allocation |
| Rust | move-aware APIs, explicit failure values, traits as contracts | lifetime syntax and a globally pervasive borrow checker in the bootstrap |
| Go, Erlang, Elixir | structured concurrency, clear deployment model, failure containment | unstructured background work and implicit shared mutable state |
| Java, C#, Kotlin, Swift | approachable tooling, null-safety lessons, strong package boundaries | mandatory garbage collection and exception-driven normal flow |
| Haskell, OCaml, F#, Standard ML | algebraic data types, exhaustive matching, module interfaces, pure transforms | mandatory laziness and opaque monadic syntax for ordinary errors |
| Lisp, Scheme, Racket, Clojure | code as structured data, small semantic core, REPL workflow | text macros that hide source structure from tools |
| Python, Ruby, Lua, JavaScript, TypeScript | readable syntax, rapid iteration, batteries chosen by users | implicit coercion and runtime-only type failure for systems code |
| SQL, Prolog, Datalog | declarative queries and explicit relation thinking | ambient mutable global query state |
| Assembly, WebAssembly, LLVM IR | explicit machine model, portable bytecode, verifiable low-level IR | exposing backend quirks in everyday source code |
| MATLAB, R, Julia | array-first numerical thinking and multiple dispatch lessons | hidden allocation in hot paths |
| COBOL, Smalltalk, Objective-C | domain vocabulary and message-oriented readability | ambient object graphs as the default storage model |

## Current Aether positions

1. Aether 0.7 implements explicit `borrow`/`move`, operation-scoped `access`,
   one bounded arena/Whole-or-Truth-buffer surface, and one terminal,
   abortive `Error[Whole]` effect. The complete scope is
   [AETHER_0.7.md](AETHER_0.7.md).
2. General typed errors/effects and resumptions, compile-time execution,
   generic shape folding, SoA lowering, C interop, structured concurrency,
   first-class resource outcomes, Buffer weave results, and fine-grained arbitrary-node edits are future directions or research hypotheses — not
   current syntax or runtime claims. Their evidence and stop conditions are in
   [research/03-synthesis-and-evidence.md](research/03-synthesis-and-evidence.md).
3. Canonical formatting, a bootstrap canonical-AST path, and the bounded,
   versioned M3 structural authoring contract exist today. See
   [AETHER_AUTHORING_PROTOCOL_v2.md](AETHER_AUTHORING_PROTOCOL_v2.md).
4. The executable target remains AETH plus the Aether VM. No host-language or
   LLVM backend is part of the current language contract.
5. Immutable nominal records remain the first aggregate value: primitive fields
   only, explicit borrowed projection, structural equality, and AETH v5/v7
   record-table encoding. Recursive layout, mutation, and partial moves remain deferred.

See [NORTH_STAR.md](NORTH_STAR.md), [CORE_CLAIMS.md](CORE_CLAIMS.md), and
[ROADMAP.md](ROADMAP.md) for the governed design direction.
