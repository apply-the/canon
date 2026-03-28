use crate::domain::execution::{
    DeniedInvocation, EvidenceBundle, GenerationPath, ValidationIndependenceAssessment,
    ValidationPath,
};

pub fn empty_evidence_bundle(run_id: &str) -> EvidenceBundle {
    EvidenceBundle {
        run_id: run_id.to_string(),
        generation_paths: Vec::new(),
        validation_paths: Vec::new(),
        denied_invocations: Vec::new(),
        trace_refs: Vec::new(),
        artifact_refs: Vec::new(),
        decision_refs: Vec::new(),
        approval_refs: Vec::new(),
    }
}

pub fn default_independence(target_id: &str) -> ValidationIndependenceAssessment {
    ValidationIndependenceAssessment {
        target_id: target_id.to_string(),
        sufficient: false,
        rationale: "independence has not been assessed yet".to_string(),
        supporting_refs: Vec::new(),
    }
}

pub fn attach_paths(
    bundle: &mut EvidenceBundle,
    generation_path: GenerationPath,
    validation_path: ValidationPath,
    denied: Vec<DeniedInvocation>,
) {
    bundle.generation_paths.push(generation_path);
    bundle.validation_paths.push(validation_path);
    bundle.denied_invocations.extend(denied);
}

pub fn assess_validation_independence(
    generation_path: &GenerationPath,
    validation_path: &ValidationPath,
) -> ValidationIndependenceAssessment {
    let generation_has_ai = generation_path
        .lineage_classes
        .iter()
        .any(|lineage| matches!(lineage, canon_adapters::LineageClass::AiVendorFamily));
    let validation_has_human = validation_path
        .lineage_classes
        .iter()
        .any(|lineage| matches!(lineage, canon_adapters::LineageClass::HumanReview));
    let validation_is_distinct_non_ai = validation_path
        .lineage_classes
        .iter()
        .all(|lineage| !matches!(lineage, canon_adapters::LineageClass::AiVendorFamily));

    let sufficient = if generation_has_ai {
        validation_has_human || validation_is_distinct_non_ai
    } else {
        true
    };

    ValidationIndependenceAssessment {
        target_id: generation_path.path_id.clone(),
        sufficient,
        rationale: if sufficient {
            "validation path is independent enough to challenge the generation path".to_string()
        } else {
            "validation path shares too much lineage with the generation path".to_string()
        },
        supporting_refs: validation_path.verification_refs.clone(),
    }
}

#[cfg(test)]
mod tests {
    use canon_adapters::LineageClass;

    use super::{
        assess_validation_independence, attach_paths, default_independence, empty_evidence_bundle,
    };
    use crate::domain::execution::{GenerationPath, ValidationPath};

    #[test]
    fn default_independence_starts_unsatisfied() {
        let assessment = default_independence("generation:req-1");
        assert!(!assessment.sufficient);
    }

    #[test]
    fn attach_paths_records_generation_and_validation_entries() {
        let mut bundle = empty_evidence_bundle("run-1");
        let generation_path = GenerationPath {
            path_id: "generation:req-1".to_string(),
            request_ids: vec!["req-1".to_string()],
            lineage_classes: vec![LineageClass::AiVendorFamily],
            derived_artifacts: vec![
                "artifacts/run-1/requirements/problem-statement.md".to_string(),
            ],
        };
        let validation_path = ValidationPath {
            path_id: "validation:req-2".to_string(),
            request_ids: vec!["req-2".to_string()],
            lineage_classes: vec![LineageClass::AiVendorFamily],
            verification_refs: vec!["runs/run-1/invocations/req-2/decision.toml".to_string()],
            independence: default_independence("generation:req-1"),
        };

        attach_paths(&mut bundle, generation_path, validation_path, Vec::new());
        assert_eq!(bundle.generation_paths.len(), 1);
        assert_eq!(bundle.validation_paths.len(), 1);
    }

    #[test]
    fn human_review_validation_path_counts_as_independent_challenge() {
        let generation_path = GenerationPath {
            path_id: "generation:req-1".to_string(),
            request_ids: vec!["req-1".to_string()],
            lineage_classes: vec![LineageClass::AiVendorFamily],
            derived_artifacts: vec![
                "artifacts/run-1/requirements/problem-statement.md".to_string(),
            ],
        };
        let validation_path = ValidationPath {
            path_id: "validation:req-2".to_string(),
            request_ids: vec!["req-2".to_string()],
            lineage_classes: vec![LineageClass::HumanReview],
            verification_refs: vec!["runs/run-1/verification/verification-00.toml".to_string()],
            independence: default_independence("generation:req-1"),
        };

        let assessment = assess_validation_independence(&generation_path, &validation_path);
        assert!(assessment.sufficient);
    }
}
