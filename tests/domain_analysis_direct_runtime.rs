use std::fs;
use std::process::Command as ProcessCommand;

use canon_engine::EngineService;
use canon_engine::domain::approval::ApprovalDecision;
use canon_engine::domain::mode::Mode;
use canon_engine::domain::policy::{RiskClass, UsageZone};
use canon_engine::domain::publish_profile::PublishProfile;
use canon_engine::domain::run::ClassificationProvenance;
use canon_engine::orchestrator::publish::publish_run_with_profile;
use canon_engine::orchestrator::service::RunRequest;
use tempfile::TempDir;

fn git(workspace: &TempDir, args: &[&str]) {
    let output = ProcessCommand::new("git")
        .args(["-c", "commit.gpgsign=false", "-c", "tag.gpgsign=false"])
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

fn init_domain_repo(workspace: &TempDir) {
    git(workspace, &["init", "-b", "main"]);
    git(workspace, &["config", "user.name", "Canon Test"]);
    git(workspace, &["config", "user.email", "canon@example.com"]);

    fs::create_dir_all(workspace.path().join("src/api")).expect("api dir");
    fs::create_dir_all(workspace.path().join("src/domain")).expect("domain dir");
    fs::create_dir_all(workspace.path().join("tech-docs/domain")).expect("tech-docs dir");
    fs::create_dir_all(workspace.path().join("db")).expect("db dir");

    fs::write(
        workspace.path().join("src/api/orders.rs"),
        "pub fn create_order(order_id: &str) -> String {\n    format!(\"order:{order_id}\")\n}\n",
    )
    .expect("orders api");
    fs::write(
        workspace.path().join("src/domain/order.rs"),
        "pub struct Order {\n    pub order_id: String,\n    pub reservation_id: String,\n}\n",
    )
    .expect("order domain");
    fs::write(
        workspace.path().join("tech-docs/domain/ordering.md"),
        "# Ordering Language\n\n- basket is still used in legacy admin flows\n- order is used in customer-facing surfaces\n",
    )
    .expect("domain doc");
    fs::write(
        workspace.path().join("db/schema.sql"),
        "create table orders (order_id text primary key, reservation_id text not null);\n",
    )
    .expect("schema");

    git(workspace, &["add", "."]);
    git(workspace, &["commit", "-m", "seed domain analysis repo"]);
}

fn default_publish_leaf(run_id: &str, descriptor: &str) -> String {
    format!("{}-{}-{}-{descriptor}", &run_id[2..6], &run_id[6..8], &run_id[8..10])
}

fn domain_language_request(input: &str, risk: RiskClass, zone: UsageZone) -> RunRequest {
    RunRequest {
        mode: Mode::DomainLanguage,
        risk,
        zone,
        system_context: None,
        classification: ClassificationProvenance::explicit(),
        owner: "domain-lead".to_string(),
        inputs: vec![input.to_string()],
        inline_inputs: Vec::new(),
        excluded_paths: Vec::new(),
        policy_root: None,
        method_root: None,
    }
}

fn domain_model_request(input: &str, risk: RiskClass, zone: UsageZone) -> RunRequest {
    RunRequest {
        mode: Mode::DomainModel,
        risk,
        zone,
        system_context: None,
        classification: ClassificationProvenance::explicit(),
        owner: "domain-architect".to_string(),
        inputs: vec![input.to_string()],
        inline_inputs: Vec::new(),
        excluded_paths: Vec::new(),
        policy_root: None,
        method_root: None,
    }
}

fn complete_domain_language_brief() -> &'static str {
    r#"# Domain Language Brief

## Domain Scope

- wholesale order fulfillment from order confirmation to shipment handoff

## Language Maturity

- stabilizing

## Upstream Sources

- tech-docs/domain/ordering.md
- src/api/orders.rs

## Downstream Consumers

- requirements
- system-shaping
- backlog

## Glossary Entries

- Order: a confirmed commercial request that has passed customer confirmation.
- Reservation: a temporary stock hold attached to an order before fulfillment.

## Source References

- customer API and billing docs use order
- legacy admin flows still use basket

## Open Gaps

- shipment line and order line are still conflated in warehouse notes

## Canonical Terms

- use order for the confirmed commercial object
- use reservation for the stock-hold object

## Deprecated Synonyms

- basket -> order
- hold -> reservation

## Migration Notes

- rename basket_id to order_id at the API edge before downstream storage cleanup

## Conflict Inventory

- line means order line in ordering and shipment line in warehouse documents
- reservation means stock hold to fulfillment and payment hold to finance

## Resolution Status

- line conflict is resolved when prefixed with order or shipment
- reservation conflict stays open until finance documentation is updated

## Escalation Triggers

- any new public API field that uses a deprecated synonym
- any schema change that introduces basket terminology again

## Context-Dependent Terms

- line
- reservation

## Disambiguation Rules

- line means order line inside ordering artifacts and shipment line inside fulfillment artifacts
- reservation means stock hold unless the packet explicitly says payment reservation

## Usage Examples

- order line count appears in ordering metrics
- shipment line count appears in warehouse manifests

## Naming Conventions

- use singular business nouns for aggregates and explicit suffixes for identifiers

## Domain Boundaries

- this packet covers ordering and handoff language, not carrier operations vocabulary

## Enforcement Guidance

- apply canonical language in requirements, API review, and code review checklists

## Code Naming Patterns

- use order_id, reservation_id, and order_line in source names

## API Surface Terms

- orders
- orderId
- reservationId

## Alignment Gaps

- legacy admin UI still exposes basket_id

## Consumer Modes

- requirements
- system-shaping
- backlog

## Handoff Expectations

- downstream packets should inherit canonical terms unless they document an explicit exception

## Adoption Risks

- warehouse onboarding material still uses shipment-first language

## Decision Drivers

- reduce vocabulary drift between product, API, and code surfaces

## Options Considered

- preserve mixed legacy terminology
- adopt finance-first terminology
- standardize on customer-facing order terminology

## Decision Evidence

- tech-docs/domain/ordering.md and src/api/orders.rs already converge on order terminology

## Recommendation

- standardize on order and reservation terminology across authored packets and public interfaces

## Consequences

- one bounded transition period will keep deprecated synonyms visible in compatibility notes

## Generation Lineage

- authored from repository language references and bounded downstream needs

## Human Authored Sections

- Domain Scope
- Conflict Inventory

## Confidence Posture

- medium confidence until warehouse-facing terms are normalized
"#
}

