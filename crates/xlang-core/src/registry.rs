//! M24 registry cache (F-REGISTRY pilot: ADR-060 offline, ADR-074 HMAC signed
//! fetch, ADR-077 Ed25519 + HTTPS).
//!
//! Guest AETH remains network-free. Host CLI may fetch only via explicit
//! `registry fetch-signed` after trust-root verify. Compile never requires network.

use std::fs;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::time::Duration;

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::sha256_hex;

pub const REGISTRY_ALG_HMAC_SHA256: &str = "hmac-sha256";
pub const REGISTRY_ALG_ED25519: &str = "ed25519";

pub const REGISTRY_CACHE_SCHEMA: &str = "aether.registry-cache/v1";
pub const REGISTRY_INDEX_FILE: &str = "aether.registry-cache.json";
pub const REGISTRY_TRUST_FILE: &str = "aether.registry-trust.json";
pub const REGISTRY_TRUST_SCHEMA: &str = "aether.registry-trust/v1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistryError {
    pub code: &'static str,
    pub message: String,
}

impl RegistryError {
    fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for RegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for RegistryError {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryCacheDocument {
    pub schema: String,
    pub packages: Vec<RegistryPackagePin>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryPackagePin {
    pub name: String,
    pub version: String,
    /// Relative path under cache root (POSIX-style, no `..`).
    pub artifact: String,
    pub sha256: String,
    /// Optional signature hex over the pin binding (M24b/M24c).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    /// Trust key id used for `signature` (M24b/M24c).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// Signature algorithm: `hmac-sha256` (default) or `ed25519` (M24c).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryTrustDocument {
    pub schema: String,
    pub keys: Vec<RegistryTrustKey>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryTrustKey {
    pub key_id: String,
    /// Relative path under cache root to key material.
    pub key_path: String,
    /// `hmac-sha256` (default) or `ed25519` (M24c).
    #[serde(default = "default_trust_algorithm")]
    pub algorithm: String,
    /// ISO-8601 UTC date `YYYY-MM-DD` when the key becomes valid (M24d).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not_before: Option<String>,
    /// ISO-8601 UTC date `YYYY-MM-DD` when the key expires (M24d).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not_after: Option<String>,
    /// When true, key must not verify or sign (M24d).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub revoked: bool,
    /// Prior key id this key rotated from (M24d).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rotated_from: Option<String>,
    /// When true, this key is a trust root that may certify signing keys (M24f).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_root: bool,
    /// Root/intermediate key id that certified this signing key (M24f).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certified_by: Option<String>,
    /// Hex certification signature over binding (M24f).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certification: Option<String>,
}

fn default_trust_algorithm() -> String {
    REGISTRY_ALG_HMAC_SHA256.to_owned()
}

#[must_use]
pub const fn f_registry_authorized() -> bool {
    true
}

#[must_use]
pub const fn registry_offline_cache_verify() -> bool {
    true
}

/// M24b: signed pin verify + explicit fetch-signed are product (HMAC pilot).
#[must_use]
pub const fn registry_signed_fetch_pilot() -> bool {
    true
}

/// M24c: Ed25519 PKI trust keys + HTTPS fetch-signed are product.
#[must_use]
pub const fn registry_ed25519_https_pilot() -> bool {
    true
}

/// M24d: trust key rotation / revoke / validity window policy is product.
#[must_use]
pub const fn registry_key_rotation_policy() -> bool {
    true
}

/// M24e: multi-root trust policy document is product.
#[must_use]
pub const fn registry_multi_root_trust_policy() -> bool {
    true
}

/// M24f: root-certified signing keys (lightweight certification chain) is product.
#[must_use]
pub const fn registry_root_certified_signing_keys() -> bool {
    true
}

pub const REGISTRY_TRUST_POLICY_FILE: &str = "aether.registry-trust-policy.json";
pub const REGISTRY_TRUST_POLICY_SCHEMA: &str = "aether.registry-trust-policy/v1";

/// Offline multi-root trust policy for a registry cache (M24e).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryTrustPolicy {
    pub schema: String,
    /// When true, every pin must carry a signature + key_id.
    #[serde(default)]
    pub require_signature: bool,
    /// Allow HMAC-SHA256 keys (default true).
    #[serde(default = "default_true")]
    pub allow_hmac: bool,
    /// Allow Ed25519 keys (default true).
    #[serde(default = "default_true")]
    pub allow_ed25519: bool,
    /// Maximum number of non-revoked keys (0 = unlimited).
    #[serde(default)]
    pub max_active_keys: u32,
    /// Preferred root key ids in verification preference order (optional).
    #[serde(default)]
    pub preferred_roots: Vec<String>,
}

fn default_true() -> bool {
    true
}

pub fn default_registry_trust_policy() -> RegistryTrustPolicy {
    RegistryTrustPolicy {
        schema: REGISTRY_TRUST_POLICY_SCHEMA.to_owned(),
        require_signature: false,
        allow_hmac: true,
        allow_ed25519: true,
        max_active_keys: 0,
        preferred_roots: Vec::new(),
    }
}

pub fn parse_registry_trust_policy(json: &str) -> Result<RegistryTrustPolicy, RegistryError> {
    let policy: RegistryTrustPolicy = serde_json::from_str(json).map_err(|error| {
        RegistryError::new(
            "AE-REG-001",
            format!("invalid registry trust policy JSON: {error}"),
        )
    })?;
    if policy.schema != REGISTRY_TRUST_POLICY_SCHEMA {
        return Err(RegistryError::new(
            "AE-REG-001",
            format!(
                "unsupported trust policy schema {} (want {REGISTRY_TRUST_POLICY_SCHEMA})",
                policy.schema
            ),
        ));
    }
    Ok(policy)
}

pub fn serialize_registry_trust_policy(
    policy: &RegistryTrustPolicy,
) -> Result<String, RegistryError> {
    serde_json::to_string_pretty(policy).map_err(|error| {
        RegistryError::new(
            "AE-REG-001",
            format!("could not serialize trust policy: {error}"),
        )
    })
}

pub fn load_or_default_trust_policy(
    cache_root: &Path,
) -> Result<RegistryTrustPolicy, RegistryError> {
    let path = cache_root.join(REGISTRY_TRUST_POLICY_FILE);
    if !path.is_file() {
        return Ok(default_registry_trust_policy());
    }
    let json = fs::read_to_string(&path).map_err(|error| {
        RegistryError::new(
            "AE-REG-005",
            format!("could not read trust policy: {error}"),
        )
    })?;
    parse_registry_trust_policy(&json)
}

pub fn write_trust_policy(
    cache_root: &Path,
    policy: &RegistryTrustPolicy,
) -> Result<(), RegistryError> {
    debug_assert!(
        f_registry_authorized() && registry_multi_root_trust_policy(),
        "ADR-084: multi-root trust policy"
    );
    fs::create_dir_all(cache_root).map_err(|error| {
        RegistryError::new(
            "AE-REG-005",
            format!("could not create cache root: {error}"),
        )
    })?;
    let json = serialize_registry_trust_policy(policy)?;
    fs::write(cache_root.join(REGISTRY_TRUST_POLICY_FILE), json).map_err(|error| {
        RegistryError::new(
            "AE-REG-005",
            format!("could not write trust policy: {error}"),
        )
    })
}

fn enforce_trust_policy(
    cache_root: &Path,
    document: &RegistryCacheDocument,
) -> Result<(), RegistryError> {
    if !registry_multi_root_trust_policy() {
        return Ok(());
    }
    let policy = load_or_default_trust_policy(cache_root)?;
    let trust = load_or_empty_trust(cache_root)?;
    let active: Vec<_> = trust.keys.iter().filter(|k| !k.revoked).collect();
    if policy.max_active_keys > 0 && active.len() as u32 > policy.max_active_keys {
        return Err(RegistryError::new(
            "AE-REG-010",
            format!(
                "trust policy max_active_keys={} exceeded (have {})",
                policy.max_active_keys,
                active.len()
            ),
        ));
    }
    for key in &trust.keys {
        if key.algorithm == REGISTRY_ALG_HMAC_SHA256 && !policy.allow_hmac {
            return Err(RegistryError::new(
                "AE-REG-010",
                format!("trust policy disallows hmac-sha256 key {}", key.key_id),
            ));
        }
        if key.algorithm == REGISTRY_ALG_ED25519 && !policy.allow_ed25519 {
            return Err(RegistryError::new(
                "AE-REG-010",
                format!("trust policy disallows ed25519 key {}", key.key_id),
            ));
        }
    }
    if policy.require_signature {
        for package in &document.packages {
            if package.signature.is_none() || package.key_id.is_none() {
                return Err(RegistryError::new(
                    "AE-REG-010",
                    format!(
                        "trust policy requires signatures; package {}@{} is unsigned",
                        package.name, package.version
                    ),
                ));
            }
        }
    }
    for root in &policy.preferred_roots {
        if !trust.keys.iter().any(|k| k.key_id == *root) {
            return Err(RegistryError::new(
                "AE-REG-010",
                format!("preferred root {root} is not installed in trust store"),
            ));
        }
    }
    Ok(())
}

pub fn empty_registry_cache() -> RegistryCacheDocument {
    RegistryCacheDocument {
        schema: REGISTRY_CACHE_SCHEMA.to_owned(),
        packages: Vec::new(),
    }
}

pub fn empty_registry_trust() -> RegistryTrustDocument {
    RegistryTrustDocument {
        schema: REGISTRY_TRUST_SCHEMA.to_owned(),
        keys: Vec::new(),
    }
}

pub fn parse_registry_cache(json: &str) -> Result<RegistryCacheDocument, RegistryError> {
    let document: RegistryCacheDocument = serde_json::from_str(json).map_err(|error| {
        RegistryError::new(
            "AE-REG-001",
            format!("invalid registry cache JSON: {error}"),
        )
    })?;
    if document.schema != REGISTRY_CACHE_SCHEMA {
        return Err(RegistryError::new(
            "AE-REG-001",
            format!(
                "unsupported registry cache schema {} (want {REGISTRY_CACHE_SCHEMA})",
                document.schema
            ),
        ));
    }
    for package in &document.packages {
        validate_pin_fields(package)?;
    }
    Ok(document)
}

pub fn serialize_registry_cache(document: &RegistryCacheDocument) -> Result<String, RegistryError> {
    serde_json::to_string_pretty(document).map_err(|error| {
        RegistryError::new(
            "AE-REG-001",
            format!("could not serialize registry cache: {error}"),
        )
    })
}

pub fn parse_registry_trust(json: &str) -> Result<RegistryTrustDocument, RegistryError> {
    let document: RegistryTrustDocument = serde_json::from_str(json).map_err(|error| {
        RegistryError::new(
            "AE-REG-001",
            format!("invalid registry trust JSON: {error}"),
        )
    })?;
    if document.schema != REGISTRY_TRUST_SCHEMA {
        return Err(RegistryError::new(
            "AE-REG-001",
            format!(
                "unsupported registry trust schema {} (want {REGISTRY_TRUST_SCHEMA})",
                document.schema
            ),
        ));
    }
    for key in &document.keys {
        if key.key_id.is_empty() {
            return Err(RegistryError::new(
                "AE-REG-006",
                "trust key_id must be non-empty",
            ));
        }
        validate_relative_artifact_path(&key.key_path)?;
    }
    Ok(document)
}

pub fn serialize_registry_trust(document: &RegistryTrustDocument) -> Result<String, RegistryError> {
    serde_json::to_string_pretty(document).map_err(|error| {
        RegistryError::new(
            "AE-REG-001",
            format!("could not serialize registry trust: {error}"),
        )
    })
}

fn validate_pin_fields(package: &RegistryPackagePin) -> Result<(), RegistryError> {
    if package.name.is_empty() || package.version.is_empty() {
        return Err(RegistryError::new(
            "AE-REG-002",
            "package name and version must be non-empty",
        ));
    }
    if package.sha256.len() != 64 || !package.sha256.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(RegistryError::new(
            "AE-REG-003",
            format!(
                "package {}@{} sha256 must be 64 hex characters",
                package.name, package.version
            ),
        ));
    }
    validate_relative_artifact_path(&package.artifact)?;
    if let Some(signature) = &package.signature {
        if !signature.chars().all(|c| c.is_ascii_hexdigit())
            || !(signature.len() == 64 || signature.len() == 128)
        {
            return Err(RegistryError::new(
                "AE-REG-007",
                format!(
                    "package {}@{} signature must be 64 (HMAC) or 128 (Ed25519) hex characters",
                    package.name, package.version
                ),
            ));
        }
        if package.key_id.as_ref().is_none_or(|id| id.is_empty()) {
            return Err(RegistryError::new(
                "AE-REG-006",
                format!(
                    "package {}@{} signature requires key_id",
                    package.name, package.version
                ),
            ));
        }
    }
    Ok(())
}

fn validate_relative_artifact_path(path: &str) -> Result<(), RegistryError> {
    if path.is_empty()
        || path.starts_with('/')
        || path.starts_with('\\')
        || path.contains("..")
        || path.contains('\\')
        || Path::new(path).is_absolute()
    {
        return Err(RegistryError::new(
            "AE-REG-004",
            format!("artifact path must be relative and path-jailed: {path}"),
        ));
    }
    Ok(())
}

/// Pin a local artifact into the cache (offline). Copies bytes under
/// `packages/<name>/<version>/` and records SHA-256.
pub fn pin_local_package(
    cache_root: &Path,
    name: &str,
    version: &str,
    artifact_path: &Path,
) -> Result<RegistryPackagePin, RegistryError> {
    debug_assert!(
        f_registry_authorized() && registry_offline_cache_verify(),
        "ADR-060: registry offline pilot"
    );
    if name.is_empty() || version.is_empty() {
        return Err(RegistryError::new(
            "AE-REG-002",
            "package name and version must be non-empty",
        ));
    }
    let bytes = fs::read(artifact_path).map_err(|error| {
        RegistryError::new(
            "AE-REG-005",
            format!(
                "could not read artifact {}: {error}",
                artifact_path.display()
            ),
        )
    })?;
    install_pin(cache_root, name, version, &bytes, None, None, None)
}

/// Install HMAC trust key material under the cache root (offline).
pub fn install_trust_key(
    cache_root: &Path,
    key_id: &str,
    key_bytes: &[u8],
) -> Result<RegistryTrustKey, RegistryError> {
    install_trust_key_with_algorithm(cache_root, key_id, key_bytes, REGISTRY_ALG_HMAC_SHA256)
}

/// Install trust key material with explicit algorithm (`hmac-sha256` or `ed25519`).
pub fn install_trust_key_with_algorithm(
    cache_root: &Path,
    key_id: &str,
    key_bytes: &[u8],
    algorithm: &str,
) -> Result<RegistryTrustKey, RegistryError> {
    debug_assert!(
        f_registry_authorized() && registry_signed_fetch_pilot(),
        "ADR-074/077: registry signed fetch pilot"
    );
    if key_id.is_empty() {
        return Err(RegistryError::new(
            "AE-REG-006",
            "trust key_id must be non-empty",
        ));
    }
    if key_bytes.is_empty() {
        return Err(RegistryError::new(
            "AE-REG-006",
            "trust key material must be non-empty",
        ));
    }
    let algorithm = normalize_algorithm(algorithm)?;
    let ext = if algorithm == REGISTRY_ALG_ED25519 {
        "ed25519"
    } else {
        "hmac"
    };
    let relative = format!("trust/keys/{key_id}.{ext}");
    validate_relative_artifact_path(&relative)?;
    let destination = resolve_cache_path(cache_root, &relative)?;
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            RegistryError::new(
                "AE-REG-005",
                format!("could not create trust key directory: {error}"),
            )
        })?;
    }
    if algorithm == REGISTRY_ALG_ED25519 {
        if key_bytes.len() != 32 {
            return Err(RegistryError::new(
                "AE-REG-006",
                "ed25519 trust key seed must be exactly 32 bytes",
            ));
        }
        let mut seed = [0u8; 32];
        seed.copy_from_slice(key_bytes);
        let signing = SigningKey::from_bytes(&seed);
        let mut material = Vec::with_capacity(64);
        material.extend_from_slice(signing.to_bytes().as_ref());
        material.extend_from_slice(signing.verifying_key().as_bytes());
        fs::write(&destination, &material).map_err(|error| {
            RegistryError::new("AE-REG-005", format!("could not write trust key: {error}"))
        })?;
    } else {
        fs::write(&destination, key_bytes).map_err(|error| {
            RegistryError::new("AE-REG-005", format!("could not write trust key: {error}"))
        })?;
    }
    let key = RegistryTrustKey {
        key_id: key_id.to_owned(),
        key_path: relative,
        algorithm,
        not_before: None,
        not_after: None,
        revoked: false,
        rotated_from: None,
        is_root: false,
        certified_by: None,
        certification: None,
    };
    let mut trust = load_or_empty_trust(cache_root)?;
    trust.keys.retain(|existing| existing.key_id != key.key_id);
    trust.keys.push(key.clone());
    trust.keys.sort_by(|a, b| a.key_id.cmp(&b.key_id));
    write_trust_index(cache_root, &trust)?;
    Ok(key)
}

