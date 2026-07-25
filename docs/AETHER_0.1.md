# Aether 0.1 Historical Kernel

Status: historical; superseded by Aether 0.2.0 and then Aether 0.3.0 on 2026-07-25.

This document records the completed Stage 0 kernel boundary: one `main` weave,
immutable bindings, Text and Whole values, `speak`, and terminal `yield`. It is
not the current language specification and does not describe the current AETH
artifact format.

Aether 0.2.0 replaced the Stage 0 grammar with the Stage 1 language:
named weaves, Text/Whole/Truth values, structured control flow, mutable root
bindings, explicit Text `borrow`/`move`, calls, Unicode-safe text primitives,
and AETH v2. Aether 0.3.0 adds Bytes, AETH v3, and a forge boundary. The
current compiler and VM reject AETH v1 and AETH v2 artifacts.

See `AETHER_0.3.md` for the implemented specification and `docs/LEGACY.md` for
the preservation policy for historical material.
