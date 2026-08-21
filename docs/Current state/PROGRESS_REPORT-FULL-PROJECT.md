# Aether / XLang — Comprehensive Project Progress Report

**Date:** 2026-08-21
**Baseline audit HEAD:** `12a2181` on `codex/xlang-local-first-studio`
**Package contract:** **0.37.0** (language **0.11** + M19e AETH v12 + M25 local package publication)
**Baseline audit:** [AUDIT_REPORT-2026-08-11-FULL-PROJECT.md](../historical%20docs/AUDIT_REPORT-2026-08-11-FULL-PROJECT.md) — **GREEN**
**Current delivery verification:** ADR-129 bounded seed-native transitive library chain — focused seed/product/bootstrap/forged artifact/hostile-input evidence, full release gate, package, and isolated consumer verification PASS (2026-08-21)
**Constitution:** AGENTS Constitution 5.0.1  

---

## 1. One-sentence status

Aether is a **local-first, seed-hosted product compiler** with verified AETH
execution, dual-compare self-host proofs, and human-authorized **F-NATIVE** /
**F-REGISTRY** pilots — now **0.37.0** package contract, with BARP having moved
product authority off the Rust bootstrap for default toolchain paths. ADR-129
now proves a bounded transitive seed bundle profile while M32a/M32b provide a
closed local verified-execution evidence and strict comparison surface.

---

## 2. What the product is today

### 2.1 Language and runtime

| Layer | Status |
| --- | --- |
| Core language surface | Canonical **0.11** (M2 resources, M4 Error[Whole], M5/M15/M23 comptime, M6 layout, M7 nurseries, M8 pure host) |
| Toolchain packages | M9–M18 projects/modules/LSP/tests/workspaces/stdlib; M19a–e resource/task; M21 FFI pilot; M22 cross-package; M23 seed-native; M25 local source-package lifecycle |
| AETH | **v11** default; **v12** for M19e `task weave` + `checkpoint` |
| Default compile | **Seed forge** (`seed/aether_seed.aeth`) — not Rust bootstrap |
| Execution | Verify-before-run VM; grant-empty pure fixtures; optional `--grant-*` / `--grant-lib` |

### 2.2 Product toolchain (BARP)

Bootstrap Authority Reduction Program ([DESIGN-BARP-001](DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md), ADR-043–128):

| Capability | Authority |
| --- | --- |
| `compile` / seed rebuild | Product seed (ADR-067) |
| `check` / `format` / `structure` / project format | Product default; `--bootstrap` recovery |
| LSP diagnostics / symbols / hover / definition / format | Product-primary |
| Structural edits | Product weave/body/record paths |
| Multi-module / multi-source | Host elaborate + seed emit + unit digests |
| SBP-001/SBP-002 bounded bundles | Seed owns one exact framed library -> entry profile and one exact foundation -> bridge -> entry profile; no host elaborator on either route and no general multi-module claim |
| Diagnostics | `AE-SEED-*` preflights + SPEAK packets; bounded seed pilot 003/004/005/006/007/010/011/012/013/014/015 |
| Bootstrap residual | Dual-compare oracle, recovery flags, full `aether.ast/v8` |

### 2.3 Law forks (human-authorized)

| Fork | Through | Highlights |
| --- | --- | --- |
| **F-NATIVE** | **M35j** (ADR-059–099) | AETH→C / object / LLVM IR / LLVM object / exe; `--target` closed matrix; host dual-run, cross link-only; probe + hermetic env (`AE-NATIVE-007`) |
| **F-REGISTRY** | **M24i** (ADR-060–100) | Offline pin/verify; HMAC/Ed25519; explicit fetch; rotation; multi-root policy; CLI root/certified-key setup; date-checked X.509-lite CA store included in cache verification (`AE-REG-012/013`) |

### 2.4 Task model