/// M24f: install an Ed25519 trust root (may certify signing keys).
pub fn install_trust_root(
    cache_root: &Path,
    key_id: &str,
    seed_bytes: &[u8],
) -> Result<RegistryTrustKey, RegistryError> {
    debug_assert!(
        f_registry_authorized() && registry_root_certified_signing_keys(),
        "ADR-088: root-certified signing keys"
    );
    let mut key =
        install_trust_key_with_algorithm(cache_root, key_id, seed_bytes, REGISTRY_ALG_ED25519)?;
    let mut trust = load_or_empty_trust(cache_root)?;
    if let Some(entry) = trust
        .keys
        .iter_mut()
        .find(|candidate| candidate.key_id == key_id)
    {
        entry.is_root = true;
        entry.certified_by = None;
        entry.certification = None;
        key = entry.clone();
    }
    write_trust_index(cache_root, &trust)?;
    Ok(key)
}

/// M24f: install a signing key certified by an active root (Ed25519 only).
///
/// Certification message: `certify\\n{key_id}\\n{algorithm}\\n{sha256(key_material)}`
/// signed by the root key.
pub fn install_certified_signing_key(
    cache_root: &Path,
    key_id: &str,
    seed_bytes: &[u8],
    root_key_id: &str,
) -> Result<RegistryTrustKey, RegistryError> {
    debug_assert!(
        f_registry_authorized() && registry_root_certified_signing_keys(),
        "ADR-088: root-certified signing keys"
    );
    let root = load_trust_key_meta(cache_root, root_key_id)?;
    if !root.is_root {
        return Err(RegistryError::new(
            "AE-REG-011",
            format!("key {root_key_id} is not a trust root"),
        ));
    }
    if root.algorithm != REGISTRY_ALG_ED25519 {
        return Err(RegistryError::new(
            "AE-REG-011",
            "M24f certification requires an Ed25519 trust root",
        ));
    }
    let mut key =
        install_trust_key_with_algorithm(cache_root, key_id, seed_bytes, REGISTRY_ALG_ED25519)?;
    let material = load_trust_key_bytes(cache_root, key_id)?;
    let digest = crate::sha256_hex(&material);
    let message = format!("certify\n{key_id}\ned25519\n{digest}");
    let root_material = load_trust_key_bytes(cache_root, root_key_id)?;
    if root_material.len() < 32 {
        return Err(RegistryError::new(
            "AE-REG-006",
            "root key material is truncated",
        ));
    }
    let mut seed = [0u8; 32];
    seed.copy_from_slice(&root_material[..32]);
    let signing = SigningKey::from_bytes(&seed);
    let certification = hex_encode(signing.sign(message.as_bytes()).to_bytes().as_ref());
    let mut trust = load_or_empty_trust(cache_root)?;
    if let Some(entry) = trust
        .keys
        .iter_mut()
        .find(|candidate| candidate.key_id == key_id)
    {
        entry.is_root = false;
        entry.certified_by = Some(root_key_id.to_owned());
        entry.certification = Some(certification);
        key = entry.clone();
    }
    write_trust_index(cache_root, &trust)?;
    Ok(key)
}

