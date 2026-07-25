# Aether in XLang

This repository now hosts Aether, a new local-first language and desktop
workbench. Its executable kernel parses only Aether syntax, emits AETH bytecode,
verifies every artifact, and runs that artifact in the Aether VM. It never
transpiles to an existing language.

## Workspace

- crates/xlang-core contains the Aether parser, semantic checks, canonical
  formatter, bytecode emitter, verifier, and VM.
- apps/xlang-cli builds the aether command-line compiler.
- apps/xlang-studio is Aether Studio, the local Tauri desktop workbench.
- legacy preserves the V1, V2, and historical AI Studio source as reference
  material only. It is not in the production build.
- SOUL.md remains the governing engineering standard.

## Command Line

    Set-Location C:\WPAI\Software\XLang
    cargo run -p aether-cli -- check (Resolve-Path .\examples\welcome.ae)
    cargo run -p aether-cli -- compile (Resolve-Path .\examples\welcome.ae) --output .\target\welcome.aeth
    cargo run -p aether-cli -- run .\target\welcome.aeth

The compiled file begins with AETH and is executed only by the Aether VM.

## Desktop Studio

    Set-Location C:\WPAI\Software\XLang\apps\xlang-studio
    npm install
    npm run desktop:dev

Studio builds the current Aether document, displays the verified AETH artifact
and VM output, and keeps compilation separate from AI review.

Docker-hosted Ollama is the only AI integration. It is loopback-only, optional,
and local; qwen2.5:3b remains the default reviewer for the GTX 1660 Ti 6 GB
environment. Set XLANG_OLLAMA_URL and XLANG_OLLAMA_MODEL before launching Studio
only when a different local endpoint or installed model is required.

## Data Handling

Compilation is in-process. The Studio WebView keeps its editor buffer and model
selection in local storage under aether.source and aether.model. No source,
artifact, model selection, or financial data is uploaded, synchronized, or
stored in a cloud service.

See docs/ARCHITECTURE.md for the runtime boundary, docs/AETHER_0.1.md for the
implemented grammar, and AUDIT_REPORT.md for the migration audit.