| Item | Status |
| --- | --- |
| M19e active-frame cancel | **Proven** (v12, checkpoints) |
| Task frame surface / checkpoint density / inventory | **Proven** tooling (ADR-085/093/097) |
| Reserved future surface | Fail-closed **AE-SEED-014** (ADR-089) |
| Task weave requires checkpoint | Fail-closed **AE-SEED-015** (ADR-101/106; exact direct seed pilot) |
| Handles / timeouts / parallel runtime | **Not implemented** (ADR-081 design only) |

---

## 3. Architecture snapshot

```
Aether source (.ae)
    │
    ├─ product path (default)
    │     host preflights (AE-SEED-*)
    │     multi-source? → host elaborate
    │     forge seed/aether_seed.aeth  → AETH bytes
    │     seed SPEAK merge on failure (pilot codes)
    │     verify_bytecode
    │
    ├─ recovery path (--bootstrap)
    │     Rust parser/validate/emit (oracle + AST)
    │
    └─ optional lowers (F-NATIVE, verified AETH only)
          → C / object / LLVM / native exe (host tools)

Verified AETH
    ├─ aether run  (VM; pure or grants)
    ├─ aether bench (embedded pure corpus; verify + decode + execute; no grants; M32b data-only compare)
    ├─ aether forge (compiler ABI only)
    └─ dual-compare tests (product ≡ bootstrap where claimed)
```

---

## 4. Evidence ladder (how we know it works)

| Evidence class | Mechanism |
| --- | --- |
| Unit / integration tests | `cargo test --workspace` |
| Seed self-host | `seed_self_host` dual-compare + multi-generation |
| Example corpus | Gate dual-compare of shipped examples |
| Quality gate | `tools/aether-gate.ps1` (quick / full / release) |
| Claim control | [CORE_CLAIMS.md](CORE_CLAIMS.md) status rules |
| ADR trail | 100+ ADRs under `docs/ADR-*.md` |
| Matrices | M* / M19E / M23 / M35A / M24A validation matrices |
| Constitution | Project Level-4 pointer → pack AGENTS.md Section 0 |

**Latest full-gate stamp:** 2026-08-14 ADR-126 delivery — **GATE PASS mode=release** (pack v5.0.1; 359 Markdown files / 1,447 local links; full-quality suite, four-way seed identity, 444-file package plus `SHA-256SUMS`, and independent consumer verification)
**Latest release-verified seed identity:** `D0D17756F587709BC85E323E0545281E304362288B687BAA0D48D8C334E18AFB`

---

## 5. Recent maturity program (post–0.36 release)

The 0.36 package contract (M19e) remains the executable language baseline. M25
now advances the toolchain package to 0.37 with a local package ecosystem step;
BARP and M32 work remain **independence and infrastructure maturity**:

### 5.1 BARP highlights (ADR-043 → 129; ADR-128/129 release verified)

- Product-default CLI toolchain; bootstrap recovery only  
- Multi-source envelope + multi-file host forge + unit digests  
- ADR-128 SBP-001 exact two-unit source bundle: verified seed `compile_bundle`
  owns frame parsing, one-edge elaboration, path mangling, and call rewrite;
  general M11/M22 remains host elaborate + seed emit (release verified)
- ADR-129 SBP-002 exact three-unit foundation -> bridge -> entry bundle:
  the same verified seed ABI owns two fixed transitive import edges, rewrites
  both qualified call boundaries, and remains separate from general M11/M22
  (release verified, including package and isolated consumer proof)