fn incomplete_domain_language_brief() -> &'static str {
    r#"# Domain Language Brief

## Domain Scope

- wholesale order fulfillment language only

## Language Maturity

- stabilizing

## Upstream Sources

- tech-docs/domain/ordering.md

## Downstream Consumers

- requirements

## Source References

- tech-docs/domain/ordering.md

## Open Gaps

- glossary still needs canonical definitions

## Canonical Terms

- use order after confirmation

## Deprecated Synonyms

- basket -> order

## Migration Notes

- retire basket_id from new packets first

## Conflict Inventory

- reservation still means different things to finance and fulfillment

## Resolution Status

- reservation remains unresolved

## Escalation Triggers

- any new API field that repeats basket terminology

## Context-Dependent Terms

- reservation

## Disambiguation Rules

- reservation means stock hold unless finance context is explicit

## Usage Examples

- stock reservation expires after checkout timeout

## Naming Conventions

- prefer singular business nouns

## Domain Boundaries

- exclude carrier terminology

## Enforcement Guidance

- review language before publishing downstream packets

## Code Naming Patterns

- prefer order_id and reservation_id

## API Surface Terms

- orders

## Alignment Gaps

- legacy admin still uses basket_id

## Consumer Modes

- requirements

## Handoff Expectations

- downstream packets should adopt order terminology

