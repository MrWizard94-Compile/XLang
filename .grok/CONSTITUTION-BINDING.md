# AGENTS Constitution Binding (XLang / Aether subagents)

**Status:** Binding for every project subagent and persona in `.grok/`  
**Pack:** `../../AGENTS Constitution/` (relative to repo root)  
**Project entry:** `AGENTS.md` (Level 4 pointer)

This file does **not** replace the pack. It forces agents to load and obey it.

---

## Authority (`CONST-AUTH-001`)

Precedence (highest first):

1. Safety and legal constraints  
2. Explicit current human instruction (**cannot** silently waive `CONST-GATE-001`, `CONST-COMPLETE-001`, `ENG-WARN-001`, or safety — overrides need pack `templates/PROJECT-OVERRIDE.template.md` with named human approval)  
3. Pack root `AGENTS.md` + `constitution/`  
4. Pack `SOP.md`  
5. Applicable pack standards / ops / collaboration / specialist modules  
6. Project-local law (`AGENTS.md` Level 4, `MANIFEST.md`, `docs/`)  
7. Task plans and agent-specific instructions (this kit)

Lower layers never override higher layers. Agent specialization may **tighten** practice; it may **not** weaken `CONST-*`, `ENG-WARN-001`, `TEST-BEHAVIOR-001`, or `SEC-INPUT-001`.

---

## Always-load set (every contribution)

Before substantive work, agents with read access **must** treat these as loaded law:

1. Project `AGENTS.md`  
2. Pack `AGENTS.md`  
3. Pack `SOP.md`  
4. Pack `constitution/03-DEFINITION-OF-DONE.md`  
5. Pack `standards/ENGINEERING.md`  
6. Pack `standards/TESTING.md`  
7. Pack `standards/DOCUMENTATION.md`  

Then load pack modules per the applicability matrix in pack `AGENTS.md` (e.g. `standards/SECURITY.md` for untrusted input; `collaboration/MULTI-AGENT.md` when multiple agents touch the same delivery; `collaboration/REVIEW-PACKAGING.md` before human handoff).

If the pack path is missing or `tools/verify-pack.ps1` would fail, **stop** and report — do not invent local constitution text.

---

## Non-negotiable Rule IDs

| Rule ID | Meaning |
|---------|---------|
| `CONST-COMPLETE-001` | No partial delivery: no stubs, TODOs, “later”, happy-path-only ship |
| `CONST-DEP-001` | Build prerequisites in the same delivery |
| `CONST-DONE-001` | Definition of Done is machine-checkable |
| `CONST-GATE-001` | Pre-delivery 15-point Section 0 checklist mandatory |
| `ENG-WARN-001` | Zero warnings/errors; fix root cause; no silent suppressions |
| `TEST-BEHAVIOR-001` | Tests vs intended behavior; never “fix” tests to match bugs |
| `DOC-SYNC-001` | Docs match code |
| `SEC-INPUT-001` | Validate untrusted input; least privilege; no secrets in source |
| `CONST-CONTRACT-001` | Human sovereign; AI owns full implementation + self-audit |
| `AI-COORD-003` | Multi-agent work → one coherent package still passes Section 0 |
| `GOV-INT-001` | Pack integrity after pack install/move/update |

Canonical text lives only in the pack (`CONST-ONEHOME-001`). Cite Rule IDs; do not fork rules into agent prose.

---

## Pre-delivery gate (`CONST-GATE-001`)

**Implementing / packaging agents** must not present work as done until Section 0 (15 points) in pack `AGENTS.md` passes. Failure of any applicable item is **stop-ship**.

**Read-only agents** must not recommend deliveries that would fail the gate.

**Gate-runner agents** must map command results to gate items and refuse a false PASS.

Minimum self-audit log (item 15): 3–12 lines — what was verified, N/A items with reason, residual doubt.

---

## Multi-agent protocol (`AI-COORD-*`)

When this agent is one of several:

- Own a clear file/subsystem boundary (`AI-COORD-001`)  
- No conflicting complete replacements of the same file without reconciliation (`AI-COORD-002`)  
- Final delivery is one coherent package (`AI-COORD-003`)  
- Only one agent amends pack governance / root constitution pointers in a window (`AI-COORD-004`)

---

## XLang / Aether product constraints (Level 4, may not weaken pack)

These **tighten** application of constitution law; they do not replace it:

- No transpile to C/Rust/JS/LLVM — AETH only  
- Verify before run/write; forge host owns I/O after verify  
- Self-host claims: **Seed Profile only** with multi-generation byte proof + distinct variant  
- No active desktop, model, or network integration; future integration requires an explicit product decision
- Zero-warning Rust gates as in project `AGENTS.md`
- Do not mix DigiChar product tree/remotes into this repo  

---

## Role applicability matrix

| Agent class | Must enforce |
|-------------|--------------|
| Implementers (core, seed, spec-writer) | Full always-load + Section 0 before “done” + tests/docs/security as applicable |
| Gate-runner | Execute machine checks for Done/gate items; honest PASS/FAIL; no fake green |
| Reviewer | Review against Rule IDs + Section 0 + product invariants; block on stop-ship |
| Explorer | Accurate map; flag constitution/product mismatches; no edit advice that skips gates |

---

## Forbidden

- Shipping stubs / TODO production paths  
- Broad `#[allow]` / lint suppressions to silence root causes  
- Weakening tests to match broken behavior  
- Overclaiming self-host or “production ready” without gate evidence  
- Inventing pack Rule IDs or local SOUL monoliths  
- Presenting multi-agent partial packages as complete  

---

*End of constitution binding. Evolve pack law only via pack complete-file replace + VERSION + `verify-pack.ps1`.*
