use std::fs;
use std::process::Command as ProcessCommand;

use assert_cmd::Command;
use tempfile::TempDir;

fn cli_command() -> Command {
    let mut command = Command::new("cargo");
    command.args([
        "run",
        "--quiet",
        "--manifest-path",
        concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"),
        "-p",
        "canon-cli",
        "--bin",
        "canon",
        "--",
    ]);
    command
}

fn git(workspace: &TempDir, args: &[&str]) {
    let output = ProcessCommand::new("git")
        .args(args)
        .current_dir(workspace.path())
        .output()
        .expect("git command");
    assert!(
        output.status.success(),
        "git {:?} failed: stdout=`{}` stderr=`{}`",
        args,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn init_repo(workspace: &TempDir) {
    git(workspace, &["init", "-b", "main"]);
    git(workspace, &["config", "user.name", "Canon Test"]);
    git(workspace, &["config", "user.email", "canon@example.com"]);

    fs::create_dir_all(workspace.path().join("src/auth")).expect("src dir");
    fs::write(
        workspace.path().join("src/auth/tokens.rs"),
        "pub fn token_owner(id: &str) -> String {\n    format!(\"owner:{id}\")\n}\n",
    )
    .expect("source file");

    git(workspace, &["add", "."]);
    git(workspace, &["commit", "-m", "seed adr publish repo"]);
}

fn change_brief() -> &'static str {
    "# Change Brief\n\n## System Slice\n\nauth session boundary and persistence layer.\n\n## Domain Slice\n\nSession lifecycle and cleanup semantics within the auth domain.\n\n## Excluded Areas\n\n- payment settlement\n- billing reports\n\n## Intended Change\n\nAdd bounded repository methods while preserving the public auth contract.\n\n## Legacy Invariants\n\n- session revocation remains eventually consistent\n- audit log ordering stays stable\n\n## Domain Invariants\n\n- a revoked session must never become active again through cleanup retries\n- audit trails must preserve causal order across repository updates\n\n## Forbidden Normalization\n\n- Do not collapse audit-ordering quirks that operators still rely on.\n\n## Change Surface\n\n- session repository\n- auth service\n- token cleanup job\n\n## Ownership\n\n- primary owner: maintainer\n\n## Cross-Context Risks\n\n- cleanup scheduling can leak into notification flows if repository boundaries widen\n\n## Implementation Plan\n\nAdd bounded repository methods and preserve the public auth contract.\n\n## Sequencing\n\n1. Add bounded repository methods.\n2. Switch callers behind the preserved contract.\n\n## Validation Strategy\n\n- contract tests\n- invariant checks\n\n## Independent Checks\n\n- rollback rehearsal by a separate operator\n\n## Decision Record\n\nPrefer additive change over normalization to preserve operator expectations.\n\n## Decision Drivers\n\n- Preserve operator expectations.\n- Keep the auth contract stable during the bounded repository change.\n\n## Options Considered\n\n- Option 1 keeps the additive repository helper inside the auth boundary.\n- Option 2 normalizes scheduling and cleanup behavior in the same slice.\n\n## Decision Evidence\n\n- Existing operator workflows still depend on the current auth cleanup ordering.\n- Contract tests already guard the preserved API surface.\n\n## Boundary Tradeoffs\n\n- keep cleanup logic inside the auth boundary even if that duplicates some scheduling code\n\n## Recommendation\n\n- Start with the additive repository helper and defer normalization to a later slice.\n\n## Why Not The Others\n\n- Normalizing cleanup behavior now would widen the change surface beyond the bounded auth slice.\n\n## Consequences\n\n- preserved surface remains explicit and reviewable\n\n## Unresolved Questions\n\n- should the cleanup job roll out in the same slice?\n\nOwner: maintainer\nRisk Level: bounded-impact\nZone: yellow\n"
}

fn migration_brief() -> &'static str {
    "# Migration Brief\n\n## Current State\n\n- auth-v1 serves login and token refresh traffic.\n\n## Target State\n\n- auth-v2 serves the same bounded traffic surface.\n\n## Transition Boundaries\n\n- login and token refresh only.\n\n## Guaranteed Compatibility\n\n- existing tokens continue to validate\n\n## Temporary Incompatibilities\n\n- admin reporting stays on v1 during the rollout\n\n## Coexistence Rules\n\n- dual-write session metadata during cutover\n\n## Options Matrix\n\n- Option 1 keeps dual-write through the cutover window.\n- Option 2 cuts directly to auth-v2 and accepts a tighter rollback window.\n\n## Ordered Steps\n\n1. enable shadow reads\n2. start dual-write\n3. cut traffic to auth-v2\n\n## Parallelizable Work\n\n- docs and dashboards can update in parallel\n\n## Cutover Criteria\n\n- error rate and token validation remain stable\n\n## Rollback Triggers\n\n- token validation failures or elevated login errors\n\n## Fallback Paths\n\n- route bounded traffic back to auth-v1\n\n## Re-Entry Criteria\n\n- compatibility regressions are resolved and revalidated\n\n## Adoption Implications\n\n- keep the auth token path bounded to auth-v2 before adjacent reporting workloads adopt it.\n\n## Verification Checks\n\n- login and token validation pass against auth-v2\n\n## Residual Risks\n\n- admin reporting remains temporarily inconsistent\n\n## Release Readiness\n\n- keep recommendation-only posture until owner accepts the packet\n\n## Migration Decisions\n\n- retain dual-write during the bounded cutover\n\n## Tradeoff Analysis\n\n- dual-write raises temporary complexity but keeps rollback safer while the bounded surface proves stable\n\n## Decision Evidence\n\n- login and token validation checks already cover the bounded auth token path on auth-v2.\n- admin reporting still lacks equivalent evidence, which keeps that rollout deferred.\n\n## Recommendation\n\n- keep dual-write for the bounded auth token path and defer broader reporting migration\n\n## Why Not The Others\n\n- a direct cutover would narrow the rollback window before compatibility evidence is strong enough\n\n## Ecosystem Health\n\n- auth-v2 dependencies are healthy enough for bounded cutover, but reporting integrations still lag behind\n\n## Deferred Decisions\n\n- move admin reporting after the bounded migration completes\n\n## Approval Notes\n\n- explicit migration-lead sign-off is required before broader rollout\n"
}

