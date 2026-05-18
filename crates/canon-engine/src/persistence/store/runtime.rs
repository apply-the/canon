use super::*;

impl WorkspaceStore {
    pub fn list_artifact_files(&self, run_id: &str) -> Result<Vec<String>, Error> {
        let run_artifacts_root = self.layout.artifacts_dir().join(run_id);
        if !run_artifacts_root.exists() {
            return Ok(Vec::new());
        }

        let mut mode_dirs = fs::read_dir(run_artifacts_root)?.collect::<Result<Vec<_>, _>>()?;
        mode_dirs.sort_by_key(|entry| entry.file_name());

        let mut entries = Vec::new();
        for mode_dir in mode_dirs {
            let mode_name = mode_dir.file_name().to_string_lossy().into_owned();
            let mode = mode_name.parse::<Mode>().map_err(|error| {
                Error::new(
                    ErrorKind::InvalidData,
                    format!(
                        "artifact manifest directory `{mode_name}` is not a supported mode: {error}"
                    ),
                )
            })?;
            let manifest_path = mode_dir.path().join("manifest.toml");
            if !manifest_path.exists() {
                continue;
            }

            let manifest: ArtifactManifest = read_toml_file(manifest_path)?;
            for record in manifest.records {
                validate_run_artifact_record(&record, run_id, mode)?;
                entries.push(format!(".canon/{}", record.relative_path));
            }
        }

        Ok(entries)
    }

    /// Lists all invocation request IDs for a run.
    pub fn list_invocation_ids(&self, run_id: &str) -> Result<Vec<String>, Error> {
        list_invocation_ids(&self.layout.run_dir(run_id))
    }

    /// Returns the names of evidence files for a run (empty if none captured).
    pub fn list_evidence_entries(&self, run_id: &str) -> Result<Vec<String>, Error> {
        let evidence_path = self.layout.run_dir(run_id).join("evidence.toml");
        if evidence_path.exists() { Ok(vec!["evidence.toml".to_string()]) } else { Ok(Vec::new()) }
    }

    /// Loads the evidence bundle for a run, returning `None` if not yet captured.
    pub fn load_evidence_bundle(&self, run_id: &str) -> Result<Option<EvidenceBundle>, Error> {
        let path = self.layout.run_evidence_path(run_id);
        if path.exists() { read_toml_file(path).map(Some) } else { Ok(None) }
    }

    /// Loads and reassembles all invocation records for a run.
    pub fn load_persisted_invocations(
        &self,
        run_id: &str,
    ) -> Result<Vec<PersistedInvocation>, Error> {
        let run_dir = self.layout.run_dir(run_id);
        let mut invocations = Vec::new();

        for request_id in list_invocation_ids(&run_dir)? {
            let request = read_toml_file(request_path(&run_dir, &request_id))?;
            let decision = read_toml_file(decision_path(&run_dir, &request_id))?;
            let invocation_root = invocation_dir(&run_dir, &request_id);
            let mut attempts = fs::read_dir(&invocation_root)?
                .filter_map(|entry| match entry {
                    Ok(entry) if entry.file_name().to_string_lossy().starts_with("attempt-") => {
                        Some(read_toml_file(entry.path()))
                    }
                    Ok(_) => None,
                    Err(error) => Some(Err(error)),
                })
                .collect::<Result<Vec<_>, _>>()?;
            attempts.sort_by_key(|attempt: &crate::domain::execution::InvocationAttempt| {
                attempt.attempt_number
            });

            let approvals = self
                .load_approval_records(run_id)?
                .into_iter()
                .filter(|approval| approval.matches_invocation(&request_id))
                .collect();

            invocations.push(PersistedInvocation { request, decision, attempts, approvals });
        }

        invocations.sort_by(|left, right| left.request.request_id.cmp(&right.request.request_id));
        Ok(invocations)
    }

    /// Loads all trace events for a run from the JSONL trace file.
    pub fn load_trace_events(&self, run_id: &str) -> Result<Vec<TraceEvent>, Error> {
        let trace_path = self.layout.traces_dir().join(format!("{run_id}.jsonl"));
        if !trace_path.exists() {
            return Ok(Vec::new());
        }

        let contents = fs::read_to_string(trace_path)?;
        parse_trace_events(&contents).map_err(|error| Error::other(error.to_string()))
    }

    /// Loads the current state manifest for a run.
    pub fn load_run_state(&self, run_id: &str) -> Result<RunStateManifest, Error> {
        read_toml_file(self.layout.run_dir(run_id).join("state.toml"))
    }

    /// Loads and canonicalizes the run manifest for a run.
    pub fn load_run_manifest(&self, run_id: &str) -> Result<RunManifest, Error> {
        let manifest: RunManifest = read_toml_file(self.layout.run_dir(run_id).join("run.toml"))?;
        Ok(manifest.canonicalize())
    }

    /// Loads the runtime context (inputs, adapter context) for a run.
    pub fn load_run_context(&self, run_id: &str) -> Result<RunContext, Error> {
        read_toml_file(self.layout.run_dir(run_id).join("context.toml"))
    }

    /// Loads the artifact contract (required output file specifications) for a run.
    pub fn load_artifact_contract(&self, run_id: &str) -> Result<ArtifactContract, Error> {
        read_toml_file(self.layout.run_dir(run_id).join("artifact-contract.toml"))
    }

