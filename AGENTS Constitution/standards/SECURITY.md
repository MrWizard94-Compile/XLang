# Security Standards

```
Document: Security Standards
Module ID: MOD-SEC-001
Version: 1.1.0
Status: Binding
Authority: AGENTS.md
Applies When: Untrusted input, auth, secrets, network surface, dependencies, deployment
Dependencies:
  - AGENTS.md
  - standards/DEPENDENCIES.md
Overrides: None
```

---

## SEC-INPUT-001 — Untrusted Input

* Treat all external or untrusted input as potentially malicious or malformed.
* Validate, sanitize, canonicalize, and authorize rigorously.
* Prefer allow-lists and strong typing.

## SEC-PRIV-001 — Least Privilege

* Apply least privilege to processes, accounts, tokens, and filesystem access.
* Avoid over-broad permissions “for convenience”.

## SEC-SURFACE-001 — Forbidden Patterns

* Avoid unsafe deserialization, injection, path traversal, hardcoded secrets, weak crypto, sensitive data in logs, TOCTOU races, and supply-chain risks.
* Even offline/single-user apps need hygiene — they often become networked or open-sourced later.

## SEC-SECRET-001 — Secrets Handling

* Never hardcode secrets, API keys, tokens, passwords, or private keys into source or documentation.
* Use environment variables, secret managers, or encrypted local stores as appropriate.
* Never log secrets, tokens, or PII.
* Config of secrets: placeholders and docs only — never real values.
* Treat any file that might contain secrets with extreme care.
* **Stop-ship**: any committed real secret is a critical defect; rotate the credential and purge from history if it entered VCS.
* Delivery self-audit must explicitly confirm no secrets in the package (`CONST-GATE-001` item 6).

## SEC-LIB-001 — Security Libraries

* Prefer well-vetted libraries over custom security primitives.
* When in doubt: research established secure alternatives and ask the human.

## SEC-SUPPLY-001 — Supply Chain (Security Angle)

* Evaluate security track record before adding dependencies.
* Pin versions; audit known vulnerabilities (detail: `DEPENDENCIES.md`).

---

*Canonical home for security and secrets. Networking protocol security also references specialist/NETWORKING.md.*