fn verify_key_certification_chain(
    cache_root: &Path,
    key: &RegistryTrustKey,
) -> Result<(), RegistryError> {
    if key.is_root {
        return Ok(());
    }
    // Uncertified keys remain allowed unless policy requires roots-only later.
    let (Some(root_id), Some(cert_hex)) = (&key.certified_by, &key.certification) else {
        return Ok(());
    };
    let root = load_or_empty_trust(cache_root)?
        .keys
        .into_iter()
        .find(|candidate| candidate.key_id == *root_id)
        .ok_or_else(|| {
            RegistryError::new(
                "AE-REG-011",
                format!("certifying root {root_id} is missing"),
            )
        })?;
    if root.revoked || !root.is_root {
        return Err(RegistryError::new(
            "AE-REG-011",
            format!("certifying root {root_id} is revoked or not a root"),
        ));
    }
    let material = load_trust_key_bytes(cache_root, &key.key_id)?;
    let digest = crate::sha256_hex(&material);
    let message = format!("certify\n{}\ned25519\n{digest}", key.key_id);
    let root_material = load_trust_key_bytes(cache_root, root_id)?;
    let verifying = ed25519_verifying_key_from_material(&root_material)?;
    let sig_bytes = hex_decode(cert_hex)
        .map_err(|_| RegistryError::new("AE-REG-011", "certification is not valid hex"))?;
    if sig_bytes.len() != 64 {
        return Err(RegistryError::new(
            "AE-REG-011",
            "certification must be 64 bytes (128 hex chars)",
        ));
    }
    let mut sig_arr = [0u8; 64];
    sig_arr.copy_from_slice(&sig_bytes);
    let signature = Signature::from_bytes(&sig_arr);
    verifying
        .verify(message.as_bytes(), &signature)
        .map_err(|_| {
            RegistryError::new(
                "AE-REG-011",
                format!(
                    "signing key {} certification by {root_id} failed verification",
                    key.key_id
                ),
            )
        })?;
    Ok(())
}

