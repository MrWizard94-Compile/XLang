# Low-Level Safety

```
Document: Low-Level Safety
Module ID: MOD-SPC-LOW-001
Version: 1.1.0
Status: Binding
Authority: AGENTS.md
Applies When: Mixins, bytecode, heavy reflection, unsafe code, custom memory management
Dependencies:
  - AGENTS.md
  - standards/ENGINEERING.md
  - standards/TESTING.md
Overrides: None
```

---

## LOW-RISK-001 — Extreme Care

* Mixins, bytecode manipulation, heavy reflection, unsafe code, and custom memory management are high-risk tools.
* Prefer the least invasive approach and higher-level APIs when available.

## LOW-REASON-001 — Documented Reason

* Always provide a clear, documented reason for every use of low-level techniques.
* Version-check; remappable/portable where relevant; clear verification steps.

## LOW-TEMP-001 — No Untracked Hacks

* Never leave “temporary” low-level hacks without a tracked removal plan.

---

*End of specialist/LOW-LEVEL-SAFETY.md*
