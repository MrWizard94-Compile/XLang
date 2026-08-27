# Forge Contract

Status: implemented host ABI for Aether 0.11.0 Stage 7/M4–M8. Supports complete
canonical Aether 0.11 Seed Profile self-hosting proofs; does not grant host
capabilities to artifacts.

## Command

    aether forge <compiler-artifact> <source-file> --output <artifact-file>
    aether forge-bundle <compiler-artifact> <bundle-file> --output <artifact-file>

All paths are local filesystem paths. The command does not call model or network
services, a shell, or another compiler.

## Required Compiler Artifact

The compiler artifact must first pass normal supported AETH v4 through v12
verification. That includes
the required runnable weave `main [] -> Whole:`. The forge bridge then locates a
named weave `compile` and requires this exact type and ownership shape:

    weave compile [borrow source: Text] -> Bytes:

The parameter identifier may differ internally, but there must be exactly one
borrowed Text parameter and the result must be Bytes. An owned Text parameter,
additional parameter, different result type, missing compile weave, invalid
artifact, or unsupported AETH version is rejected before the weave executes.

## Invocation Sequence

1. Read the compiler artifact and source file locally.
2. Verify the compiler artifact as supported AETH v4 through v12.
3. Check the compile weave ABI.
4. Invoke compile with the complete source file as one bounded Text argument.
5. Preserve any compiler weave stdout as diagnostic text on standard error.
6. Require the returned value to be Bytes.
7. Verify those Bytes as a complete supported AETH v4 through v12 artifact.
8. Confirm the output directory exists and write the verified artifact.

The host never parses, transforms, or generates the supplied source during forge
invocation. It only transports source Text in and verified artifact Bytes out.

## Capability Boundary

The invoked compiler artifact has no file, process, network, shell, or ambient
host authority. Forge owns all host I/O after verification. This contract is
unchanged by M2 resources, M4 effects, M5 comptime, M6 layout tables, M7
nurseries, or M8 pure host-call fixtures: guest code cannot open ambient
capabilities through forge.

## M8 pure host services (product run path)

`aether run` and library invoke helpers install a **capability-closed pure
fixture catalog** only:

| Host weave | Signature | Behavior |
| --- | --- | --- |
| `whole_inc` | `[value: Whole] -> Whole` | checked `value + 1` |
| `text_extent` | `[borrow message: Text] -> Whole` | UTF-8 byte length |

Missing or denied services fail closed with `AE-HOST-003`. No file, process,
network, shell, or model service is installed. The forge path still does not
expose I/O-bearing host services to the guest seed compiler.

## Seed Profile relationship

Product compilation embeds the checked-in seed artifact and uses this forge ABI
through `compile_with_seed`. Seed self-host proofs require multi-generation
byte identity under forge. See [SEED_PROFILE.md](SEED_PROFILE.md).

## Multi-source product path (ADR-075/078/082/086)

Product multi-file compilation uses the host multi-unit path:

1. Detect `aether.multi-source/v1` envelope (or project/workspace elaborate).  
2. Host-elaborate import graph into one single-world program.  
3. Seed-forge that single Text via the standard `compile` weave above.  

Seed does **not** natively parse multi-source envelopes
(`seed_native_multi_module_elaboration() == false`). A future seed-native
multi-file ABI would require a new forge weave signature and Seed Profile claim.

## Bounded seed-native source-bundle paths (ADR-128/129/130 / SBP-001/002/003)

`aether forge-bundle` is a separate closed ABI, not an extension of ordinary
`forge`. The verified compiler must expose:

```aether
weave compile_bundle [borrow bundle: Text] -> Bytes:
```

It receives exactly one caller-selected versioned bundle Text file. On this
path, the host does not decode framing, resolve an import, collect an export,
mangle a name, or rewrite source. The seed validates and elaborates either the
exact v1 two-unit ASCII/LF pure Whole library -> named entry profile (16,384
scalars per unit, 32,768 total source, 33,280 total wire), the exact v2
three-unit foundation -> bridge -> named entry profile (16,384 scalars per
unit, 49,152 total source, 49,920 total wire), or the exact v3 four-unit left
leaf + right leaf -> merge -> named entry profile (16,384 scalars per unit,
65,536 total source, 66,560 total wire). v2 has two fixed transitive import
edges; v3 has three fixed edges, two of which form the merge fan-in. Neither is
an arbitrary module graph. Profile failure SPEAKs `AE-SEED-016` and returns no
runnable artifact.

