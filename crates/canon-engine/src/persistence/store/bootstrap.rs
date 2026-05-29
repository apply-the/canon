use super::*;

const METHOD_FILES: &[(&str, &str)] = &[
    ("requirements.toml", include_str!("../../../../../defaults/methods/requirements.toml")),
    ("discovery.toml", include_str!("../../../../../defaults/methods/discovery.toml")),
    ("system-shaping.toml", include_str!("../../../../../defaults/methods/system-shaping.toml")),
    ("change.toml", include_str!("../../../../../defaults/methods/change.toml")),
    ("backlog.toml", include_str!("../../../../../defaults/methods/backlog.toml")),
    ("architecture.toml", include_str!("../../../../../defaults/methods/architecture.toml")),
    ("implementation.toml", include_str!("../../../../../defaults/methods/implementation.toml")),
    ("refactor.toml", include_str!("../../../../../defaults/methods/refactor.toml")),
    ("verification.toml", include_str!("../../../../../defaults/methods/verification.toml")),
    ("review.toml", include_str!("../../../../../defaults/methods/review.toml")),
    ("pr-review.toml", include_str!("../../../../../defaults/methods/pr-review.toml")),
    ("incident.toml", include_str!("../../../../../defaults/methods/incident.toml")),
    (
        "security-assessment.toml",
        include_str!("../../../../../defaults/methods/security-assessment.toml"),
    ),
    (
        "system-assessment.toml",
        include_str!("../../../../../defaults/methods/system-assessment.toml"),
    ),
    ("migration.toml", include_str!("../../../../../defaults/methods/migration.toml")),
    (
        "supply-chain-analysis.toml",
        include_str!("../../../../../defaults/methods/supply-chain-analysis.toml"),
    ),
];

const POLICY_FILES: &[(&str, &str)] = &[
    ("risk.toml", include_str!("../../../../../defaults/policies/risk.toml")),
    ("zones.toml", include_str!("../../../../../defaults/policies/zones.toml")),
    ("gates.toml", include_str!("../../../../../defaults/policies/gates.toml")),
    ("verification.toml", include_str!("../../../../../defaults/policies/verification.toml")),
    ("adapters.toml", include_str!("../../../../../defaults/policies/adapters.toml")),
];