fn read_single_adr(workspace: &TempDir) -> (String, String) {
    let adr_dir = workspace.path().join("docs").join("adr");
    let adr_entry = fs::read_dir(&adr_dir)
        .expect("adr registry dir")
        .next()
        .expect("adr entry")
        .expect("adr dir entry");
    let adr_name = adr_entry.file_name().to_string_lossy().to_string();
    let adr_text = fs::read_to_string(adr_entry.path()).expect("adr text");
    (adr_name, adr_text)
}

#[test]
fn change_publish_only_emits_adr_when_opted_in() {
    let workspace = TempDir::new().expect("temp dir");
    let brief_path = workspace.path().join("change.md");
    fs::write(&brief_path, change_brief()).expect("brief file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "change",
            "--system-context",
            "existing",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "maintainer",
            "--input",
            "change.md",
            "--output",
            "json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).expect("json output");
    let run_id = json["run_id"].as_str().expect("run id");

    cli_command().current_dir(workspace.path()).args(["publish", run_id]).assert().success();
    assert!(
        !workspace.path().join("docs").join("adr").exists(),
        "change publish should not emit an ADR without --adr"
    );

    let publish_output = cli_command()
        .current_dir(workspace.path())
        .args(["publish", run_id, "--adr"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let publish_text = String::from_utf8(publish_output).expect("utf8 publish output");
    let (adr_name, adr_text) = read_single_adr(&workspace);

    assert!(adr_name.starts_with("ADR-0001-"));
    assert!(publish_text.contains(&format!("docs/adr/{adr_name}")));
    assert!(adr_text.starts_with(
        "# ADR 0001: Prefer additive change over normalization to preserve operator expectations."
    ));
    assert!(adr_text.contains("## Context"));
    assert!(
        adr_text.contains("Add bounded repository methods and preserve the public auth contract.")
    );
    assert!(adr_text.contains("## Decision"));
    assert!(
        adr_text.contains(
            "Prefer additive change over normalization to preserve operator expectations."
        )
    );
    assert!(adr_text.contains("## Consequences"));
    assert!(adr_text.contains("preserved surface remains explicit and reviewable"));
    assert!(adr_text.contains("## Alternatives Considered"));
    assert!(
        adr_text
            .contains("Option 1 keeps the additive repository helper inside the auth boundary.")
    );
}

#[test]
fn migration_publish_only_emits_adr_when_opted_in() {
    let workspace = TempDir::new().expect("temp dir");
    init_repo(&workspace);
    fs::write(workspace.path().join("migration.md"), migration_brief()).expect("brief file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "migration",
            "--system-context",
            "existing",
            "--risk",
            "systemic-impact",
            "--zone",
            "yellow",
            "--owner",
            "migration-lead",
            "--input",
            "migration.md",
            "--output",
            "json",
        ])
        .assert()
        .code(3)
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).expect("json output");
    let run_id = json["run_id"].as_str().expect("run id");

    cli_command()
        .current_dir(workspace.path())
        .args([
            "approve",
            "--run",
            run_id,
            "--target",
            "gate:risk",
            "--by",
            "migration-lead",
            "--decision",
            "approve",
            "--rationale",
            "bounded compatibility packet accepted for rollout review",
        ])
        .assert()
        .success();

    cli_command().current_dir(workspace.path()).args(["publish", run_id]).assert().success();
    assert!(
        !workspace.path().join("docs").join("adr").exists(),
        "migration publish should not emit an ADR without --adr"
    );

    let publish_output = cli_command()
        .current_dir(workspace.path())
        .args(["publish", run_id, "--adr"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let publish_text = String::from_utf8(publish_output).expect("utf8 publish output");
    let (adr_name, adr_text) = read_single_adr(&workspace);

    assert!(adr_name.starts_with("ADR-0001-"));
    assert!(publish_text.contains(&format!("docs/adr/{adr_name}")));
    assert!(adr_text.starts_with("# ADR 0001: retain dual-write during the bounded cutover"));
    assert!(adr_text.contains("## Context"));
    assert!(adr_text.contains("auth-v1 serves login and token refresh traffic."));
    assert!(adr_text.contains("## Decision"));
    assert!(adr_text.contains("retain dual-write during the bounded cutover"));
    assert!(adr_text.contains("## Consequences"));
    assert!(adr_text.contains("dual-write raises temporary complexity but keeps rollback safer while the bounded surface proves stable"));
    assert!(adr_text.contains("## Alternatives Considered"));
    assert!(adr_text.contains("Option 1 keeps dual-write through the cutover window."));
}