- SPEAK protocol `AETHER_SEED_ERROR:` + host packet ABI  
- Seed SPEAK pilot: **AE-SEED-003/004/005/006/007/010/011/012/013/014/015**; 003/007 are bounded lexical checks, 014 scans canonical reserved-task prefixes, 015 requires an exact checkpoint in a canonical task body, 010 detects only a canonical ordinary-Whole Text-literal or exact Truth-literal yield, 011 detects canonical total-Whole direct-bind/root-yield/root-revise/root-speak target existence, delimiter-bounded root-handle target existence, literal-erroring-header root-forward target existence, and literal total-root-nursery immediate zero-argument, single-digit Whole, positive two-digit Whole, or exact-bright Truth spawn target existence across declared top-level weave headers (the handle form requires fixed `into` / `otherwise error into` delimiters that leave a destination suffix; the forward form requires `-> Whole raises Whole:`; the nursery forms require root `together:` plus immediate `spawn call target into destination`, `spawn call target digit into destination` with one ASCII decimal `digit`, `spawn call target digits into destination` with exactly two ASCII decimal characters and a nonzero first digit, or `spawn call target bright into destination`), and 013 detects only a canonical ordinary-Whole `choose same` / `choose less` / exact `choose bright:` / exact `choose dim:` / exact `choose not bright:` / exact `choose not dim:` nested yield (dual-compare rebuild)
- ADR-125 additionally recognizes only the immediate exact-`dim` Truth child `spawn call target dim into destination`; full release, seed identity, and packaged consumer verification pass. Exact empty Text is separately ADR-126.
- ADR-126 additionally recognizes only the immediate exact-empty-Text child `spawn call target "" into destination`; its valid target is an ordinary one-Text-parameter weave, not a task-frame expansion. Targeted red/green, direct, valid, predecessor, boundary, priority, product/self-host, tracker, full release, seed identity, and packaged consumer verification pass.
- Forge SPEAK capture on failure and verify merge  
- Yield-in-truth-choose fail-closed (AE-SEED-013)  
- Task checkpoint required (AE-SEED-015)  

### 5.2 M32a/M32b verified-execution evidence (ADR-104/105)

`aether bench` selects only the embedded `welcome`, `arena-buffer`, and
`task-loop` workload sources. It product-seed-compiles each selected workload
once, explicitly verifies its AETH, and records bounded raw local samples of
`verify + decode + execute` with empty grants. Unprofiled reports remain v1.
M32b's explicit non-secret `--profile` produces v2 with a safe environment
fingerprint and checked stdout SHA-256; `aether bench compare` reads only two
explicit 256 KiB-capped strict v2 data files and requires equal profile,
environment, selection, source, and behavior before reporting medians. It never
executes input reports, sources, or artifacts. There is no caller-supplied
program, native/JIT path, network/process/model authority, performance
threshold, hardware/toolchain attestation, or broad speed claim.

### 5.3 F-NATIVE (M35a → M35j)

C pilot → locals → SPEAK/multi-weave → host-cc dual-exec → object → LLVM IR →
LLVM object → native exe → toolchain probe/hermetic → **operator `--target`
matrix (host dual-run; cross link-only)**

### 5.4 F-REGISTRY (M24a → M24i)

Offline pin → HMAC signed → Ed25519/HTTPS → rotation → multi-root policy →
root certs → multi-level chains → **explicit CLI root/certified-key setup** →
X.509-lite → **CA store + chain verify + cache-gate validation**

### 5.5 M25 local source-package publication (ADR-107)

`aether pkg pack|verify|publish|install|verify-cache` packages exactly one
complete locked project into a transparent `aether.package/v1` directory bundle.
It binds raw manifest/unit bytes, rejects hostile paths, symlinks, nonregular
and unlisted bundle files, bounds input size, stages output, preserves existing
targets, and gives an explicit local cache a deterministic collision-safe
identity. Direct/cache installation creates a normal locked project; core
evidence builds and runs two independent M22 workspace consumers of one
installed package. M25 adds no network/resolver/signing/guest authority.

### 5.6 Recent tip commits (illustrative)

| Commit | Slice |
| --- | --- |
| `ec5e33c` | ADR-102 lexical seed-SPEAK tab / legacy-`fn` pilot |
| `f294701` | M24i CA-store / M35j target-flow operator hardening |
| `12a2181` | ADR-098–101 multi-code SPEAK, targets, CA store, checkpoint |
| `1e2fa31` | ADR-094–097 SPEAK empty pilot, native probe, X.509-lite, task inventory |
| `b5305cf` | ADR-090–093 forge SPEAK capture, native exe, multi-level certs, checkpoints |
| `f03e310` | ADR-086–089 SPEAK matrix, LLVM object, root certs, task reserve |

