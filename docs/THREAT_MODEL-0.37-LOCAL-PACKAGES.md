# Threat Model — Aether 0.37 Local Package Publication

**Status:** Current M25 security review
**Date:** 2026-08-11
**Product pin:** Aether package **0.37.0**; language 0.11; AETH v4–v12 compatibility unchanged
**Decision:** [ADR-107](ADR-107-m25-local-package-publication.md)
**Contract:** [AETHER_0.37.md](AETHER_0.37.md)
**Related rules:** `SEC-INPUT-001`, `CONST-DEP-001`, `RND-INVAR-001`, `TEST-BEHAVIOR-001`, `DOC-SYNC-001`

## 1. Scope and trust boundary

M25 accepts a caller-selected local project manifest, package bundle, cache
directory, and explicit output directory. These filesystem inputs are
untrusted: they may be malformed, stale, tampered, path-hostile, oversized, or
partly controlled by another local process. M25 never accepts a URL, network
response, package script, executable payload, archive extractor, or guest
capability request.

The operator decides which local path to pack, verify, publish, or install.
The tool provides integrity and confinement checks within that chosen boundary;
it does not authenticate the author of a source directory or turn a local cache
into a trusted registry.

## 2. Assets protected

| Asset | Required property |
| --- | --- |
| Locked project identity | Pack must not publish a missing, stale, incomplete, or mixed project snapshot. |
| Bundle content | Metadata, expected paths, and raw source bytes must agree exactly before acceptance. |
| Cache identity | `name@version` must never be silently replaced by different verified content. |
| Install target | Explicit existing output must survive a rejected install unchanged. |
| Workspace authority | Installed output remains a normal M18/M22 project; M25 cannot add implicit dependencies or grants. |
| Product authority | Package handling must not create network, shell, process, FFI, or guest-visible authority. |

## 3. Threats and controls

| Threat | Control | Residual |
| --- | --- | --- |
| `..`, absolute, separator, or unsafe path component | Strict ASCII forward-slash path grammar; canonical root containment; fixed `project/` prefix. | The operator may still choose any permitted local root. |
| Symlink / special-file redirection | `symlink_metadata` regular-file and non-symlink directory checks on source, bundle, cache, and output paths. | A hostile concurrent local process with filesystem privileges can still race any pathname API; staged snapshots and post-copy verification reduce, not eliminate, OS-level TOCTOU risk. |
| Hidden payload / unlisted file | Bundle-tree audit derives allowed files from the parsed project document and rejects extras. | Source-tree files not declared by the project are intentionally not copied or authenticated. |
| Altered manifest/unit/digest | Per-file SHA-256, project-manifest SHA-256, domain-separated content SHA-256, and locked-project verification. | SHA-256 proves integrity against an expected bundle, not publisher identity. |
| Oversized metadata / file / cache | Bounded metadata, file count, file bytes, total bytes, version bytes, and cache identities. | Bound values are a product policy, not a resource-exhaustion guarantee against a hostile OS. |
| Cache poisoning / replacement | Deterministic local cache identity; existing equal content is a verified no-op; differing content rejects `AE-PACKAGE-005`. | No signing, revocation, remote mirror, or cache repair protocol exists. |
| Partial output on failure | Per-operation staging, staged verification, cleanup attempt, and rename finalization; explicit existing outputs reject. | Cleanup can fail under locks or ACLs, leaving only a bounded staging directory beside the requested output. |
| Dependency or authority escalation | No resolver, auto-install, or workspace mutation; existing workspace `depends_on` and grant rules remain authoritative. | Operators can still manually add a package path to a workspace by design. |

## 4. Verification evidence

The M25 core tests exercise deterministic pack identity; direct/cache install;
two M22 workspace consumers; missing, stale, and incomplete locks; altered
units and content metadata; traversal; unlisted files; symlinks; nonregular
files; size bounds; existing-output preservation; cache collision; and malformed
cache enumeration. The standard gate exercises the real CLI pack, verify,
idempotent publish, cache verify, direct/cache install, and installed-project
identity flow using `examples/package-publish/source/`.

## 5. Explicit non-claims

M25 is not a remote package manager, signing system, provenance system,
transparency log, secure archive extractor, dependency resolver, sandbox, or
multi-writer transaction protocol. It does not mitigate a host administrator,
malware with write access to selected roots, compromised filesystem APIs, or an
operator who intentionally installs malicious but internally consistent Aether
source. Existing Aether source verification and host grant controls still apply
when the installed project is later built or run.

*End of THREAT_MODEL-0.37-LOCAL-PACKAGES.md*
