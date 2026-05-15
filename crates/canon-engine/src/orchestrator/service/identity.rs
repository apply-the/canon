use super::EngineService;
use super::*;

impl EngineService {
    pub(super) fn map_init_summary(summary: StoreInitSummary) -> InitSummary {
        InitSummary {
            repo_root: summary.repo_root,
            canon_root: summary.canon_root,
            methods_materialized: summary.methods_materialized,
            policies_materialized: summary.policies_materialized,
            skills_materialized: summary.skills_materialized,
            claude_md_created: summary.claude_md_created,
        }
    }

    pub(super) fn map_skills_summary(summary: StoreSkillsSummary) -> SkillsSummary {
        SkillsSummary {
            skills_dir: summary.skills_dir,
            skills_materialized: summary.skills_materialized,
            skills_skipped: summary.skills_skipped,
            claude_md_created: summary.claude_md_created,
        }
    }
    pub(super) fn authored_input_name(path: &str) -> Option<&str> {
        Path::new(path).file_name().and_then(|name| name.to_str())
    }

    pub(super) fn resolve_approver(&self, explicit_approver: &str) -> String {
        self.resolve_identity(explicit_approver)
    }

    pub(super) fn resolve_owner(&self, explicit_owner: &str) -> String {
        self.resolve_identity(explicit_owner)
    }

    pub(super) fn resolve_identity(&self, explicit_identity: &str) -> String {
        let explicit_identity = explicit_identity.trim();
        if !explicit_identity.is_empty() {
            return explicit_identity.to_string();
        }

        self.resolve_git_owner(GitConfigScope::Local)
            .or_else(|| self.resolve_git_owner(GitConfigScope::Global))
            .unwrap_or_default()
    }

    pub(super) fn resolve_git_owner(&self, scope: GitConfigScope) -> Option<String> {
        let name = self.git_config_value(scope, "user.name");
        let email = self.git_config_value(scope, "user.email");

        match (name, email) {
            (Some(name), Some(email)) => Some(format!("{name} <{email}>")),
            (Some(name), None) => Some(name),
            (None, Some(email)) => Some(email),
            (None, None) => None,
        }
    }

    pub(super) fn git_config_value(&self, scope: GitConfigScope, key: &str) -> Option<String> {
        let shell = ShellAdapter;
        let request = shell.read_only_request("resolve owner identity from git config");
        let scope_arg = match scope {
            GitConfigScope::Local => "--local",
            GitConfigScope::Global => "--global",
        };

        let output = shell
            .run(
                &request,
                "git",
                &["config", scope_arg, "--get", key],
                Some(&self.repo_root),
                false,
            )
            .ok()?;
        let value = output.stdout.trim();
        if value.is_empty() { None } else { Some(value.to_string()) }
    }
}
