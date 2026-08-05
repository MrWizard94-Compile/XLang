# ADR-014: post-M10 track portfolio (modules, host I/O, LSP, and related)

**Status:** Accepted (portfolio / process decision) — tracks complete through **M16**;
T-CT ([ADR-019](ADR-019-m15-comptime-expansion.md)) and T-RX first slice
([ADR-020](ADR-020-m16-resource-effect.md)) implemented; portfolio itself
authorizes no code without per-track ADRs  
**Date:** 2026-08-04 (status refreshed after M16)  
**Decision makers:** Product direction under AGENTS Constitution; agent authors portfolio ADR after M10 Done  
**Related Rule IDs:** `DOC-ADR-001`, `DOC-SYNC-001`, `CONST-DEP-001`, `CONST-COMPLETE-001`, `SEC-INPUT-001`, `TEST-BEHAVIOR-001`, `CONST-CONTRACT-001`  

## Context

Aether has completed:

- Language pilots **M0–M8** (surface **0.11** / AETH **v11**)  
- Tooling **M9–M10** (package **0.13**: offline multi-unit projects, independent compile)  
- Product completion **TP-1** (integrity) and **TP-2** (local technical preview)  
- Post-M10 queue through **M11 modules, M12 edits, M13 LSP, M14 host I/O** (package **0.19**)

The next growth surface is large. Candidates include **language modules**, **host
I/O capabilities**, **bounded LSP**, fine-grained structural edits, comptime
expansion, resource↔effect interaction, C/FFI, multi-package graphs, and a
native backend. Implementing any of these without a dedicated design loop would
violate `CONST-DEP-001`, `DOC-ADR-001`, and the technical-preview threat model
freeze.

Agents must not “just start” modules, host I/O, or LSP from chat pressure.

## Decision

### D1 — Portfolio law

Every post-M10 feature track requires its **own** SOP mini-cycle before code:

```text
research/spike bounds → design doc → track ADR (implementable) → validation matrix
→ vertical slice → gates → DOC-SYNC → delivery report
```

ADR-014 is a **portfolio** decision. It does **not** authorize product code for
any named track.

### D2 — Hard blocks (unchanged)

| Track | Status under current law |
| --- | --- |
| Native / LLVM / transpile-to-other-language | **Blocked** until F-NATIVE human authorize ([ADR-037](ADR-037-f-native-law-fork.md)); source transpile remains forbidden even if fork opens |
| Network package registry | **Blocked** until F-REGISTRY human authorize ([ADR-038](ADR-038-f-registry-law-fork.md)) |
| Ambient guest file/process/network/shell | **Blocked** |
| Blanket “better than all languages” | **Prohibited claim** |

### D3 — Recommended order (default queue)

Priority balances **north-star AI authoring**, **multi-unit usefulness**, and
**capability risk** (lowest-risk tooling before high-risk host expansion).

| Pri | Track ID | Working title | Why this order | Requires before code |
| --- | --- | --- | --- | --- |
| **1** | **T-MOD** | Language modules / import surface | Unlocks real multi-file programs after M10; seed dual-compare is hard but well-scoped | DESIGN + implementable ADR + matrix + seed plan |
| **2** | **T-EDIT** | Fine-grained structural edits | AI-first north star; stays host tooling if bounded | DESIGN + ADR + hostile edit corpus |
| **3** | **T-LSP** | Bounded LSP (diagnostics + structure only) | Tooling value; must not become second compiler authority | DESIGN + ADR + authority model |
| **4** | **T-CT** | Comptime expansion (post-M5) | Language power; fuel/authority hazards | DESIGN + ADR + DoS limits |
| **5** | **T-RX** | Resource ↔ effect interaction | Real programs; soundness risk | DESIGN + ADR + cancel/cleanup matrix |
| **6** | **T-HOST** | Host I/O capabilities (beyond pure fixtures) | Systems usefulness; **threat-model rewrite required** | New threat model + DESIGN + ADR + deny-by-default tests |
| **7** | **T-FFI** | C / foreign ABI beyond pure host | Highest ownership/hostile-input risk | Threat model + DESIGN + ADR |
| **8** | **T-PKG** | Multi-package / multi-root offline graphs | After modules; still no registry | DESIGN + ADR after T-MOD |
| **∅** | **T-NATIVE** | Native backend | Law conflict | Law change first |

Human may reorder with an explicit written override (ROADMAP or superseding ADR).

### D4 — Default next design assignment

Unless the human redirects:

1. **Historical default after M10:** T-MOD → T-EDIT → T-LSP → T-CT → T-RX →
   T-HOST → T-FFI → T-PKG (T-HOST completed as M14; T-CT as M15; T-RX slice as M16).  
2. **Current default next:** deeper T-RX, **T-PKG**, stdlib/test tooling, or
   **T-FFI** — each requires its own design/ADR (not free-form continuation).  
3. **Do not** claim full resourceful effects or nursery+resource without a new ADR.  
4. **Do not** claim full comptime metaprogramming until later T-CT slices have
   their own ADRs.

### D5 — Entry criteria shared by every track

A track may leave “Proposed” only when all hold:

1. **Falsifiable spike** with stop condition (see track cards below).  
2. **Capability story** consistent with verify-before-run and guest untrusted.  
3. **Seed plan**: byte-identity path for valid corpus **or** explicit
   bootstrap-only tooling claim (never silent).  
