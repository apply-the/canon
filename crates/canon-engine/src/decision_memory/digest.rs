//! Versioned, domain-separated canonical digests for governance state.

use serde::Serialize;
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

use super::DecisionMemoryError;

/// Canonicalization schema for graph and bundle content.
pub const GRAPH_DIGEST_SCHEMA_VERSION: &str = "canon-governance-c14n-v1";
const SHA256_HEX_LENGTH: usize = 64;

/// Typed SHA-256 digest paired with its canonicalization identity.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, serde::Deserialize)]
pub struct ContentDigest {
    /// Canonicalization version that produced the digest.
    pub canonicalization_version: String,
    /// Lowercase SHA-256 hexadecimal representation.
    pub sha256: String,
}

impl ContentDigest {
    /// Computes a domain-separated digest over deterministic JSON.
    pub fn compute<T: Serialize>(domain: &str, value: &T) -> Result<Self, DecisionMemoryError> {
        let normalized = normalize_json(
            serde_json::to_value(value).map_err(DecisionMemoryError::serialization)?,
        );
        let bytes = serde_json::to_vec(&normalized).map_err(DecisionMemoryError::serialization)?;
        let mut hasher = Sha256::new();
        hasher.update(GRAPH_DIGEST_SCHEMA_VERSION.as_bytes());
        hasher.update([0]);
        hasher.update(domain.as_bytes());
        hasher.update([0]);
        hasher.update(bytes);
        Ok(Self {
            canonicalization_version: GRAPH_DIGEST_SCHEMA_VERSION.to_string(),
            sha256: format!("{:x}", hasher.finalize()),
        })
    }

    /// Creates a digest from a frozen wire value after strict validation.
    pub fn from_sha256(
        identity: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, DecisionMemoryError> {
        let identity = identity.into();
        let value = value.into();
        if !is_sha256(&value) {
            return Err(DecisionMemoryError::MalformedDigest { identity });
        }
        Ok(Self {
            canonicalization_version: GRAPH_DIGEST_SCHEMA_VERSION.to_string(),
            sha256: value,
        })
    }
}

pub(crate) fn is_sha256(value: &str) -> bool {
    value.len() == SHA256_HEX_LENGTH
        && value.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn normalize_json(value: Value) -> Value {
    match value {
        Value::Object(object) => {
            let mut entries = object.into_iter().collect::<Vec<_>>();
            entries.sort_by(|left, right| left.0.cmp(&right.0));
            let mut normalized = Map::new();
            for (key, value) in entries {
                normalized.insert(key, normalize_json(value));
            }
            Value::Object(normalized)
        }
        Value::Array(values) => Value::Array(values.into_iter().map(normalize_json).collect()),
        scalar => scalar,
    }
}
