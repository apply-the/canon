//! Stable Canon profile registry and legacy mode implementation modules.
//!
//! The stable registry is the admission and projection boundary for Canon
//! 1.0. Legacy mode modules remain readable for historical records and
//! internal migration compatibility, but they do not expand the stable
//! governance surface.

use std::fmt;

use canon_contracts::Profile;

use crate::domain::mode::Mode;

/// Architecture mode execution logic.
pub mod architecture;
/// Backlog mode execution logic.
pub mod backlog;
/// Brainstorming mode execution logic.
pub mod brainstorming;
/// Change mode execution logic.
pub mod change;
/// Debugging mode execution logic.
pub mod debugging;
/// Discovery mode execution logic.
pub mod discovery;
/// Implementation mode execution logic.
pub mod implementation;
/// Incident mode execution logic.
pub mod incident;
/// Migration mode execution logic.
pub mod migration;
/// PR Review mode execution logic.
pub mod pr_review;
/// Refactor mode execution logic.
pub mod refactor;
/// Requirements mode execution logic.
pub mod requirements;
/// Review mode execution logic.
pub mod review;
/// System Shaping mode execution logic.
pub mod system_shaping;
/// Verification mode execution logic.
pub mod verification;

const STABLE_PROFILE_COUNT: usize = 9;

const STABLE_PROFILES: [StableProfileDefinition; STABLE_PROFILE_COUNT] = [
    StableProfileDefinition::new(
        "discovery",
        Profile::Discovery,
        Mode::Discovery,
        ProfilePurpose::Governance,
    ),
    StableProfileDefinition::new(
        "requirements",
        Profile::Requirements,
        Mode::Requirements,
        ProfilePurpose::Governance,
    ),
    StableProfileDefinition::new(
        "architecture",
        Profile::Architecture,
        Mode::Architecture,
        ProfilePurpose::Governance,
    ),
    StableProfileDefinition::new(
        "backlog",
        Profile::Backlog,
        Mode::Backlog,
        ProfilePurpose::Governance,
    ),
    StableProfileDefinition::new(
        "change",
        Profile::Change,
        Mode::Change,
        ProfilePurpose::ChangeIntent,
    ),
    StableProfileDefinition::new(
        "refactor",
        Profile::Refactor,
        Mode::Refactor,
        ProfilePurpose::Governance,
    ),
    StableProfileDefinition::new(
        "verification",
        Profile::Verification,
        Mode::Verification,
        ProfilePurpose::EvidenceRequirements,
    ),
    StableProfileDefinition::new(
        "pr-review",
        Profile::PrReview,
        Mode::PrReview,
        ProfilePurpose::EvidenceRequirements,
    ),
    StableProfileDefinition::new(
        "incident",
        Profile::Incident,
        Mode::Incident,
        ProfilePurpose::Governance,
    ),
];

const STABLE_MODES: [Mode; STABLE_PROFILE_COUNT] = [
    Mode::Discovery,
    Mode::Requirements,
    Mode::Architecture,
    Mode::Backlog,
    Mode::Change,
    Mode::Refactor,
    Mode::Verification,
    Mode::PrReview,
    Mode::Incident,
];

/// A stable profile's bounded governance responsibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProfilePurpose {
    Governance,
    ChangeIntent,
    EvidenceRequirements,
}

/// One immutable entry in Canon's stable governance profile registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StableProfileDefinition {
    id: &'static str,
    contract_profile: Profile,
    mode: Mode,
    purpose: ProfilePurpose,
}

impl StableProfileDefinition {
    const fn new(
        id: &'static str,
        contract_profile: Profile,
        mode: Mode,
        purpose: ProfilePurpose,
    ) -> Self {
        Self { id, contract_profile, mode, purpose }
    }

    /// Returns the exact stable wire identifier.
    pub const fn id(self) -> &'static str {
        self.id
    }

    /// Returns the immutable public-contract profile represented by this entry.
    pub const fn contract_profile(self) -> Profile {
        self.contract_profile
    }

    /// Returns the existing internal mode used to process this governance profile.
    pub const fn mode(self) -> Mode {
        self.mode
    }

    /// Reports whether this profile governs change intent and its constraints.
    pub const fn governs_change_intent(self) -> bool {
        matches!(self.purpose, ProfilePurpose::ChangeIntent)
    }

    /// Reports whether this profile governs verification requirements and evidence.
    pub const fn governs_evidence_requirements(self) -> bool {
        matches!(self.purpose, ProfilePurpose::EvidenceRequirements)
    }

    /// Stable Canon profiles never execute implementation.
    pub const fn executes_implementation(self) -> bool {
        false
    }
}

/// Read-only access to Canon's exact stable profile set.
#[derive(Debug, Clone, Copy)]
pub struct StableProfileRegistry;

impl StableProfileRegistry {
    /// Iterates over the exact stable profile set in frozen order.
    pub fn iter(self) -> impl ExactSizeIterator<Item = StableProfileDefinition> {
        STABLE_PROFILES.iter().copied()
    }

    /// Parses an exact canonical stable profile identifier.
    pub fn parse(self, value: &str) -> Result<StableProfileDefinition, ProfileRegistryError> {
        self.iter()
            .find(|entry| entry.id == value)
            .ok_or_else(|| ProfileRegistryError::UnsupportedProfile { value: value.to_string() })
    }

    /// Resolves an internal mode only when it belongs to the stable registry.
    pub fn for_mode(self, mode: Mode) -> Option<StableProfileDefinition> {
        self.iter().find(|entry| entry.mode == mode)
    }
}

/// Returns Canon's process-wide immutable stable profile registry.
pub const fn stable_profile_registry() -> StableProfileRegistry {
    StableProfileRegistry
}

/// Returns the stable internal modes in the registry's frozen order.
pub const fn stable_modes() -> &'static [Mode; STABLE_PROFILE_COUNT] {
    &STABLE_MODES
}

/// Failure to admit a noncanonical or nonstable governance profile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileRegistryError {
    /// The supplied value is not an exact stable profile identifier.
    UnsupportedProfile {
        /// Rejected caller-supplied identifier.
        value: String,
    },
}

impl fmt::Display for ProfileRegistryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedProfile { value } => {
                write!(formatter, "unsupported stable profile: {value}")
            }
        }
    }
}

impl std::error::Error for ProfileRegistryError {}
