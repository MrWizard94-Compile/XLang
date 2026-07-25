# Aether in XLang

This repository hosts Aether, a new local-first language and desktop workbench.
Aether 0.3.0 parses only Aether source, emits AETH v3 bytecode, verifies every
artifact, and runs it in the Aether VM. It never transpiles source to an
existing language.

## Stage 2: Forge Foundation

Stage 2 adds bounded `Bytes` values to the Stage 1 language. `Bytes` use the
same explicit `borrow name` or `move name` access discipline as `Text`, while
`Whole` and `Truth` remain copied values. The language now supports hexadecimal
`bytes` literals, `encode`, `decode`, `extent`, `octet`, `slice`, `fuse`, and
`append`, plus checked `quotient` and `remainder` arithmetic.

The core also provides two verified host boundaries:

- `invoke_bytecode` invokes a named weave with typed host values.
- `aether forge` accepts only a verified compiler artifact with
  `compile [borrow source: Text] -> Bytes`, supplies the source text, verifies
  the returned bytes as AETH, and then writes the output artifact.

This is a forge foundation, not a self-hosted compiler. No Aether-written
compiler source or self-reproducing compiler artifact is claimed in this
release. The Rust implementation remains the bootstrap compiler and VM.

## Workspace

- `crates/xlang-core` contains the parser, semantic checks, canonical formatter,
  AETH emitter, verifier, VM, and typed invocation boundary.
- `apps/xlang-cli` builds the `aether` command-line compiler and forge bridge.
- `apps/xlang-studio` is Aether Studio, the local Tauri desktop workbench.
- `examples` contains verified Aether 0.3 source programs.
- `legacy` preserves V1, V2, and historical AI Studio material as reference
  only. It is not in the production build.
- `SOUL.md` remains the governing engineering standard.

## Command Line

    Set-Location C:\WPAI\Software\XLang
    cargo run -p aether-cli -- check (Resolve-Path .\examples\welcome.ae)
    cargo run -p aether-cli -- compile (Resolve-Path .\examples\welcome.ae) --output .\target\welcome.aeth
    cargo run -p aether-cli -- run .\target\welcome.aeth

When an Aether compiler artifact has been built and verified, invoke its fixed
ABI through the forge bridge:

    cargo run -p aether-cli -- forge .\target\compiler.aeth .\examples\welcome.ae --output .\target\forged.aeth

The compiled file begins with `AETH` and format version `3`. AETH v2 artifacts
are intentionally rejected by the Aether 0.3 VM.

## Desktop Studio

    Set-Location C:\WPAI\Software\XLang\apps\xlang-studio
    npm install
    npm run desktop:dev

Studio builds the current Aether document, displays the verified AETH artifact
and VM output, and keeps compilation separate from AI review. Its starting
document is valid Aether 0.3 source and demonstrates `Bytes` operations.

Docker-hosted Ollama is the only AI integration. It is loopback-only, optional,
and local. `qwen2.5:3b` remains the default reviewer for the GTX 1660 Ti 6 GB
environment. Set `XLANG_OLLAMA_URL` and `XLANG_OLLAMA_MODEL` before launching
Studio only when a different local endpoint or installed model is required.

## Data Handling

Compilation and forge invocation run locally. The Studio WebView keeps its
editor buffer and model selection in local storage under `aether.source` and
`aether.model`. No source, artifact, model selection, or financial data is
uploaded, synchronized, or stored in a cloud service.

See `docs/ARCHITECTURE.md` for runtime boundaries,
`docs/AETHER_0.3.md` for the executable grammar,
`docs/FORGE_CONTRACT.md` for the compiler ABI, and `AUDIT_REPORT.md` for the
migration audit.
