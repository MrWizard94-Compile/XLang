# XLang Audit Report

Date: 2026-07-25

## Finding Summary

| Area | Result | Decision |
| --- | --- | --- |
| `legacy/xlang-v1-prototype` | Does not compile. It has a missing `Span: Ord` implementation and a moved-expression borrow. | Preserve as a prototype; exclude from production build. |
| `legacy/xlang-v2-snapshot` | Builds cleanly with Rust 2021 and passes 12 frontend tests. | Promote its source into `crates/xlang-core`. |
| `legacy/aether-genesis-ai-studio` | Stale AI Studio/Gemini metadata and dependencies; no live Gemini import was found. Its simulation evaluates generated JavaScript through `new Function`. | Preserve as historical material; do not use it to compile or execute XLang. |
| Docker Ollama | `ollama-engine` is running on loopback port `11434`; `qwen2.5:3b` was pulled and answered a live chat request. | Make it the only AI integration path in Studio. |

## Production Baseline

`xlang-core` is the V2 bootstrap frontend extracted into a normal Rust crate. It
lexes, parses, and type-checks functions, `let` bindings, return statements,
integer, boolean, and string values, expressions, `while`, and counted `for`
loops. The explicit bootstrap limits remain documented in its source and manifest.

XLang Studio invokes that frontend in the Tauri process. Ollama is a separate,
optional reviewer: it receives source only after the user asks for a review and
cannot alter compiler output or execute generated code.

## Legacy Preservation

The original folders were moved intact beneath `legacy/` to make the production
workspace clear while retaining source history and design reference. The V1
compiled binary is deliberately ignored by Git; no original source was deleted.