## Adoption Risks

- finance docs still overload reservation

## Decision Drivers

- reduce vocabulary drift

## Options Considered

- keep mixed terms

## Decision Evidence

- tech-docs/domain/ordering.md

## Recommendation

- standardize on order terminology

## Consequences

- one release of compatibility notes remains necessary

## Generation Lineage

- repository docs and code references

## Human Authored Sections

- Domain Scope

## Confidence Posture

- medium confidence
"#
}

fn complete_domain_model_brief() -> &'static str {
    r#"# Domain Model Brief

## Domain Scope

- order fulfillment from confirmed order to shipment creation

## Model Maturity

- evolving

## Upstream Sources

- tech-docs/domain/ordering.md
- src/domain/order.rs

## Downstream Consumers

- system-shaping
- architecture
- backlog

## Concepts

- Order: aggregate that owns customer intent, lines, and confirmation state.
- Reservation: bounded hold that protects inventory for one order.

## Ownership Boundaries

- Ordering owns Order and Reservation identifiers until fulfillment handoff
- Fulfillment owns Shipment after the order-confirmed event is emitted

## Open Gaps

- return authorization is not yet modeled in this packet

## Relationships

- Order consumes one Reservation before confirmation.
- Shipment fulfills one confirmed Order through an event handoff.

## Cardinality Rules

- one Order owns one or more order lines
- one Reservation belongs to exactly one Order

## Boundary Crossings

- order-confirmed crosses from ordering into fulfillment

## Bounded Contexts

- Ordering
- Fulfillment

## Context Relationships

- Ordering publishes confirmed orders and Fulfillment consumes them to plan shipment work

## Integration Seams

- order-confirmed event stream

## Entity Lifecycles

- Order moves from draft to confirmed to fulfilled
- Reservation moves from held to consumed or released

## State Transitions

- draft -> confirmed when payment and reservation checks pass
- confirmed -> fulfilled when shipment creation succeeds

## Invariant Guards

- fulfillment cannot start before payment and reservation are both valid

## Invariants

- a confirmed Order must own at least one order line
- a Shipment cannot exist without one confirmed Order

## Enforcement Points

- order aggregate command handlers
- fulfillment creation service

## Violation Consequences

- reject the command and keep the order in its prior valid state

## Business Policies

- inventory reservations expire after a bounded checkout window

## Constraint Rules

- expired reservations must be released before confirmation retries

## Exception Handling

- operations may reopen one reservation through a manual override flow

## Impact Rules

- partial cancellation releases reservation capacity and may replan shipment work

## Affected Concepts

- Order
- Reservation

## Downstream Effects

- cancellation updates shipment planning and customer status messaging

## Code Mapping

- src/domain/order.rs maps the Order aggregate
- src/api/orders.rs maps the order creation boundary

## Data Store Mapping

- orders table stores order_id and reservation_id

## Alignment Gaps

- legacy reporting still refers to basket identifiers

## Model Gaps

- return and split-shipment concepts are inferred, not yet formalized

## Risk Signals

- shipment split logic is only partially captured in repo evidence

## Recommended Follow-Ups

- shape bounded contexts for split shipment planning before architecture finalization

## Consumer Modes

- system-shaping
- architecture
- backlog

## Handoff Expectations

- downstream packets should preserve order and reservation invariants unless they document an explicit exception

## Adoption Risks

- legacy reporting and warehouse workflows still depend on basket-era language

## Generation Lineage

- authored from the domain-language packet and current code mappings

## Human Authored Sections

- Domain Scope
- Invariants

## Confidence Posture

- medium confidence until split-shipment behavior is explicitly modeled
"#
}

fn incomplete_domain_model_brief() -> &'static str {
    r#"# Domain Model Brief

## Domain Scope

- order fulfillment concepts only

## Model Maturity

