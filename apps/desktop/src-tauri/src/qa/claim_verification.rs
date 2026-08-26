use super::{
    claim_segments, compact, extract_citation_ids, grounding::is_factual_claim, natural_answer,
    EvidenceItem,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub const CLAIM_VERIFIER_VERSION: &str = "deterministic-claim-verifier-v2";
pub const ATOMIC_CLAIM_EXTRACTOR_VERSION: &str = "atomic-claim-extractor-v1";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ClaimType {
    KnowledgeFact,
    GeneralKnowledge,
    ReasonedInference,
    ResearchSuggestion,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VerificationStatus {
    Unverified,
    Supported,
    PartiallySupported,
    Contradicted,
    NotVerifiable,
    NotApplicable,
    Unavailable,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AtomicClaim {
    pub id: String,
    pub text: String,
    pub evidence_ids: Vec<String>,
    pub claim_type: ClaimType,
    pub verification_status: VerificationStatus,
    #[serde(default)]
    pub confidence: Option<f32>,
    pub verification_method: String,
    pub alignment_score: f64,
    pub reason: String,
}

pub type VerifiedClaim = AtomicClaim;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClaimVerificationReport {
    pub claim_extractor_version: String,
    pub verifier_version: String,
    pub verification_status: String,
    pub fallback: bool,
    pub claim_count: usize,
    pub supported_count: usize,
    pub partially_supported_count: usize,
    pub contradicted_count: usize,
    pub not_verifiable_count: usize,
    pub not_applicable_count: usize,
    pub general_knowledge_count: usize,
    pub reasoned_inference_count: usize,
    pub research_suggestion_count: usize,
    pub repaired_count: usize,
    pub claims: Vec<VerifiedClaim>,
}

pub fn extract_atomic_claims(answer: &str) -> Vec<AtomicClaim> {
    let body_end = answer_body_end(answer);
    claim_segments(&answer[..body_end])
        .into_iter()
        .flat_map(|segment| split_atomic_segment(&segment))
        .filter_map(|text| {
            let claim_type = classify_claim(&text);
            (is_factual_claim(&text) || claim_type != ClaimType::KnowledgeFact)
                .then_some((text, claim_type))
        })
        .enumerate()
        .map(|(index, (text, claim_type))| AtomicClaim {
            id: format!("C{}", index + 1),
            evidence_ids: extract_citation_ids(&text),
            text,
            claim_type,
            verification_status: VerificationStatus::Unverified,
            confidence: None,
            verification_method: "not_run".to_string(),
            alignment_score: 0.0,
            reason: "not_verified".to_string(),
        })
        .collect()
}

fn answer_body_end(answer: &str) -> usize {
    [
        answer.find(natural_answer::APPENDIX_HEADING),
        answer.find(super::MODEL_SUPPLEMENT_HEADING),
    ]
    .into_iter()
    .flatten()
    .min()
    .unwrap_or(answer.len())
}

fn split_atomic_segment(segment: &str) -> Vec<String> {
    const CONNECTORS: &[&str] = &[
        "，因为",
        "，因此",
        "，所以",
        "，从而",
        "，同时",
        "，并且",
        "，而且",
        "，但是",
        "，然而",
        "，而",
        ", because ",
        ", therefore ",
        ", so ",
        ", while ",
        ", but ",
    ];
    let mut clauses = vec![segment.trim().to_string()];
    for connector in CONNECTORS {
        let mut next = Vec::new();
        for clause in clauses {
            let Some(position) = clause.find(connector) else {
                next.push(clause);
                continue;
            };
            let separator_len = connector
                .chars()
                .next()
                .map(char::len_utf8)
                .unwrap_or_default();
            let left = clause[..position].trim();
            let right = clause[position + separator_len..].trim();
            if atomic_clause_is_informative(left)
                && atomic_clause_is_informative(right)
                && !is_uncertainty_qualifier(right)
            {
                next.push(left.to_string());
                next.push(right.to_string());
            } else {
                next.push(clause);
            }
        }
        clauses = next;
    }
    clauses
}

fn atomic_clause_is_informative(value: &str) -> bool {
    let without_citations = value
        .replace(['[', ']', '（', '）', '(', ')'], " ")
        .replace(|character: char| character.is_ascii_digit(), " ");
    without_citations
        .chars()
        .filter(|character| character.is_alphanumeric())
        .count()
        >= 4
}

fn is_uncertainty_qualifier(value: &str) -> bool {
    let lower = value.to_lowercase();
    [
        "但目前没有直接实验验证",
        "但是目前没有直接实验验证",
        "然而目前没有直接实验验证",
        "but there is no direct experimental evidence",
        "but direct experimental evidence is unavailable",
    ]
    .iter()
    .any(|prefix| lower.starts_with(prefix))
}

pub trait VerificationProvider {
    fn version(&self) -> &'static str;
    fn verify(
        &self,
        claim: &str,
        evidence: &[&EvidenceItem],
    ) -> Result<(VerificationStatus, f64, String), String>;
}

#[derive(Debug, Default)]
pub struct DeterministicClaimVerifier;

impl VerificationProvider for DeterministicClaimVerifier {
    fn version(&self) -> &'static str {
        CLAIM_VERIFIER_VERSION
    }

    fn verify(
        &self,
        claim: &str,
        evidence: &[&EvidenceItem],
    ) -> Result<(VerificationStatus, f64, String), String> {
        if evidence.is_empty() {
            return Ok((
                VerificationStatus::NotVerifiable,
                0.0,
                "no_aligned_evidence".to_string(),
            ));
        }

        let claim_features = lexical_features(claim);
        let evidence_text = evidence
            .iter()
            .map(|item| format!("{} {}", item.title, item.snippet))
            .collect::<Vec<_>>()
            .join(" ");
        let evidence_features = lexical_features(&evidence_text);
        let overlap = claim_features.intersection(&evidence_features).count();
        let score = if claim_features.is_empty() {
            0.0
        } else {
            overlap as f64 / claim_features.len() as f64
        };
        let claim_numbers = number_tokens(claim);
        let evidence_numbers = number_tokens(&evidence_text);
        if !claim_numbers.is_subset(&evidence_numbers) {
            return Ok((
                VerificationStatus::NotVerifiable,
                score,
                "numeric_detail_missing_from_evidence".to_string(),
            ));
        }
        if score >= 0.30 && has_negation(claim) != has_negation(&evidence_text) {
            return Ok((
                VerificationStatus::Contradicted,
                score,
                "negation_conflicts_with_evidence".to_string(),
            ));
        }
        if score >= 0.80 {
            Ok((
                VerificationStatus::Supported,
                score,
                "lexical_alignment_threshold".to_string(),
            ))
        } else if score >= 0.16 {
            Ok((
                VerificationStatus::PartiallySupported,
                score,
                "partial_lexical_alignment".to_string(),
            ))
        } else {
            Ok((
                VerificationStatus::NotVerifiable,
                score,
                "insufficient_claim_evidence_alignment".to_string(),
            ))
        }
    }
}

pub fn verify_and_repair(
    answer: &str,
    evidence: &[EvidenceItem],
) -> (String, ClaimVerificationReport) {
    verify_and_repair_with(answer, evidence, &DeterministicClaimVerifier)
}

pub fn verify_and_repair_with(
    answer: &str,
    evidence: &[EvidenceItem],
    provider: &dyn VerificationProvider,
) -> (String, ClaimVerificationReport) {
    let by_id = evidence
        .iter()
        .map(|item| (item.id.as_str(), item))
        .collect::<HashMap<_, _>>();
    let mut report = ClaimVerificationReport {
        claim_extractor_version: ATOMIC_CLAIM_EXTRACTOR_VERSION.to_string(),
        verifier_version: provider.version().to_string(),
        verification_status: "succeeded".to_string(),
        fallback: provider.version() == CLAIM_VERIFIER_VERSION,
        ..ClaimVerificationReport::default()
    };

    for mut claim in extract_atomic_claims(answer) {
        let claim_type = claim.claim_type;
        increment_type(&mut report, claim_type);
        let ids = claim.evidence_ids.clone();
        let aligned = ids
            .iter()
            .filter_map(|id| by_id.get(id.as_str()).copied())
            .collect::<Vec<_>>();

        let (verification_status, score, reason, method) =
            if claim_type == ClaimType::ResearchSuggestion {
                (
                    VerificationStatus::NotApplicable,
                    0.0,
                    "research_suggestion_not_evidence_claim".to_string(),
                    "claim_type_rule".to_string(),
                )
            } else if ids.is_empty() {
                (
                    VerificationStatus::NotVerifiable,
                    0.0,
                    "missing_explicit_evidence_mapping".to_string(),
                    "mapping_gate".to_string(),
                )
            } else if aligned.len() != ids.len() {
                (
                    VerificationStatus::NotVerifiable,
                    0.0,
                    "unknown_or_unavailable_evidence_id".to_string(),
                    "mapping_gate".to_string(),
                )
            } else if aligned.iter().all(|item| item.kind == "graph") {
                (
                    VerificationStatus::NotVerifiable,
                    0.0,
                    "graph_only_evidence_is_not_claim_support".to_string(),
                    "mapping_gate".to_string(),
                )
            } else {
                match provider.verify(&claim.text, &aligned) {
                    Ok((status, score, reason)) => (
                        status,
                        score,
                        reason,
                        "deterministic_lexical_heuristic".to_string(),
                    ),
                    Err(reason) => {
                        report.verification_status = "unavailable".to_string();
                        report.fallback = false;
                        report.claims.clear();
                        report.claim_count = 0;
                        return (answer.to_string(), report_with_reason(report, reason));
                    }
                }
            };

        increment_status(&mut report, verification_status);
        claim.verification_status = verification_status;
        claim.confidence = Some(score.clamp(0.0, 1.0) as f32);
        claim.verification_method = method;
        claim.alignment_score = score;
        claim.reason = reason;
        report.claims.push(claim);
    }
    report.claim_count = report.claims.len();

    let mut repaired = answer.to_string();
    for claim in &report.claims {
        let replacement = match claim.verification_status {
            VerificationStatus::Contradicted => {
                Some("当前证据与该陈述存在冲突，本轮不采纳该结论。")
            }
            VerificationStatus::NotVerifiable => Some("当前证据不足以核验该陈述。"),
            VerificationStatus::PartiallySupported => Some("现有证据仅部分支持："),
            VerificationStatus::Unverified
            | VerificationStatus::Unavailable
            | VerificationStatus::Supported
            | VerificationStatus::NotApplicable => None,
        };
        let Some(replacement) = replacement else {
            continue;
        };
        if claim.verification_status == VerificationStatus::PartiallySupported {
            if let Some(index) = repaired.find(&claim.text) {
                repaired.insert_str(index, replacement);
                report.repaired_count += 1;
            }
        } else if repaired.contains(&claim.text) {
            repaired = repaired.replacen(&claim.text, replacement, 1);
            report.repaired_count += 1;
        }
    }
    (repaired, report)
}

fn classify_claim(value: &str) -> ClaimType {
    if has_any(
        value,
        &[
            "建议",
            "可以考虑",
            "后续研究",
            "值得探索",
            "可尝试",
            "research suggestion",
        ],
    ) {
        ClaimType::ResearchSuggestion
    } else if has_any(
        value,
        &[
            "知识库之外",
            "通用知识",
            "一般而言",
            "通常来说",
            "general knowledge",
        ],
    ) {
        ClaimType::GeneralKnowledge
    } else if has_any(
        value,
        &[
            "可以推断",
            "可推知",
            "这意味着",
            "综合来看",
            "由此推测",
            "reasoned inference",
        ],
    ) {
        ClaimType::ReasonedInference
    } else {
        ClaimType::KnowledgeFact
    }
}

fn report_with_reason(
    mut report: ClaimVerificationReport,
    reason: String,
) -> ClaimVerificationReport {
    report.claims.push(VerifiedClaim {
        id: "provider".to_string(),
        text: String::new(),
        evidence_ids: Vec::new(),
        claim_type: ClaimType::KnowledgeFact,
        verification_status: VerificationStatus::NotVerifiable,
        confidence: None,
        verification_method: "provider_error".to_string(),
        alignment_score: 0.0,
        reason: compact(&reason, 120),
    });
    report
}

fn increment_type(report: &mut ClaimVerificationReport, claim_type: ClaimType) {
    match claim_type {
        ClaimType::KnowledgeFact => {}
        ClaimType::GeneralKnowledge => report.general_knowledge_count += 1,
        ClaimType::ReasonedInference => report.reasoned_inference_count += 1,
        ClaimType::ResearchSuggestion => report.research_suggestion_count += 1,
    }
}

fn increment_status(report: &mut ClaimVerificationReport, status: VerificationStatus) {
    match status {
        VerificationStatus::Unverified | VerificationStatus::Unavailable => {}
        VerificationStatus::Supported => report.supported_count += 1,
        VerificationStatus::PartiallySupported => report.partially_supported_count += 1,
        VerificationStatus::Contradicted => report.contradicted_count += 1,
        VerificationStatus::NotVerifiable => report.not_verifiable_count += 1,
        VerificationStatus::NotApplicable => report.not_applicable_count += 1,
    }
}

fn has_any(value: &str, needles: &[&str]) -> bool {
    let lower = value.to_lowercase();
    needles.iter().any(|needle| lower.contains(needle))
}

fn has_negation(value: &str) -> bool {
    has_any(
        value,
        &[
            "不", "未", "无", "没有", "并非", "不是", "not", "never", "without",
        ],
    )
}

fn number_tokens(value: &str) -> HashSet<String> {
    without_citation_ids(value)
        .split(|character: char| !character.is_ascii_digit() && character != '.')
        .filter(|token| token.chars().any(|character| character.is_ascii_digit()))
        .map(|token| token.trim_matches('.').to_string())
        .filter(|token| !token.is_empty())
        .collect()
}

fn lexical_features(value: &str) -> HashSet<String> {
    const STOP: &[&str] = &[
        "the",
        "and",
        "this",
        "that",
        "with",
        "from",
        "claim",
        "statement",
        "当前",
        "证据",
        "结论",
        "研究",
        "方法",
        "模型",
        "可以",
        "进行",
        "通过",
    ];
    let without_ids = without_citation_ids(value).to_lowercase();
    let mut features = without_ids
        .split(|character: char| !character.is_alphanumeric() && character != '-')
        .filter(|token| token.chars().count() >= 3 && !STOP.contains(token))
        .map(str::to_string)
        .collect::<HashSet<_>>();
    let chinese = without_ids
        .chars()
        .filter(|character| ('\u{4e00}'..='\u{9fff}').contains(character))
        .collect::<Vec<_>>();
    for window in chinese.windows(2) {
        let token = window.iter().collect::<String>();
        if !STOP.contains(&token.as_str()) {
            features.insert(token);
        }
    }
    features
}

fn without_citation_ids(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut output = String::with_capacity(value.len());
    let mut copied = 0;
    let mut index = 0;
    while index + 3 < bytes.len() {
        if bytes[index] == b'[' && bytes[index + 1] == b'E' {
            let mut end = index + 2;
            while bytes.get(end).is_some_and(u8::is_ascii_digit) {
                end += 1;
            }
            if end > index + 2 && bytes.get(end) == Some(&b']') {
                output.push_str(&value[copied..index]);
                copied = end + 1;
                index = end + 1;
                continue;
            }
        }
        index += 1;
    }
    output.push_str(&value[copied..]);
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct FrozenCases {
        schema_version: String,
        cases: Vec<FrozenCase>,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct FrozenCase {
        id: String,
        claim: String,
        evidence: String,
        expected_claim_type: ClaimType,
        expected_verification_status: VerificationStatus,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct AtomicCases {
        schema_version: String,
        case_count: usize,
        cases: Vec<AtomicCase>,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct AtomicCase {
        id: String,
        answer: String,
        expected_claim_types: Vec<ClaimType>,
        expected_evidence_ids: Vec<Vec<String>>,
    }

    fn evidence(snippet: &str) -> EvidenceItem {
        EvidenceItem {
            id: "E1".to_string(),
            kind: "paper".to_string(),
            tier: "primary_source".to_string(),
            title: "ROSE wireless charging".to_string(),
            snippet: snippet.to_string(),
            ..EvidenceItem::default()
        }
    }

    #[test]
    fn statuses_cover_supported_partial_contradiction_and_unsupported() {
        let verifier = DeterministicClaimVerifier;
        let source =
            evidence("ROSE uses particle swarm optimization for mobile charger scheduling.");
        let aligned = vec![&source];
        assert_eq!(
            verifier
                .verify("ROSE uses particle swarm optimization [E1].", &aligned)
                .unwrap()
                .0,
            VerificationStatus::Supported
        );
        assert_eq!(
            verifier
                .verify(
                    "ROSE uses particle swarm optimization and proves optimality [E1].",
                    &aligned
                )
                .unwrap()
                .0,
            VerificationStatus::PartiallySupported
        );
        assert_eq!(
            verifier
                .verify(
                    "ROSE does not use particle swarm optimization [E1].",
                    &aligned
                )
                .unwrap()
                .0,
            VerificationStatus::Contradicted
        );
        assert_eq!(
            verifier
                .verify("The moon is made of cheese [E1].", &aligned)
                .unwrap()
                .0,
            VerificationStatus::NotVerifiable
        );
    }

    #[test]
    fn claim_type_never_bypasses_evidence_mapping() {
        let answer = "一般而言，PSO guarantees a global optimum.";
        let (_, report) = verify_and_repair(answer, &[evidence("PSO is a heuristic method.")]);
        assert_eq!(report.claims[0].claim_type, ClaimType::GeneralKnowledge);
        assert_eq!(
            report.claims[0].verification_status,
            VerificationStatus::NotVerifiable
        );
        assert!(report.claims[0].evidence_ids.is_empty());
    }

    #[test]
    fn no_id_claim_is_not_bound_to_every_evidence_item() {
        let answer = "ROSE schedules a charger.";
        let (_, report) = verify_and_repair(answer, &[evidence("ROSE schedules a charger.")]);
        assert_eq!(report.not_verifiable_count, 1);
        assert!(report.claims[0].evidence_ids.is_empty());
        assert_eq!(report.claims[0].reason, "missing_explicit_evidence_mapping");
    }

    #[test]
    fn frozen_claim_status_matrix_passes() {
        let cases: FrozenCases = serde_json::from_str(include_str!(
            "../../../../../evals/claim-verification-cases.json"
        ))
        .expect("valid frozen claim verification cases");
        assert_eq!(cases.schema_version, "claim-verification-cases-v2");
        let verifier = DeterministicClaimVerifier;
        for case in cases.cases {
            let source = evidence(&case.evidence);
            let aligned = if case.evidence.is_empty() {
                Vec::new()
            } else {
                vec![&source]
            };
            let claim_type = classify_claim(&case.claim);
            let status = if claim_type == ClaimType::ResearchSuggestion {
                VerificationStatus::NotApplicable
            } else {
                verifier
                    .verify(&case.claim, &aligned)
                    .unwrap_or_else(|error| panic!("{}: {error}", case.id))
                    .0
            };
            assert_eq!(claim_type, case.expected_claim_type, "{}", case.id);
            assert_eq!(status, case.expected_verification_status, "{}", case.id);
        }
    }

    #[test]
    fn frozen_atomic_claim_matrix_blocks_claim_smuggling_and_preserves_local_citations() {
        let fixture: AtomicCases =
            serde_json::from_str(include_str!("../../../../../evals/atomic_claim_cases.json"))
                .expect("valid atomic claim cases");
        assert_eq!(fixture.schema_version, "qa-atomic-claim-cases-v1");
        assert_eq!(fixture.case_count, fixture.cases.len());
        assert!(fixture.cases.len() >= 50);
        for case in fixture.cases {
            let claims = extract_atomic_claims(&case.answer);
            let claim_types = claims
                .iter()
                .map(|claim| claim.claim_type)
                .collect::<Vec<_>>();
            let evidence_ids = claims
                .iter()
                .map(|claim| claim.evidence_ids.clone())
                .collect::<Vec<_>>();
            assert_eq!(claim_types, case.expected_claim_types, "{}", case.id);
            assert_eq!(evidence_ids, case.expected_evidence_ids, "{}", case.id);
            assert!(claims.iter().all(|claim| {
                claim.verification_status == VerificationStatus::Unverified
                    && claim.confidence.is_none()
            }));
        }
    }

    #[test]
    fn suggestion_reason_is_verified_as_two_independent_claims() {
        let answer = "建议采用 PSO，因为已有研究证明 PSO 总能获得全局最优。[E1]";
        let (_, report) = verify_and_repair(
            answer,
            &[evidence(
                "PSO is a heuristic and does not guarantee a global optimum.",
            )],
        );
        assert_eq!(report.claims.len(), 2);
        assert_eq!(report.claims[0].claim_type, ClaimType::ResearchSuggestion);
        assert_eq!(
            report.claims[0].verification_status,
            VerificationStatus::NotApplicable
        );
        assert_eq!(report.claims[1].claim_type, ClaimType::KnowledgeFact);
        assert_ne!(
            report.claims[1].verification_status,
            VerificationStatus::NotApplicable
        );
        assert_eq!(report.claims[1].evidence_ids, vec!["E1"]);
    }

    #[test]
    fn unsupported_factual_claim_is_removed_instead_of_becoming_verified() {
        let answer = "The moon is made of cheese [E1].";
        let (repaired, report) =
            verify_and_repair(answer, &[evidence("ROSE schedules a charger.")]);
        assert_eq!(report.not_verifiable_count, 1);
        assert_eq!(report.repaired_count, 1);
        assert_eq!(repaired, "当前证据不足以核验该陈述。");
    }

    #[test]
    fn provider_failure_is_explicitly_unavailable() {
        struct Failed;
        impl VerificationProvider for Failed {
            fn version(&self) -> &'static str {
                "failed-provider"
            }
            fn verify(
                &self,
                _: &str,
                _: &[&EvidenceItem],
            ) -> Result<(VerificationStatus, f64, String), String> {
                Err("provider timeout".to_string())
            }
        }
        let answer = "ROSE schedules a charger [E1].";
        let (unchanged, report) =
            verify_and_repair_with(answer, &[evidence("ROSE schedules a charger.")], &Failed);
        assert_eq!(unchanged, answer);
        assert_eq!(report.verification_status, "unavailable");
        assert_eq!(report.claims[0].reason, "provider timeout");
    }
}