/// M24d: revoke a trust key (fails closed on subsequent verify/sign).
pub fn revoke_trust_key(
    cache_root: &Path,
    key_id: &str,
) -> Result<RegistryTrustKey, RegistryError> {
    debug_assert!(
        f_registry_authorized() && registry_key_rotation_policy(),
        "ADR-080: registry key rotation policy"
    );
    let mut trust = load_or_empty_trust(cache_root)?;
    let key = trust
        .keys
        .iter_mut()
        .find(|candidate| candidate.key_id == key_id)
        .ok_or_else(|| {
            RegistryError::new("AE-REG-006", format!("unknown trust key_id {key_id}"))
        })?;
    key.revoked = true;
    let out = key.clone();
    write_trust_index(cache_root, &trust)?;
    Ok(out)
}

/// M24d: set validity window for a trust key (`YYYY-MM-DD` or empty to clear).
pub fn set_trust_key_validity(
    cache_root: &Path,
    key_id: &str,
    not_before: Option<&str>,
    not_after: Option<&str>,
) -> Result<RegistryTrustKey, RegistryError> {
    debug_assert!(
        f_registry_authorized() && registry_key_rotation_policy(),
        "ADR-080: registry key rotation policy"
    );
    if let Some(date) = not_before {
        validate_iso_date(date)?;
    }
    if let Some(date) = not_after {
        validate_iso_date(date)?;
    }
    let mut trust = load_or_empty_trust(cache_root)?;
    let key = trust
        .keys
        .iter_mut()
        .find(|candidate| candidate.key_id == key_id)
        .ok_or_else(|| {
            RegistryError::new("AE-REG-006", format!("unknown trust key_id {key_id}"))
        })?;
    key.not_before = not_before.map(str::to_owned);
    key.not_after = not_after.map(str::to_owned);
    let out = key.clone();
    write_trust_index(cache_root, &trust)?;
    Ok(out)
}

/// M24d: install a new key and mark the prior key revoked (rotation).
pub fn rotate_trust_key(
    cache_root: &Path,
    old_key_id: &str,
    new_key_id: &str,
    new_key_bytes: &[u8],
    algorithm: &str,
) -> Result<RegistryTrustKey, RegistryError> {
    debug_assert!(
        f_registry_authorized() && registry_key_rotation_policy(),
        "ADR-080: registry key rotation policy"
    );
    if old_key_id == new_key_id {
        return Err(RegistryError::new(
            "AE-REG-006",
            "rotate requires distinct old and new key ids",
        ));
    }
    let _old = load_trust_key_meta(cache_root, old_key_id)?;
    let mut new_key =
        install_trust_key_with_algorithm(cache_root, new_key_id, new_key_bytes, algorithm)?;
    // Attach rotation metadata and revoke old.
    let mut trust = load_or_empty_trust(cache_root)?;
    if let Some(old) = trust
        .keys
        .iter_mut()
        .find(|candidate| candidate.key_id == old_key_id)
    {
        old.revoked = true;
    }
    if let Some(new) = trust
        .keys
        .iter_mut()
        .find(|candidate| candidate.key_id == new_key_id)
    {
        new.rotated_from = Some(old_key_id.to_owned());
        new_key = new.clone();
    }
    write_trust_index(cache_root, &trust)?;
    Ok(new_key)
}

fn validate_iso_date(date: &str) -> Result<(), RegistryError> {
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 3
        || parts[0].len() != 4
        || parts[1].len() != 2
        || parts[2].len() != 2
        || !parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
    {
        return Err(RegistryError::new(
            "AE-REG-006",
            format!("trust key validity date must be YYYY-MM-DD, got {date:?}"),
        ));
    }
    Ok(())
}

fn trust_key_is_active(key: &RegistryTrustKey) -> Result<(), RegistryError> {
    if key.revoked {
        return Err(RegistryError::new(
            "AE-REG-009",
            format!("trust key {} is revoked", key.key_id),
        ));
    }
    let today = utc_today_ymd();
    if let Some(not_before) = &key.not_before {
        if today.as_str() < not_before.as_str() {
            return Err(RegistryError::new(
                "AE-REG-009",
                format!(
                    "trust key {} is not yet valid (not_before={not_before})",
                    key.key_id
                ),
            ));
        }
    }
    if let Some(not_after) = &key.not_after {
        if today.as_str() > not_after.as_str() {
            return Err(RegistryError::new(
                "AE-REG-009",
                format!(
                    "trust key {} is expired (not_after={not_after})",
                    key.key_id
                ),
            ));
        }
    }
    Ok(())
}

fn trust_key_is_active_in_cache(
    cache_root: &Path,
    key: &RegistryTrustKey,
) -> Result<(), RegistryError> {
    trust_key_is_active(key)?;
    if registry_root_certified_signing_keys() {
        verify_key_certification_chain(cache_root, key)?;
    }
    Ok(())
}

fn utc_today_ymd() -> String {
    // Use system local date as YYYY-MM-DD (operator host clock).
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    // Approximate civil date from UNIX days (good enough for policy tests).
    let days = (now / 86_400) as i64;
    let (y, m, d) = unix_days_to_ymd(days);
    format!("{y:04}-{m:02}-{d:02}")
}

fn unix_days_to_ymd(mut days: i64) -> (i32, u32, u32) {
    // Algorithm from civil_from_days (Howard Hinnant).
    days += 719_468;
    let era = if days >= 0 { days } else { days - 146_096 } / 146_097;
    let doe = (days - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = (yoe as i64) + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y as i32, m as u32, d as u32)
}

/// Generate a fresh Ed25519 trust key under the cache root (M24c).
pub fn generate_ed25519_trust_key(
    cache_root: &Path,
    key_id: &str,
) -> Result<RegistryTrustKey, RegistryError> {
    debug_assert!(
        f_registry_authorized() && registry_ed25519_https_pilot(),
        "ADR-077: Ed25519 registry pilot"
    );
    let signing = SigningKey::generate(&mut OsRng);
    install_trust_key_with_algorithm(
        cache_root,
        key_id,
        signing.to_bytes().as_ref(),
        REGISTRY_ALG_ED25519,
    )
}

