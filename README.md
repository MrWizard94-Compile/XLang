# Aether in XLang

This repository hosts Aether, a new local-first language and desktop workbench.
The Stage 1 compiler parses only Aether source, emits AETH v2 bytecode, verifies
every artifact, and runs it in the Aether VM. It never transpiles to an existing
language.

## Stage 1

Aether 0.2.0 adds named weaves, `Text`, `Whole`, and `Truth` values, explicit
local mutation, structured `choose` and `while` control flow, function calls,
local `borrow` and `move` checks for `Text`, and bounded Unicode text
primitives. The compiler and VM share one deterministic, verified artifact
format.

This is a self-hosting substrate, not a self-hosted compiler. Explicit
allocators, arenas, records, collections, typed error sets, compile-time
execution, native targets, and a compiler written in Aether remain later
milestones. The host bootstrap is not an Aether runtime dependency.

## Workspace

- `crates/xlang-core` contains the parser, semantic checks, canonical formatter,
  AETH emitter, verifier, and VM.
- `apps/xlang-cli` builds the `aether` command-line compiler.
- `apps/xlang-studio` is Aether Studio, the local Tauri desktop workbench.
- `examples` contains verified Stage 1 source programs.
- `legacy` preserves V1, V2, and historical AI Studio material as reference
  only. It is not in the production build.
- `SOUL.md` remains the governing engineering standard.

## Command Line

    Set-Location C:\WPAI\Software\XLang
    cargo run -p aether-cli -- check (Resolve-Path .\examples\welcome.ae)
    cargo run -p aether-cli -- compile (Resolve-Path .\examples\control-flow.ae) --output .\target\control-flow.aeth
    cargo run -p aether-cli -- run .\target\control-flow.aeth

The compiled file begins with `AETH` and format version `2`. It is executed only
by the Aether VM.

## Desktop Studio

    Set-Location C:\WPAI\Software\XLang\apps\xlang-studio
    npm install
    npm run desktop:dev

Studio builds the current Aether document, displays the verified AETH artifact
and VM output, and keeps compilation separate from AI review. Its starting
document is valid Aether 0.2 source.

Docker-hosted Ollama is the only AI integration. It is loopback-only, optional,
and local. `qwen2.5:3b` remains the default reviewer for the GTX 1660 Ti 6 GB
environment. Set `XLANG_OLLAMA_URL` and `XLANG_OLLAMA_MODEL` before launching
Studio only when a different local endpoint or installed model is required.

## Data Handling

Compilation is in-process. The Studio WebView keeps its editor buffer and model
selection in local storage under `aether.source` and `aether.model`. No source,
artifact, model selection, or financial data is uploaded, synchronized, or
stored in a cloud service.

See `docs/ARCHITECTURE.md` for runtime boundaries,
`docs/AETHER_0.2.md` for the executable grammar, and `AUDIT_REPORT.md` for the
migration audit.
