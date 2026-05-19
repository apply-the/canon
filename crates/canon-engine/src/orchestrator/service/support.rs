use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PacketShape {
    DirectoryBacked,
    SingleFile,
    MultiInput,
}

impl PacketShape {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::DirectoryBacked => "directory-backed",
            Self::SingleFile => "single-file",
            Self::MultiInput => "multi-input",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum AuthorityStatus {
    ExplicitAuthoritativeBrief,
    SingleInputAuthoritativeBrief,
    DerivedAuthoritativeInput,
    AmbiguousCurrentBrief,
}

impl AuthorityStatus {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::ExplicitAuthoritativeBrief => "explicit-authoritative-brief",
            Self::SingleInputAuthoritativeBrief => "single-input-authoritative-brief",
            Self::DerivedAuthoritativeInput => "derived-authoritative-input",
            Self::AmbiguousCurrentBrief => "ambiguous-current-brief",
        }
    }
}

pub(super) fn authority_approval_state(approvals: &[ApprovalRecord]) -> AuthorityApprovalState {
    match approvals.last().map(|record| record.decision) {
        Some(ApprovalDecision::Approve) => AuthorityApprovalState::Granted,
        Some(ApprovalDecision::Reject) => AuthorityApprovalState::Rejected,
        None => AuthorityApprovalState::NotNeeded,
    }
}

pub(super) fn collect_files_recursively(
    directory: &Path,
    files: &mut Vec<PathBuf>,
) -> std::io::Result<()> {
    for entry in std::fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_files_recursively(&path, files)?;
        } else if path.is_file() {
            files.push(path);
        }
    }

    Ok(())
}

pub(super) fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut encoded = String::with_capacity(digest.len() * 2);
    for byte in digest {
        use std::fmt::Write as _;
        let _ = write!(&mut encoded, "{byte:02x}");
    }
    encoded
}

pub(super) fn preserve_multiline_summary(value: &str) -> String {
    let mut lines = Vec::new();
    let mut previous_blank = false;

    for raw_line in value.lines() {
        let line = raw_line.split_whitespace().collect::<Vec<_>>().join(" ");
        if line.is_empty() {
            if !previous_blank && !lines.is_empty() {
                lines.push(String::new());
            }
            previous_blank = true;
        } else {
            lines.push(line);
            previous_blank = false;
        }
    }

    lines.join("\n").trim().to_string()
}

pub(super) fn inline_input_label(index: usize) -> String {
    format!("inline-input-{:02}.md", index + 1)
}

pub(super) fn process_failure_excerpt(stdout: &str, stderr: &str) -> String {
    let stderr = stderr.trim();
    if !stderr.is_empty() {
        return stderr.to_string();
    }

    let stdout = stdout.trim();
    if !stdout.is_empty() {
        return stdout.to_string();
    }

    "no process output captured".to_string()
}

pub(super) fn capability_tag(capability: CapabilityKind) -> &'static str {
    match capability {
        CapabilityKind::ReadRepository => "context",
        CapabilityKind::GenerateContent => "generate",
        CapabilityKind::CritiqueContent => "critique",
        CapabilityKind::ProposeWorkspaceEdit => "edit",
        CapabilityKind::InspectDiff => "inspect-diff",
        CapabilityKind::ReadArtifact => "read-artifact",
        CapabilityKind::EmitArtifact => "emit-artifact",
        CapabilityKind::RunCommand => "run-command",
        CapabilityKind::ValidateWithTool => "validate",
        CapabilityKind::InvokeStructuredTool => "structured-tool",
        CapabilityKind::ExecuteBoundedTransformation => "transform",
    }
}

pub(super) fn canonical_mode_input_binding(mode: Mode) -> Option<(&'static str, &'static str)> {
    match mode {
        Mode::Backlog => Some(("backlog.md", "backlog")),
        Mode::Incident => Some(("incident.md", "incident")),
        Mode::Implementation => Some(("implementation.md", "implementation")),
        Mode::Migration => Some(("migration.md", "migration")),
        Mode::SystemAssessment => Some(("system-assessment.md", "system-assessment")),
        Mode::SecurityAssessment => Some(("security-assessment.md", "security-assessment")),
        Mode::SupplyChainAnalysis => Some(("supply-chain-analysis.md", "supply-chain-analysis")),
        Mode::Refactor => Some(("refactor.md", "refactor")),
        Mode::DomainLanguage => Some(("domain-language.md", "domain-language")),
        Mode::DomainModel => Some(("domain-model.md", "domain-model")),
        _ => None,
    }
}
