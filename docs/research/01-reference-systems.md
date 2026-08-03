# Aether Reference Systems Study

**Status:** SOP phases 1 and 3 evidence log
**Date:** 2026-07-28; current-contract update: 2026-08-01
**Decision boundary:** This study informs Aether's future design. It does not
change the historical Aether 0.5 contract or authorize a new backend. The
current executable contract is Aether 0.8; its bounded resource, M4 effect,
and M5 literal-comptime implementations do not alter this research record's
backend boundary.

## Purpose and method

Aether is intended to become an AI-first systems language rather than a
collection of copied language features. This study records ten reference
systems chosen to test the original Aether brief's hard problems: ownership,
explicit allocation, compile-time work, typed effects, data layout, artifact
verification, interoperation, and human/AI tooling.

Each source below is a primary project specification or official project
documentation, accessed on 2026-07-28. “Adopt,” “adapt,” and “reject” below
mean *subjects for Aether design and proof*, not features already implemented.
Where a reference project is experimental, that status is a reason to study its
trade-offs carefully rather than to copy it.

## Reference systems

| Reference system | Purpose and primary users | Official evidence examined | Reusable strength to test | Aether relevance and boundary |
| --- | --- | --- | --- | --- |
| Rust | Memory-safe systems programming for developers who need ownership-aware APIs. | [Ownership chapter](https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html), [traits reference](https://doc.rust-lang.org/stable/reference/items/traits.html), [Result API](https://doc.rust-lang.org/core/result/) | Compile-time ownership rules, explicit result values, toolchain integration. | Study safety proof and diagnostics; do not inherit lifetime syntax or macro dependence by default. |
| Zig | Low-level systems programming for developers who want visible memory and compile-time control. | [Language reference](https://ziglang.org/documentation/master/) | Allocator parameters, compile-time execution, C-facing build discipline. | Primary evidence for the original explicit-allocation experiment; Aether 0.6 now has a deliberately narrower named arena surface. |
| Swift | General-purpose, strongly typed application and systems-adjacent programming. | [Type system](https://docs.swift.org/swift-book/documentation/the-swift-programming-language/types/), [ownership declarations](https://docs.swift.org/swift-book/documentation/the-swift-programming-language/declarations/) | Named value types and explicit borrowing/consuming parameter modes. | Compare ergonomic ownership syntax and type-directed tooling without importing ARC or platform assumptions. |
| Hylo | Experimental high-level systems language centered on mutable value semantics and generic programming. | [Introduction](https://hylo-lang.org/introduction/), [language specification](https://hylo-lang.org/docs/reference/specification/) | Value semantics, projections, and a distinct ownership model without explicit lifetimes. | The closest conceptual probe for the original mutable-value-semantics goal; experimental status prevents treating it as settled proof. |
| Koka | Research language for typed functional programming with effect types and handlers. | [Official language book](https://koka-lang.github.io/koka/doc/book.html) | Inferred effect rows and handlers as a way to expose observable behavior. | Supplies a typed-effect comparison point; Aether implements only one closed abortive `Error[Whole]` effect, not inferred rows or resumptive handlers. |
| OCaml 5 | General-purpose functional language with modular programming and standardized effect-handler support. | [Effect-handler manual](https://ocaml.org/manual/effects.html) | Defined handler semantics and examples spanning resumable control flow and concurrency. | Tests whether a small effect core can support multiple execution strategies; not a direct syntax model. |
| Roc | Functional language organized around applications and host-provided platforms. | [Platforms and applications](https://www.roc-lang.org/platforms), [tutorial](https://www.roc-lang.org/tutorial) | A clear boundary between pure application logic and host-provided effects. | Useful for capability-oriented host boundaries and AI-readable documents; it does not define Aether's runtime or artifact format. |
| Go | Production language with built-in goroutines, channels, and a compact deployment story. | [Language specification](https://go.dev/ref/spec) | Simple concurrency surface and explicit communication primitives. | Counterexample and comparison for unstructured task lifetime: Aether will not claim a concurrency model until structured cancellation is proved. |
| Odin | Systems language explicitly supporting data-oriented and structure-of-arrays representations. | [Language overview](https://odin-lang.org/docs/overview/), [FAQ](https://odin-lang.org/docs/faq/) | First-class SoA forms, custom allocator culture, and C-facing systems programming. | Direct probe for data-layout syntax; Aether has no SoA lowering or data-oriented dispatch today. |
| WebAssembly | Portable, typed virtual instruction-set standard for embedders and language toolchains. | [Core specification introduction](https://webassembly.github.io/spec/core/intro/introduction.html), [validation algorithm](https://webassembly.github.io/spec/core/appendix/algorithm.html), [component concepts](https://component-model.bytecodealliance.org/design/component-model-concepts.html) | Verify-before-execution, binary validation, explicit imports, and machine-readable interfaces. | Architectural comparison for AETH verification and future interop boundaries; AETH remains Aether-owned and is not WebAssembly. |

## Cross-reference findings

### Ownership and resource visibility

Rust, Swift, Hylo, and Zig all make resource behavior visible, but by different
means. Rust proves aliasing through ownership and borrowing; Swift makes
borrowing and consuming available in declarations; Hylo investigates mutable
value semantics and projections; Zig makes allocation selection an API concern.
The Aether question is not “which syntax wins?” It is whether a value-first,
destructive-move model can give local reasoning while keeping annotation burden
and implementation complexity measurable.

### Effects and task lifetime

Koka and OCaml show that effect handlers can model more than exceptions, while
Roc shows how a host boundary can make environmental interaction explicit. Go
shows the usability value of a lightweight concurrency surface, but its `go`
statement is deliberately not evidence that fire-and-forget work is safe for
Aether's target. Aether needs a typed-effect and structured-task proposal before
selecting syntax or a scheduler.

### Data layout, artifacts, and interfaces

Odin makes SoA layout visible rather than relying wholly on optimizer guesses.
WebAssembly demonstrates the value of a typed, validated binary with explicit
imports and a separately specified execution boundary. These are evidence for
Aether's existing verify-before-run posture and for measuring future data-layout
work; they do not prove that an automatic AoS-to-SoA transformation is correct
for all programs.

## What this study does not establish

- It does not establish that Aether is “better than all languages.” That is an
  empirical claim requiring scoped criteria, reproducible comparisons, and
  evidence; see [CORE_CLAIMS.md](../CORE_CLAIMS.md).
- It does not change the current AETH-only production law. Aether source is not
  translated to LLVM, C, Rust, JavaScript, or another language.
- It does not import a reference project's grammar, runtime, license, source,
  test corpus, or implementation into Aether.
- It does not turn a research direction into a language feature. Every feature
  still requires a specification, an accepted ADR where material, seed parity,
  behavior tests, and the normal constitution gate.

## Next research artifacts

The comparison categories and their dependencies are in
[02-component-decomposition.md](02-component-decomposition.md). The design
synthesis and evidence requirements are in
[03-synthesis-and-evidence.md](03-synthesis-and-evidence.md).
