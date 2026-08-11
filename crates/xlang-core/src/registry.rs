//! M24 registry cache (F-REGISTRY pilot, ADR-060 offline + ADR-074 M24b signed fetch).
//!
//! Guest AETH remains network-free. Host CLI may fetch only via explicit
//! `registry fetch-signed` after HMAC trust-root verify. Compile never requires network.

use std::fs;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::time::Duration;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::sha256_hex;

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
    /// Optional HMAC-SHA256 signature hex over the pin binding (M24b).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    /// Trust key id used for `signature` (M24b).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryTrustDocument {
    pub schema: String,
    pub keys: Vec<RegistryTrustKey>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryTrustKey {
    pub key_id: String,
    /// Relative path under cache root to raw HMAC key bytes.
    pub key_path: String,
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
        if signature.len() != 64 || !signature.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(RegistryError::new(
                "AE-REG-007",
                format!(
                    "package {}@{} signature must be 64 hex characters (HMAC-SHA256)",
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
    install_pin(cache_root, name, version, &bytes, None, None)
}

/// Install HMAC trust key material under the cache root (offline).
pub fn install_trust_key(
    cache_root: &Path,
    key_id: &str,
    key_bytes: &[u8],
) -> Result<RegistryTrustKey, RegistryError> {
    debug_assert!(
        f_registry_authorized() && registry_signed_fetch_pilot(),
        "ADR-074: registry signed fetch pilot"
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
    let relative = format!("trust/keys/{key_id}.hmac");
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
    fs::write(&destination, key_bytes).map_err(|error| {
        RegistryError::new("AE-REG-005", format!("could not write trust key: {error}"))
    })?;
    let key = RegistryTrustKey {
        key_id: key_id.to_owned(),
        key_path: relative,
    };
    let mut trust = load_or_empty_trust(cache_root)?;
    trust.keys.retain(|existing| existing.key_id != key.key_id);
    trust.keys.push(key.clone());
    trust.keys.sort_by(|a, b| a.key_id.cmp(&b.key_id));
    write_trust_index(cache_root, &trust)?;
    Ok(key)
}

/// Compute HMAC-SHA256 pin signature: HMAC(key, name || "\\n" || version || "\\n" || sha256).
#[must_use]
pub fn sign_package_binding(key: &[u8], name: &str, version: &str, sha256: &str) -> String {
    let message = format!("{name}\n{version}\n{sha256}");
    hex_encode(&hmac_sha256(key, message.as_bytes()))
}

/// Pin a local artifact and attach an HMAC signature under `key_id`.
pub fn pin_local_package_signed(
    cache_root: &Path,
    name: &str,
    version: &str,
    artifact_path: &Path,
    key_id: &str,
) -> Result<RegistryPackagePin, RegistryError> {
    debug_assert!(
        f_registry_authorized() && registry_signed_fetch_pilot(),
        "ADR-074: registry signed fetch pilot"
    );
    let key = load_trust_key_bytes(cache_root, key_id)?;
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
    let signature = sign_package_binding(&key, name, version, &digest);
    install_pin(
        cache_root,
        name,
        version,
        &bytes,
        Some(signature),
        Some(key_id.to_owned()),
    )
}

/// Explicit signed fetch (M24b). Source may be `file://` or `http://`/`https://`
/// (HTTPS is accepted only as a URL form; pilot performs plain HTTP GET to host:port
/// and does **not** implement TLS — use file:// or http:// for this pilot).
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
        "ADR-074: registry signed fetch pilot"
    );
    if name.is_empty() || version.is_empty() {
        return Err(RegistryError::new(
            "AE-REG-002",
            "package name and version must be non-empty",
        ));
    }
    if signature_hex.len() != 64 || !signature_hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(RegistryError::new(
            "AE-REG-007",
            "signature must be 64 hex characters (HMAC-SHA256)",
        ));
    }
    let key = load_trust_key_bytes(cache_root, key_id)?;
    let bytes = fetch_bytes(source_url)?;
    let digest = sha256_hex(&bytes);
    let expected = sign_package_binding(&key, name, version, &digest);
    if !hex_eq_ct(&expected, signature_hex) {
        return Err(RegistryError::new(
            "AE-REG-007",
            format!("package {name}@{version} signature verification failed"),
        ));
    }
    install_pin(
        cache_root,
        name,
        version,
        &bytes,
        Some(signature_hex.to_ascii_lowercase()),
        Some(key_id.to_owned()),
    )
}

pub fn verify_registry_cache(cache_root: &Path) -> Result<RegistryCacheDocument, RegistryError> {
    debug_assert!(
        f_registry_authorized() && registry_offline_cache_verify(),
        "ADR-060: registry offline pilot"
    );
    let document = load_or_empty_cache(cache_root)?;
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
            let key = load_trust_key_bytes(cache_root, key_id)?;
            let expected = sign_package_binding(&key, &package.name, &package.version, &actual);
            if !hex_eq_ct(&expected, signature) {
                return Err(RegistryError::new(
                    "AE-REG-007",
                    format!(
                        "package {}@{} signature verification failed",
                        package.name, package.version
                    ),
                ));
            }
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
        return Err(RegistryError::new(
            "AE-REG-008",
            "M24b pilot does not implement TLS; use http:// or file:// for fetch-signed",
        ));
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

fn load_trust_key_bytes(cache_root: &Path, key_id: &str) -> Result<Vec<u8>, RegistryError> {
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
}
