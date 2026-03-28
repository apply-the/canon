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
    layers
        .iter()
        .copied()
        .map(|layer| VerificationRecord {
            layer,
            target_paths: target_paths.to_vec(),
            disposition: format!(
                "{} recorded against the pr-review artifact bundle.",
                layer_summary(layer)
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
