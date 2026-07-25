# Language Landscape

Date: 2026-07-25

## Scope

No finite survey can literally enumerate every programming language ever created.
This ledger decomposes the influential language families and their representative
designs into reusable decisions. It is a design input, not a claim that Aether
implements every feature listed here.

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

## Sources Studied

- Rust ownership, trait, and Result references: https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html, https://doc.rust-lang.org/stable/reference/items/traits.html, and https://doc.rust-lang.org/core/result/.
- Zig language reference for explicit allocation and compile-time execution: https://ziglang.org/documentation/master/.
- Go specification for compact grammar, packages, and concurrency: https://go.dev/ref/spec.
- OCaml references for modules, interfaces, variants, and pattern matching: https://ocaml.org/docs/modules and https://ocaml.org/docs/basic-data-types.
- Haskell report for typed functional design and type classes: https://www.haskell.org/definition/haskell98-report.pdf.

## Aether Decisions

1. Values are moved by default. Borrowing and copying are explicit source forms.
2. Allocation is visible. Heap-owning APIs receive an `arena` capability.
3. Errors are typed values and must be matched, forwarded, or converted.
4. Compile-time work uses normal Aether syntax under `forge` blocks, not macros.
5. The canonical formatter and AST serialization are compiler-owned and deterministic.
6. The first executable target is Aether bytecode and the Aether VM. No host-language
   backend is part of the language contract.
7. C ABI interop, SoA lowering, generic shape folding, and structured concurrency are
   specified future milestones after the bootstrap's move and allocator invariants hold.