/// Compute HMAC-SHA256 pin signature: HMAC(key, name || "\\n" || version || "\\n" || sha256).
#[must_use]
pub fn sign_package_binding(key: &[u8], name: &str, version: &str, sha256: &str) -> String {
    let message = format!("{name}\n{version}\n{sha256}");
    hex_encode(&hmac_sha256(key, message.as_bytes()))
}

fn sign_package_binding_with_algorithm(
    cache_root: &Path,
    key_id: &str,
    name: &str,
    version: &str,
    sha256: &str,
) -> Result<(String, String), RegistryError> {
    let meta = load_trust_key_meta(cache_root, key_id)?;
    let material = load_trust_key_bytes(cache_root, key_id)?;
    let message = format!("{name}\n{version}\n{sha256}");
    if meta.algorithm == REGISTRY_ALG_ED25519 {
        if material.len() < 32 {
            return Err(RegistryError::new(
                "AE-REG-006",
                "ed25519 trust key material is truncated",
            ));
        }
        let mut seed = [0u8; 32];
        seed.copy_from_slice(&material[..32]);
        let signing = SigningKey::from_bytes(&seed);
        let signature = signing.sign(message.as_bytes());
        Ok((hex_encode(signature.to_bytes().as_ref()), meta.algorithm))
    } else {
        Ok((
            hex_encode(&hmac_sha256(&material, message.as_bytes())),
            meta.algorithm,
        ))
    }
}

fn verify_package_binding(
    cache_root: &Path,
    key_id: &str,
    name: &str,
    version: &str,
    sha256: &str,
    signature_hex: &str,
) -> Result<(), RegistryError> {
    let meta = load_trust_key_meta(cache_root, key_id)?;
    let material = load_trust_key_bytes(cache_root, key_id)?;
    let message = format!("{name}\n{version}\n{sha256}");
    if meta.algorithm == REGISTRY_ALG_ED25519 {
        let verifying = ed25519_verifying_key_from_material(&material)?;
        let sig_bytes = hex_decode(signature_hex)
            .map_err(|_| RegistryError::new("AE-REG-007", "ed25519 signature is not valid hex"))?;
        if sig_bytes.len() != 64 {
            return Err(RegistryError::new(
                "AE-REG-007",
                "ed25519 signature must be 64 bytes (128 hex chars)",
            ));
        }
        let mut sig_arr = [0u8; 64];
        sig_arr.copy_from_slice(&sig_bytes);
        let signature = Signature::from_bytes(&sig_arr);
        verifying
            .verify(message.as_bytes(), &signature)
            .map_err(|_| {
                RegistryError::new(
                    "AE-REG-007",
                    format!("package {name}@{version} ed25519 signature verification failed"),
                )
            })?;
        Ok(())
    } else {
        let expected = sign_package_binding(&material, name, version, sha256);
        if !hex_eq_ct(&expected, signature_hex) {
            return Err(RegistryError::new(
                "AE-REG-007",
                format!("package {name}@{version} signature verification failed"),
            ));
        }
        Ok(())
    }
}

/// Pin a local artifact and attach a signature under `key_id` (HMAC or Ed25519).
pub fn pin_local_package_signed(
    cache_root: &Path,
    name: &str,
    version: &str,
    artifact_path: &Path,
    key_id: &str,
) -> Result<RegistryPackagePin, RegistryError> {
    debug_assert!(
        f_registry_authorized() && registry_signed_fetch_pilot(),
        "ADR-074/077: registry signed fetch pilot"
    );
    let bytes = fs::read(artifact_path).map_err(|error| {
        RegistryError::new(
            "AE-REG-005",
            format!(
                "could not read artifact {}: {error}",
                artifact_path.display()
            ),
        )
    })?;
    let digest = sha256_hex(&bytes);
    let (signature, algorithm) =
        sign_package_binding_with_algorithm(cache_root, key_id, name, version, &digest)?;
    install_pin(
        cache_root,
        name,
        version,
        &bytes,
        Some(signature),
        Some(key_id.to_owned()),
        Some(algorithm),
    )
}

/// Explicit signed fetch (M24b/M24c). `file://`, local path, `http://`, or `https://`.
///
/// Never called by `compile` / `run` / `project build`.
pub fn fetch_signed_package(
    cache_root: &Path,
    name: &str,
    version: &str,
    source_url: &str,
    signature_hex: &str,
    key_id: &str,
) -> Result<RegistryPackagePin, RegistryError> {
    debug_assert!(
        f_registry_authorized() && registry_signed_fetch_pilot(),
        "ADR-074/077: registry signed fetch pilot"
    );
    if name.is_empty() || version.is_empty() {
        return Err(RegistryError::new(
            "AE-REG-002",
            "package name and version must be non-empty",
        ));
    }
    if !signature_hex.chars().all(|c| c.is_ascii_hexdigit())
        || !(signature_hex.len() == 64 || signature_hex.len() == 128)
    {
        return Err(RegistryError::new(
            "AE-REG-007",
            "signature must be 64 (HMAC) or 128 (Ed25519) hex characters",
        ));
    }
    let bytes = fetch_bytes(source_url)?;
    let digest = sha256_hex(&bytes);
    verify_package_binding(cache_root, key_id, name, version, &digest, signature_hex)?;
    let algorithm = load_trust_key_meta(cache_root, key_id)?.algorithm;
    install_pin(
        cache_root,
        name,
        version,
        &bytes,
        Some(signature_hex.to_ascii_lowercase()),
        Some(key_id.to_owned()),
        Some(algorithm),
    )
}

pub fn verify_registry_cache(cache_root: &Path) -> Result<RegistryCacheDocument, RegistryError> {
    debug_assert!(
        f_registry_authorized() && registry_offline_cache_verify(),
        "ADR-060: registry offline pilot"
    );
    let document = load_or_empty_cache(cache_root)?;
    enforce_trust_policy(cache_root, &document)?;
    for package in &document.packages {
        validate_pin_fields(package)?;
        let path = resolve_cache_path(cache_root, &package.artifact)?;
        let bytes = fs::read(&path).map_err(|error| {
            RegistryError::new(
                "AE-REG-005",
                format!(
                    "package {}@{} missing artifact {}: {error}",
                    package.name, package.version, package.artifact
                ),
            )
        })?;
        let actual = sha256_hex(&bytes);
        if actual != package.sha256 {
            return Err(RegistryError::new(
                "AE-REG-003",
                format!(
                    "package {}@{} digest mismatch (expected {}, got {actual})",
                    package.name, package.version, package.sha256
                ),
            ));
        }
        if let (Some(signature), Some(key_id)) = (&package.signature, &package.key_id) {
            verify_package_binding(
                cache_root,
                key_id,
                &package.name,
                &package.version,
                &actual,
                signature,
            )?;
        }
    }
    Ok(document)
}