**Previous release-verified implementation:** ADR-124 extends the direct `AE-SEED-011`
seed-SPEAK witness to an ordinary total-Whole root `together:` line only when
its immediately following four-space child is the exact-Truth-literal
`spawn call target bright into destination` form with a destination suffix. It
preserves ADR-121 zero-argument, ADR-122 one-digit, and ADR-123 positive
two-digit behavior while keeping `dim`, names, general Truth, multi-argument,
and full M7 semantic forms outside the new witness. A later-declared checkpointed
one-Truth task target remains seed/bootstrap-identical, verified, and executable;
non-task-target, wrong-parameter, erroring-parent, and missing-world priority
boundaries remain outside or above the scanner. Product forge preserves the
exact seed packet with `origin: seed-speak`; full M7 nesting, task identity,
argument, destination, effect/result, resource, ownership, and scheduler
diagnostics remain with the full compiler. Targeted red/green, direct, valid,
boundary, priority, product/self-host, and tracker evidence pass. The 2026-08-14
ADR-124 zero-warning release gate also passes: pack v5.0.1, 355 Markdown files /
1,410 local links, four-way seed identity
`2F310B043653185DBE62ED61DB4D78A0DD8F2B0D20DD4B978DE634E05CF2ED68`, and a
440-file technical-preview package plus `SHA-256SUMS` with independent consumer
verification. The full seed SPEAK conformance matrix and seed-native multi-file
remain residual.

**Prior release-verified implementation:** ADR-125 extends the
same direct `AE-SEED-011` witness to the exact-Truth-literal
`spawn call target dim into destination` form only, with a destination suffix.
It preserves the ADR-124 `bright` witness and all prior Whole forms. Names,
`not dim`, general Truth, multiple arguments, and full M7 semantic forms remain
outside the new seed witness. A later-declared checkpointed one-Truth task target
remains seed/bootstrap-identical, verified, and executable; product forge
preserves the seed packet, while wrong-parameter, erroring-parent, and
missing-world priority behavior retain their prior authority. Targeted red/green,
direct, valid, predecessor, boundary, priority, product/self-host, and tracker
evidence pass. The 2026-08-14 ADR-125 zero-warning release gate also passes:
pack v5.0.1, 357 Markdown files / 1,427 local links, four-way seed identity
`110D8FF718D7FF578903C75398CB465C6499F5CBF2995F7C5617B4802DEFD6DB`, and a
442-file technical-preview package plus `SHA-256SUMS` with independent consumer
verification. Full seed SPEAK conformance and seed-native multi-file remain
residual.

**Current release-verified implementation:** ADR-129 adds a separate,
scalar-framed three-unit source-bundle route to the verified seed
`compile_bundle` weave. The profile parses exactly foundation -> bridge -> entry
inside Aether code, validates two exact import edges and source boundaries,
rewrites both qualified helper-call boundaries to established M11 mangled names,
and matches the bootstrap M11 elaboration reference byte-for-byte. It runs to
exit 84 and external `forge-bundle` produces the same verified artifact. It adds
no general module-graph, resolver, package, source syntax, AETH, VM, or
guest-capability claim. Hostile framing, paths, worlds, imports, order, calls,
and resource forms reject with one `AE-SEED-016` seed-SPEAK packet. ADR-128 v1
remains release verified. The release gate passed 48 seed self-host tests,
32-example identity, four-way seed identity
`552128E5F8AFB4F1584B4BBC5A83D5AEDBA241AE4CCEB814B3C48CD0F419C2B0`, a
457-file package, isolated consumer verification, and the unlisted-file tamper
probe.

