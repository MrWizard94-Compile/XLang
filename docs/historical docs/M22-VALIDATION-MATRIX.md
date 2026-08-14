# M22 cross-package import matrix

**Status:** Implementation green (package 0.24.0)  
**Date:** 2026-08-04  

| ID | Case | Expect |
| --- | --- | --- |
| P1 | app imports util lib weave | build+run |
| N1 | package not in depends_on | fail |
| N2 | path escape package root | fail |
| N3 | import main unit | fail |
