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
| M24D-001 | rotate-key revokes old, new signs | PASS |
| M24D-002 | revoke-key blocks sign/verify | FAIL closed AE-REG-009 |
| M24D-003 | set-key-validity out of window | FAIL closed AE-REG-009 |
| M24E-001 | require_signature policy rejects unsigned | AE-REG-010 |
| M24E-002 | signed pins pass under require_signature | PASS |
| M24F-001 | install root + certified signer + signed pin | PASS |
| M24F-002 | bad certification | FAIL closed AE-REG-011 |
| M24G-001 | root → intermediate → leaf chain | PASS verify |
| M24G-002 | intermediate may certify leaf | PASS |
| M24H-001 | issue X.509-lite cert + verify | PASS |
| M24H-002 | PEM encode/decode round-trip | PASS |
| M24H-003 | tampered signature | FAIL closed AE-REG-012 |
| M24I-001 | store mid+leaf certs | PASS |
| M24I-002 | store chain verify to root | PASS |
| M24I-003 | missing issuer cert in store | FAIL closed AE-REG-013 |
| M24I-004 | invalid/reversed/expired certificate date window | FAIL closed AE-REG-012 |
| M24I-005 | non-root self-signed anchor | FAIL closed AE-REG-013; failed store does not persist |
| M24I-006 | `verify-cache` with malformed/unanchored CA store | FAIL closed AE-REG-013 before package-cache acceptance |
| M24I-007 | `trust-root` / `certify-ed25519-key` CLI key setup | explicit local 32-byte seed files create a root → intermediate → leaf chain; root rejects a parent flag |
| M24I-008 | `store-x509-lite` / `verify-x509-lite-store` CLI argument contract | exact one of each mandatory certificate field; duplicate/missing field rejected |
| M24I-009 | inactive issuer or subject trust-key window | FAIL closed AE-REG-012 on the same verification date |

---

*End of M24A-VALIDATION-MATRIX.md*