    /// Loads all gate evaluations for a run, sorted by gate name.
    pub fn load_gate_evaluations(&self, run_id: &str) -> Result<Vec<GateEvaluation>, Error> {
        let mut gates = fs::read_dir(self.layout.run_gates_dir(run_id))?
            .map(|entry| {
                let entry = entry?;
                read_toml_file(entry.path())
            })
            .collect::<Result<Vec<_>, _>>()?;
        gates.sort_by_key(|gate: &GateEvaluation| gate.gate.as_str().to_string());
        Ok(gates)
    }

    /// Loads all approval records for a run, sorted by recorded-at timestamp.
    pub fn load_approval_records(&self, run_id: &str) -> Result<Vec<ApprovalRecord>, Error> {
        let approvals_dir = self.layout.run_approvals_dir(run_id);
        if !approvals_dir.exists() {
            return Ok(Vec::new());
        }

        let mut approvals = fs::read_dir(approvals_dir)?
            .map(|entry| {
                let entry = entry?;
                read_toml_file(entry.path())
            })
            .collect::<Result<Vec<_>, _>>()?;
        approvals.sort_by_key(|approval: &ApprovalRecord| approval.recorded_at);
        Ok(approvals)
    }

    /// Loads persisted artifacts from disk that satisfy the given artifact contract.
    pub fn load_persisted_artifacts(
        &self,
        run_id: &str,
        mode: Mode,
        contract: &ArtifactContract,
    ) -> Result<Vec<PersistedArtifact>, Error> {
        let artifact_root = self.layout.run_artifact_dir(run_id, mode);
        let manifest: ArtifactManifest = read_toml_file(artifact_root.join("manifest.toml"))?;

        contract
            .artifact_requirements
            .iter()
            .map(|requirement| {
                let Some(record) = manifest
                    .records
                    .iter()
                    .find(|record| record.file_name == requirement.file_name)
                    .cloned()
                else {
                    return if requirement.required {
                        Err(Error::other(format!(
                            "artifact `{}` missing from persisted manifest",
                            requirement.file_name
                        )))
                    } else {
                        Ok(None)
                    };
                };
                let path = artifact_storage_path(&self.layout, &record, run_id, mode)?;
                let contents = fs::read_to_string(path)?;
                Ok(Some(PersistedArtifact { record, contents }))
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|artifacts| artifacts.into_iter().flatten().collect())
    }

    /// Persists gate evaluations for a run, writing one TOML file per gate.
    pub fn persist_gate_evaluations(
        &self,
        run_id: &str,
        gates: &[GateEvaluation],
    ) -> Result<(), Error> {
        let gates_dir = self.layout.run_gates_dir(run_id);
        self.filesystem.create_dir_all(&gates_dir).map_err(adapter_error_to_io)?;
        let mut invocations = Vec::new();

        for gate in gates {
            let gate_path = gates_dir.join(format!("{}.toml", gate.gate.as_str()));
            write_toml_file(gate_path.clone(), gate)?;
            invocations.push(self.filesystem.trace_write(&gate_path, "persist gate evaluation"));
        }

        self.append_trace_stream(run_id, &invocations)?;

        Ok(())
    }

    /// Persists the current lifecycle state of a run.
    pub fn persist_run_state(&self, run_id: &str, state: &RunStateManifest) -> Result<(), Error> {
        let path = self.layout.run_dir(run_id).join("state.toml");
        write_toml_file(path.clone(), state)?;
        self.append_trace_stream(run_id, &[self.filesystem.trace_write(&path, "persist run state")])
    }

    /// Persists an approval record for a run.
    pub fn persist_approval_record(
        &self,
        run_id: &str,
        approval: &ApprovalRecord,
    ) -> Result<(), Error> {
        let approvals_dir = self.layout.run_approvals_dir(run_id);
        self.filesystem.create_dir_all(&approvals_dir).map_err(adapter_error_to_io)?;
        let next_index = fs::read_dir(&approvals_dir)?.count();
        let path = approvals_dir.join(format!("approval-{next_index:02}.toml"));
        write_toml_file(path.clone(), approval)?;
        self.append_trace_stream(
            run_id,
            &[self.filesystem.trace_write(&path, "persist approval record")],
        )
    }

    /// Appends adapter invocation records to the run's trace stream.
    pub fn persist_adapter_invocations(
        &self,
        run_id: &str,
        invocations: &[AdapterInvocation],
    ) -> Result<(), Error> {
        self.append_trace_stream(run_id, invocations)
    }

    pub(super) fn append_trace_stream(
        &self,
        run_id: &str,
        invocations: &[AdapterInvocation],
    ) -> Result<(), Error> {
        let events = invocations
            .iter()
            .map(|invocation| TraceEvent::from_adapter_invocation(run_id, invocation))
            .collect::<Vec<_>>();
        self.append_trace_events(run_id, &events)
    }

    pub(super) fn append_trace_events(
        &self,
        run_id: &str,
        events: &[TraceEvent],
    ) -> Result<(), Error> {
        if events.is_empty() {
            return Ok(());
        }

        let trace_path = self.layout.traces_dir().join(format!("{run_id}.jsonl"));
        let mut buffer = String::new();
        for event in events {
            let line =
                serde_json::to_string(event).map_err(|error| Error::other(error.to_string()))?;
            buffer.push_str(&line);
            buffer.push('\n');
        }

        use std::io::Write;
        let mut file = fs::OpenOptions::new().create(true).append(true).open(trace_path)?;
        file.write_all(buffer.as_bytes())?;
        Ok(())
    }
}