const SKILL_FILES: &[(&str, &str)] = &[
    (
        "canon-init/SKILL.md",
        include_str!("../../../../../defaults/embedded-skills/canon-init/skill-source.md"),
    ),
    (
        "canon-requirements/SKILL.md",
        include_str!("../../../../../defaults/embedded-skills/canon-requirements/skill-source.md"),
    ),
    (
        "canon-change/SKILL.md",
        include_str!("../../../../../defaults/embedded-skills/canon-change/skill-source.md"),
    ),
    (
        "canon-backlog/SKILL.md",
        include_str!("../../../../../defaults/embedded-skills/canon-backlog/skill-source.md"),
    ),
    (
        "canon-pr-review/SKILL.md",
        include_str!("../../../../../defaults/embedded-skills/canon-pr-review/skill-source.md"),
    ),
    (
        "canon-status/SKILL.md",
        include_str!("../../../../../defaults/embedded-skills/canon-status/skill-source.md"),
    ),
    (
        "canon-inspect-invocations/SKILL.md",
        include_str!(
            "../../../../../defaults/embedded-skills/canon-inspect-invocations/skill-source.md"
        ),
    ),
    (
        "canon-inspect-evidence/SKILL.md",
        include_str!(
            "../../../../../defaults/embedded-skills/canon-inspect-evidence/skill-source.md"
        ),
    ),
    (
        "canon-inspect-artifacts/SKILL.md",
        include_str!(
            "../../../../../defaults/embedded-skills/canon-inspect-artifacts/skill-source.md"
        ),
    ),
    (
        "canon-inspect-clarity/SKILL.md",
        include_str!(
            "../../../../../defaults/embedded-skills/canon-inspect-clarity/skill-source.md"
        ),
    ),
    (
        "canon-approve/SKILL.md",
        include_str!("../../../../../defaults/embedded-skills/canon-approve/skill-source.md"),
    ),
    (
        "canon-resume/SKILL.md",
        include_str!("../../../../../defaults/embedded-skills/canon-resume/skill-source.md"),
    ),
    (
        "canon-publish/SKILL.md",
        include_str!("../../../../../defaults/embedded-skills/canon-publish/skill-source.md"),
    ),
    (
        "canon-discovery/SKILL.md",
        include_str!("../../../../../defaults/embedded-skills/canon-discovery/skill-source.md"),
    ),
    (
        "canon-system-shaping/SKILL.md",
        include_str!(
            "../../../../../defaults/embedded-skills/canon-system-shaping/skill-source.md"
        ),
    ),
    (
        "canon-architecture/SKILL.md",
        include_str!("../../../../../defaults/embedded-skills/canon-architecture/skill-source.md"),
    ),
    (
        "canon-implementation/SKILL.md",
        include_str!(
            "../../../../../defaults/embedded-skills/canon-implementation/skill-source.md"
        ),
    ),
    (
        "canon-refactor/SKILL.md",
        include_str!("../../../../../defaults/embedded-skills/canon-refactor/skill-source.md"),
    ),
    (
        "canon-review/SKILL.md",
        include_str!("../../../../../defaults/embedded-skills/canon-review/skill-source.md"),
    ),
    (
        "canon-incident/SKILL.md",
        include_str!("../../../../../defaults/embedded-skills/canon-incident/skill-source.md"),
    ),
    (
        "canon-security-assessment/SKILL.md",
        include_str!(
            "../../../../../defaults/embedded-skills/canon-security-assessment/skill-source.md"
        ),
    ),
    (
        "canon-system-assessment/SKILL.md",
        include_str!(
            "../../../../../defaults/embedded-skills/canon-system-assessment/skill-source.md"
        ),
    ),
    (
        "canon-migration/SKILL.md",
        include_str!("../../../../../defaults/embedded-skills/canon-migration/skill-source.md"),
    ),
    (
        "canon-supply-chain-analysis/SKILL.md",
        include_str!(
            "../../../../../defaults/embedded-skills/canon-supply-chain-analysis/skill-source.md"
        ),
    ),
    (
        "canon-verification/SKILL.md",
        include_str!("../../../../../defaults/embedded-skills/canon-verification/skill-source.md"),
    ),
];

#[derive(Clone, Copy)]
enum SharedSkillFile {
    CheckRuntimeSh,
    CheckRuntimePs1,
    RenderNextStepsSh,
    RenderNextStepsPs1,
    RenderSupportStateSh,
    RenderSupportStatePs1,
    RuntimeCompatibilityToml,
    SkillIndexMd,
    SkillTemplateMd,
    OutputShapesMd,
    SupportStatesMd,
}

impl SharedSkillFile {
    fn relative_path(self) -> &'static str {
        match self {
            Self::CheckRuntimeSh => "canon-shared/scripts/check-runtime.sh",
            Self::CheckRuntimePs1 => "canon-shared/scripts/check-runtime.ps1",
            Self::RenderNextStepsSh => "canon-shared/scripts/render-next-steps.sh",
            Self::RenderNextStepsPs1 => "canon-shared/scripts/render-next-steps.ps1",
            Self::RenderSupportStateSh => "canon-shared/scripts/render-support-state.sh",
            Self::RenderSupportStatePs1 => "canon-shared/scripts/render-support-state.ps1",
            Self::RuntimeCompatibilityToml => "canon-shared/references/runtime-compatibility.toml",
            Self::SkillIndexMd => "canon-shared/references/skill-index.md",
            Self::SkillTemplateMd => "canon-shared/references/skill-template.md",
            Self::OutputShapesMd => "canon-shared/references/output-shapes.md",
            Self::SupportStatesMd => "canon-shared/references/support-states.md",
        }
    }

    fn contents(self) -> &'static str {
        match self {
            Self::CheckRuntimeSh => include_str!(
                "../../../../../defaults/embedded-skills/canon-shared/scripts/check-runtime.sh"
            ),
            Self::CheckRuntimePs1 => include_str!(
                "../../../../../defaults/embedded-skills/canon-shared/scripts/check-runtime.ps1"
            ),
            Self::RenderNextStepsSh => include_str!(
                "../../../../../defaults/embedded-skills/canon-shared/scripts/render-next-steps.sh"
            ),
            Self::RenderNextStepsPs1 => include_str!(
                "../../../../../defaults/embedded-skills/canon-shared/scripts/render-next-steps.ps1"
            ),
            Self::RenderSupportStateSh => include_str!(
                "../../../../../defaults/embedded-skills/canon-shared/scripts/render-support-state.sh"
            ),
            Self::RenderSupportStatePs1 => include_str!(
                "../../../../../defaults/embedded-skills/canon-shared/scripts/render-support-state.ps1"
            ),
            Self::RuntimeCompatibilityToml => include_str!(
                "../../../../../defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml"
            ),
            Self::SkillIndexMd => include_str!(
                "../../../../../defaults/embedded-skills/canon-shared/references/skill-index.md"
            ),
            Self::SkillTemplateMd => include_str!(
                "../../../../../defaults/embedded-skills/canon-shared/references/skill-template.md"
            ),
            Self::OutputShapesMd => include_str!(
                "../../../../../defaults/embedded-skills/canon-shared/references/output-shapes.md"
            ),
            Self::SupportStatesMd => include_str!(
                "../../../../../defaults/embedded-skills/canon-shared/references/support-states.md"
            ),
        }
    }
}

