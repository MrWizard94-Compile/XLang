# M24a validation matrix — offline registry cache

| ID | Check | Expected |
| --- | --- | --- |
| M24A-001 | pin-local + verify-cache matching digests | PASS |
| M24A-002 | tampered artifact digest | FAIL closed |
| M24A-003 | path escape in package entry | FAIL closed |
| M24A-004 | missing artifact file | FAIL closed |
| M24A-005 | no network sockets used on pin/verify | N/A product (offline pin/verify) |
| M24B-001 | trust-key + pin-local-signed + verify signature | PASS |
| M24B-002 | fetch-signed file:// with valid HMAC | PASS install |
| M24B-003 | fetch-signed bad signature | FAIL closed AE-REG-007 |
| M24B-004 | https:// fetch (M24c) | TLS via ureq when network/path available |
| M24B-005 | compile/run never call fetch | by construction |
| M24C-001 | generate ed25519 trust key + signed pin | PASS |
| M24C-002 | ed25519 verify-cache | PASS |
| M24C-003 | bad ed25519 signature | FAIL closed AE-REG-007 |

---

*End of M24A-VALIDATION-MATRIX.md*