**Earlier release-verified implementation:** ADR-126 extends the
same direct `AE-SEED-011` witness only to the exact empty-Text
`spawn call target "" into destination` form, with a nonempty destination
suffix. It preserves the ADR-124 `bright`, ADR-125 `dim`, and prior Whole
forms. The valid source target is a later-declared ordinary
`weave worker [message: Text] -> Whole`, not a Text task-frame parameter.
Nonempty/escaped Text, Bytes, names, multiple arguments, M7 semantic forms, and
full parser/type ownership remain outside the seed witness. Targeted red/green,
direct, valid, predecessor, boundary, priority, product/self-host, tracker, and
45-test full seed-self-host evidence pass. The 2026-08-14 zero-warning release
gate also passes: pack v5.0.1, 359 Markdown files / 1,447 local links, four-way
seed identity `D0D17756F587709BC85E323E0545281E304362288B687BAA0D48D8C334E18AFB`,
and a 444-file technical-preview package plus `SHA-256SUMS` with independent
consumer verification.

---

## 6. Maturity scorecard

| Domain | Maturity | Notes |
| --- | --- | --- |
| Seed self-host | **High** | Byte-identical multi-generation |
| Product vs bootstrap independence | **High** | Default product; residual oracle documented |
| Diagnostics honesty | **Medium–High** | Host strong; seed SPEAK pilot expanding |
| Multi-module | **Medium** | Host elaborate works; exact v1 two-unit and v2 transitive-three-unit profiles do not establish general seed-native modules |
| Native lower | **Medium** | Useful pilot; host-dependent |
| Registry | **Medium** | Offline + signed + lite CA; not full PKI |
| Task concurrency | **Medium** | M19e solid; no handles/timeouts/parallel |
| Foreign ABI | **Low–Medium** | Bounded pilot only |
| Verified-execution performance evidence | **Medium** | M32a fixed-corpus reports plus M32b strict profile-bound local comparison; no collected pinned baseline, attestation, or comparative speed claim |
| Local package ecosystem | **Medium** | M25 transparent locked source bundles, collision-safe cache, explicit install, and two-consumer reuse; no resolver or publisher identity |
| Package versioning | **Stable** | 0.37.0 toolchain contract; language/AETH surface unchanged |

---

## 7. Explicit non-goals (current law)

- Ambient guest network / shell / model calls  
- OS-thread parallel runtime as product claim  
- Task handles / timeouts as implemented features  
- Full RFC 5280 X.509 DER  
- Bundled hermetic cross-compile sysroot  
- Claiming seed SPEAK complete for every AE-SEED code  
- Claiming general seed-native multi-file forge

---

## 8. Recommended next increments

Ordered for dependency honesty (`CONST-DEP-001`):

1. **BARP:** select one broader `AE-SEED-011` or `AE-SEED-013` form with an explicit false-positive boundary, or a separately designed next bounded seed-native multi-file profile
2. **M32 evidence operation:** collect a human-declared pinned local baseline and candidate report under one M32b profile before any scoped performance-improvement claim
3. **F-NATIVE M35k+:** optional bundled/hermetic tool path when operators need reproducibility
4. **F-REGISTRY:** RFC 5280-shaped DER only if human re-authorizes beyond X.509-lite
5. **Task runtime:** implementable ADRs for handles and/or timeouts under ADR-081 invariants
6. **M25 follow-on only by ADR:** source-package signing/provenance, resolver/ranges, assets, or remote distribution must not be inferred from the local workflow

---

## 9. How to re-verify

```powershell
# Constitution pack
pwsh -File "..\..\AGENTS Constitution\tools\verify-pack.ps1"

# Full project gate
pwsh -File tools\aether-gate.ps1 -Mode full
# expected: GATE PASS mode=full

# Optional packaging gate
pwsh -File tools\aether-gate.ps1 -Mode release
```

---

## 10. Document map