- evolving

## Upstream Sources

- src/domain/order.rs

## Downstream Consumers

- architecture

## Concepts

- Order: aggregate for a confirmed purchase.

## Ownership Boundaries

- Ordering owns Order

## Open Gaps

- fulfillment ownership is incomplete

## Cardinality Rules

- one Order owns one or more order lines

## Boundary Crossings

- fulfillment handoff remains event-driven

## Bounded Contexts

- Ordering

## Context Relationships

- Ordering hands confirmed work to fulfillment

## Integration Seams

- order-confirmed event stream

## Entity Lifecycles

- Order moves from draft to confirmed

## State Transitions

- draft -> confirmed when validation passes

## Invariant Guards

- confirmation requires a valid reservation

## Invariants

- confirmed Order must own at least one order line

## Enforcement Points

- order aggregate command handlers

## Violation Consequences

- reject invalid confirmation

## Business Policies

- reservation windows are bounded

## Constraint Rules

- expired reservations must be released

## Exception Handling

- operations may reopen one reservation manually

## Impact Rules

- cancellation releases reservation capacity

## Affected Concepts

- Order

## Downstream Effects

- shipment planning must be recalculated

## Code Mapping

- src/domain/order.rs maps Order

## Data Store Mapping

- orders table persists order_id and reservation_id

## Alignment Gaps

- basket-era reporting is still present

## Model Gaps

- fulfillment context is under-modeled

## Risk Signals

- split shipment logic is inferred only

## Recommended Follow-Ups

- formalize fulfillment context next

## Consumer Modes

- architecture

## Handoff Expectations

- downstream packets should preserve ordering invariants

## Adoption Risks

- warehouse process names are still legacy-heavy

## Generation Lineage

- current code mappings and domain notes

## Human Authored Sections

- Domain Scope

## Confidence Posture

- medium confidence
"#
}

