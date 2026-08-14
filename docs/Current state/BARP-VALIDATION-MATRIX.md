# BARP validation matrix — seed diagnostic authority reduction

| ID | Check | Expected |
| --- | --- | --- |
| BARP-SPEAK-001 | Empty source | Seed SPEAK `AE-SEED-005`, `origin: seed-speak`, blank Bytes result |
| BARP-SPEAK-002 | Missing `world` | Seed SPEAK `AE-SEED-006` |
| BARP-SPEAK-003 | Non-main weave without `main` | Seed SPEAK `AE-SEED-004` |
| BARP-SPEAK-004 | Raw `import unit` in a single source | Seed SPEAK `AE-SEED-012` |
| BARP-SPEAK-005 | Input starts with or follows a line feed with ASCII tab | Seed SPEAK `AE-SEED-003`; bounded tab-indentation pilot |
| BARP-SPEAK-006 | Input starts with or follows a line feed with `fn ` | Seed SPEAK `AE-SEED-007`; bounded top-level legacy-`fn` pilot |
| BARP-SPEAK-007 | Valid source text containing escaped `\\nfn` inside a Text literal | Does not trigger the lexical `fn` pilot |
| BARP-SPEAK-008 | Seed source rebuild through bootstrap, product, and forge | Byte-identical to checked-in `seed/aether_seed.aeth` |
| BARP-SPEAK-009 | Full conformance tracker | Remains `false`; no diagnostic-parity claim |
| BARP-SPEAK-010 | Tab plus legacy `fn ` at a source-line start | Stable priority emits `AE-SEED-003`; lower-priority pilot does not replace it |
| BARP-SPEAK-011 | Canonical reserved-task prefix after any ASCII-space indentation | Seed SPEAK `AE-SEED-014` for `timeout `, `task handle `, `handle task `, `parallel together`, or `together parallel` |
| BARP-SPEAK-012 | Valid `speak` Text literal containing `timeout ` | Does not trigger the line-aware reserved-task pilot |
| BARP-SPEAK-013 | Missing `world` plus a reserved-task prefix | Existing higher-priority seed pilot remains `AE-SEED-006` |
| BARP-SPEAK-014 | Canonical top-level task weave with no checkpoint before the next top-level line | Seed SPEAK `AE-SEED-015`, `origin: seed-speak`, blank Bytes result |
| BARP-SPEAK-015 | Canonical top-level task weave with no checkpoint at end of source | Seed SPEAK `AE-SEED-015` |
| BARP-SPEAK-016 | Valid task while body whose first nested statement is exact `checkpoint` | Does not SPEAK; seed emits verified AETH v12 |
| BARP-SPEAK-017 | `checkpointed`, `checkpoint later`, or `speak "checkpoint"` inside a task | Does not satisfy the pilot; Seed SPEAK `AE-SEED-015` |
| BARP-SPEAK-018 | Reserved task surface plus a missing task checkpoint | Existing higher-priority seed pilot remains `AE-SEED-014` |
| BARP-SPEAK-019 | Product-path missing/typo checkpoint diagnostic packet | `AE-SEED-015`, `origin: host-preflight` |
| BARP-SPEAK-020 | Canonical ordinary `weave … -> Whole:` with an indented `yield "…"` line | Seed SPEAK `AE-SEED-010`, `origin: seed-speak`, blank Bytes result |
| BARP-SPEAK-021 | Valid `weave … -> Text:` with an indented `yield "…"` line | Does not trigger the Whole-result pilot; seed emits verified AETH |
| BARP-SPEAK-022 | Missing `world` plus an otherwise canonical Whole Text-literal yield | Existing higher-priority seed pilot remains `AE-SEED-006` |
| BARP-SPEAK-023 | Product-path canonical Whole Text-literal yield diagnostic packet | `AE-SEED-010`, `origin: seed-speak`; forge packet merge preserves this exact bounded witness |
| BARP-SPEAK-024 | Product-path exact Whole Truth-literal yield diagnostic packet | `AE-SEED-010`, `origin: seed-speak`; the broader type family remains outside the pilot |
| BARP-SPEAK-025 | Canonical ordinary `weave … -> Whole:` with `choose same …:` and an indented branch `yield` | Seed SPEAK `AE-SEED-013`, `origin: seed-speak`, blank Bytes result |
| BARP-SPEAK-026 | `choose same` that revises then reaches a root-level `yield` | Does not trigger the nested-yield pilot; seed emits verified AETH |
| BARP-SPEAK-027 | Resource `choose allocate` / `append` / `at` with permitted nested yields | Does not trigger the truth-condition pilot; seed emits verified AETH |
| BARP-SPEAK-028 | Missing `world` plus an otherwise canonical invalid `choose same` yield | Existing higher-priority seed pilot remains `AE-SEED-006` |
| BARP-SPEAK-029 | Product-path canonical truth-choose nested-yield diagnostic packet | `AE-SEED-013`, `origin: host-preflight`; the full product safety preflight remains in force |
| BARP-SPEAK-030 | Canonical ordinary `Whole` `bind … <- call nope 1` with no matching top-level declaration header | Seed SPEAK `AE-SEED-011`, `origin: seed-speak`, blank Bytes result |
| BARP-SPEAK-031 | Canonical ordinary `Whole` call to a matching top-level ordinary weave declared after the call | Does not trigger the pilot; seed emits verified AETH |
| BARP-SPEAK-032 | Text literal containing call-shaped characters | Does not trigger the pilot; seed emits verified AETH |
| BARP-SPEAK-033 | Missing `world` plus an otherwise canonical unknown call | Existing higher-priority seed pilot remains `AE-SEED-006` |
| BARP-SPEAK-034 | Product-path canonical unknown-call diagnostic packet | `AE-SEED-011`, `origin: seed-speak`; forge packet merge preserves this exact bounded witness |
| BARP-SPEAK-035 | Canonical ordinary `Whole` call to a matching top-level `host weave` declaration | Does not trigger the pilot; seed emits verified AETH and leaves host call semantics to the full compiler |
| BARP-SPEAK-036 | Canonical ordinary `Whole` root `yield call nope 1` with no matching top-level declaration header | Seed SPEAK `AE-SEED-011`, `origin: seed-speak`, blank Bytes result |
| BARP-SPEAK-037 | Canonical ordinary `Whole` root `yield call helper 41` with a matching ordinary weave declared after the call | Does not trigger the pilot; seed emits verified AETH |
| BARP-SPEAK-038 | Canonical ordinary `Whole` root `yield call whole_inc 41` with a matching `host weave` declaration | Does not trigger the pilot; seed emits verified AETH and leaves host call semantics to the full compiler |
| BARP-SPEAK-039 | Text literal containing root-yield-call-shaped characters | Does not trigger the pilot; seed emits verified AETH |
| BARP-SPEAK-040 | Missing `world` plus an otherwise canonical root-yield unknown call | Existing higher-priority seed pilot remains `AE-SEED-006` |
| BARP-SPEAK-041 | Product-path canonical root-yield unknown-call diagnostic packet | `AE-SEED-011`, `origin: seed-speak`; forge packet merge preserves this exact bounded witness |
| BARP-SPEAK-042 | Canonical ordinary `weave … -> Whole:` with `choose less …:` and a deeper branch `yield` | Seed SPEAK `AE-SEED-013`, `origin: seed-speak`, blank Bytes result |
| BARP-SPEAK-043 | `choose less` that revises then reaches a root-level `yield` | Does not trigger the nested-yield pilot; seed artifact verifies and matches bootstrap |
| BARP-SPEAK-044 | Missing `world` plus an otherwise canonical invalid `choose less` yield | Existing higher-priority seed pilot remains `AE-SEED-006` |
| BARP-SPEAK-045 | Product-path canonical `choose less` nested-yield diagnostic packet | `AE-SEED-013`, `origin: host-preflight`; ADR-070 retains the broader pre-forge safety boundary |
| BARP-SPEAK-046 | Canonical ordinary `weave … -> Whole:` with exact `choose bright:` and a deeper branch `yield` | Seed SPEAK `AE-SEED-013`, `origin: seed-speak`, blank Bytes result |
| BARP-SPEAK-047 | Canonical ordinary `weave … -> Whole:` with exact `choose dim:` and a deeper branch `yield` | Seed SPEAK `AE-SEED-013`, `origin: seed-speak`, blank Bytes result |
| BARP-SPEAK-048 | Exact `choose bright:` / `choose dim:` that revise then reach a root-level `yield` | Does not trigger the literal nested-yield pilot; seed artifact verifies, matches bootstrap, and exits with the expected result |
| BARP-SPEAK-049 | Valid bare Truth variable or `choose not dim:` branch that revises then reaches a root-level `yield` | Does not trigger the literal pilot; seed artifact verifies and matches bootstrap |
| BARP-SPEAK-050 | Missing `world` plus an otherwise canonical literal Truth invalid yield | Existing higher-priority seed pilot remains `AE-SEED-006` |
| BARP-SPEAK-051 | Product-path canonical literal Truth nested-yield diagnostic packet | `AE-SEED-013`, `origin: host-preflight`; ADR-070 retains the broader pre-forge safety boundary |
| BARP-SPEAK-052 | Canonical ordinary `weave … -> Whole:` with exact `choose not bright:` and a deeper branch `yield` | Seed SPEAK `AE-SEED-013`, `origin: seed-speak`, blank Bytes result |
| BARP-SPEAK-053 | Canonical ordinary `weave … -> Whole:` with exact `choose not dim:` and a deeper branch `yield` | Seed SPEAK `AE-SEED-013`, `origin: seed-speak`, blank Bytes result |
| BARP-SPEAK-054 | Exact `choose not bright:` / `choose not dim:` that revise then reach a root-level `yield` | Does not trigger the unary-literal nested-yield pilot; seed artifact verifies, matches bootstrap, and exits with the expected result |
| BARP-SPEAK-055 | Valid `choose not flag:` branch that revises then reaches a root-level `yield` | Does not trigger the exact unary-literal pilot; seed artifact verifies and matches bootstrap |
| BARP-SPEAK-056 | Missing `world` plus an otherwise canonical unary-literal Truth invalid yield | Existing higher-priority seed pilot remains `AE-SEED-006` |
| BARP-SPEAK-057 | Product-path canonical unary-literal Truth nested-yield diagnostic packet | `AE-SEED-013`, `origin: host-preflight`; ADR-070 retains the broader pre-forge safety boundary |
| BARP-SPEAK-058 | Canonical ordinary `weave … -> Whole:` with exact `yield bright` | Seed SPEAK `AE-SEED-010`, `origin: seed-speak`, blank Bytes result |
| BARP-SPEAK-059 | Canonical ordinary `weave … -> Whole:` with exact `yield dim` | Seed SPEAK `AE-SEED-010`, `origin: seed-speak`, blank Bytes result |
| BARP-SPEAK-060 | `Truth`-return weave with exact `yield bright` / `yield dim`, plus a Whole `main` | Does not trigger the Whole-only pilot; seed artifact verifies, matches bootstrap, and exits with the expected result |
| BARP-SPEAK-061 | Missing `world` plus an otherwise canonical Whole Truth-literal yield | Existing higher-priority seed pilot remains `AE-SEED-006` |
| BARP-SPEAK-062 | Product-path exact Whole Truth-literal yield packet | `AE-SEED-010`, `origin: seed-speak`; forge packet merge preserves the bounded witness |
| BARP-SPEAK-063 | Canonical ordinary `Whole` `bind result <- call nope` with an end-of-line zero-argument target and no declaration header | Seed SPEAK `AE-SEED-011`, `origin: seed-speak`, blank Bytes result |
| BARP-SPEAK-064 | Canonical ordinary `Whole` root `yield call nope` with an end-of-line zero-argument target and no declaration header | Seed SPEAK `AE-SEED-011`, `origin: seed-speak`, blank Bytes result |
| BARP-SPEAK-065 | Canonical zero-argument bind/root-yield call to an ordinary weave declared after the call | Does not trigger the pilot; seed artifact verifies, matches bootstrap, and exits with the expected result |
| BARP-SPEAK-066 | Text literal containing a zero-argument bind-call-shaped sequence | Does not trigger the pilot; seed artifact verifies |
| BARP-SPEAK-067 | Missing `world` plus an otherwise canonical zero-argument unknown call | Existing higher-priority seed pilot remains `AE-SEED-006` |
| BARP-SPEAK-068 | Product-path canonical zero-argument unknown-call packet | `AE-SEED-011`, `origin: seed-speak`; forge packet merge preserves the bounded witness |
| BARP-SPEAK-069 | Canonical ordinary `Whole` root `revise result <- call nope 41` with no matching top-level declaration header | Seed SPEAK `AE-SEED-011`, `origin: seed-speak`, blank Bytes result |
| BARP-SPEAK-070 | Canonical ordinary `Whole` root `revise result <- call nope` with an end-of-line zero-argument target and no declaration header | Seed SPEAK `AE-SEED-011`, `origin: seed-speak`, blank Bytes result |
| BARP-SPEAK-071 | Canonical root revise calls to ordinary weaves declared after the call, for argument-bearing and zero-argument tails | Does not trigger the pilot; seed artifact verifies, matches bootstrap, and exits with the expected result |
| BARP-SPEAK-072 | Text literal containing a revise-call-shaped sequence | Does not trigger the pilot; seed artifact verifies |
| BARP-SPEAK-073 | Missing `world` plus an otherwise canonical root revise-call unknown target | Existing higher-priority seed pilot remains `AE-SEED-006` |
| BARP-SPEAK-074 | Product-path canonical root revise-call unknown-target packet | `AE-SEED-011`, `origin: seed-speak`; forge packet merge preserves the bounded witness |
| BARP-SPEAK-075 | Canonical ordinary `Whole` root `speak call nope 41` with no matching top-level declaration header | Seed SPEAK `AE-SEED-011`, `origin: seed-speak`, blank Bytes result |
| BARP-SPEAK-076 | Canonical ordinary `Whole` root `speak call nope` with an end-of-line zero-argument target and no declaration header | Seed SPEAK `AE-SEED-011`, `origin: seed-speak`, blank Bytes result |
| BARP-SPEAK-077 | Canonical root speak calls to later-declared Text-result weaves, for argument-bearing and zero-argument tails | Does not trigger the pilot; seed artifact verifies, matches bootstrap, and exits with the expected result |
| BARP-SPEAK-078 | Root speak Text literal containing a speak-call-shaped sequence | Does not trigger the pilot; seed artifact verifies |
| BARP-SPEAK-079 | Missing `world` plus an otherwise canonical root speak-call unknown target | Existing higher-priority seed pilot remains `AE-SEED-006` |
| BARP-SPEAK-080 | Product-path canonical root speak-call unknown-target packet | `AE-SEED-011`, `origin: seed-speak`; forge packet merge preserves the bounded witness |
| BARP-SPEAK-081 | Canonical ordinary `Whole` root `handle call nope 41 into result otherwise error into code` and zero-argument `handle call nope into result otherwise error into code`, each with no matching top-level declaration header | Seed SPEAK `AE-SEED-011`, `origin: seed-speak`, blank Bytes result |
| BARP-SPEAK-082 | Canonical root handle calls to later-declared `raises Whole` weaves, for argument-bearing and zero-argument tails | Does not trigger the pilot; seed artifact verifies, matches bootstrap, and exits with the expected result |
| BARP-SPEAK-083 | Root speak Text literal containing a complete handle-call-shaped sequence | Does not trigger the handle pilot; seed artifact verifies |
| BARP-SPEAK-084 | Root `handle call nope` with no `into` / `otherwise error into` tail | Does not trigger the bounded unknown-target pilot; full syntax diagnostics retain authority |
| BARP-SPEAK-085 | Missing `world` plus an otherwise canonical root handle-call unknown target | Existing higher-priority seed pilot remains `AE-SEED-006` |
| BARP-SPEAK-086 | Product-path canonical root handle-call unknown-target packet | `AE-SEED-011`, `origin: seed-speak`; forge packet merge preserves the bounded witness |
| BARP-SPEAK-087 | Canonical erroring `Whole` root `forward call nope 41` and zero-argument `forward call nope`, each under a literal `-> Whole raises Whole:` header marker with no matching top-level declaration header | Seed SPEAK `AE-SEED-011`, schema `aether.seed-error/v1`, `origin: seed-speak`, position `1:1`, blank Bytes result |
| BARP-SPEAK-088 | Canonical root forward calls to later-declared `raises Whole` weaves, for argument-bearing and zero-argument tails | Does not trigger the pilot; seed artifact verifies, matches bootstrap, and exits with the expected result |
| BARP-SPEAK-089 | Erroring-weave `speak` Text literal containing a forward-call-shaped sequence | Does not trigger the forward pilot; seed artifact verifies |
| BARP-SPEAK-090 | Root `forward call nope` under a total `-> Whole:` caller | Does not trigger the erroring-caller pilot; full M4 semantic diagnostics retain authority |
| BARP-SPEAK-091 | Erroring-Whole root `forward call` with no target | Does not trigger the bounded unknown-target pilot; full syntax diagnostics retain authority |
| BARP-SPEAK-092 | Missing `world` plus an otherwise canonical erroring-Whole root forward-call unknown target | Existing higher-priority seed pilot remains `AE-SEED-006` |
| BARP-SPEAK-093 | Product-path canonical root forward-call unknown-target packet | `AE-SEED-011`, `origin: seed-speak`; forge packet merge preserves the bounded witness |
| BARP-SPEAK-094 | Canonical ordinary total-Whole root `together:` followed immediately by four-space zero-argument `spawn call nope into result`, with no matching top-level declaration header | Seed SPEAK `AE-SEED-011`, schema `aether.seed-error/v1`, `origin: seed-speak`, position `1:1`, blank Bytes result |
| BARP-SPEAK-095 | Canonical root nursery spawn to a later-declared checkpointed task weave | Does not trigger the pilot; seed artifact verifies, matches bootstrap, and exits with the expected result |
| BARP-SPEAK-096 | Spawn-shaped Text literal, erroring-Whole parent nursery, intervening blank/non-spawn line, six-space descendant, or three-or-more-digit/negative/`dim`/name/general-Truth/multi-argument child | Do not trigger the literal total-root immediate four-space zero-argument, single-digit, positive-two-digit-Whole, or exact-`bright`-Truth-child pilots; full M7 diagnostics retain authority |
| BARP-SPEAK-097 | Root nursery `spawn call nope` with no `into` delimiter or no destination suffix | Does not trigger the bounded unknown-target pilot; full syntax diagnostics retain authority |
| BARP-SPEAK-098 | Missing `world` plus an otherwise canonical root-nursery spawn unknown target | Existing higher-priority seed pilot remains `AE-SEED-006` |
| BARP-SPEAK-099 | Product-path canonical root-nursery spawn unknown-target packet | `AE-SEED-011`, `origin: seed-speak`; forge packet merge preserves the bounded witness |
| BARP-SPEAK-100 | Canonical ordinary total-Whole root `together:` followed immediately by four-space `spawn call nope 3 into result`, with no matching top-level declaration header | Seed SPEAK `AE-SEED-011`, schema `aether.seed-error/v1`, `origin: seed-speak`, position `1:1`, blank Bytes result |
| BARP-SPEAK-101 | Canonical root nursery one-digit spawn to a later-declared checkpointed `task weave` with one Whole parameter | Does not trigger the pilot; seed artifact verifies, matches bootstrap, and exits `3` |
| BARP-SPEAK-102 | Immediate root nursery three-or-more-digit, negative, Truth, multi-argument, or missing-destination source; delayed/descendant child; or an erroring-Whole parent | Does not trigger the one-digit bounded unknown-target pilot; full M7 syntax/semantic diagnostics retain authority |
| BARP-SPEAK-103 | Missing `world` plus an otherwise canonical one-digit root-nursery spawn unknown target | Existing higher-priority seed pilot remains `AE-SEED-006` |
| BARP-SPEAK-104 | Product-path canonical one-digit root-nursery spawn unknown-target packet | `AE-SEED-011`, `origin: seed-speak`; forge packet merge preserves the bounded witness |
| BARP-SPEAK-105 | Canonical ordinary total-Whole root `together:` followed immediately by four-space `spawn call nope 42 into result`, with no matching top-level declaration header | Seed SPEAK `AE-SEED-011`, schema `aether.seed-error/v1`, `origin: seed-speak`, position `1:1`, blank Bytes result |
| BARP-SPEAK-106 | Canonical immediate positive two-digit root-nursery spawn unknown targets using lexical endpoints `10` and `99` | Both literals reach the ADR-123 target-existence witness; leading-zero literals retain earlier literal-error priority |
| BARP-SPEAK-107 | Canonical root nursery positive two-digit spawn to a later-declared checkpointed `task weave` with one Whole parameter | Does not trigger the pilot; seed artifact verifies, matches bootstrap, and exits `42` |
| BARP-SPEAK-108 | Immediate root nursery leading-zero, three-or-more-digit, signed, Truth, multi-argument, missing-destination, delayed, descendant, non-task-target, or erroring-Whole-parent source | Does not trigger the two-digit bounded unknown-target pilot; leading-zero forms fail literal validation before target resolution and full M7 syntax/semantic diagnostics retain authority |
| BARP-SPEAK-109 | Missing `world` plus an otherwise canonical positive two-digit root-nursery spawn unknown target | Existing higher-priority seed pilot remains `AE-SEED-006` |
| BARP-SPEAK-110 | Product-path canonical positive two-digit root-nursery spawn unknown-target packet | `AE-SEED-011`, `origin: seed-speak`; forge packet merge preserves the bounded witness |
| BARP-SPEAK-111 | Canonical ordinary total-Whole root `together:` followed immediately by four-space `spawn call nope bright into result`, with no matching top-level declaration header | Seed SPEAK `AE-SEED-011`, schema `aether.seed-error/v1`, `origin: seed-speak`, position `1:1`, blank Bytes result |
| BARP-SPEAK-112 | Canonical root nursery exact-`bright` spawn to a later-declared checkpointed `task weave` with one Truth parameter | Does not trigger the pilot; seed artifact verifies, matches bootstrap, and exits `42` |
| BARP-SPEAK-113 | Immediate root nursery `dim`, name, repeated-argument, missing-destination, delayed, descendant, non-task-target, wrong-parameter, or erroring-Whole-parent source | Does not trigger the exact-`bright` bounded unknown-target pilot; existing seed failures or full M7 syntax/semantic diagnostics retain authority |
| BARP-SPEAK-114 | Missing `world` plus an otherwise canonical exact-`bright` root-nursery spawn unknown target | Existing higher-priority seed pilot remains `AE-SEED-006` |
| BARP-SPEAK-115 | Product-path canonical exact-`bright` root-nursery spawn unknown-target packet | `AE-SEED-011`, `origin: seed-speak`; forge packet merge preserves the bounded witness |

## Boundary

This matrix covers only the named seed-SPEAK pilot conditions. It does not claim
seed-internal parser packets, full indentation diagnostics, complete legacy
syntax classification, source spans, parser parity, or seed-native multi-file
elaboration.

---

*End of BARP-VALIDATION-MATRIX.md.*
