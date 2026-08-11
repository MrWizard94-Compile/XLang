# Forge Contract

Status: implemented host ABI for Aether 0.11.0 Stage 7/M4–M8. Supports complete
canonical Aether 0.11 Seed Profile self-hosting proofs; does not grant host
capabilities to artifacts.

## Command

    aether forge <compiler-artifact> <source-file> --output <artifact-file>

All paths are local filesystem paths. The command does not call model or network
services, a shell, or another compiler.

## Required Compiler Artifact

The compiler artifact must first pass normal supported AETH v4, v5, v6, v7, v8,
v9, v10, or v11 verification. That includes
the required runnable weave `main [] -> Whole:`. The forge bridge then locates a
named weave `compile` and requires this exact type and ownership shape:

    weave compile [borrow source: Text] -> Bytes:

The parameter identifier may differ internally, but there must be exactly one
borrowed Text parameter and the result must be Bytes. An owned Text parameter,
additional parameter, different result type, missing compile weave, invalid
artifact, or unsupported AETH version is rejected before the weave executes.

## Invocation Sequence

1. Read the compiler artifact and source file locally.
2. Verify the compiler artifact as supported AETH v4 through v11.
3. Check the compile weave ABI.
4. Invoke compile with the complete source file as one bounded Text argument.
5. Preserve any compiler weave stdout as diagnostic text on standard error.
6. Require the returned value to be Bytes.
7. Verify those Bytes as a complete supported AETH v4 through v11 artifact.
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

## Seed SPEAK diagnostic contract (ADR-082/086/090/094)

When the seed compiler fails, SPEAK lines of the form:

    AETHER_SEED_ERROR:{"schema":"aether.seed-error/v1",...}

are the preferred structured diagnostic channel. Host product preflights already
emit the same line format. **ADR-094/098:** seed.ae SPEAKs AE-SEED-004/005/006/012
pilot codes (origin `seed-speak`) and host merges SPEAK on forge **and** verify
failure. Full seed SPEAK matrix remains residual
(`seed_speak_emit_conformance_complete() == false`).