#[test]
fn domain_language_direct_run_exercises_service_summary_and_publish_paths() {
    let workspace = TempDir::new().expect("temp dir");
    init_domain_repo(&workspace);
    fs::write(workspace.path().join("domain-language.md"), complete_domain_language_brief())
        .expect("brief file");

    let service = EngineService::new(workspace.path());
    let summary = service
        .run(domain_language_request(
            "domain-language.md",
            RiskClass::SystemicImpact,
            UsageZone::Yellow,
        ))
        .expect("domain-language run");

    assert_eq!(summary.state, "AwaitingApproval");
    assert_eq!(summary.artifact_count, 11);
    assert!(summary.approval_targets.iter().any(|target| target == "gate:risk"));
    assert!(summary.artifact_paths.iter().any(|path| path.ends_with("language-overview.md")));
    assert!(
        summary.artifact_paths.iter().any(|path| path.ends_with("language-decision-record.md"))
    );

    let mode_result = summary.mode_result.as_ref().expect("mode result");
    assert_eq!(mode_result.execution_posture.as_deref(), Some("recommendation-only"));
    assert_eq!(mode_result.primary_artifact_title, "Language Overview");
    assert_eq!(mode_result.headline, "Domain-language packet is publishable for governed review.");
    assert!(
        mode_result
            .artifact_packet_summary
            .contains("Packet defines 2 glossary term(s) and 2 language conflict(s).")
    );
    assert!(mode_result.artifact_packet_summary.contains("stabilizing"));
    assert!(mode_result.primary_artifact_path.ends_with("domain-language/01-language-overview.md"));

    let pre_approval_publish = service
        .publish(&summary.run_id, None, false)
        .expect("awaiting-approval domain-language packet should publish");
    let pre_approval_leaf = default_publish_leaf(&summary.run_id, "domain-language");
    assert!(
        pre_approval_publish
            .published_to
            .ends_with(&format!("tech-docs/domain/language/{pre_approval_leaf}"))
    );
    assert!(
        pre_approval_publish
            .published_files
            .iter()
            .any(|path| path.ends_with("language-overview.md"))
    );
    assert!(
        pre_approval_publish
            .published_files
            .iter()
            .any(|path| path.ends_with("packet-metadata.json"))
    );

    let approval = service
        .approve(
            &summary.run_id,
            "gate:risk",
            "domain-lead",
            ApprovalDecision::Approve,
            "bounded domain-language packet accepted for governed review",
        )
        .expect("gate approval");
    assert_eq!(approval.state, "Completed");

    let status = service.status(&summary.run_id).expect("status");
    assert_eq!(status.state, "Completed");
    assert!(status.approval_targets.is_empty());
    assert_eq!(
        status.mode_result.as_ref().map(|result| result.primary_artifact_title.as_str()),
        Some("Language Overview")
    );
    assert_eq!(
        status.mode_result.as_ref().and_then(|result| result.execution_posture.as_deref()),
        Some("recommendation-only")
    );

    let project_memory_publish = publish_run_with_profile(
        workspace.path(),
        &summary.run_id,
        PublishProfile::ProjectMemory,
        None,
    )
    .expect("project-memory publish should succeed");
    assert_eq!(project_memory_publish.published_to, "tech-docs/project/domain-language.md");

    let metadata_path = workspace
        .path()
        .join("tech-docs")
        .join("project")
        .join("domain-language.packet-metadata.json");
    let metadata: serde_json::Value = serde_json::from_slice(
        &fs::read(&metadata_path).expect("read project-memory domain-language metadata"),
    )
    .expect("parse domain-language project-memory metadata");
    assert_eq!(metadata["publication_target_class"], "stable");
    assert_eq!(metadata["expertise_input"]["expertise_kind"], "domain-language");
    assert_eq!(metadata["expertise_input"]["domain_families"][0], "systems");
    assert_eq!(metadata["lineage"]["promotion_state"], "auto");

    let published = service.publish(&summary.run_id, None, false).expect("publish should succeed");
    let leaf = default_publish_leaf(&summary.run_id, "domain-language");
    assert!(published.published_to.ends_with(&format!("tech-docs/domain/language/{leaf}")));
    assert!(published.published_files.iter().any(|path| path.ends_with("language-overview.md")));
    assert!(published.published_files.iter().any(|path| path.ends_with("packet-metadata.json")));

    let published_overview = workspace
        .path()
        .join("tech-docs")
        .join("domain")
        .join("language")
        .join(&leaf)
        .join("01-language-overview.md");
    assert!(published_overview.exists());
    let overview_contents = fs::read_to_string(published_overview).expect("published overview");
    assert!(overview_contents.contains("## Domain Scope"));
}

#[test]
fn domain_language_direct_run_exposes_blocked_gate_and_missing_body_markers() {
    let workspace = TempDir::new().expect("temp dir");
    init_domain_repo(&workspace);
    fs::write(workspace.path().join("domain-language.md"), incomplete_domain_language_brief())
        .expect("brief file");

    let service = EngineService::new(workspace.path());
    let summary = service
        .run(domain_language_request(
            "domain-language.md",
            RiskClass::BoundedImpact,
            UsageZone::Yellow,
        ))
        .expect("domain-language run");

    assert_eq!(summary.state, "Blocked");
    assert_eq!(summary.blocking_classification.as_deref(), Some("artifact-blocked"));
    assert!(summary.blocked_gates.iter().any(|gate| gate.gate == "architecture"));

    let mode_result = summary.mode_result.as_ref().expect("mode result");
    assert_eq!(mode_result.execution_posture.as_deref(), Some("recommendation-only"));
    assert_eq!(mode_result.primary_artifact_title, "Language Overview");
    assert!(mode_result.headline.contains("explicit missing-context marker(s)"));
    assert!(mode_result.artifact_packet_summary.contains("missing-context marker(s)"));

    let glossary = fs::read_to_string(
        workspace
            .path()
            .join(".canon")
            .join("artifacts")
            .join(&summary.run_id)
            .join("domain-language")
            .join("02-domain-glossary.md"),
    )
    .expect("glossary artifact");
    assert!(glossary.contains("## Missing Authored Body"));
    assert!(glossary.contains("`## Glossary Entries`"));

    let status = service.status(&summary.run_id).expect("status");
    assert_eq!(status.state, "Blocked");
    assert!(status.blocked_gates.iter().any(|gate| gate.gate == "architecture"));
}

