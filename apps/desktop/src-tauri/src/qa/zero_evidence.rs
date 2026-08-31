use super::{context::AnswerCompletenessValidation, grounding, natural_answer, EvidenceItem};
use serde::{Deserialize, Serialize};

pub const ZERO_EVIDENCE_AUDIT_SCHEMA_VERSION: &str = "qa-zero-evidence-audit-v1";
pub const NO_EVIDENCE_NOTICE: &str = "当前知识库没有检索到可用于核验这个问题的参考来源。下面如有分析，只能视为一般知识或假设性讨论，不能视为当前知识库结论。";
const LEGACY_NO_EVIDENCE_NOTICE: &str =
    "当前知识库没有检索到参考来源。以下内容来自模型的一般知识，未经本库证据核验。";
pub const ZERO_EVIDENCE_GENERAL_HEADING: &str = "## 一般知识参考（未经本库核验）";
pub const ZERO_EVIDENCE_NEXT_STEP_HEADING: &str = "## 建议下一步";
pub const ZERO_EVIDENCE_ENTITY_BOUNDARY: &str =
    "当前知识库无法确认问题中未被证据支持的命名对象、其定义或工作机制；以下内容不应解释为该对象真实存在。";

const FORBIDDEN_KB_ATTRIBUTIONS: &[&str] = &[
    "根据当前知识库",
    "当前知识库显示",
    "当前知识库表明",
    "本库证据",
    "本库文献显示",
    "according to the current knowledge base",
    "the current knowledge base shows",
];

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceAvailabilityMode {
    Grounded,
    PartialCoverage,
    #[default]
    ZeroUsableEvidence,
}

impl EvidenceAvailabilityMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Grounded => "grounded",
            Self::PartialCoverage => "partial_coverage",
            Self::ZeroUsableEvidence => "zero_usable_evidence",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceAvailability {
    pub mode: EvidenceAvailabilityMode,
    pub raw_evidence_count: usize,
    pub support_eligible_evidence_count: usize,
    pub graph_only_evidence_count: usize,
    pub required_facet_count: usize,
    pub covered_required_facet_count: usize,
    pub reason: String,
}

