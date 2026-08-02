# Testing Standards

```
Document: Testing Standards
Module ID: MOD-TEST-001
Version: 1.1.0
Status: Binding
Authority: AGENTS.md
Applies When: Any behavior change, new code, or regression risk
Dependencies:
  - AGENTS.md
  - constitution/03-DEFINITION-OF-DONE.md
  - standards/ENGINEERING.md
Overrides: None
```

---

## TEST-BEHAVIOR-001 — Tests Against Intended Behavior

* Tests are written against **intended** behavior, not against a broken implementation.
* When a test fails:
  1. Investigate why the code does not match intended behavior.
  2. Fix the **code** (or clarify intent with the human), not the test.
* Only modify tests when intended behavior has legitimately changed and that change is approved.

## TEST-QUALITY-001 — Meaningful Tests

* Prefer intention-revealing tests over shallow coverage chasing.
* Include unit tests for pure logic, integration/contract tests for boundaries, and e2e or property-based tests for complex or critical subsystems.
* For new behavior, consider tests first (or alongside) to drive implementation.

## TEST-DOMAIN-001 — Domain Mandates

* Pure logic / novel computation: property tests, invariant assertions, determinism checks.
* Game engines / simulation: unit tests + deterministic reproduction for runtime/mixin/network/render paths.
* Automation / scripting: RAM budgets, races, restart recovery, shared-state mutation.
* Web/backend/frontend/mobile: contract tests; matrix where relevant; a11y and performance budgets.
* Non-deterministic systems: seeding, controlled clocks, or eventual-consistency assertions.
* **No flaky tests.**
* Always include negative tests and resource-exhaustion cases on critical paths.

## TEST-REGRESSION-001 — Safety Net

* Tests are living documentation and a refactor safety net.
* Never delete or weaken tests to green the build without human-approved intent change.
* Flaky tests are defects: quarantine only with tracking + fix plan; do not ignore.
* Changing tests to match broken production code is a **stop-ship** violation of `TEST-BEHAVIOR-001`.

---

*Canonical home for testing philosophy. Link TEST-BEHAVIOR-001 from SOP and delivery reports.*
