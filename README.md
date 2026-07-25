# XLang

XLang is a local-first language workbench. The production path is an audited Rust
frontend exposed through a command-line checker and XLang Studio, a Tauri desktop
application. Studio can ask a model served by Docker-hosted Ollama to review source,
but the model is not part of compilation and cannot execute code.

## Workspace

- `crates/xlang-core`: lexer, parser, and monomorphic type checker.
- `apps/xlang-cli`: `xlang check <file>` command-line validation.
- `apps/xlang-studio`: React/Tauri desktop compiler workbench.
- `legacy`: preserved V1, V2, and AI Studio source snapshots. They are historical
  reference material, not part of the production build.
- `SOUL.md`: governing engineering standard, preserved verbatim from WPAI.

## Run

Install the Studio frontend dependencies, then launch the desktop application:

```powershell
Set-Location C:\\WPAI\\Software\\XLang\\apps\\xlang-studio
npm install
npm run desktop:dev
```

Use the model selector after Docker Ollama is reachable on `127.0.0.1:11434`.
The default is `qwen2.5:3b`, selected for responsive local review on the available
GTX 1660 Ti 6 GB environment. Set `XLANG_OLLAMA_URL` and `XLANG_OLLAMA_MODEL` only
when launching the desktop app if a different local configuration is required.

For compiler-only validation:

```powershell
Set-Location C:\\WPAI\\Software\\XLang
cargo run -p xlang-cli -- check (Resolve-Path .\\examples\\welcome.xl)
```

## Data Handling

Compilation stays in-process inside the desktop app. Source text and the selected
model are stored only in the Studio WebView's local storage under `xlang.source`
and `xlang.model`; nothing is uploaded, synchronized, or persisted in a cloud
service. Ollama review requests travel only to the validated loopback endpoint.

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for the runtime boundary and
[AUDIT_REPORT.md](AUDIT_REPORT.md) for the starting-point assessment.