fn install_pin(
    cache_root: &Path,
    name: &str,
    version: &str,
    bytes: &[u8],
    signature: Option<String>,
    key_id: Option<String>,
    algorithm: Option<String>,
) -> Result<RegistryPackagePin, RegistryError> {
    let digest = sha256_hex(bytes);
    let relative = format!("packages/{name}/{version}/artifact.bin");
    validate_relative_artifact_path(&relative)?;
    let destination = resolve_cache_path(cache_root, &relative)?;
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            RegistryError::new(
                "AE-REG-005",
                format!("could not create package directory: {error}"),
            )
        })?;
    }
    fs::write(&destination, bytes).map_err(|error| {
        RegistryError::new(
            "AE-REG-005",
            format!("could not write package artifact: {error}"),
        )
    })?;

    let pin = RegistryPackagePin {
        name: name.to_owned(),
        version: version.to_owned(),
        artifact: relative,
        sha256: digest,
        signature,
        key_id,
        algorithm,
    };
    validate_pin_fields(&pin)?;
    let mut document = load_or_empty_cache(cache_root)?;
    document
        .packages
        .retain(|existing| !(existing.name == pin.name && existing.version == pin.version));
    document.packages.push(pin.clone());
    document
        .packages
        .sort_by(|a, b| (&a.name, &a.version).cmp(&(&b.name, &b.version)));
    write_cache_index(cache_root, &document)?;
    Ok(pin)
}

fn fetch_bytes(source_url: &str) -> Result<Vec<u8>, RegistryError> {
    if let Some(path) = source_url.strip_prefix("file://") {
        // Support file:///C:/... on Windows by stripping leading slash before drive.
        let path = if cfg!(windows)
            && path.starts_with('/')
            && path.len() >= 3
            && path.as_bytes()[2] == b':'
        {
            &path[1..]
        } else {
            path
        };
        return fs::read(path).map_err(|error| {
            RegistryError::new(
                "AE-REG-008",
                format!("could not read file URL {source_url}: {error}"),
            )
        });
    }
    if source_url.starts_with("http://") {
        return http_get_bytes(source_url);
    }
    if source_url.starts_with("https://") {
        debug_assert!(
            registry_ed25519_https_pilot(),
            "ADR-077: HTTPS fetch requires M24c pilot"
        );
        return https_get_bytes(source_url);
    }
    // Bare local path convenience (still offline).
    if Path::new(source_url).is_file() {
        return fs::read(source_url).map_err(|error| {
            RegistryError::new(
                "AE-REG-008",
                format!("could not read source path {source_url}: {error}"),
            )
        });
    }
    Err(RegistryError::new(
        "AE-REG-008",
        format!("unsupported fetch URL scheme (want file:// or http://): {source_url}"),
    ))
}

/// Minimal HTTP/1.0 GET for explicit registry fetch (no redirects, no TLS).
fn http_get_bytes(url: &str) -> Result<Vec<u8>, RegistryError> {
    let rest = url
        .strip_prefix("http://")
        .ok_or_else(|| RegistryError::new("AE-REG-008", "expected http:// URL"))?;
    let (host_port, path) = match rest.split_once('/') {
        Some((host_port, path)) => (host_port, format!("/{path}")),
        None => (rest, "/".to_owned()),
    };
    if host_port.is_empty() || host_port.contains('@') {
        return Err(RegistryError::new("AE-REG-008", "invalid http URL host"));
    }
    let (host, port) = if let Some((host, port)) = host_port.split_once(':') {
        let port: u16 = port.parse().map_err(|_| {
            RegistryError::new("AE-REG-008", format!("invalid http port in {host_port}"))
        })?;
        (host, port)
    } else {
        (host_port, 80)
    };
    let mut stream = TcpStream::connect((host, port)).map_err(|error| {
        RegistryError::new(
            "AE-REG-008",
            format!("could not connect to {host}:{port}: {error}"),
        )
    })?;
    stream.set_read_timeout(Some(Duration::from_secs(30))).ok();
    stream.set_write_timeout(Some(Duration::from_secs(30))).ok();
    let request = format!(
        "GET {path} HTTP/1.0\r\nHost: {host_port}\r\nConnection: close\r\nUser-Agent: aether-registry-m24b\r\n\r\n"
    );
    stream.write_all(request.as_bytes()).map_err(|error| {
        RegistryError::new(
            "AE-REG-008",
            format!("could not write HTTP request: {error}"),
        )
    })?;
    let mut response = Vec::new();
    stream.read_to_end(&mut response).map_err(|error| {
        RegistryError::new(
            "AE-REG-008",
            format!("could not read HTTP response: {error}"),
        )
    })?;
    let header_end = response
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .ok_or_else(|| {
            RegistryError::new("AE-REG-008", "HTTP response missing header terminator")
        })?;
    let headers = std::str::from_utf8(&response[..header_end])
        .map_err(|_| RegistryError::new("AE-REG-008", "HTTP headers are not valid UTF-8"))?;
    let status_line = headers.lines().next().unwrap_or("");
    if !status_line.contains(" 200 ") && !status_line.ends_with(" 200") {
        return Err(RegistryError::new(
            "AE-REG-008",
            format!("HTTP fetch failed: {status_line}"),
        ));
    }
    Ok(response[header_end + 4..].to_vec())
}

fn load_or_empty_cache(cache_root: &Path) -> Result<RegistryCacheDocument, RegistryError> {
    let index = cache_root.join(REGISTRY_INDEX_FILE);
    if !index.is_file() {
        return Ok(empty_registry_cache());
    }
    let json = fs::read_to_string(&index).map_err(|error| {
        RegistryError::new(
            "AE-REG-005",
            format!("could not read registry index: {error}"),
        )
    })?;
    parse_registry_cache(&json)
}

fn write_cache_index(
    cache_root: &Path,
    document: &RegistryCacheDocument,
) -> Result<(), RegistryError> {
    fs::create_dir_all(cache_root).map_err(|error| {
        RegistryError::new(
            "AE-REG-005",
            format!("could not create cache root: {error}"),
        )
    })?;
    let json = serialize_registry_cache(document)?;
    fs::write(cache_root.join(REGISTRY_INDEX_FILE), json).map_err(|error| {
        RegistryError::new(
            "AE-REG-005",
            format!("could not write registry index: {error}"),
        )
    })
}

fn load_or_empty_trust(cache_root: &Path) -> Result<RegistryTrustDocument, RegistryError> {
    let index = cache_root.join(REGISTRY_TRUST_FILE);
    if !index.is_file() {
        return Ok(empty_registry_trust());
    }
    let json = fs::read_to_string(&index).map_err(|error| {
        RegistryError::new(
            "AE-REG-005",
            format!("could not read registry trust: {error}"),
        )
    })?;
    parse_registry_trust(&json)
}

fn write_trust_index(
    cache_root: &Path,
    document: &RegistryTrustDocument,
) -> Result<(), RegistryError> {
    fs::create_dir_all(cache_root).map_err(|error| {
        RegistryError::new(
            "AE-REG-005",
            format!("could not create cache root: {error}"),
        )
    })?;
    let json = serialize_registry_trust(document)?;
    fs::write(cache_root.join(REGISTRY_TRUST_FILE), json).map_err(|error| {
        RegistryError::new(
            "AE-REG-005",
            format!("could not write registry trust: {error}"),
        )
    })
}

