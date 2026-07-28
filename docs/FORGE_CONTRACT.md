# Forge Contract

Status: implemented host ABI for Aether 0.4.0 Stage 3. Supports Seed-Profile
self-hosting proofs; does not grant host capabilities to artifacts.

## Command

    aether forge <compiler-artifact> <source-file> --output <artifact-file>

All paths are local filesystem paths. The command does not call Ollama, network
services, a shell, or another compiler.

## Required Compiler Artifact

The compiler artifact must first pass normal AETH v4 verification. That includes
the required runnable weave `main [] -> Whole:`. The forge bridge then locates a
named weave `compile` and requires this exact type and ownership shape:

    weave compile [borrow source: Text] -> Bytes:

The parameter identifier may differ internally, but there must be exactly one
borrowed Text parameter and the result must be Bytes. An owned Text parameter,
additional parameter, different result type, missing compile weave, invalid
artifact, or unsupported AETH version is rejected before the weave executes.

## Invocation Sequence

1. Read the compiler artifact and source file locally.
2. Verify the compiler artifact as AETH v4.
3. Check the compile weave ABI.
4. Invoke compile with the complete source file as one bounded Text argument.
5. Preserve any compiler weave stdout as diagnostic text on standard error.
6. Require the returned value to be Bytes.
7. Verify those Bytes as a complete AETH v4 artifact.
8. Confirm the output directory exists and write the verified artifact.

The host never parses, transforms, or generates the supplied source during forge
invocation. It only transports source Text in and verified artifact Bytes out.

## Capabilities

The invoked Aether artifact has no direct file, process, network, cloud AI,
Ollama, shell, or artifact-writing capability. Its observable effect is only
stdout and its typed yield value. The host owns input reading and final artifact
writing after verification.

## Seed-Profile Use

Stage 3 uses this contract for the checked-in seed compiler:

1. Bootstrap-compile `seed/aether_seed.ae` with the Rust core.
2. Forge that artifact against the same source.
3. Require byte-identical output and successful verification.
4. Forge a second generation and a distinct source variant (regression test).

Matching bytes under this ABI is the only accepted self-hosting evidence for the
Seed Profile. See [SEED_PROFILE.md](SEED_PROFILE.md).

## Current Limitation

Full-language self-hosting still requires an Aether compiler whose accepted
surface equals Aether 0.4, with the same reproducible comparison standard. The
Rust bootstrap compiler remains the complete implementation for the full
language.
