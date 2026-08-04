# Forge Contract

Status: implemented host ABI for Aether 0.10.0 Stage 7/M4–M7. Supports complete
canonical Aether 0.10 Seed Profile self-hosting proofs; does not grant host
capabilities to artifacts.

## Command

    aether forge <compiler-artifact> <source-file> --output <artifact-file>

All paths are local filesystem paths. The command does not call model or network
services, a shell, or another compiler.

## Required Compiler Artifact

The compiler artifact must first pass normal supported AETH v4, v5, v6, v7, v8,
v9, or v10 verification. That includes
the required runnable weave `main [] -> Whole:`. The forge bridge then locates a
named weave `compile` and requires this exact type and ownership shape:

    weave compile [borrow source: Text] -> Bytes:

The parameter identifier may differ internally, but there must be exactly one
borrowed Text parameter and the result must be Bytes. An owned Text parameter,
additional parameter, different result type, missing compile weave, invalid
artifact, or unsupported AETH version is rejected before the weave executes.

## Invocation Sequence

1. Read the compiler artifact and source file locally.
2. Verify the compiler artifact as supported AETH v4 through v10.
3. Check the compile weave ABI.
4. Invoke compile with the complete source file as one bounded Text argument.
5. Preserve any compiler weave stdout as diagnostic text on standard error.
6. Require the returned value to be Bytes.
7. Verify those Bytes as a complete supported AETH v4 through v10 artifact.
8. Confirm the output directory exists and write the verified artifact.

The host never parses, transforms, or generates the supplied source during forge
invocation. It only transports source Text in and verified artifact Bytes out.

## Capability Boundary

The invoked compiler artifact has no file, process, network, shell, or ambient
host authority. Forge owns all host I/O after verification. This contract is
unchanged by M2 resources, M4 effects, M5 comptime, M6 layout tables, or M7
nurseries: guest code cannot open those capabilities through forge.

## Seed Profile relationship

Product compilation embeds the checked-in seed artifact and uses this forge ABI
through `compile_with_seed`. Seed self-host proofs require multi-generation
byte identity under forge. See [SEED_PROFILE.md](SEED_PROFILE.md).