fn load_trust_key_meta(cache_root: &Path, key_id: &str) -> Result<RegistryTrustKey, RegistryError> {
    let trust = load_or_empty_trust(cache_root)?;
    let key = trust
        .keys
        .iter()
        .find(|candidate| candidate.key_id == key_id)
        .cloned()
        .ok_or_else(|| {
            RegistryError::new(
                "AE-REG-006",
                format!("unknown trust key_id {key_id}; install with registry trust-key"),
            )
        })?;
    trust_key_is_active_in_cache(cache_root, &key)?;
    Ok(key)
}

fn load_trust_key_bytes(cache_root: &Path, key_id: &str) -> Result<Vec<u8>, RegistryError> {
    // Load path from trust index without re-entering active/certification checks
    // (those call this function to verify bindings).
    let trust = load_or_empty_trust(cache_root)?;
    let key = trust
        .keys
        .iter()
        .find(|candidate| candidate.key_id == key_id)
        .ok_or_else(|| {
            RegistryError::new(
                "AE-REG-006",
                format!("unknown trust key_id {key_id}; install with registry trust-key"),
            )
        })?;
    let path = resolve_cache_path(cache_root, &key.key_path)?;
    fs::read(&path).map_err(|error| {
        RegistryError::new(
            "AE-REG-006",
            format!("could not read trust key {}: {error}", key.key_path),
        )
    })
}

fn normalize_algorithm(algorithm: &str) -> Result<String, RegistryError> {
    match algorithm {
        REGISTRY_ALG_HMAC_SHA256 | "hmac" => Ok(REGISTRY_ALG_HMAC_SHA256.to_owned()),
        REGISTRY_ALG_ED25519 => Ok(REGISTRY_ALG_ED25519.to_owned()),
        other => Err(RegistryError::new(
            "AE-REG-006",
            format!("unsupported trust algorithm {other:?}"),
        )),
    }
}

fn ed25519_verifying_key_from_material(material: &[u8]) -> Result<VerifyingKey, RegistryError> {
    if material.len() >= 64 {
        let mut public = [0u8; 32];
        public.copy_from_slice(&material[32..64]);
        VerifyingKey::from_bytes(&public)
            .map_err(|_| RegistryError::new("AE-REG-006", "ed25519 public key material is invalid"))
    } else if material.len() == 32 {
        let mut seed = [0u8; 32];
        seed.copy_from_slice(material);
        Ok(SigningKey::from_bytes(&seed).verifying_key())
    } else {
        Err(RegistryError::new(
            "AE-REG-006",
            "ed25519 trust key material has invalid length",
        ))
    }
}

fn https_get_bytes(url: &str) -> Result<Vec<u8>, RegistryError> {
    let response = ureq::get(url).call().map_err(|error| {
        RegistryError::new(
            "AE-REG-008",
            format!("HTTPS fetch failed for {url}: {error}"),
        )
    })?;
    let mut bytes = Vec::new();
    response
        .into_reader()
        .read_to_end(&mut bytes)
        .map_err(|error| {
            RegistryError::new(
                "AE-REG-008",
                format!("HTTPS body read failed for {url}: {error}"),
            )
        })?;
    Ok(bytes)
}

fn hex_decode(input: &str) -> Result<Vec<u8>, ()> {
    if !input.len().is_multiple_of(2) {
        return Err(());
    }
    let mut out = Vec::with_capacity(input.len() / 2);
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let hi = hex_nibble(bytes[i])?;
        let lo = hex_nibble(bytes[i + 1])?;
        out.push((hi << 4) | lo);
        i += 2;
    }
    Ok(out)
}

fn hex_nibble(byte: u8) -> Result<u8, ()> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        _ => Err(()),
    }
}

fn resolve_cache_path(cache_root: &Path, relative: &str) -> Result<PathBuf, RegistryError> {
    validate_relative_artifact_path(relative)?;
    let root = cache_root
        .canonicalize()
        .unwrap_or_else(|_| cache_root.to_path_buf());
    let joined = root.join(relative);
    if let Ok(canonical) = joined.canonicalize() {
        if !canonical.starts_with(&root) {
            return Err(RegistryError::new(
                "AE-REG-004",
                format!("artifact path escapes cache root: {relative}"),
            ));
        }
        return Ok(canonical);
    }
    // File may not exist yet (pin write path): jail parent.
    if let Some(parent) = joined.parent() {
        if parent.exists() {
            let parent_canon = parent.canonicalize().map_err(|error| {
                RegistryError::new("AE-REG-004", format!("path jail failed: {error}"))
            })?;
            if !parent_canon.starts_with(&root) {
                return Err(RegistryError::new(
                    "AE-REG-004",
                    format!("artifact path escapes cache root: {relative}"),
                ));
            }
        }
    }
    Ok(joined)
}

fn hmac_sha256(key: &[u8], message: &[u8]) -> [u8; 32] {
    const BLOCK: usize = 64;
    let mut key_block = [0u8; BLOCK];
    if key.len() > BLOCK {
        let digested = Sha256::digest(key);
        key_block[..32].copy_from_slice(&digested);
    } else {
        key_block[..key.len()].copy_from_slice(key);
    }
    let mut ipad = [0x36u8; BLOCK];
    let mut opad = [0x5cu8; BLOCK];
    for i in 0..BLOCK {
        ipad[i] ^= key_block[i];
        opad[i] ^= key_block[i];
    }
    let mut inner = Sha256::new();
    inner.update(ipad);
    inner.update(message);
    let inner_hash = inner.finalize();
    let mut outer = Sha256::new();
    outer.update(opad);
    outer.update(inner_hash);
    let result = outer.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&result);
    out
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

