# Aether 0.1 Historical Kernel

Status: superseded by Aether 0.2.0 on 2026-07-25.

This document records the completed Stage 0 kernel boundary: one `main` weave,
immutable bindings, Text and Whole values, `speak`, and terminal `yield`. It is
not the current language specification and does not describe the current AETH
artifact format.

Aether 0.2.0 replaces the Stage 0 grammar with the complete Stage 1 language:
named weaves, Text/Whole/Truth values, structured control flow, mutable root
bindings, explicit Text `borrow`/`move`, calls, Unicode-safe text primitives,
and AETH v2. The current compiler and VM reject AETH v1 artifacts.

See `AETHER_0.2.md` for the implemented specification and `docs/LEGACY.md` for
the preservation policy for historical material.
