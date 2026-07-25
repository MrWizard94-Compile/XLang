# Aether 0.1 Language Specification

## Identity

Aether is a new value-oriented systems language derived from the architectural
intent in Aether.md, not from the syntax of the legacy XLang prototypes. Its
first compiler emits Aether bytecode for the Aether virtual machine.

## Kernel Source Form

    world genesis

    weave main [] -> Whole:
      bind greeting <- "Hello from Aether\n"
      speak greeting
      yield 0

The kernel accepts exact two-space indentation. Semicolons, braces, C-style
declarations, and legacy fn or let syntax are not Aether syntax.

## Implemented Forms

| Form | Meaning |
| --- | --- |
| world name | Declares one lowercase module identity. |
| weave main [] -> Whole: | Declares the single Stage 0 entry weave. |
| bind name <- value | Introduces one immutable Text or Whole binding. |
| speak value | Writes a Text value to deterministic stdout. |
| yield value | Terminates the weave with a Whole exit code. |

Text literals support only the escapes backslash, quote, newline, carriage
return, and tab. Names are lower-case ASCII identifiers. The kernel rejects
implicit conversion, shadowing, use-before-bind, noncanonical indentation,
trailing whitespace, and a statement after yield.

## Artifact and Runtime

The compiler emits a binary artifact beginning with the ASCII magic AETH and a
format version byte. A compact stack instruction set holds Text and Whole
values, stores immutable local slots, loads them, writes Text, and yields a
Whole. The verifier rejects an invalid header, malformed constants, unknown
opcodes, invalid local slots, stack underflow, invalid type flow, nonempty
stacks at yield, and instructions after termination before the VM runs it.

Both source formatting and AST serialization are deterministic. CRLF input is
accepted, then normalized by the canonical formatter to LF.

## Deliberate Boundary

This is the executable Aether 0.1 kernel, not a claim that every later Aether
feature already exists. The terms MVS, arena, forge, choose, match, and move
from Aether.md remain design constraints for the next independently verified
language stages; they are deliberately rejected by this grammar today rather
than accepted as incomplete syntax.

The host implementation exists only to create the first Aether compiler and VM.
The self-hosting milestone requires an Aether compiler written in Aether,
compiled to verified AETH bytecode, then used to recompile its own source to an
identical artifact. The bytecode format and VM remain Aether-owned; C, Rust,
LLVM, and JavaScript are not Aether targets.