const SHARED_SKILL_FILES: &[SharedSkillFile] = &[
    SharedSkillFile::CheckRuntimeSh,
    SharedSkillFile::CheckRuntimePs1,
    SharedSkillFile::RenderNextStepsSh,
    SharedSkillFile::RenderNextStepsPs1,
    SharedSkillFile::RenderSupportStateSh,
    SharedSkillFile::RenderSupportStatePs1,
    SharedSkillFile::RuntimeCompatibilityToml,
    SharedSkillFile::SkillIndexMd,
    SharedSkillFile::SkillTemplateMd,
    SharedSkillFile::OutputShapesMd,
    SharedSkillFile::SupportStatesMd,
];

impl WorkspaceStore {
    /// Initializes or repairs the `.canon` runtime state, optionally materializing skills.
    pub fn init_runtime_state(
        &self,
        skill_target: Option<SkillMaterializationTarget>,
    ) -> Result<InitSummary, Error> {
        self.ensure_layout()?;
        let methods_materialized =
            self.materialize_defaults(self.layout.methods_dir(), METHOD_FILES)?;
        let policies_materialized =
            self.materialize_defaults(self.layout.policies_dir(), POLICY_FILES)?;
        let skills_materialized = skill_target
            .map(|target| self.materialize_skills(target, false))
            .transpose()?
            .unwrap_or(0);
        let claude_md_created = skill_target
            .filter(|target| target.creates_claude_md())
            .map(|_| self.materialize_claude_md())
            .transpose()?
            .unwrap_or(false);

        Ok(InitSummary {
            repo_root: self.layout.repo_root.display().to_string(),
            canon_root: self.layout.canon_root.display().to_string(),
            methods_materialized,
            policies_materialized,
            skills_materialized,
            claude_md_created,
        })
    }

    /// Returns the names of all materialized method files.
    pub fn list_method_files(&self) -> Result<Vec<String>, Error> {
        list_file_names(self.layout.methods_dir())
    }

    /// Returns the names of all materialized policy files.
    pub fn list_policy_files(&self) -> Result<Vec<String>, Error> {
        list_file_names(self.layout.policies_dir())
    }

    /// Loads and merges all policy files, applying any overrides from the given root.
    pub fn load_policy_set(&self, override_root: Option<&Path>) -> Result<PolicySet, Error> {
        let risk_file: RiskPolicyFile =
            read_toml_file(self.layout.policies_dir().join("risk.toml"))?;
        let zone_file: ZonePolicyFile =
            read_toml_file(self.layout.policies_dir().join("zones.toml"))?;
        let gate_file: GatePolicyFile =
            read_toml_file(self.layout.policies_dir().join("gates.toml"))?;
        let verification_file: VerificationPolicyFile =
            read_toml_file(self.layout.policies_dir().join("verification.toml"))?;
        let _ = risk_file.version;
        let _ = zone_file.version;
        let _ = gate_file.version;
        let adapter_file: AdapterPolicyFile =
            read_toml_file(self.layout.policies_dir().join("adapters.toml"))?;
        let _ = adapter_file.version;
        let _ = adapter_file.adapter.len();
        let _ = verification_file.version;
        let _ = verification_file.layers.low.len();
        let _ = verification_file.layers.bounded.len();
        let _ = verification_file.layers.systemic.len();

        let mut policy_set = PolicySet {
            risk_classes: risk_file.class,
            zones: zone_file.zone,
            gate_policy: GatePolicy { mandatory_gates: gate_file.mandatory_gates },
            adapter_matrix: adapter_file.adapter,
            constraint_profiles: adapter_file.constraint_profile,
            runtime_disabled_adapters: adapter_file.rules.runtime_disabled_adapters,
            validation_independence: verification_file.independence,
            block_mutation_for_red_or_systemic: adapter_file
                .rules
                .block_mutation_for_red_or_systemic,
        };

        if let Some(override_root) = override_root {
            let overrides = self.load_policy_overrides(override_root)?;
            policy_set.apply_overrides(overrides);
        }

        Ok(policy_set)
    }