#[test]
fn domain_model_direct_run_exercises_service_summary_json_and_publish_paths() {
    let workspace = TempDir::new().expect("temp dir");
    init_domain_repo(&workspace);
    fs::write(workspace.path().join("domain-model.md"), complete_domain_model_brief())
        .expect("brief file");

    let service = EngineService::new(workspace.path());
    let summary = service
        .run(domain_model_request("domain-model.md", RiskClass::SystemicImpact, UsageZone::Yellow))
        .expect("domain-model run");

    assert_eq!(summary.state, "AwaitingApproval");
    assert_eq!(summary.artifact_count, 14);
    assert!(summary.approval_targets.iter().any(|target| target == "gate:risk"));
    assert!(summary.artifact_paths.iter().any(|path| path.ends_with("model-overview.md")));
    assert!(summary.artifact_paths.iter().any(|path| path.ends_with("domain-model.json")));

    let mode_result = summary.mode_result.as_ref().expect("mode result");
    assert_eq!(mode_result.execution_posture.as_deref(), Some("recommendation-only"));
    assert_eq!(mode_result.primary_artifact_title, "Model Overview");
    assert_eq!(mode_result.headline, "Domain-model packet is publishable for governed review.");
    assert!(
        mode_result
            .artifact_packet_summary
            .contains("Packet defines 2 concept(s), 2 relationship(s), and 2 invariant(s).")
    );
    assert!(mode_result.artifact_packet_summary.contains("evolving"));
    assert!(mode_result.primary_artifact_path.ends_with("domain-model/01-model-overview.md"));

    let domain_model_json = fs::read_to_string(
        workspace
            .path()
            .join(".canon")
            .join("artifacts")
            .join(&summary.run_id)
            .join("domain-model")
            .join("12-domain-model.json"),
    )
    .expect("domain-model.json artifact");
    let domain_model_value: serde_json::Value =
        serde_json::from_str(&domain_model_json).expect("domain-model json");
    assert_eq!(domain_model_value["schema_version"].as_str(), Some("1"));
    assert!(
        domain_model_value["domain_scope"]
            .as_str()
            .is_some_and(|scope| scope.contains("order fulfillment"))
    );

    let pre_approval_publish = service
        .publish(&summary.run_id, None, false)
        .expect("awaiting-approval domain-model packet should publish");
    let pre_approval_leaf = default_publish_leaf(&summary.run_id, "domain-model");
    assert!(
        pre_approval_publish
            .published_to
            .ends_with(&format!("tech-docs/domain/model/{pre_approval_leaf}"))
    );
    assert!(
        pre_approval_publish.published_files.iter().any(|path| path.ends_with("model-overview.md"))
    );
    assert!(
        pre_approval_publish.published_files.iter().any(|path| path.ends_with("domain-model.json"))
    );
    assert!(
        pre_approval_publish
            .published_files
            .iter()
            .any(|path| path.ends_with("packet-metadata.json"))
    );

    let approval = service
        .approve(
            &summary.run_id,
            "gate:risk",
            "domain-architect",
            ApprovalDecision::Approve,
            "bounded domain-model packet accepted for governed review",
        )
        .expect("gate approval");
    assert_eq!(approval.state, "Completed");

    let status = service.status(&summary.run_id).expect("status");
    assert_eq!(status.state, "Completed");
    assert!(status.approval_targets.is_empty());
    assert_eq!(
        status.mode_result.as_ref().map(|result| result.primary_artifact_title.as_str()),
        Some("Model Overview")
    );
    assert_eq!(
        status.mode_result.as_ref().and_then(|result| result.execution_posture.as_deref()),
        Some("recommendation-only")
    );

    let project_memory_publish = publish_run_with_profile(
        workspace.path(),
        &summary.run_id,
        PublishProfile::ProjectMemory,
        None,
    )
    .expect("project-memory publish should succeed");
    assert_eq!(project_memory_publish.published_to, "tech-docs/project/domain-model.md");

    let metadata_path = workspace
        .path()
        .join("tech-docs")
        .join("project")
        .join("domain-model.packet-metadata.json");
    let metadata: serde_json::Value = serde_json::from_slice(
        &fs::read(&metadata_path).expect("read project-memory domain-model metadata"),
    )
    .expect("parse domain-model project-memory metadata");
    assert_eq!(metadata["publication_target_class"], "stable");
    assert_eq!(metadata["expertise_input"]["expertise_kind"], "domain-model");
    assert_eq!(metadata["expertise_input"]["domain_families"][0], "systems");
    assert_eq!(metadata["lineage"]["promotion_state"], "auto");

    let published = service.publish(&summary.run_id, None, false).expect("publish should succeed");
    let leaf = default_publish_leaf(&summary.run_id, "domain-model");
    assert!(published.published_to.ends_with(&format!("tech-docs/domain/model/{leaf}")));
    assert!(published.published_files.iter().any(|path| path.ends_with("model-overview.md")));
    assert!(published.published_files.iter().any(|path| path.ends_with("domain-model.json")));
    assert!(published.published_files.iter().any(|path| path.ends_with("packet-metadata.json")));

    let published_overview = workspace
        .path()
        .join("tech-docs")
        .join("domain")
        .join("model")
        .join(&leaf)
        .join("01-model-overview.md");
    assert!(published_overview.exists());
    let overview_contents = fs::read_to_string(published_overview).expect("published overview");
    assert!(overview_contents.contains("## Domain Scope"));
}