The profile does not alter the general M11/M22 multi-source rule above:
`seed_native_multi_module_elaboration() == false` remains true, and ordinary
projects, workspaces, raw imports, and `aether.multi-source/v1` use the
host-elaborated route. See [ADR-128](../historical%20docs/ADR-128-barp-seed-native-whole-library-bundle-profile.md),
[ADR-129](../historical%20docs/ADR-129-barp-seed-native-transitive-library-chain-profile.md),
[ADR-130](../historical%20docs/ADR-130-barp-seed-native-fanin-bundle-profile.md),
[SBP-002](../historical%20docs/DESIGN-SBP-002-SEED-NATIVE-TRANSITIVE-CHAIN-PROFILE.md),
[SBP-003](../historical%20docs/DESIGN-SBP-003-SEED-NATIVE-FANIN-PROFILE.md),
[the SBP-002 threat model](THREAT_MODEL-SBP-002-SEED-CHAIN.md), and
[the SBP-003 threat model](THREAT_MODEL-SBP-003-SEED-FANIN.md).

## Seed SPEAK diagnostic contract (ADR-082/086/090/094/098/102/103/106/108/109/110/111/112/113/114/115)

When the seed compiler fails, SPEAK lines of the form:

    AETHER_SEED_ERROR:{"schema":"aether.seed-error/v1",...}

are the preferred structured diagnostic channel. Host product preflights already
emit the same line format. **ADR-094/098/102/103/106/108/109/110/111/112/113/114/115/116/117/118/119/120/121/122/123/124/125/126:** seed.ae SPEAKs
AE-SEED-003/004/005/006/007/010/011/012/013/014/015 pilot codes (origin `seed-speak`);
003 and 007 are bounded line-start lexical checks, 014 is a canonical-lowercase
line-prefix guard for reserved task proposals, 015 is a canonical top-level
task-body scan that requires an exact indented `checkpoint` line, and 010 is a
canonical ordinary-Whole body scan for an indented `yield "..."` line or exact
`yield bright` / `yield dim` line. ADR-109/112/113/114
add only canonical ordinary-Whole `choose same` / `choose less` / exact
`choose bright:` / exact `choose dim:` / exact `choose not bright:` / exact
`choose not dim:` nested-yield witnesses for 013; the product preflight retains
broader 013 coverage. Host merges SPEAK on
forge **and** verify failure. ADR-110/111/116/117/118/119/120/121/122/123/124/125/126 add only canonical
total-Whole direct-bind, root-yield-call, root-revise-call, root-speak-call, and
delimiter-bounded root-handle-call target-existence checks plus an erroring-Whole
root-forward-call plus immediate root-nursery zero-argument, single-digit,
positive two-digit Whole, exact-`bright` Truth, exact-`dim` Truth, and exact
empty-Text spawn-call target-existence checks against canonical top-level declaration
headers (ordinary, export, host, foreign, or task). The witness accepts forward
ordinary headers, exact end-of-line zero-argument targets, fixed `into` /
`otherwise error into` handle delimiters, only the literal `-> Whole raises
Whole:` forward caller marker, and only a literal total root `together:` whose
immediate four-space child has either the zero-argument
`spawn call target into destination` shape, exactly one ASCII decimal digit,
exactly two ASCII decimal digits with a nonzero first digit, exact `bright`,
exact `dim`, or exact `"" into ` immediately after the opaque target with a
nonempty destination; it is not general name binding,
destination or terminality validation, full header parsing, Text-result
validation, M4 effect/resource validation, or call-kind approval. Full seed
SPEAK matrix remains residual
(`seed_speak_emit_conformance_complete() == false`).
