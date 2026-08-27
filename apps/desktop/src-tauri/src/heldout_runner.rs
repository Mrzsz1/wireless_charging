use crate::qa::{
    natural_visible_body_source, project_claim_after_repair, project_natural_visible_text,
    EvidenceItem, QaRunManifest,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashSet};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use uuid::Uuid;

pub const CONTRACT_SCHEMA_VERSION: &str = "qa-heldout-contract-v1";
pub const RUN_SCHEMA_VERSION: &str = "qa-heldout-run-v2";

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HeldoutContract {
    pub schema_version: String,
    pub allowed_types: Vec<String>,
    pub minimum_case_count: usize,
    pub dataset_role: String,
    pub split: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HeldoutCase {
    pub id: String,
    #[serde(rename = "type")]
    pub case_type: String,
    pub question: String,
    #[serde(flatten)]
    pub _metadata: BTreeMap<String, Value>,
}

#[derive(Debug, Clone)]
pub struct ValidatedDataset {
    pub version: String,
    pub cases_sha256: String,
    pub cases: Vec<HeldoutCase>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HeldoutRuntimeConfig {
    pub provider: String,
    pub model: String,
    pub reasoning_effort: String,
}

#[derive(Debug, Clone)]
pub struct HeldoutRunOptions {
    pub dataset: PathBuf,
    pub output_dir: PathBuf,
    pub repository: PathBuf,
    pub provider: String,
    pub model: String,
    pub reasoning_effort: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitSnapshot {
    pub commit: String,
}

#[derive(Debug, Clone)]
pub struct QaCaseAudit {
    pub answer: String,
    pub evidence: Vec<EvidenceItem>,
    pub run_manifest: QaRunManifest,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HeldoutAnswerClaim {
    pub claim_id: String,
    pub text: String,
    pub cited_evidence_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HeldoutCaseRunMetadata {
    pub schema_version: String,
    pub dataset_version: String,
    pub dataset_sha256: String,
    pub git_commit: String,
    pub runtime_id: String,
    pub provider: String,
    pub model: String,
    pub reasoning_effort: String,
    pub session_id: String,
    pub semantic_verifier_provider: String,
    pub semantic_verifier_model: String,
    pub reranker_model: String,
    pub embedding_model: String,
    pub knowledge_base_snapshot: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HeldoutCaseBundle {
    pub question: String,
    pub answer: String,
    pub answer_claims: Vec<HeldoutAnswerClaim>,
    pub evidence: Vec<EvidenceItem>,
    pub run_manifest: QaRunManifest,
    pub heldout_run: HeldoutCaseRunMetadata,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct HeldoutRunSummary {
    schema_version: String,
    status: String,
    dataset_version: String,
    dataset_sha256: String,
    git_commit: String,
    runtime_id: String,
    runtime: HeldoutRuntimeConfig,
    case_count: usize,
    case_ids: Vec<String>,
}

fn canonical_json(value: &Value, output: &mut String) -> Result<(), String> {
    match value {
        Value::Null => output.push_str("null"),
        Value::Bool(value) => output.push_str(if *value { "true" } else { "false" }),
        Value::Number(value) => output.push_str(&value.to_string()),
        Value::String(value) => output.push_str(
            &serde_json::to_string(value)
                .map_err(|error| format!("HELDOUT_DATASET_INVALID: {error}"))?,
        ),
        Value::Array(values) => {
            output.push('[');
            for (index, value) in values.iter().enumerate() {
                if index > 0 {
                    output.push(',');
                }
                canonical_json(value, output)?;
            }
            output.push(']');
        }
        Value::Object(values) => {
            output.push('{');
            let mut keys = values.keys().collect::<Vec<_>>();
            keys.sort();
            for (index, key) in keys.into_iter().enumerate() {
                if index > 0 {
                    output.push(',');
                }
                output.push_str(
                    &serde_json::to_string(key)
                        .map_err(|error| format!("HELDOUT_DATASET_INVALID: {error}"))?,
                );
                output.push(':');
                canonical_json(&values[key], output)?;
            }
            output.push('}');
        }
    }
    Ok(())
}

fn sha256_hex(bytes: impl AsRef<[u8]>) -> String {
    format!("{:x}", Sha256::digest(bytes.as_ref()))
}

fn load_json(path: &Path, label: &str) -> Result<Value, String> {
    let raw = fs::read_to_string(path).map_err(|error| format!("{label}_READ_FAILED: {error}"))?;
    serde_json::from_str(&raw).map_err(|error| format!("{label}_INVALID: {error}"))
}

pub fn load_contract(path: &Path) -> Result<HeldoutContract, String> {
    let contract = serde_json::from_value::<HeldoutContract>(load_json(path, "HELDOUT_CONTRACT")?)
        .map_err(|error| format!("HELDOUT_CONTRACT_INVALID: {error}"))?;
    if contract.schema_version != CONTRACT_SCHEMA_VERSION
        || contract.minimum_case_count < 30
        || contract.dataset_role != "production_accuracy"
        || contract.split != "heldout"
        || contract.allowed_types.is_empty()
        || contract
            .allowed_types
            .iter()
            .any(|value| value.trim().is_empty())
        || contract.allowed_types.iter().collect::<HashSet<_>>().len()
            != contract.allowed_types.len()
    {
        return Err("HELDOUT_CONTRACT_INVALID: schema_or_values".to_string());
    }
    Ok(contract)
}

pub fn load_and_validate_dataset(
    dataset_path: &Path,
    contract: &HeldoutContract,
) -> Result<ValidatedDataset, String> {
    let value = load_json(dataset_path, "HELDOUT_DATASET")?;
    let object = value
        .as_object()
        .ok_or_else(|| "HELDOUT_DATASET_INVALID: root_not_object".to_string())?;
    if object.get("dataset_role").and_then(Value::as_str) != Some(&contract.dataset_role)
        || object.get("split").and_then(Value::as_str) != Some(&contract.split)
        || object.get("status").and_then(Value::as_str) != Some("frozen")
    {
        return Err("HELDOUT_DATASET_INVALID: role_split_or_status".to_string());
    }
    let minimum = object
        .get("minimum_case_count")
        .and_then(Value::as_u64)
        .ok_or_else(|| "HELDOUT_DATASET_INVALID: minimum_case_count".to_string())?
        as usize;
    if minimum < contract.minimum_case_count {
        return Err("HELDOUT_DATASET_INVALID: minimum_case_count".to_string());
    }
    if let Some(allowed) = object
        .get("case_schema")
        .and_then(Value::as_object)
        .and_then(|schema| schema.get("allowed_types"))
    {
        let expected = serde_json::to_value(&contract.allowed_types)
            .map_err(|error| format!("HELDOUT_CONTRACT_INVALID: {error}"))?;
        if allowed != &expected {
            return Err("HELDOUT_DATASET_INVALID: allowed_types_drift".to_string());
        }
    }
    let cases_value = object
        .get("cases")
        .and_then(Value::as_array)
        .ok_or_else(|| "HELDOUT_DATASET_INVALID: cases".to_string())?;
    if cases_value.len() < minimum {
        return Err("HELDOUT_DATASET_INVALID: case_count".to_string());
    }
    let cases = cases_value
        .iter()
        .cloned()
        .map(|value| {
            serde_json::from_value::<HeldoutCase>(value)
                .map_err(|error| format!("HELDOUT_DATASET_INVALID: case_schema:{error}"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let allowed = contract.allowed_types.iter().collect::<HashSet<_>>();
    let mut ids = HashSet::new();
    for case in &cases {
        if case.id.trim().is_empty()
            || case.question.trim().is_empty()
            || !allowed.contains(&case.case_type)
            || !ids.insert(case.id.clone())
        {
            return Err(format!("HELDOUT_DATASET_INVALID: case={}", case.id));
        }
    }
    let curation = object
        .get("curation")
        .and_then(Value::as_object)
        .ok_or_else(|| "HELDOUT_DATASET_INVALID: curation".to_string())?;
    if curation.get("independent").and_then(Value::as_bool) != Some(true) {
        return Err("HELDOUT_DATASET_INVALID: independent".to_string());
    }
    let curator_hash = curation
        .get("curator_id_hash")
        .and_then(Value::as_str)
        .unwrap_or_default();
    if curator_hash.len() != 64
        || !curator_hash
            .bytes()
            .all(|value| value.is_ascii_digit() || (b'a'..=b'f').contains(&value))
    {
        return Err("HELDOUT_DATASET_INVALID: curator_id_hash".to_string());
    }
    if curation
        .get("frozen_at")
        .and_then(Value::as_str)
        .map_or(true, |value| value.trim().is_empty())
    {
        return Err("HELDOUT_DATASET_INVALID: frozen_at".to_string());
    }
    let mut canonical = String::new();
    canonical_json(
        object
            .get("cases")
            .ok_or_else(|| "HELDOUT_DATASET_INVALID: cases".to_string())?,
        &mut canonical,
    )?;
    let actual_hash = sha256_hex(canonical.as_bytes());
    if curation.get("cases_sha256").and_then(Value::as_str) != Some(actual_hash.as_str()) {
        return Err("HELDOUT_DATASET_INVALID: cases_sha256".to_string());
    }
    let version = object
        .get("version")
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| "HELDOUT_DATASET_INVALID: version".to_string())?;
    Ok(ValidatedDataset {
        version: version.to_string(),
        cases_sha256: actual_hash,
        cases,
    })
}

fn git_output(repository: &Path, arguments: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repository)
        .args(arguments)
        .output()
        .map_err(|error| format!("HELDOUT_GIT_FAILED: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "HELDOUT_GIT_FAILED: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

pub fn clean_git_snapshot(repository: &Path) -> Result<GitSnapshot, String> {
    let status = git_output(
        repository,
        &["status", "--porcelain", "--untracked-files=all"],
    )?;
    if !status.is_empty() {
        return Err("HELDOUT_GIT_DIRTY: official run requires a clean worktree".to_string());
    }
    let commit = git_output(repository, &["rev-parse", "HEAD"])?;
    if commit.len() != 40 || !commit.bytes().all(|value| value.is_ascii_hexdigit()) {
        return Err("HELDOUT_GIT_FAILED: invalid_commit".to_string());
    }
    Ok(GitSnapshot { commit })
}

fn evidence_integrity(audit: &QaCaseAudit) -> Result<HashSet<String>, String> {
    if audit.answer.trim().is_empty() || audit.evidence.is_empty() {
        return Err("HELDOUT_AUDIT_INVALID: empty_answer_or_evidence".to_string());
    }
    let evidence_ids = audit
        .evidence
        .iter()
        .map(|item| item.id.clone())
        .collect::<HashSet<_>>();
    if evidence_ids.len() != audit.evidence.len()
        || audit.run_manifest.evidence_checksums.len() != audit.evidence.len()
    {
        return Err("HELDOUT_AUDIT_INVALID: evidence_checksum_cardinality".to_string());
    }
    let checksum_by_id = audit
        .run_manifest
        .evidence_checksums
        .iter()
        .map(|checksum| (checksum.evidence_id.as_str(), checksum))
        .collect::<BTreeMap<_, _>>();
    for item in &audit.evidence {
        let checksum = checksum_by_id
            .get(item.id.as_str())
            .ok_or_else(|| "HELDOUT_AUDIT_INVALID: evidence_checksum_missing".to_string())?;
        let bytes = serde_json::to_vec(item)
            .map_err(|error| format!("HELDOUT_AUDIT_INVALID: evidence_serialize:{error}"))?;
        if checksum.sha256 != sha256_hex(bytes) || checksum.stable_source_id.trim().is_empty() {
            return Err("HELDOUT_AUDIT_INVALID: evidence_checksum_mismatch".to_string());
        }
    }
    Ok(evidence_ids)
}

fn build_bundle(
    case: &HeldoutCase,
    audit: QaCaseAudit,
    metadata: HeldoutCaseRunMetadata,
) -> Result<HeldoutCaseBundle, String> {
    let evidence_ids = evidence_integrity(&audit)?;
    let claims = &audit.run_manifest.claim_verifications;
    if claims.is_empty() || audit.run_manifest.answer_completeness.claim_count != claims.len() {
        return Err("HELDOUT_AUDIT_INVALID: claim_count".to_string());
    }
    let mut claim_ids = HashSet::new();
    let mut answer_cursor = 0;
    let visible_answer_body = natural_visible_body_source(&audit.answer);
    let answer_claims = claims
        .iter()
        .map(|claim| {
            if claim.id.trim().is_empty() {
                return Err("HELDOUT_AUDIT_INVALID: claim_projection:empty_claim_id".to_string());
            }
            if !claim_ids.insert(claim.id.clone()) {
                return Err(format!(
                    "HELDOUT_AUDIT_INVALID: claim_projection:duplicate_claim_id:claim={}",
                    claim.id
                ));
            }
            let repaired_text = project_claim_after_repair(claim);
            let canonical_text = project_natural_visible_text(&repaired_text);
            let visible_text = resolve_visible_claim_source(
                visible_answer_body,
                &canonical_text,
                &mut answer_cursor,
            )
            .map_err(|reason| {
                format!(
                    "HELDOUT_AUDIT_INVALID: claim_projection:{reason}:claim={}",
                    claim.id
                )
            })?;
            let unique_citation_count = claim.evidence_ids.iter().collect::<HashSet<_>>().len();
            if visible_text.is_empty() || !audit.answer.contains(&visible_text) {
                return Err(format!(
                    "HELDOUT_AUDIT_INVALID: claim_projection:visible_text_integrity:claim={}",
                    claim.id
                ));
            }
            if unique_citation_count != claim.evidence_ids.len() {
                return Err(format!(
                    "HELDOUT_AUDIT_INVALID: claim_projection:duplicate_citation_id:claim={}",
                    claim.id
                ));
            }
            if claim
                .evidence_ids
                .iter()
                .any(|id| !evidence_ids.contains(id))
            {
                return Err(format!(
                    "HELDOUT_AUDIT_INVALID: claim_projection:unknown_citation_id:claim={}",
                    claim.id
                ));
            }
            Ok(HeldoutAnswerClaim {
                claim_id: claim.id.clone(),
                text: visible_text,
                cited_evidence_ids: claim.evidence_ids.clone(),
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(HeldoutCaseBundle {
        question: case.question.clone(),
        answer: audit.answer,
        answer_claims,
        evidence: audit.evidence,
        run_manifest: audit.run_manifest,
        heldout_run: metadata,
    })
}

fn resolve_visible_claim_source(
    answer_body: &str,
    canonical_text: &str,
    cursor: &mut usize,
) -> Result<String, &'static str> {
    if canonical_text.is_empty() {
        return Err("empty_visible_projection");
    }
    if *cursor > answer_body.len() {
        return Err("cursor_out_of_range");
    }
    if let Some(relative) = answer_body[*cursor..].find(canonical_text) {
        let start = *cursor + relative;
        *cursor = start + canonical_text.len();
        return Ok(canonical_text.to_string());
    }
    if let Some(start) = answer_body.find(canonical_text) {
        // Adjacent or overlapping atomic claims can be collapsed by the
        // existing AnswerRepair into the same fixed visible sentence. Reuse
        // only an exact final-answer span; never relax containment or fuzz it.
        *cursor = (*cursor).max(start + canonical_text.len());
        return Ok(canonical_text.to_string());
    }

    // Claim extraction deliberately masks Markdown link destinations and
    // code/math literals. Match the remaining visible chunks in order, then
    // return the exact final Markdown source span so containment stays strict.
    let chunks = canonical_text.split_whitespace().collect::<Vec<_>>();
    if chunks.len() < 2 {
        return Err("single_visible_chunk_not_found");
    }
    let mut candidate_cursor = *cursor;
    let mut found_first_chunk = false;
    while let Some(first_relative) = answer_body[candidate_cursor..].find(chunks[0]) {
        found_first_chunk = true;
        let start = candidate_cursor + first_relative;
        let mut end = start + chunks[0].len();
        let mut matched = true;
        for chunk in chunks.iter().skip(1) {
            let remaining = &answer_body[end..];
            let line_end = remaining.find('\n').unwrap_or(remaining.len());
            let bounded_end = remaining
                .char_indices()
                .nth(2_048)
                .map(|(index, _)| index)
                .unwrap_or(remaining.len())
                .min(line_end);
            let Some(relative) = remaining[..bounded_end].find(chunk) else {
                matched = false;
                break;
            };
            if contains_claim_boundary(&remaining[..relative]) {
                matched = false;
                break;
            }
            end += relative + chunk.len();
        }
        if matched {
            *cursor = end;
            return Ok(answer_body[start..end].to_string());
        }
        candidate_cursor = start + chunks[0].len();
    }
    if found_first_chunk {
        Err("ordered_visible_chunks_not_found")
    } else {
        Err("first_visible_chunk_not_found")
    }
}

fn contains_claim_boundary(value: &str) -> bool {
    let mut characters = value.chars().peekable();
    while let Some(character) = characters.next() {
        if matches!(character, '。' | '！' | '？' | '!' | '?' | ';' | '；') {
            return true;
        }
        if character == '.' && characters.peek().map_or(true, |next| next.is_whitespace()) {
            return true;
        }
    }
    false
}

fn atomic_json<T: Serialize>(path: &Path, value: &T) -> Result<(), String> {
    let part = path.with_extension("json.part");
    let mut bytes = serde_json::to_vec_pretty(value)
        .map_err(|error| format!("HELDOUT_WRITE_FAILED: {error}"))?;
    bytes.push(b'\n');
    let mut file = File::create(&part).map_err(|error| format!("HELDOUT_WRITE_FAILED: {error}"))?;
    file.write_all(&bytes)
        .and_then(|_| file.sync_all())
        .map_err(|error| format!("HELDOUT_WRITE_FAILED: {error}"))?;
    fs::rename(&part, path).map_err(|error| format!("HELDOUT_WRITE_FAILED: {error}"))
}

fn runtime_id(config: &HeldoutRuntimeConfig) -> Result<String, String> {
    let value = serde_json::to_value(config)
        .map_err(|error| format!("HELDOUT_RUNTIME_INVALID: {error}"))?;
    let mut canonical = String::new();
    canonical_json(&value, &mut canonical)?;
    Ok(sha256_hex(canonical.as_bytes())[..16].to_string())
}

pub fn run_with_executor<F>(
    dataset: &ValidatedDataset,
    output_root: &Path,
    git: &GitSnapshot,
    config: &HeldoutRuntimeConfig,
    embedding_model: &str,
    mut executor: F,
) -> Result<PathBuf, String>
where
    F: FnMut(&HeldoutCase, &str) -> Result<QaCaseAudit, String>,
{
    let runtime_id = runtime_id(config)?;
    let parent = output_root.join(&dataset.cases_sha256).join(&git.commit);
    let final_path = parent.join(&runtime_id);
    let part_path = parent.join(format!(".{runtime_id}.part"));
    if final_path.exists() || part_path.exists() {
        return Err("HELDOUT_RUN_EXISTS: official run identity already exists".to_string());
    }
    fs::create_dir_all(&parent).map_err(|error| format!("HELDOUT_WRITE_FAILED: {error}"))?;
    fs::create_dir(&part_path).map_err(|error| format!("HELDOUT_WRITE_FAILED: {error}"))?;
    let mut case_ids = Vec::with_capacity(dataset.cases.len());
    let mut session_ids = HashSet::new();
    let mut observed_runtime: Option<(String, String, String, String, String, String)> = None;
    for case in &dataset.cases {
        let session_id = Uuid::new_v4().to_string();
        if !session_ids.insert(session_id.clone()) {
            return Err("HELDOUT_SESSION_COLLISION".to_string());
        }
        let audit = executor(case, &session_id)?;
        let metadata = HeldoutCaseRunMetadata {
            schema_version: RUN_SCHEMA_VERSION.to_string(),
            dataset_version: dataset.version.clone(),
            dataset_sha256: dataset.cases_sha256.clone(),
            git_commit: git.commit.clone(),
            runtime_id: runtime_id.clone(),
            provider: audit.run_manifest.provider.clone(),
            model: audit.run_manifest.model_resolved.clone(),
            reasoning_effort: config.reasoning_effort.clone(),
            session_id,
            semantic_verifier_provider: audit.run_manifest.verification_provider.clone(),
            semantic_verifier_model: audit.run_manifest.verification_model.clone(),
            reranker_model: audit.run_manifest.reranker_model.clone(),
            embedding_model: embedding_model.to_string(),
            knowledge_base_snapshot: audit.run_manifest.index_snapshot_id.clone(),
        };
        let current_runtime = (
            metadata.provider.clone(),
            metadata.model.clone(),
            metadata.semantic_verifier_provider.clone(),
            metadata.semantic_verifier_model.clone(),
            metadata.reranker_model.clone(),
            metadata.knowledge_base_snapshot.clone(),
        );
        if metadata.provider != config.provider
            || observed_runtime
                .as_ref()
                .is_some_and(|observed| observed != &current_runtime)
        {
            return Err("HELDOUT_RUNTIME_DRIFT: runtime changed between cases".to_string());
        }
        observed_runtime.get_or_insert(current_runtime);
        let bundle = build_bundle(case, audit, metadata)?;
        atomic_json(&part_path.join(format!("{}.json", case.id)), &bundle)?;
        case_ids.push(case.id.clone());
    }
    atomic_json(
        &part_path.join("run.json"),
        &HeldoutRunSummary {
            schema_version: RUN_SCHEMA_VERSION.to_string(),
            status: "complete".to_string(),
            dataset_version: dataset.version.clone(),
            dataset_sha256: dataset.cases_sha256.clone(),
            git_commit: git.commit.clone(),
            runtime_id,
            runtime: config.clone(),
            case_count: case_ids.len(),
            case_ids,
        },
    )?;
    fs::rename(&part_path, &final_path)
        .map_err(|error| format!("HELDOUT_WRITE_FAILED: {error}"))?;
    Ok(final_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::qa::{AnswerCompletenessValidation, EvidenceChecksum, VerifiedClaim};
    use serde_json::json;
    use tempfile::TempDir;

    fn contract() -> HeldoutContract {
        HeldoutContract {
            schema_version: CONTRACT_SCHEMA_VERSION.to_string(),
            allowed_types: vec!["direct_factual".to_string()],
            minimum_case_count: 30,
            dataset_role: "production_accuracy".to_string(),
            split: "heldout".to_string(),
        }
    }

    fn cases() -> Vec<Value> {
        (0..30)
            .map(|index| {
                json!({"id":format!("synthetic-{index:02}"),"type":"direct_factual","question":format!("Synthetic development fixture {index}")})
            })
            .collect()
    }

    fn dataset_value(status: &str, independent: bool) -> Value {
        let cases = cases();
        let mut canonical = String::new();
        canonical_json(&Value::Array(cases.clone()), &mut canonical).unwrap();
        json!({
            "version":"synthetic-v1","dataset_role":"production_accuracy","split":"heldout",
            "status":status,"minimum_case_count":30,
            "curation":{"independent":independent,"curator_id_hash":"d".repeat(64),"frozen_at":"2026-08-27T00:00:00Z","cases_sha256":sha256_hex(canonical)},
            "cases":cases
        })
    }

    fn write_dataset(root: &Path, value: &Value) -> PathBuf {
        let path = root.join("dataset.json");
        fs::write(&path, serde_json::to_vec(value).unwrap()).unwrap();
        path
    }

    fn fixture_audit(provider: &str, model: &str) -> QaCaseAudit {
        let evidence = EvidenceItem {
            id: "E1".to_string(),
            kind: "wiki".to_string(),
            tier: "direct".to_string(),
            title: "Synthetic evidence".to_string(),
            snippet: "Synthetic fixture only".to_string(),
            score: 1.0,
            rank: 1,
            ..EvidenceItem::default()
        };
        let digest = sha256_hex(serde_json::to_vec(&evidence).unwrap());
        let claim = serde_json::from_value::<VerifiedClaim>(json!({
            "id":"C1","text":"Synthetic claim [E1]","evidenceIds":["E1"],
            "claimType":"knowledge_fact","verificationStatus":"supported","confidence":1.0,
            "verificationMethod":"fixture","alignmentScore":1.0,"reason":"synthetic"
        }))
        .unwrap();
        let mut manifest = QaRunManifest {
            provider: provider.to_string(),
            model_resolved: model.to_string(),
            index_snapshot_id: "snapshot-fixture".to_string(),
            reranker_model: "reranker-fixture".to_string(),
            evidence_checksums: vec![EvidenceChecksum {
                evidence_id: "E1".to_string(),
                stable_source_id: "wiki:fixture".to_string(),
                sha256: digest,
            }],
            answer_completeness: AnswerCompletenessValidation {
                claim_count: 1,
                ..AnswerCompletenessValidation::default()
            },
            claim_verifications: vec![claim],
            ..QaRunManifest::default()
        };
        manifest.verification_provider = provider.to_string();
        manifest.verification_model = model.to_string();
        QaCaseAudit {
            answer:
                "Synthetic claim\n\n## 参考证据\n\n- [知识库 · Synthetic evidence](evidence:E1)\n"
                    .to_string(),
            evidence: vec![evidence],
            run_manifest: manifest,
        }
    }

    fn fixture_metadata() -> HeldoutCaseRunMetadata {
        HeldoutCaseRunMetadata {
            schema_version: RUN_SCHEMA_VERSION.to_string(),
            dataset_version: "synthetic-v1".to_string(),
            dataset_sha256: "b".repeat(64),
            git_commit: "a".repeat(40),
            runtime_id: "runtime".to_string(),
            provider: "fixture-provider".to_string(),
            model: "fixture-model".to_string(),
            reasoning_effort: "low".to_string(),
            session_id: Uuid::new_v4().to_string(),
            semantic_verifier_provider: String::new(),
            semantic_verifier_model: String::new(),
            reranker_model: String::new(),
            embedding_model: String::new(),
            knowledge_base_snapshot: String::new(),
        }
    }

    #[test]
    fn runner_rejects_unfrozen_non_independent_hash_and_count_failures() {
        let root = TempDir::new().unwrap();
        for (name, mut value) in [
            ("unfrozen", dataset_value("draft", true)),
            ("non-independent", dataset_value("frozen", false)),
            ("bad-hash", dataset_value("frozen", true)),
            ("too-small", dataset_value("frozen", true)),
        ] {
            if name == "bad-hash" {
                value["curation"]["cases_sha256"] = Value::String("0".repeat(64));
            } else if name == "too-small" {
                value["cases"] = Value::Array(vec![]);
            }
            let path = write_dataset(root.path(), &value);
            assert!(
                load_and_validate_dataset(&path, &contract()).is_err(),
                "{name}"
            );
        }
    }

    #[test]
    fn runner_rejects_duplicate_ids_and_noncanonical_types() {
        let root = TempDir::new().unwrap();
        for (name, mut value) in [
            ("duplicate", dataset_value("frozen", true)),
            ("type", dataset_value("frozen", true)),
        ] {
            if name == "duplicate" {
                let first_id = value["cases"][0]["id"].clone();
                value["cases"][1]["id"] = first_id;
            } else {
                value["cases"][0]["type"] = Value::String("solve".to_string());
            }
            let cases = value["cases"].clone();
            let mut canonical = String::new();
            canonical_json(&cases, &mut canonical).unwrap();
            value["curation"]["cases_sha256"] = Value::String(sha256_hex(canonical));
            let path = write_dataset(root.path(), &value);
            assert!(
                load_and_validate_dataset(&path, &contract()).is_err(),
                "{name}"
            );
        }
    }

    #[test]
    fn rust_contract_and_canonical_hash_match_the_shared_python_contract() {
        let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
        let loaded = load_contract(&repository.join("evals/heldout_contract.json")).unwrap();
        assert_eq!(loaded.allowed_types.len(), 10);
        let root = TempDir::new().unwrap();
        let path = write_dataset(root.path(), &dataset_value("frozen", true));
        let dataset = load_and_validate_dataset(&path, &contract()).unwrap();
        assert_eq!(
            dataset.cases_sha256,
            "5b85bd973950e35531f45cd4c898b998a80819ecf8855a4b7045a3610d20a7d2"
        );
    }

    #[test]
    fn runner_writes_atomic_isolated_bundles_and_refuses_existing_identity() {
        let root = TempDir::new().unwrap();
        let dataset_path = write_dataset(root.path(), &dataset_value("frozen", true));
        let dataset = load_and_validate_dataset(&dataset_path, &contract()).unwrap();
        let git = GitSnapshot {
            commit: "a".repeat(40),
        };
        let config = HeldoutRuntimeConfig {
            provider: "fixture-provider".to_string(),
            model: "fixture-model".to_string(),
            reasoning_effort: "low".to_string(),
        };
        let mut sessions = HashSet::new();
        let output = run_with_executor(
            &dataset,
            &root.path().join("runs"),
            &git,
            &config,
            "embedding-fixture",
            |_, session| {
                assert!(sessions.insert(session.to_string()));
                Ok(fixture_audit("fixture-provider", "fixture-model"))
            },
        )
        .unwrap();
        assert_eq!(sessions.len(), 30);
        assert!(output.join("run.json").is_file());
        assert_eq!(fs::read_dir(&output).unwrap().count(), 31);
        let bundle: Value =
            serde_json::from_slice(&fs::read(output.join("synthetic-00.json")).unwrap()).unwrap();
        assert_eq!(
            bundle
                .pointer("/heldoutRun/provider")
                .and_then(Value::as_str),
            Some("fixture-provider")
        );
        assert_eq!(
            bundle
                .pointer("/heldoutRun/schemaVersion")
                .and_then(Value::as_str),
            Some("qa-heldout-run-v2")
        );
        assert_eq!(
            bundle
                .pointer("/answerClaims/0/text")
                .and_then(Value::as_str),
            Some("Synthetic claim")
        );
        assert_eq!(
            bundle
                .pointer("/answerClaims/0/citedEvidenceIds/0")
                .and_then(Value::as_str),
            Some("E1")
        );
        assert_eq!(
            bundle.pointer("/heldoutRun/model").and_then(Value::as_str),
            Some("fixture-model")
        );
        assert_eq!(
            bundle
                .pointer("/heldoutRun/gitCommit")
                .and_then(Value::as_str),
            Some("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
        );
        assert_eq!(
            bundle
                .pointer("/heldoutRun/datasetSha256")
                .and_then(Value::as_str),
            Some(dataset.cases_sha256.as_str())
        );
        assert!(bundle
            .pointer("/heldoutRun/sessionId")
            .and_then(Value::as_str)
            .is_some_and(|value| Uuid::parse_str(value).is_ok()));
        assert!(!fs::read_dir(&output).unwrap().any(|entry| entry
            .unwrap()
            .path()
            .extension()
            .is_some_and(|value| value == "part")));
        assert!(run_with_executor(
            &dataset,
            &root.path().join("runs"),
            &git,
            &config,
            "embedding-fixture",
            |_, _| Ok(fixture_audit("fixture-provider", "fixture-model")),
        )
        .is_err());
    }

    #[test]
    fn runner_fails_when_resolved_runtime_changes_between_cases() {
        let root = TempDir::new().unwrap();
        let dataset_path = write_dataset(root.path(), &dataset_value("frozen", true));
        let dataset = load_and_validate_dataset(&dataset_path, &contract()).unwrap();
        let config = HeldoutRuntimeConfig {
            provider: "fixture-provider".to_string(),
            model: "fixture-model".to_string(),
            reasoning_effort: "low".to_string(),
        };
        let mut calls = 0;
        let result = run_with_executor(
            &dataset,
            &root.path().join("runs"),
            &GitSnapshot {
                commit: "a".repeat(40),
            },
            &config,
            "embedding-fixture",
            |_, _| {
                calls += 1;
                Ok(fixture_audit(
                    "fixture-provider",
                    if calls == 1 {
                        "fixture-model"
                    } else {
                        "drifted-model"
                    },
                ))
            },
        );
        assert!(result.is_err());
        assert_eq!(calls, 2);
    }

    #[test]
    fn runner_rejects_invalid_visible_claim_projection_and_checksum() {
        let case: HeldoutCase = serde_json::from_value(cases()[0].clone()).unwrap();

        let mut claim_bad = fixture_audit("fixture-provider", "fixture-model");
        claim_bad.answer = "different answer".to_string();
        assert!(build_bundle(&case, claim_bad, fixture_metadata())
            .unwrap_err()
            .contains("claim_projection:first_visible_chunk_not_found:claim=C1"));

        let mut unknown_citation = fixture_audit("fixture-provider", "fixture-model");
        unknown_citation.run_manifest.claim_verifications[0].evidence_ids = vec!["E99".to_string()];
        assert!(build_bundle(&case, unknown_citation, fixture_metadata())
            .unwrap_err()
            .contains("claim_projection:unknown_citation_id:claim=C1"));

        let mut duplicate_claim = fixture_audit("fixture-provider", "fixture-model");
        duplicate_claim
            .run_manifest
            .claim_verifications
            .push(duplicate_claim.run_manifest.claim_verifications[0].clone());
        duplicate_claim.run_manifest.answer_completeness.claim_count = 2;
        assert!(build_bundle(&case, duplicate_claim, fixture_metadata())
            .unwrap_err()
            .contains("claim_projection:duplicate_claim_id:claim=C1"));

        let mut empty_projection = fixture_audit("fixture-provider", "fixture-model");
        empty_projection.run_manifest.claim_verifications[0].text = "[E1]".to_string();
        assert!(build_bundle(&case, empty_projection, fixture_metadata())
            .unwrap_err()
            .contains("claim_projection:empty_visible_projection:claim=C1"));

        let mut checksum_bad = fixture_audit("fixture-provider", "fixture-model");
        checksum_bad.run_manifest.evidence_checksums[0].sha256 = "0".repeat(64);
        assert_eq!(
            build_bundle(&case, checksum_bad, fixture_metadata()).unwrap_err(),
            "HELDOUT_AUDIT_INVALID: evidence_checksum_mismatch"
        );
    }

    #[test]
    fn runner_projects_the_claim_text_that_answer_repair_left_visible() {
        let case: HeldoutCase = serde_json::from_value(cases()[0].clone()).unwrap();
        let mut audit = fixture_audit("fixture-provider", "fixture-model");
        audit.run_manifest.claim_verifications[0] =
            serde_json::from_value::<VerifiedClaim>(json!({
                "id":"C1","text":"Synthetic claim [E1]","evidenceIds":["E1"],
                "claimType":"knowledge_fact","verificationStatus":"not_verifiable","confidence":0.0,
                "verificationMethod":"fixture","alignmentScore":0.0,"reason":"synthetic"
            }))
            .unwrap();
        audit.answer = "当前证据不足以支持这一结论。\n\n## 参考证据\n\n- [知识库 · Synthetic evidence](evidence:E1)\n"
            .to_string();

        let bundle = build_bundle(&case, audit, fixture_metadata()).unwrap();

        assert_eq!(bundle.answer_claims[0].text, "当前证据不足以支持这一结论。");
        assert_eq!(bundle.answer_claims[0].cited_evidence_ids, vec!["E1"]);
    }

    #[test]
    fn runner_recovers_exact_final_markdown_for_masked_link_targets() {
        let answer = "[Paper](https://example.test/paper) supports charging.\n\n## 参考证据";
        let canonical = "[Paper](                          ) supports charging.";
        let mut cursor = 0;

        let projected = resolve_visible_claim_source(
            natural_visible_body_source(answer),
            canonical,
            &mut cursor,
        )
        .unwrap();

        assert_eq!(
            projected,
            "[Paper](https://example.test/paper) supports charging."
        );
        assert!(answer.contains(&projected));
    }

    #[test]
    fn source_recovery_skips_an_earlier_nonmatching_visible_chunk() {
        let answer =
            "[Paper](intro) summary. [Paper](https://example.test/paper) supports charging.";
        let canonical = "[Paper](                      ) supports charging.";
        let mut cursor = 0;

        let projected = resolve_visible_claim_source(answer, canonical, &mut cursor).unwrap();

        assert_eq!(
            projected,
            "[Paper](https://example.test/paper) supports charging."
        );
    }

    #[test]
    fn exact_visible_repair_sentence_can_be_shared_by_overlapping_claims() {
        let answer = "当前证据不足以支持这一结论。";
        let mut cursor = answer.len();

        let projected =
            resolve_visible_claim_source(answer, "当前证据不足以支持这一结论。", &mut cursor)
                .unwrap();

        assert_eq!(projected, answer);
        assert_eq!(cursor, answer.len());
    }

    #[test]
    fn runner_rejects_dirty_git_worktree() {
        let root = TempDir::new().unwrap();
        for args in [
            vec!["init"],
            vec!["config", "user.email", "fixture@example.test"],
            vec!["config", "user.name", "Fixture"],
        ] {
            assert!(Command::new("git")
                .arg("-C")
                .arg(root.path())
                .args(args)
                .status()
                .unwrap()
                .success());
        }
        fs::write(root.path().join("tracked.txt"), "fixture").unwrap();
        assert!(Command::new("git")
            .arg("-C")
            .arg(root.path())
            .args(["add", "."])
            .status()
            .unwrap()
            .success());
        assert!(Command::new("git")
            .arg("-C")
            .arg(root.path())
            .args(["commit", "-m", "fixture"])
            .status()
            .unwrap()
            .success());
        assert!(clean_git_snapshot(root.path()).is_ok());
        fs::write(root.path().join("dirty.txt"), "dirty").unwrap();
        assert!(clean_git_snapshot(root.path()).is_err());
    }
}
