//! Opaque governance identifiers prevent accidental interchange of stable fields.

use serde::{Deserialize, Serialize};

macro_rules! string_identifier {
    ($(#[$metadata:meta])* $name:ident) => {
        $(#[$metadata])*
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            /// Creates a typed value from its stable wire representation.
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }
        }
    };
}

string_identifier!(
    /// Identifies one governance bundle.
    BundleId
);
string_identifier!(
    /// Carries the canonical digest of a governance bundle.
    BundleDigest
);
string_identifier!(
    /// Identifies one authored governance packet.
    PacketId
);
string_identifier!(
    /// Describes one admitted scope element.
    ScopeItem
);
string_identifier!(
    /// Describes one deterministic acceptance criterion.
    AcceptanceCriterion
);
string_identifier!(
    /// Identifies a claim that evidence or approval addresses.
    Claim
);
string_identifier!(
    /// Refers to immutable evidence without exposing its storage.
    EvidenceReference
);
string_identifier!(
    /// Records one external reviewer finding.
    Finding
);

/// Monotonic decision-memory revision.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct Revision(u64);

impl Revision {
    /// Creates a decision-memory revision.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}
