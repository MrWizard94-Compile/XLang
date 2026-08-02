# Constrained Hardware and Cost

```
Document: Constrained Hardware and Cost
Module ID: MOD-SPC-HW-001
Version: 1.1.0
Status: Binding
Authority: AGENTS.md
Applies When: Limited RAM/CPU/GPU, cognitive-load packaging, high compute/token cost
Dependencies:
  - AGENTS.md
  - standards/PERFORMANCE.md
  - collaboration/REVIEW-PACKAGING.md
Overrides: None
```

---

## HW-RESPECT-001 — Real Constraints

Design with real constraints:

* Hardware (GPU VRAM, RAM, CPU, storage)  
* Economic (prefer free/open/offline tools)  
* Cognitive (ADHD/ODD/AuDHD, hyperfocus windows, context switching)  
* Time and energy  

## HW-MEM-001 — Allocation Discipline

* Minimize allocations in hot paths.
* Prefer object reuse, primitive arrays, and stack patterns where safe and clear.
* Document expected peak memory for new subsystems.
* Avoid large temporary collections that cause GC or memory pressure.
* Prefer streaming / incremental algorithms over “load everything” for large datasets.
* Profile under realistic constrained conditions when introducing heavy systems.

## HW-ALT-001 — Lighter Alternatives

* When a solution has high resource cost, document the cost and provide a lighter alternative if feasible.
* Prefer algorithms that degrade gracefully under constrained hardware.
* Never assume high-end workstations or unlimited cognitive bandwidth.

## COST-TOKEN-001 — Token and Compute Economics

* Be conscious of AI tokens, GPU compute, storage, and human review time.
* Prefer solutions that minimize future token burn (docs, self-contained deliveries, selective module loading).
* High compute tasks: document expected cost; seek confirmation if large.
* Optimize for long-term empire economics, not only the current conversation.

---

*Canonical home for hardware, cognitive, and cost constraints.*
