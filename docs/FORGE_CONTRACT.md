# Forge Contract

Status: implemented host ABI for Aether 0.3.0 Stage 2, not a self-hosting claim.

## Command

    aether forge <compiler-artifact> <source-file> --output <artifact-file>

All paths are local filesystem paths. The command does not call Ollama, network
services, a shell, or another compiler.

## Required Compiler Artifact

The compiler artifact must first pass normal AETH v3 verification. That includes
the required runnable weave main [] -> Whole:. The forge bridge then locates a
named weave compile and requires this exact type and ownership shape:

    weave compile [borrow source: Text] -> Bytes:

The parameter identifier may differ internally, but there must be exactly one
borrowed Text parameter and the result must be Bytes. An owned Text parameter,
additional parameter, different result type, missing compile weave, invalid
artifact, or unsupported AETH version is rejected before the weave executes.

## Invocation Sequence

1. Read the compiler artifact and source file locally.
2. Verify the compiler artifact as AETH v3.
3. Check the compile weave ABI.
4. Invoke compile with the complete source file as one bounded Text argument.
5. Preserve any compiler weave stdout as diagnostic text on standard error.
6. Require the returned value to be Bytes.
7. Verify those Bytes as a complete AETH v3 artifact.
8. Confirm the output directory exists and write the verified artifact.

The host never parses, transforms, or generates the supplied source during forge
invocation. It only transports source Text in and verified artifact Bytes out.

## Capabilities

The invoked Aether artifact has no direct file, process, network, cloud AI,
Ollama, shell, or artifact-writing capability. Its observable effect is only
stdout and its typed yield value. The host owns input reading and final artifact
writing after verification.

## Current Limitation

Stage 2 makes it possible to prove a future Aether compiler artifact through a
small, inspectable ABI. It does not provide an Aether-written compiler yet. The
Rust bootstrap compiler remains the mechanism that compiles Aether source in
this release. Calling forge with a handcrafted or static compiler artifact is a
contract test, not self-hosting evidence.

A self-hosting release must include the Aether compiler source, its verified
compiler artifact, an invocation that compiles the compiler source, and
reproducible artifact comparison evidence.