    pub(super) fn ensure_layout(&self) -> Result<(), Error> {
        for directory in [
            self.layout.canon_root.clone(),
            self.layout.sessions_dir(),
            self.layout.artifacts_dir(),
            self.layout.decisions_dir(),
            self.layout.traces_dir(),
            self.layout.methods_dir(),
            self.layout.policies_dir(),
            self.layout.runs_dir(),
        ] {
            self.filesystem.create_dir_all(&directory).map_err(adapter_error_to_io)?;
        }

        Ok(())
    }

    fn materialize_defaults(
        &self,
        directory: PathBuf,
        files: &[(&str, &str)],
    ) -> Result<usize, Error> {
        let mut written = 0;

        for (name, contents) in files {
            let path = directory.join(name);
            if !path.exists() {
                write_text_file(&path, contents)?;
                written += 1;
            }
        }

        Ok(written)
    }

    /// Materialize skill files into the requested AI-tool skill directory.
    /// When `force` is false, existing files are skipped (idempotent).
    /// When `force` is true, all files are overwritten (update mode).
    /// Returns the number of files written.
    fn materialize_skills(
        &self,
        target: SkillMaterializationTarget,
        force: bool,
    ) -> Result<usize, Error> {
        let skills_dir = target.skills_dir(&self.layout);
        let mut written = 0;

        let shared_skill_files =
            SHARED_SKILL_FILES.iter().copied().map(|file| (file.relative_path(), file.contents()));

        for (relative_path, contents) in SKILL_FILES.iter().copied().chain(shared_skill_files) {
            let path = skills_dir.join(relative_path);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            if force || !path.exists() {
                write_text_file(&path, contents)?;
                #[cfg(unix)]
                if relative_path.ends_with(".sh") {
                    use std::os::unix::fs::PermissionsExt;
                    fs::set_permissions(&path, fs::Permissions::from_mode(0o755))?;
                }
                written += 1;
            }
        }

        Ok(written)
    }

    /// Materialize a minimal CLAUDE.md that imports AGENTS.md.
    /// Skips if the file already exists to avoid overwriting user customizations.
    fn materialize_claude_md(&self) -> Result<bool, Error> {
        let path = self.layout.claude_md_path();
        if path.exists() {
            return Ok(false);
        }
        write_text_file(&path, "@AGENTS.md\n")?;
        Ok(true)
    }

    /// Install skills without requiring `.canon/` to exist.
    pub fn install_skills(
        &self,
        target: SkillMaterializationTarget,
    ) -> Result<SkillsSummary, Error> {
        let total = SKILL_FILES.len() + SHARED_SKILL_FILES.len();
        let written = self.materialize_skills(target, false)?;
        let claude_md_created =
            if target.creates_claude_md() { self.materialize_claude_md()? } else { false };

        Ok(SkillsSummary {
            skills_dir: target.skills_dir(&self.layout).display().to_string(),
            skills_materialized: written,
            skills_skipped: total - written,
            claude_md_created,
        })
    }

    /// Force-update all skills, overwriting existing files.
    pub fn update_skills(
        &self,
        target: SkillMaterializationTarget,
    ) -> Result<SkillsSummary, Error> {
        let total = SKILL_FILES.len() + SHARED_SKILL_FILES.len();
        let written = self.materialize_skills(target, true)?;
        let claude_md_created =
            if target.creates_claude_md() { self.materialize_claude_md()? } else { false };

        Ok(SkillsSummary {
            skills_dir: target.skills_dir(&self.layout).display().to_string(),
            skills_materialized: written,
            skills_skipped: total - written,
            claude_md_created,
        })
    }