4. **Threat model delta** if host authority expands (mandatory for T-HOST, T-FFI,
   registry).  
5. **Human accept** of the implementable track ADR.

### D6 — Claim hygiene

- CORE_CLAIMS may list post-M10 tracks as **Accepted direction** or **Research
  hypothesis** only after this ADR.  
- No track may be marked **Proven now** until matrix green + delivery report.  
- Technical preview threat model remains binding until explicitly revised.

## Track cards (brief)

### T-MOD — Language modules

| Field | Content |
| --- | --- |
| Goal | Named import/export (or equivalent) so units can share weaves/types without concatenation hacks |
| Depends on | M10 paths/projects; ownership/resource law; seed emission |
| Stop if | Seed cannot prove identity; modules become ambient globals; cycles/unsound visibility |
| Non-goals | Network packages; dynamic loading; reflection |
| Likely AETH impact | New metadata and/or link step — version bump if bytecode shape changes |
| Threat | Low if pure language; medium if module resolution reads outside project root |

### T-EDIT — Fine-grained structural edits

| Field | Content |
| --- | --- |
| Goal | Edit protocol beyond top-level insert/replace/delete (bodies, nested nodes) |
| Depends on | Authoring v6 stability; ownership-preserving reparse |
| Stop if | JSON paths become a second unsafe language; stale/base bypass |
| Non-goals | Executing edits as code; model-network integration |
| Threat | Low–medium (host-only tooling) |

### T-LSP — Bounded language server

| Field | Content |
| --- | --- |
| Goal | Diagnostics + structure/navigation for editors; offline |
| Depends on | Stable diagnostics; optionally T-EDIT |
| Stop if | LSP becomes product compile authority or silently writes without explicit path |
| Non-goals | Full refactor suite; remote multi-tenant IDE |
| Threat | Medium (long-lived process; path authority) |

### T-HOST — Host I/O capabilities

| Field | Content |
| --- | --- |
| Goal | Capability-mediated guest→host I/O (read/write/env) with explicit grants |
| Depends on | M8 pure host pilot; **new threat model** superseding TP freeze for this surface |
| Stop if | Ambient authority; missing-service not fail-closed; untested path escape |
| Non-goals | Full POSIX; unrestricted shell |
| Threat | **High** — must revise [THREAT_MODEL-TECHNICAL-PREVIEW.md](THREAT_MODEL-TECHNICAL-PREVIEW.md) |

### T-FFI — C / foreign ABI

| Field | Content |
| --- | --- |
| Goal | Narrow typed FFI beyond pure fixtures |
| Depends on | Ownership mapping; hostile input; T-HOST lessons recommended first |
| Stop if | Header soup; unsound ownership; libloading without pin |
| Threat | **Highest** |

### T-CT, T-RX, T-PKG

Deferred detail until their design docs; still subject to D1–D5. Comptime must
not observe host I/O. Resource↔effect must not orphan cleanup. Multi-package
must stay offline and path-confined.

## Consequences

### Positive

- Clear queue after M10 without false “1.0 complete” claims.  
- Agents cannot invent host I/O or LSP as silent completion work.  
- Human can reorder tracks with a single override document.

### Costs

- Modules (most useful next) still need a full design before any code.  
- Host I/O and FFI remain distant relative to developer appetite.

### Risks

| Risk | Mitigation |
| --- | --- |
| Scope thrash across tracks | One active design track at a time (default T-MOD) |
| Fake modules via host concatenation | Explicitly rejected (see ADR-013 alternatives) |
| LSP second compiler | Authority boundary in T-LSP design |
| Host I/O without threat rewrite | D2/D5 gate |

## Alternatives considered

| Option | Outcome |
| --- | --- |
| Implement modules immediately without ADR | Rejected (`CONST-DEP-001`) |
| Host I/O next for “real programs” | Rejected as default (threat cost); allowed if human overrides order |
| LSP next for DX | Allowed after T-EDIT preference; not default #1 |
| Single mega-ADR that implements all tracks | Rejected (`CONST-COMPLETE-001` / cognitive load) |
| Freeze all language growth forever | Rejected; portfolio queue is explicit |

## Links

- [ADR-011](ADR-011-m8-host-abi-pilot.md) pure host pilot  
- [ADR-012](ADR-012-m9-project-tooling.md) project tooling  
- [ADR-013](ADR-013-m10-multi-unit-projects.md) multi-unit independent compile  
- [THREAT_MODEL-TECHNICAL-PREVIEW.md](THREAT_MODEL-TECHNICAL-PREVIEW.md)  
- [ROADMAP.md](ROADMAP.md)  
- [CORE_CLAIMS.md](CORE_CLAIMS.md)  
- [NORTH_STAR.md](NORTH_STAR.md) (direction ≠ current behavior)  
- Companion brief: [DESIGN-POST-M10-TRACK-PORTFOLIO.md](DESIGN-POST-M10-TRACK-PORTFOLIO.md)

## Implementation gate for this ADR

**None.** ADR-014 authorizes documentation and sequencing only.

Next engineering action under default queue:

1. Author `DESIGN-M11-LANGUAGE-MODULES.md` (or equivalent title).  
2. Author implementable `ADR-015` (modules) + `M11-VALIDATION-MATRIX.md`.  
3. Human accept ADR-015 before any parser/seed/VM code.

---

*End of ADR-014.*