fn hex_eq_ct(a: &str, b: &str) -> bool {
    let a = a.as_bytes();
    let b = b.as_bytes();
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (left, right) in a.iter().zip(b.iter()) {
        diff |= left.to_ascii_lowercase() ^ right.to_ascii_lowercase();
    }
    diff == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static NEXT: AtomicUsize = AtomicUsize::new(0);

    fn temp_dir() -> PathBuf {
        let sequence = NEXT.fetch_add(1, Ordering::Relaxed);
        let path =
            std::env::temp_dir().join(format!("aether-registry-{}-{sequence}", std::process::id()));
        fs::create_dir_all(&path).expect("temp");
        path
    }

    #[test]
    fn pin_and_verify_cache_round_trip() {
        assert!(f_registry_authorized());
        assert!(registry_offline_cache_verify());
        let root = temp_dir();
        let artifact = root.join("input.aeth");
        fs::write(&artifact, b"AETH\x0bpure-fixture").expect("write");
        let pin = pin_local_package(&root, "demo", "1.0.0", &artifact).expect("pin");
        assert_eq!(pin.name, "demo");
        assert_eq!(pin.sha256.len(), 64);
        let verified = verify_registry_cache(&root).expect("verify");
        assert_eq!(verified.packages.len(), 1);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn verify_rejects_tampered_artifact() {
        let root = temp_dir();
        let artifact = root.join("input.aeth");
        fs::write(&artifact, b"AETH\x0bgood").expect("write");
        let pin = pin_local_package(&root, "demo", "1.0.0", &artifact).expect("pin");
        let dest = root.join(&pin.artifact);
        fs::write(&dest, b"AETH\x0btampered").expect("tamper");
        let error = verify_registry_cache(&root).expect_err("tamper must fail");
        assert_eq!(error.code, "AE-REG-003");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn rejects_path_escape_in_index() {
        let json = r#"{
  "schema": "aether.registry-cache/v1",
  "packages": [{
    "name": "x",
    "version": "1",
    "artifact": "../escape.bin",
    "sha256": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
  }]
}"#;
        let error = parse_registry_cache(json).expect_err("escape");
        assert_eq!(error.code, "AE-REG-004");
    }

    #[test]
    fn signed_pin_and_file_fetch_round_trip() {
        assert!(registry_signed_fetch_pilot());
        let root = temp_dir();
        install_trust_key(&root, "ops", b"test-hmac-key-material-32b!!").expect("trust");
        let artifact = root.join("payload.bin");
        fs::write(&artifact, b"AETH\x0bsigned-payload").expect("write");
        let signed =
            pin_local_package_signed(&root, "demo", "2.0.0", &artifact, "ops").expect("signed pin");
        assert!(signed.signature.is_some());
        verify_registry_cache(&root).expect("signed verify");

        // Fetch into a second cache from file:// with the same signature.
        let other = temp_dir();
        install_trust_key(&other, "ops", b"test-hmac-key-material-32b!!").expect("trust other");
        let url = format!(
            "file://{}",
            artifact.display().to_string().replace('\\', "/")
        );
        let fetched = fetch_signed_package(
            &other,
            "demo",
            "2.0.0",
            &url,
            signed.signature.as_ref().expect("sig"),
            "ops",
        )
        .expect("fetch-signed file");
        assert_eq!(fetched.sha256, signed.sha256);
        verify_registry_cache(&other).expect("fetched verify");

        let bad = fetch_signed_package(
            &other,
            "demo",
            "2.0.0",
            &url,
            "0000000000000000000000000000000000000000000000000000000000000000",
            "ops",
        )
        .expect_err("bad sig");
        assert_eq!(bad.code, "AE-REG-007");

        let _ = fs::remove_dir_all(&root);
        let _ = fs::remove_dir_all(&other);
    }

    #[test]
    fn ed25519_signed_pin_round_trip() {
        assert!(registry_ed25519_https_pilot());
        let root = temp_dir();
        generate_ed25519_trust_key(&root, "release").expect("ed25519 key");
        let artifact = root.join("payload.bin");
        fs::write(&artifact, b"AETH\x0bed25519-payload").expect("write");
        let signed = pin_local_package_signed(&root, "demo", "3.0.0", &artifact, "release")
            .expect("ed25519 pin");
        assert_eq!(signed.algorithm.as_deref(), Some(REGISTRY_ALG_ED25519));
        assert_eq!(signed.signature.as_ref().map(|s| s.len()), Some(128));
        verify_registry_cache(&root).expect("ed25519 verify");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn key_rotation_and_revoke_fail_closed() {
        assert!(registry_key_rotation_policy());
        let root = temp_dir();
        install_trust_key(&root, "v1", b"old-hmac-key-material-bytes!!").expect("v1");
        let artifact = root.join("payload.bin");
        fs::write(&artifact, b"AETH\x0brotate").expect("write");
        pin_local_package_signed(&root, "demo", "1.0.0", &artifact, "v1").expect("signed v1");
        let rotated = rotate_trust_key(
            &root,
            "v1",
            "v2",
            b"new-hmac-key-material-bytes!!",
            REGISTRY_ALG_HMAC_SHA256,
        )
        .expect("rotate");
        assert_eq!(rotated.key_id, "v2");
        assert_eq!(rotated.rotated_from.as_deref(), Some("v1"));
        // Old key is revoked: signing with v1 fails.
        let err =
            pin_local_package_signed(&root, "demo", "1.0.1", &artifact, "v1").expect_err("revoked");
        assert_eq!(err.code, "AE-REG-009");
        // New key works.
        pin_local_package_signed(&root, "demo", "1.0.1", &artifact, "v2").expect("signed v2");
        revoke_trust_key(&root, "v2").expect("revoke v2");
        let err = pin_local_package_signed(&root, "demo", "1.0.2", &artifact, "v2")
            .expect_err("revoked v2");
        assert_eq!(err.code, "AE-REG-009");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn multi_root_trust_policy_require_signature() {
        assert!(registry_multi_root_trust_policy());
        let root = temp_dir();
        install_trust_key(&root, "ops", b"policy-hmac-key-material!!!!").expect("key");
        let artifact = root.join("payload.bin");
        fs::write(&artifact, b"AETH\x0bpolicy").expect("write");
        pin_local_package(&root, "demo", "1.0.0", &artifact).expect("unsigned pin");
        verify_registry_cache(&root).expect("default policy allows unsigned");
        let mut policy = default_registry_trust_policy();
        policy.require_signature = true;
        policy.preferred_roots = vec!["ops".to_owned()];
        write_trust_policy(&root, &policy).expect("write policy");
        let err = verify_registry_cache(&root).expect_err("require signature");
        assert_eq!(err.code, "AE-REG-010");
        pin_local_package_signed(&root, "demo", "1.0.0", &artifact, "ops").expect("signed");
        verify_registry_cache(&root).expect("signed under policy");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn root_certified_signing_key_chain() {
        assert!(registry_root_certified_signing_keys());
        let root = temp_dir();
        let mut root_seed = [0u8; 32];
        root_seed[0] = 7;
        install_trust_root(&root, "ca-root", &root_seed).expect("root");
        let mut leaf_seed = [0u8; 32];
        leaf_seed[0] = 9;
        let leaf =
            install_certified_signing_key(&root, "signer", &leaf_seed, "ca-root").expect("leaf");
        assert_eq!(leaf.certified_by.as_deref(), Some("ca-root"));
        assert!(leaf.certification.as_ref().is_some_and(|c| c.len() == 128));
        let artifact = root.join("payload.bin");
        fs::write(&artifact, b"AETH\x0bcertified").expect("write");
        pin_local_package_signed(&root, "demo", "1.0.0", &artifact, "signer").expect("signed");
        verify_registry_cache(&root).expect("verify certified chain");
        // Bad certification fails closed.
        let mut trust = load_or_empty_trust(&root).expect("trust");
        if let Some(entry) = trust.keys.iter_mut().find(|k| k.key_id == "signer") {
            entry.certification = Some("00".repeat(64));
        }
        write_trust_index(&root, &trust).expect("write");
        let err = pin_local_package_signed(&root, "demo", "1.0.1", &artifact, "signer")
            .expect_err("bad cert");
        assert_eq!(err.code, "AE-REG-011");
        let _ = fs::remove_dir_all(&root);
    }
}
