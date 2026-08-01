# XLang / Aether — Grok agent kit

Specialized subagents and personas for this repository. Discovered from
`.grok/agents/` and `.grok/personas/` (see Grok user guide `16-subagents.md`).

## AGENTS Constitution IS LAW

Every agent and persona in this kit is bound by:

| Layer | Path |
|-------|------|
| Binding summary | [CONSTITUTION-BINDING.md](CONSTITUTION-BINDING.md) |
| Project entry (Level 4) | [`../AGENTS.md`](../AGENTS.md) |
| Pack (Levels 1–3) | [`../../../AGENTS Constitution/`](../../../AGENTS%20Constitution/) |

**Always-load set:** project `AGENTS.md` → pack `AGENTS.md`, `SOP.md`,
`constitution/03-DEFINITION-OF-DONE.md`, `standards/ENGINEERING.md`,
`standards/TESTING.md`, `standards/DOCUMENTATION.md`.

**Non-negotiable:** `CONST-COMPLETE-001`, `CONST-DEP-001`, `CONST-DONE-001`,
`CONST-GATE-001`, `ENG-WARN-001`, `TEST-BEHAVIOR-001`, `DOC-SYNC-001`,
`SEC-INPUT-001`, `CONST-CONTRACT-001`.

Agents may **tighten** product practice. They may **not** weaken pack `CONST-*`
rules without a filled pack `PROJECT-OVERRIDE` and named human approval.

All agent bodies open with **AGENTS Constitution IS LAW**. `agents_md: true` is
set so project Level-4 law is injected. Prefer persona `agents-constitution`
when spawning generic types that still need pack discipline.

---

## Agents (spawn with `subagent_type`)

| Agent | Mode | Use when |
|-------|------|----------|
| `aether-explorer` | read-only | Map compiler, seed, CLI, docs, or opcodes |
| `aether-core-engineer` | full | Change Rust bootstrap (`crates/xlang-core`, CLI) |
| `aether-seed-engineer` | full | Edit `seed/aether_seed.ae`, self-host proof, AETH emit |
| `aether-spec-writer` | full | Specs, MANIFEST, honest Stage/self-host claims |
| `aether-gate-runner` | execute | Run fmt/clippy/tests/seed forge; Section 0 mapping |
| `aether-reviewer` | read-only | Review diffs vs Rule IDs + Aether invariants |

Manage in TUI: `/config-agents` or `/agents`.

## Personas (behavioral overlays)

| Persona | Focus |
|---------|--------|
| `agents-constitution` | Universal pack always-load + Section 0 (use liberally) |
| `aether-honest-claims` | No overclaim self-host / transpile |
| `aether-zero-warning` | `ENG-WARN-001` / Done discipline |
| `aether-seed-discipline` | Seed Profile shape + proof workflow |

## Roles

| Role | Default capability |
|------|--------------------|
| `aether-implementer` | `all` (still gate-bound) |
| `aether-verifier` | `execute` (honest gate reports) |

## Product constraints (Level 4)

1. Project entry: `AGENTS.md`
2. Product contract: `MANIFEST.md`, `docs/`
3. No DigiChar remote/path mixing — canonical path `C:\WPAI\Software\XLang`
4. Seed self-host claims only with multi-generation proof
