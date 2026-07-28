# XLang / Aether — Grok agent kit

Specialized subagents and personas for this repository. Discovered from
`.grok/agents/` and `.grok/personas/` (see Grok user guide `16-subagents.md`).

## Agents (spawn with `subagent_type`)

| Agent | Mode | Use when |
|-------|------|----------|
| `aether-explorer` | read-only | Map compiler, seed, Studio, docs, or opcodes |
| `aether-core-engineer` | full | Change Rust bootstrap (`crates/xlang-core`, CLI) |
| `aether-seed-engineer` | full | Edit `seed/aether_seed.ae`, self-host proof, AETH emit |
| `aether-studio-engineer` | full | Tauri/React Studio (`apps/xlang-studio`) |
| `aether-spec-writer` | full | Specs, MANIFEST, honest Stage/self-host claims |
| `aether-gate-runner` | execute | Run fmt/clippy/tests/seed forge; report only |
| `aether-reviewer` | read-only | Review diffs against AGENTS + Aether invariants |

Manage in TUI: `/config-agents` or `/agents`.

## Personas (behavioral overlays)

| Persona | Focus |
|---------|--------|
| `aether-honest-claims` | Never overclaim self-hosting or transpiler absence |
| `aether-zero-warning` | Zero warnings, fix root cause, no suppressions |
| `aether-seed-discipline` | Seed Profile rules: shallow exprs, `vN` slots, forge ABI |

## Law

1. Project entry: `AGENTS.md` (Level 4 pointer)
2. Pack: `../../AGENTS Constitution/` (binding quality law)
3. Product contract: `MANIFEST.md`, `docs/`

Never push this repo to the DigiChar remote. Canonical path:
`C:\WPAI\Software\XLang`.
