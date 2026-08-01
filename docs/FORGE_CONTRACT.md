# Forge Contract

Status: implemented host ABI for Aether 0.6.0 Stage 7. Supports complete
canonical Aether 0.6 Seed Profile self-hosting proofs; does not grant host
capabilities to artifacts.

## Command

    aether forge <compiler-artifact> <source-file> --output <artifact-file>

All paths are local filesystem paths. The command does not call model or network
services, a shell, or another compiler.

## Required Compiler Artifact

The compiler artifact must first pass normal supported AETH v4, v5, or v6 verification. That includes
the required runnable weave `main [] -> Whole:`. The forge bridge then locates a
named weave `compile` and requires this exact type and ownership shape:

    weave compile [borrow source: Text] -> Bytes:

The parameter identifier may differ internally, but there must be exactly one
borrowed Text parameter and the result must be Bytes. An owned Text parameter,
additional parameter, different result type, missing compile weave, invalid
artifact, or unsupported AETH version is rejected before the weave executes.

## Invocation Sequence

1. Read the compiler artifact and source file locally.
2. Verify the compiler artifact as supported AETH v4, v5, or v6.
3. Check the compile weave ABI.
4. Invoke compile with the complete source file as one bounded Text argument.
5. Preserve any compiler weave stdout as diagnostic text on standard error.
6. Require the returned value to be Bytes.
7. Verify those Bytes as a complete supported AETH v4, v5, or v6 artifact.
8. Confirm the output directory exists and write the verified artifact.

The host never parses, transforms, or generates the supplied source during forge
invocation. It only transports source Text in and verified artifact Bytes out.

## Capabilities

The invoked Aether artifact has no direct file, process, network, model,
shell, or artifact-writing capability. Its observable effect is only
stdout and its typed yield value. The host owns input reading and final artifact
writing after verification.

## Seed-Profile Use

Stage 7 uses this contract for the checked-in seed compiler:

1. Bootstrap-compile `seed/aether_seed.ae` with the Rust core.
2. Forge that artifact against the same source.
3. Require byte-identical output and successful verification.
4. Forge a second generation and a distinct source variant (regression test).
5. Forge multi-weave Seed Profile programs with `call` and require
   byte-identical bootstrap match plus a successful run.
6. Compare the prior canonical surface and the complete documented Aether 0.6
   M2 resource corpus against bootstrap, including immutable records, text
   escapes, a final source line without a terminator, arena exhaustion, buffer
   full, lookup fallback, Truth elements, and an access-bound helper.

Matching bytes under this ABI is the only accepted self-hosting evidence for the
Seed Profile. See [SEED_PROFILE.md](SEED_PROFILE.md).

## Current Limitation

Canonical Aether 0.6 source-emission parity is proven under this contract. This
does not claim parity for invalid-source diagnostics: the Rust bootstrap remains
the diagnostic authority. Any future language extension must meet the same
reproducible comparison standard before it joins the seed product path.