| Doc | Role |
| --- | --- |
| [MANIFEST.md](../../MANIFEST.md) | Executable product contract |
| [CORE_CLAIMS.md](CORE_CLAIMS.md) | Proven vs residual claims |
| [ROADMAP.md](ROADMAP.md) | Track order + human backlog |
| [SEED_PROFILE.md](SEED_PROFILE.md) | Seed emission honesty |
| [FORGE_CONTRACT.md](FORGE_CONTRACT.md) | Host forge ABI |
| [DESIGN-BARP-001](DESIGN-BARP-001-BOOTSTRAP-AUTHORITY-REDUCTION.md) | BARP program |
| [DELIVERY_REPORT-2026-08-11-M25-LOCAL-PACKAGE-PUBLICATION.md](../historical%20docs/DELIVERY_REPORT-2026-08-11-M25-LOCAL-PACKAGE-PUBLICATION.md) | M25 0.37 delivery / full and release evidence |
| [DELIVERY_REPORT-2026-08-11-BARP-UNKNOWN-CALL-SPEAK.md](../historical%20docs/DELIVERY_REPORT-2026-08-11-BARP-UNKNOWN-CALL-SPEAK.md) | ADR-110 direct seed diagnostic delivery / current verification status |
| [DELIVERY_REPORT-2026-08-11-BARP-ROOT-YIELD-UNKNOWN-CALL-SPEAK.md](../historical%20docs/DELIVERY_REPORT-2026-08-11-BARP-ROOT-YIELD-UNKNOWN-CALL-SPEAK.md) | ADR-111 root-yield seed diagnostic delivery / current verification status |
| [DELIVERY_REPORT-2026-08-13-BARP-LESS-CHOOSE-SPEAK.md](../historical%20docs/DELIVERY_REPORT-2026-08-13-BARP-LESS-CHOOSE-SPEAK.md) | ADR-112 less-choose seed diagnostic delivery / current verification status |
| [DELIVERY_REPORT-2026-08-14-BARP-LITERAL-TRUTH-CHOOSE-SPEAK.md](../historical%20docs/DELIVERY_REPORT-2026-08-14-BARP-LITERAL-TRUTH-CHOOSE-SPEAK.md) | ADR-113 literal-Truth seed diagnostic delivery / current verification status |
| [DELIVERY_REPORT-2026-08-14-BARP-UNARY-LITERAL-TRUTH-CHOOSE-SPEAK.md](../historical%20docs/DELIVERY_REPORT-2026-08-14-BARP-UNARY-LITERAL-TRUTH-CHOOSE-SPEAK.md) | ADR-114 unary-literal Truth seed diagnostic delivery / current verification status |
| [DELIVERY_REPORT-2026-08-14-BARP-WHOLE-TRUTH-YIELD-SPEAK.md](../historical%20docs/DELIVERY_REPORT-2026-08-14-BARP-WHOLE-TRUTH-YIELD-SPEAK.md) | ADR-115 Whole Truth-literal yield seed diagnostic delivery / current verification status |
| [DELIVERY_REPORT-2026-08-21-BARP-M23-COMPTIME-CALL-SOURCE-ORDER-SPEAK.md](../historical%20docs/DELIVERY_REPORT-2026-08-21-BARP-M23-COMPTIME-CALL-SOURCE-ORDER-SPEAK.md) | ADR-127 source-order integrity delivery / final release evidence |
| [DELIVERY_REPORT-2026-08-21-BARP-SEED-NATIVE-WHOLE-LIBRARY-BUNDLE-PROFILE.md](../historical%20docs/DELIVERY_REPORT-2026-08-21-BARP-SEED-NATIVE-WHOLE-LIBRARY-BUNDLE-PROFILE.md) | ADR-128 bounded seed-native bundle delivery / final release evidence |
| [DELIVERY_REPORT-2026-08-21-BARP-SEED-NATIVE-TRANSITIVE-LIBRARY-CHAIN-PROFILE.md](../historical%20docs/DELIVERY_REPORT-2026-08-21-BARP-SEED-NATIVE-TRANSITIVE-LIBRARY-CHAIN-PROFILE.md) | ADR-129 bounded transitive-chain delivery / final release evidence |
| [AUDIT_REPORT-2026-08-11-FULL-PROJECT.md](../historical%20docs/AUDIT_REPORT-2026-08-11-FULL-PROJECT.md) | This audit stamp |
| ADRs 043–129 | Maturity decision trail |

---

*End of PROGRESS_REPORT-FULL-PROJECT.md*