#[test]
fn domain_model_direct_run_exposes_blocked_gate_and_missing_body_markers() {
    let workspace = TempDir::new().expect("temp dir");
    init_domain_repo(&workspace);
    fs::write(workspace.path().join("domain-model.md"), incomplete_domain_model_brief())
        .expect("brief file");

    let service = EngineService::new(workspace.path());
    let summary = service
        .run(domain_model_request("domain-model.md", RiskClass::BoundedImpact, UsageZone::Yellow))
        .expect("domain-model run");

    assert_eq!(summary.state, "Blocked");
    assert_eq!(summary.blocking_classification.as_deref(), Some("artifact-blocked"));
    assert!(summary.blocked_gates.iter().any(|gate| gate.gate == "architecture"));

    let mode_result = summary.mode_result.as_ref().expect("mode result");
    assert_eq!(mode_result.execution_posture.as_deref(), Some("recommendation-only"));
    assert_eq!(mode_result.primary_artifact_title, "Model Overview");
    assert!(mode_result.headline.contains("explicit missing-context marker(s)"));
    assert!(mode_result.artifact_packet_summary.contains("missing-context marker(s)"));

    let relationship_map = fs::read_to_string(
        workspace
            .path()
            .join(".canon")
            .join("artifacts")
            .join(&summary.run_id)
            .join("domain-model")
            .join("03-relationship-map.md"),
    )
    .expect("relationship-map artifact");
    assert!(relationship_map.contains("## Missing Authored Body"));
    assert!(relationship_map.contains("`## Relationships`"));

    let status = service.status(&summary.run_id).expect("status");
    assert_eq!(status.state, "Blocked");
    assert!(status.blocked_gates.iter().any(|gate| gate.gate == "architecture"));
}