impl EvidenceAvailability {
    pub fn is_zero_usable(&self) -> bool {
        self.mode == EvidenceAvailabilityMode::ZeroUsableEvidence
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ZeroEvidenceProjection {
    pub markdown: String,
    pub removed_citation_ids: Vec<String>,
    pub fallback_applied: bool,
    pub fallback_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ZeroEvidenceAudit {
    pub schema_version: String,
    pub applicable: bool,
    pub status: String,
    pub availability_mode: String,
    pub reason: String,
    pub raw_evidence_count: usize,
    pub support_eligible_evidence_count: usize,
    pub graph_only_evidence_count: usize,
    pub notice_present: bool,
    pub notice_count: usize,
    pub visible_body_non_empty: bool,
    pub epistemic_boundary_present: bool,
    pub citation_token_count: usize,
    pub unknown_citation_count: usize,
    pub reference_appendix_present: bool,
    pub evidence_link_present: bool,
    pub forbidden_kb_attribution_count: usize,
    pub trusted_context_empty: bool,
    pub fallback_applied: bool,
    pub fallback_reason: String,
    pub complete: bool,
    pub error_codes: Vec<String>,
}

pub fn support_eligible_evidence(item: &EvidenceItem) -> bool {
    matches!(item.kind.as_str(), "paper" | "book" | "wiki")
}

pub fn classify_evidence_availability(
    evidence: &[EvidenceItem],
    required_facet_count: usize,
    covered_required_facet_count: usize,
) -> EvidenceAvailability {
    let support_eligible_evidence_count = evidence
        .iter()
        .filter(|item| support_eligible_evidence(item))
        .count();
    let graph_only_evidence_count = evidence.iter().filter(|item| item.kind == "graph").count();
    let covered_required_facet_count = covered_required_facet_count.min(required_facet_count);
    let (mode, reason) = if support_eligible_evidence_count == 0 {
        (
            EvidenceAvailabilityMode::ZeroUsableEvidence,
            if evidence.is_empty() {
                "no_selected_evidence"
            } else if graph_only_evidence_count == evidence.len() {
                "graph_only"
            } else {
                "no_support_eligible_evidence"
            },
        )
    } else if required_facet_count > covered_required_facet_count {
        (
            EvidenceAvailabilityMode::PartialCoverage,
            "required_facet_gap",
        )
    } else {
        (
            EvidenceAvailabilityMode::Grounded,
            "support_eligible_evidence",
        )
    };
    EvidenceAvailability {
        mode,
        raw_evidence_count: evidence.len(),
        support_eligible_evidence_count,
        graph_only_evidence_count,
        required_facet_count,
        covered_required_facet_count,
        reason: reason.to_string(),
    }
}

fn forbidden_kb_attribution_count(value: &str) -> usize {
    let normalized = value.to_lowercase();
    FORBIDDEN_KB_ATTRIBUTIONS
        .iter()
        .map(|phrase| normalized.matches(phrase).count())
        .sum()
}

fn remove_backend_owned_envelope(value: &str) -> String {
    let without_notices = value
        .replace(NO_EVIDENCE_NOTICE, "")
        .replace(LEGACY_NO_EVIDENCE_NOTICE, "")
        .replace(ZERO_EVIDENCE_ENTITY_BOUNDARY, "");
    without_notices
        .lines()
        .filter(|line| line.trim() != ZERO_EVIDENCE_GENERAL_HEADING)
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}

pub fn deterministic_zero_evidence_fallback() -> String {
    format!(
        "{NO_EVIDENCE_NOTICE}\n\n{ZERO_EVIDENCE_ENTITY_BOUNDARY}\n\n{ZERO_EVIDENCE_NEXT_STEP_HEADING}\n\n- 补充对应论文、书籍或资料到知识库；\n- 提供更准确的论文标题、作者、算法名或模型名；\n- 改为询问一个不依赖该未知对象的一般性无线充电调度问题。"
    )
}

pub fn project_zero_evidence_answer(answer: &str) -> ZeroEvidenceProjection {
    let (visible, mut removed_citation_ids) =
        natural_answer::project_visible_text_with_removed_ids(answer);
    removed_citation_ids.sort();
    removed_citation_ids.dedup();
    let forbidden_count = forbidden_kb_attribution_count(&visible);
    let deterministic_fallback_already_applied = visible
        .lines()
        .any(|line| line.trim() == ZERO_EVIDENCE_NEXT_STEP_HEADING);
    let body = remove_backend_owned_envelope(&visible);
    let fallback_reason = if forbidden_count > 0 {
        "forbidden_kb_attribution"
    } else if body.is_empty() {
        "empty_provider_body"
    } else {
        ""
    };
    if !fallback_reason.is_empty() {
        return ZeroEvidenceProjection {
            markdown: deterministic_zero_evidence_fallback(),
            removed_citation_ids,
            fallback_applied: true,
            fallback_reason: fallback_reason.to_string(),
        };
    }
    if deterministic_fallback_already_applied {
        return ZeroEvidenceProjection {
            markdown: deterministic_zero_evidence_fallback(),
            removed_citation_ids,
            fallback_applied: true,
            fallback_reason: "deterministic_fallback".to_string(),
        };
    }
    ZeroEvidenceProjection {
        markdown: format!(
            "{NO_EVIDENCE_NOTICE}\n\n{ZERO_EVIDENCE_ENTITY_BOUNDARY}\n\n{ZERO_EVIDENCE_GENERAL_HEADING}\n\n{body}"
        ),
        removed_citation_ids,
        fallback_applied: false,
        fallback_reason: String::new(),
    }
}

fn actionable_body_non_empty(answer: &str) -> bool {
    let body = answer
        .replace(NO_EVIDENCE_NOTICE, "")
        .replace(ZERO_EVIDENCE_ENTITY_BOUNDARY, "")
        .lines()
        .filter(|line| {
            let line = line.trim();
            !line.is_empty()
                && line != ZERO_EVIDENCE_GENERAL_HEADING
                && line != ZERO_EVIDENCE_NEXT_STEP_HEADING
        })
        .collect::<Vec<_>>()
        .join("\n");
    !body.trim().is_empty()
}

pub fn audit_zero_evidence_answer(
    answer: &str,
    availability: &EvidenceAvailability,
    unknown_citation_count: usize,
    trusted_context: &str,
    projection: Option<&ZeroEvidenceProjection>,
) -> ZeroEvidenceAudit {
    if !availability.is_zero_usable() {
        return ZeroEvidenceAudit {
            schema_version: ZERO_EVIDENCE_AUDIT_SCHEMA_VERSION.to_string(),
            applicable: false,
            status: "not_applicable".to_string(),
            availability_mode: availability.mode.as_str().to_string(),
            reason: availability.reason.clone(),
            raw_evidence_count: availability.raw_evidence_count,
            support_eligible_evidence_count: availability.support_eligible_evidence_count,
            graph_only_evidence_count: availability.graph_only_evidence_count,
            trusted_context_empty: trusted_context.is_empty(),
            complete: true,
            ..ZeroEvidenceAudit::default()
        };
    }
    let notice_count = answer.matches(NO_EVIDENCE_NOTICE).count();
    let citation_token_count = grounding::extract_citation_ids(answer).len();
    let reference_appendix_present = answer
        .lines()
        .any(|line| line.trim() == natural_answer::APPENDIX_HEADING);
    let evidence_link_present = answer.to_lowercase().contains("evidence:");
    let forbidden_kb_attribution_count = forbidden_kb_attribution_count(answer);
    let visible_body_non_empty = actionable_body_non_empty(answer);
    let epistemic_boundary_present = answer.contains(ZERO_EVIDENCE_ENTITY_BOUNDARY);
    let trusted_context_empty = trusted_context.is_empty();
    let mut error_codes = Vec::new();
    if notice_count == 0 {
        error_codes.push("ZERO_EVIDENCE_NOTICE_MISSING".to_string());
    } else if notice_count > 1 {
        error_codes.push("ZERO_EVIDENCE_NOTICE_DUPLICATED".to_string());
    }
    if !visible_body_non_empty {
        error_codes.push("ZERO_EVIDENCE_BODY_EMPTY".to_string());
    }
    if !epistemic_boundary_present {
        error_codes.push("ZERO_EVIDENCE_EPISTEMIC_BOUNDARY_MISSING".to_string());
    }
    if citation_token_count > 0 || unknown_citation_count > 0 {
        error_codes.push("ZERO_EVIDENCE_FAKE_CITATION".to_string());
    }
    if reference_appendix_present {
        error_codes.push("ZERO_EVIDENCE_REFERENCE_APPENDIX_PRESENT".to_string());
    }
    if evidence_link_present {
        error_codes.push("ZERO_EVIDENCE_EVIDENCE_LINK_PRESENT".to_string());
    }
    if forbidden_kb_attribution_count > 0 {
        error_codes.push("ZERO_EVIDENCE_KB_ATTRIBUTION".to_string());
    }
    if !trusted_context_empty {
        error_codes.push("ZERO_EVIDENCE_TRUSTED_CONTEXT_NONEMPTY".to_string());
    }
    let complete = error_codes.is_empty();
    ZeroEvidenceAudit {
        schema_version: ZERO_EVIDENCE_AUDIT_SCHEMA_VERSION.to_string(),
        applicable: true,
        status: if complete { "succeeded" } else { "failed" }.to_string(),
        availability_mode: availability.mode.as_str().to_string(),
        reason: availability.reason.clone(),
        raw_evidence_count: availability.raw_evidence_count,
        support_eligible_evidence_count: availability.support_eligible_evidence_count,
        graph_only_evidence_count: availability.graph_only_evidence_count,
        notice_present: notice_count > 0,
        notice_count,
        visible_body_non_empty,
        epistemic_boundary_present,
        citation_token_count,
        unknown_citation_count,
        reference_appendix_present,
        evidence_link_present,
        forbidden_kb_attribution_count,
        trusted_context_empty,
        fallback_applied: projection.is_some_and(|value| value.fallback_applied),
        fallback_reason: projection
            .map(|value| value.fallback_reason.clone())
            .unwrap_or_default(),
        complete,
        error_codes,
    }
}

pub fn validate_zero_evidence_completeness(
    audit: &ZeroEvidenceAudit,
) -> AnswerCompletenessValidation {
    const REQUIRED: &[&str] = &[
        "zero_evidence_notice",
        "visible_body",
        "epistemic_boundary",
        "no_evidence_citation",
        "no_reference_appendix",
        "no_kb_attribution",
        "trusted_context_empty",
    ];
    AnswerCompletenessValidation {
        applicable: audit.applicable,
        required_sections: Vec::new(),
        missing_sections: Vec::new(),
        required_elements: REQUIRED.iter().map(|value| (*value).to_string()).collect(),
        missing_elements: audit
            .error_codes
            .iter()
            .map(|code| code.to_lowercase())
            .collect(),
        claim_count: 0,
        minimum_claim_count: 0,
        complete: audit.applicable && audit.complete,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn evidence(kind: &str) -> EvidenceItem {
        EvidenceItem {
            id: "E1".to_string(),
            kind: kind.to_string(),
            ..EvidenceItem::default()
        }
    }

    fn zero_availability() -> EvidenceAvailability {
        classify_evidence_availability(&[], 0, 0)
    }

    #[test]
    fn z1_to_z6_projection_owns_notice_and_removes_fake_provenance() {
        let raw = format!(
            "{NO_EVIDENCE_NOTICE}\n\n{NO_EVIDENCE_NOTICE}\n\n分析 [E1]。[link](evidence:E1)\n\n## 参考证据\n- [E1]"
        );
        let projected = project_zero_evidence_answer(&raw);
        assert_eq!(projected.markdown.matches(NO_EVIDENCE_NOTICE).count(), 1);
        assert!(projected.markdown.contains(ZERO_EVIDENCE_GENERAL_HEADING));
        assert!(!projected.markdown.contains("[E1]"));
        assert!(!projected.markdown.contains("evidence:E1"));
        assert!(!projected
            .markdown
            .contains(natural_answer::APPENDIX_HEADING));
        assert_eq!(projected.removed_citation_ids, ["E1"]);
    }

    #[test]
    fn z7_and_z8_empty_or_false_attribution_uses_safe_fallback() {
        let empty = project_zero_evidence_answer("");
        assert!(empty.fallback_applied);
        assert_eq!(empty.fallback_reason, "empty_provider_body");
        assert!(empty.markdown.contains(ZERO_EVIDENCE_NEXT_STEP_HEADING));
        let projected_again = project_zero_evidence_answer(&empty.markdown);
        assert_eq!(projected_again.markdown, empty.markdown);
        assert!(projected_again.fallback_applied);

        let false_attribution =
            project_zero_evidence_answer("根据当前知识库，QTC-9 使用潮汐引力。");
        assert!(false_attribution.fallback_applied);
        assert_eq!(
            false_attribution.fallback_reason,
            "forbidden_kb_attribution"
        );
        assert!(!false_attribution.markdown.contains("根据当前知识库"));
    }

    #[test]
    fn z9_and_z10_graph_only_is_zero_but_graph_plus_paper_is_grounded() {
        let graph_only = classify_evidence_availability(&[evidence("graph")], 0, 0);
        assert_eq!(
            graph_only.mode,
            EvidenceAvailabilityMode::ZeroUsableEvidence
        );
        assert_eq!(graph_only.support_eligible_evidence_count, 0);
        assert_eq!(graph_only.graph_only_evidence_count, 1);
        assert_eq!(graph_only.reason, "graph_only");

        let grounded =
            classify_evidence_availability(&[evidence("graph"), evidence("paper")], 0, 0);
        assert_eq!(grounded.mode, EvidenceAvailabilityMode::Grounded);
        assert_eq!(grounded.support_eligible_evidence_count, 1);
    }

    #[test]
    fn z11_zero_evidence_audit_fails_closed_on_trusted_context() {
        let projection = project_zero_evidence_answer("一般分析。");
        let audit = audit_zero_evidence_answer(
            &projection.markdown,
            &zero_availability(),
            0,
            "must-not-be-trusted",
            Some(&projection),
        );
        assert!(!audit.complete);
        assert!(audit
            .error_codes
            .contains(&"ZERO_EVIDENCE_TRUSTED_CONTEXT_NONEMPTY".to_string()));
    }

    #[test]
    fn z14_partial_coverage_and_z15_no_supported_claims_remain_nonzero_modes() {
        let partial = classify_evidence_availability(&[evidence("wiki")], 2, 1);
        assert_eq!(partial.mode, EvidenceAvailabilityMode::PartialCoverage);
        assert_eq!(partial.reason, "required_facet_gap");

        let grounded_without_claims = classify_evidence_availability(&[evidence("paper")], 0, 0);
        assert_eq!(
            grounded_without_claims.mode,
            EvidenceAvailabilityMode::Grounded
        );
        let audit = audit_zero_evidence_answer(
            grounding::NO_SUPPORTED_CLAIMS_NOTICE,
            &grounded_without_claims,
            0,
            "",
            None,
        );
        assert!(!audit.applicable);
        assert_eq!(audit.status, "not_applicable");
    }

    #[test]
    fn zero_completeness_requires_actionable_body_and_has_zero_claim_minimum() {
        let incomplete =
            audit_zero_evidence_answer(NO_EVIDENCE_NOTICE, &zero_availability(), 0, "", None);
        let incomplete = validate_zero_evidence_completeness(&incomplete);
        assert!(!incomplete.complete);
        assert_eq!(incomplete.minimum_claim_count, 0);

        let projection = project_zero_evidence_answer("");
        let complete = audit_zero_evidence_answer(
            &projection.markdown,
            &zero_availability(),
            0,
            "",
            Some(&projection),
        );
        let complete = validate_zero_evidence_completeness(&complete);
        assert!(complete.complete, "{complete:?}");
        assert_eq!(complete.minimum_claim_count, 0);
    }
}