    /// List all embedded skill names and their support state.
    pub fn list_skills(&self) -> Vec<SkillEntry> {
        SKILL_FILES
            .iter()
            .filter_map(|(relative_path, contents)| {
                let name = relative_path.split('/').next()?;
                let support_state = if contents.contains("`available-now`") {
                    "available-now"
                } else {
                    "discoverable"
                };
                Some(SkillEntry {
                    name: name.to_string(),
                    support_state: support_state.to_string(),
                })
            })
            .collect()
    }

    fn load_policy_overrides(&self, override_root: &Path) -> Result<PolicySetOverrides, Error> {
        let risk_overrides = if override_root.join("risk.toml").exists() {
            let risk_file = read_toml_file::<RiskPolicyFile>(override_root.join("risk.toml"))?;
            let _ = risk_file.version;
            risk_file.class
        } else {
            Vec::new()
        };
        let zone_overrides = if override_root.join("zones.toml").exists() {
            let zone_file = read_toml_file::<ZonePolicyFile>(override_root.join("zones.toml"))?;
            let _ = zone_file.version;
            zone_file.zone
        } else {
            Vec::new()
        };
        let gate_override = if override_root.join("gates.toml").exists() {
            let gate_file = read_toml_file::<GatePolicyFile>(override_root.join("gates.toml"))?;
            let _ = gate_file.version;
            Some(GatePolicy { mandatory_gates: gate_file.mandatory_gates })
        } else {
            None
        };
        let adapter_override = if override_root.join("adapters.toml").exists() {
            {
                let adapter_policy =
                    read_toml_file::<AdapterPolicyFile>(override_root.join("adapters.toml"))?;
                let _ = adapter_policy.version;
                Some((
                    adapter_policy.adapter,
                    adapter_policy.constraint_profile,
                    adapter_policy.rules.runtime_disabled_adapters,
                    adapter_policy.rules.block_mutation_for_red_or_systemic,
                ))
            }
        } else {
            None
        };
        let verification_override = if override_root.join("verification.toml").exists() {
            let verification_file =
                read_toml_file::<VerificationPolicyFile>(override_root.join("verification.toml"))?;
            let _ = verification_file.version;
            Some(verification_file.independence)
        } else {
            None
        };

        let adapter_matrix_overrides = adapter_override
            .as_ref()
            .map(|override_tuple| override_tuple.0.clone())
            .unwrap_or_default();
        let constraint_profile_overrides = adapter_override
            .as_ref()
            .map(|override_tuple| override_tuple.1.clone())
            .unwrap_or_default();
        let runtime_disabled_adapters =
            adapter_override.as_ref().map(|override_tuple| override_tuple.2.clone());
        let block_mutation_override =
            adapter_override.as_ref().map(|override_tuple| override_tuple.3);

        Ok(PolicySetOverrides {
            risk_classes: risk_overrides,
            zones: zone_overrides,
            gate_policy: gate_override,
            adapter_matrix: adapter_matrix_overrides,
            constraint_profiles: constraint_profile_overrides,
            runtime_disabled_adapters,
            validation_independence: verification_override,
            block_mutation_for_red_or_systemic: block_mutation_override,
        })
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    #[test]
    fn install_skills_for_claude_creates_claude_md_and_lists_support_states() {
        let workspace = TempDir::new().expect("temp dir");
        let store = WorkspaceStore::new(workspace.path());

        let summary = store
            .install_skills(SkillMaterializationTarget::Claude)
            .expect("skills install should succeed");

        assert!(summary.skills_dir.ends_with(".claude/skills"));
        assert!(summary.skills_materialized > 0);
        assert!(summary.skills_skipped == 0);
        assert!(summary.claude_md_created);
        assert!(workspace.path().join(".claude").join("skills").is_dir());
        assert!(workspace.path().join("CLAUDE.md").exists());

        let listed = store.list_skills();
        assert!(listed.iter().any(|entry| entry.support_state == "available-now"));
    }
}
