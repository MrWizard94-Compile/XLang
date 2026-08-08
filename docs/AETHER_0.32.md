# Aether 0.32 Toolchain Contract (M19d multi-weave arenas + Policy B cooperative)

**Status:** Historical package 0.32 — language **0.11** surface forms + M19d
multi-weave arenas / Policy A+ resourceful total spawn callees; AETH **v11**;
current package is [AETHER_0.35.md](AETHER_0.35.md)
**Depends on:** ADR-035, ADR-036  

## Surface

- Total weaves (not only `main`) may declare one `bind name <- arena N`.  
- Program header capacity = **sum** of all arena declarations (≤ 1_000_000).  
- Nursery Policy **A+**: total spawn callees may use self-owned M2/M6 resources;
  spawn **arguments** stay non-resource; exclusive `access` still cannot cross
  `together`.  
- Policy **B cooperative** (ADR-036): unstarted cancel has no spawn-local owners;
  total resourceful spawns end owners at return. No mid-frame cancel claim.

## Example

`examples/spawn-arena.ae` — parent arena 16 + worker arena 32 → capacity 48; exit 7.

## Product path

Seed-hosted default compile; seed≡bootstrap for spawn-arena and prior corpora.

## Non-goals

Mid-frame cancel, free-on-raise, capacity reclaim, erroring resourceful spawns,
Buffer results across weaves.

*End of AETHER_0.32.md*
