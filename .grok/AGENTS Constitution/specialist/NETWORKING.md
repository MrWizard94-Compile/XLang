# Networking and Protocols

```
Document: Networking and Protocols
Module ID: MOD-SPC-NET-001
Version: 1.1.0
Status: Binding
Authority: AGENTS.md
Applies When: Custom protocols, multiplayer, network packets, sync
Dependencies:
  - AGENTS.md
  - standards/SECURITY.md
  - standards/OBSERVABILITY.md
Overrides: None
```

---

## NET-PROTO-001 — Versioned Robust Protocols

* Custom packets/protocols must be versioned, validated, and robust against malformed or out-of-order messages.
* Include protocol version negotiation or compatibility checks when relevant.

## NET-AUTH-001 — Authoritative Source of Truth

* Prefer authoritative server-side (or single source of truth) logic.
* Client prediction only where necessary and documented.

## NET-DOC-001 — Full Protocol Documentation

* Document IDs, fields, serialization, expected flow, and failure modes.
* Handle desyncs, late joins, partial updates, and recovery cleanly.

## NET-CHATTY-001 — Hot Path Discipline

* Avoid chatty protocols on hot paths; batch where possible (`PERF-HOT-001`).

---

*Canonical home for networking discipline. Security input rules remain SEC-INPUT-001.*
