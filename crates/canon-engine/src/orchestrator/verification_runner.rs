use time::OffsetDateTime;

use crate::domain::verification::{VerificationLayer, VerificationRecord};
use crate::review::critique::CritiqueNote;

pub fn requirements_verification_records(target_paths: &[String]) -> Vec<VerificationRecord> {
    requirements_critique_notes()
        .into_iter()
        .map(|note| VerificationRecord {
            layer: note.layer,
            target_paths: target_paths.to_vec(),
            disposition: note.summary,
            recorded_at: OffsetDateTime::now_utc(),
            request_ids: Vec::new(),
            validation_path_id: None,
            evidence_bundle: None,
        })
        .collect()
}

fn requirements_critique_notes() -> Vec<CritiqueNote> {
    vec![
        CritiqueNote {
            layer: VerificationLayer::SelfCritique,
            summary: "Self-critique recorded against the requirements artifact bundle.".to_string(),
        },
        CritiqueNote {
            layer: VerificationLayer::AdversarialCritique,
            summary:
                "Adversarial critique recorded to challenge hidden scope growth and weak tradeoffs."
                    .to_string(),
        },
    ]
}

pub fn brownfield_verification_records(
    layers: &[VerificationLayer],
    target_paths: &[String],
) -> Vec<VerificationRecord> {
    layers
        .iter()
        .copied()
        .map(|layer| VerificationRecord {
            layer,
            target_paths: target_paths.to_vec(),
            disposition: format!(
                "{} recorded against the brownfield artifact bundle.",
                layer_summary(layer)
            ),
            recorded_at: OffsetDateTime::now_utc(),
            request_ids: Vec::new(),
            validation_path_id: None,
            evidence_bundle: None,
        })
        .collect()
}

pub fn pr_review_verification_records(
    layers: &[VerificationLayer],
    target_paths: &[String],
) -> Vec<VerificationRecord> {
    mode_verification_records("pr-review", layers, target_paths)
}

pub fn analysis_verification_records(
    mode_name: &str,
    layers: &[VerificationLayer],
    target_paths: &[String],
) -> Vec<VerificationRecord> {
    mode_verification_records(mode_name, layers, target_paths)
}

fn mode_verification_records(
    mode_name: &str,
    layers: &[VerificationLayer],
    target_paths: &[String],
) -> Vec<VerificationRecord> {
    layers
        .iter()
        .copied()
        .map(|layer| VerificationRecord {
            layer,
            target_paths: target_paths.to_vec(),
            disposition: format!(
                "{} recorded against the {} artifact bundle.",
                layer_summary(layer),
                mode_name
            ),
            recorded_at: OffsetDateTime::now_utc(),
            request_ids: Vec::new(),
            validation_path_id: None,
            evidence_bundle: None,
        })
        .collect()
}

fn layer_summary(layer: VerificationLayer) -> &'static str {
    match layer {
        VerificationLayer::SelfCritique => "Self-critique",
        VerificationLayer::AdversarialCritique => "Adversarial critique",
        VerificationLayer::PeerReview => "Peer review",
        VerificationLayer::ArchitecturalReview => "Architectural review",
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::verification::VerificationLayer;

    use super::{
        analysis_verification_records, brownfield_verification_records,
        pr_review_verification_records, requirements_verification_records,
    };

    #[test]
    fn requirements_verification_records_emit_two_default_layers() {
        let targets = vec!["artifacts/run-1/requirements/problem-statement.md".to_string()];

        let records = requirements_verification_records(&targets);

        assert_eq!(records.len(), 2);
        assert_eq!(records[0].layer, VerificationLayer::SelfCritique);
        assert_eq!(records[1].layer, VerificationLayer::AdversarialCritique);
        assert_eq!(records[0].target_paths, targets);
    }

    #[test]
    fn brownfield_and_pr_review_verification_records_preserve_layers_and_targets() {
        let layers = vec![VerificationLayer::PeerReview, VerificationLayer::ArchitecturalReview];
        let targets = vec!["artifacts/run-1/pr-review/review-summary.md".to_string()];

        let brownfield = brownfield_verification_records(&layers, &targets);
        let pr_review = pr_review_verification_records(&layers, &targets);

        assert_eq!(brownfield.len(), 2);
        assert!(brownfield[0].disposition.contains("Peer review"));
        assert!(pr_review[1].disposition.contains("Architectural review"));
        assert_eq!(pr_review[0].target_paths, targets);
    }

    #[test]
    fn analysis_verification_records_use_the_supplied_mode_name() {
        let layers = vec![VerificationLayer::SelfCritique];
        let targets = vec!["artifacts/run-1/discovery/problem-map.md".to_string()];

        let records = analysis_verification_records("discovery", &layers, &targets);

        assert_eq!(records.len(), 1);
        assert!(records[0].disposition.contains("discovery artifact bundle"));
    }
}
