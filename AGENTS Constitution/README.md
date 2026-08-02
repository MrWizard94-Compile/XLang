# AGENTS Constitution — Universal Pack

**Universal · reusable · movable · locked** quality + process law for human–AI engineering empires.

| | |
|--|--|
| **Pack version** | **5.0.1** (`VERSION`) |
| **Lock** | **LOCKED baseline** — [LOCK.md](LOCK.md) |
| **SOUL** | [AGENTS.md](AGENTS.md) |
| **Process** | [SOP.md](SOP.md) |
| **Identity / portability** | [PACK.md](PACK.md) |
| **How to adopt or move** | [ADOPT.md](ADOPT.md) |

This folder is self-contained. **Move or rename it freely.** Re-run verify after every copy/move.  
**Do not casually edit binding law** — see [LOCK.md](LOCK.md).

---

## Start here

1. [PACK.md](PACK.md) — what this pack is  
2. [LOCK.md](LOCK.md) — amendment freeze  
3. [ADOPT.md](ADOPT.md) — install / point / move  
4. [AGENTS.md](AGENTS.md) — Level 1 constitution  
5. [SOP.md](SOP.md) — Level 2 delivery process  
6. [RULE-REGISTRY.md](RULE-REGISTRY.md) · [MODULE-INDEX.md](MODULE-INDEX.md) · [INTEGRITY.md](INTEGRITY.md)  

---

## Always load (AI)

* `AGENTS.md`  
* `SOP.md`  
* `constitution/03-DEFINITION-OF-DONE.md`  
* `standards/ENGINEERING.md`  
* `standards/TESTING.md`  
* `standards/DOCUMENTATION.md`  

Then load modules per the applicability matrix in `AGENTS.md`.

---

## Verify (path-independent)

```powershell
pwsh -File tools/verify-pack.ps1
pwsh -File tools/self-audit.ps1
pwsh -File tools/run-full-constitution-self.ps1
pwsh -File tools/write-checksums.ps1
```

| Tool | Writes | Gate |
|------|--------|------|
| `verify-pack.ps1` | console | `GOV-INT-001` |
| `self-audit.ps1` | `reports/SELF-AUDIT-latest.md` | `CONST-GATE-001` meta |
| `run-full-constitution-self.ps1` | `reports/FULL-CONSTITUTION-SELF-RUN-latest.md` | Full law + SOP |
| `write-checksums.ps1` | `reports/CHECKSUMS.sha256` | Release aid |

`reports/` is **not law** — see `reports/README.md`.

---

## Layout

```text
VERSION  PACK.md  LOCK.md  ADOPT.md  README.md
AGENTS.md  SOP.md  RULE-REGISTRY.md  MODULE-INDEX.md  INTEGRITY.md
constitution/  standards/  operations/  collaboration/  specialist/
templates/  tools/
docs/          # advisory SOP evidence
reports/       # generated
_archive_*/    # historical only
```

---

## Four levels

1. **Constitution** — `AGENTS.md` + `constitution/`  
2. **Operating law** — `SOP.md`  
3. **Modules** — standards / operations / collaboration / specialist  
4. **Project law** — consumer repos + optional `templates/PROJECT-POINTER.template.md`  

---

## Evolution (locked)

* Human-directed amendments only (`GOV-LOCK-001`)  
* Complete file replacements; bump `VERSION`  
* `verify-pack.ps1` must exit 0  
* Design history (advisory): [Modularize.md](Modularize.md)  

---

*Drop this folder anywhere. Point projects at it. Enforce it. Move it without rewriting the law. Change it only on purpose.*
