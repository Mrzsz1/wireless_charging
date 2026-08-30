use super::{claim_verification, extract_citation_ids, EvidenceItem};
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};

pub const SCHEMA_VERSION: &str = "qa-direct-grounded-answer-v1";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct DirectGroundedAnswer {
    schema_version: String,
    claims: Vec<DirectGroundedClaim>,
    insufficient_evidence: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct DirectGroundedClaim {
    text: String,
    evidence_ids: Vec<String>,
}

pub fn provider_output_schema(evidence: &[EvidenceItem]) -> Value {
    let ids = evidence
        .iter()
        .map(|item| Value::String(item.id.clone()))
        .collect::<Vec<_>>();
    json!({
        "type": "object",
        "additionalProperties": false,
        "required": ["schemaVersion", "claims", "insufficientEvidence"],
        "properties": {
            "schemaVersion": { "type": "string", "enum": [SCHEMA_VERSION] },
            "claims": {
                "type": "array",
                "minItems": 0,
                "maxItems": 3,
                "items": {
                    "type": "object",
                    "additionalProperties": false,
                    "required": ["text", "evidenceIds"],
                    "properties": {
                        "text": { "type": "string" },
                        "evidenceIds": {
                            "type": "array",
                            "items": { "type": "string", "enum": ids }
                        }
                    }
                }
            },
            "insufficientEvidence": { "type": "boolean" }
        }
    })
}

fn invalid(reason: &str) -> String {
    format!("DIRECT_GROUNDED_ANSWER_INVALID: {reason}")
}

pub fn parse_validate_render(raw: &str, evidence: &[EvidenceItem]) -> Result<String, String> {
    let answer =
        serde_json::from_str::<DirectGroundedAnswer>(raw).map_err(|_| invalid("schema_or_json"))?;
    if answer.schema_version != SCHEMA_VERSION {
        return Err(invalid("schema_version"));
    }
    if answer.insufficient_evidence {
        return if answer.claims.is_empty() {
            Ok(super::grounding::NO_SUPPORTED_CLAIMS_NOTICE.to_string())
        } else {
            Err(invalid("insufficient_with_claims"))
        };
    }
    if answer.claims.is_empty() || answer.claims.len() > 3 {
        return Err(invalid("claim_count"));
    }
    let known = evidence
        .iter()
        .map(|item| (item.id.as_str(), item.kind.as_str()))
        .collect::<HashMap<_, _>>();
    let mut rendered = Vec::with_capacity(answer.claims.len());
    for claim in answer.claims {
        let text = claim.text.trim();
        if text.is_empty()
            || text.chars().count() > 1_200
            || text.contains('\r')
            || text.contains('\n')
            || text.starts_with('#')
            || !extract_citation_ids(text).is_empty()
        {
            return Err(invalid("claim_text"));
        }
        if claim.evidence_ids.is_empty()
            || claim.evidence_ids.iter().collect::<HashSet<_>>().len() != claim.evidence_ids.len()
            || claim
                .evidence_ids
                .iter()
                .any(|id| !known.contains_key(id.as_str()))
            || !claim
                .evidence_ids
                .iter()
                .any(|id| known.get(id.as_str()).is_some_and(|kind| *kind != "graph"))
        {
            return Err(invalid("evidence_ids"));
        }
        let citations = claim
            .evidence_ids
            .iter()
            .map(|id| format!("[{id}]"))
            .collect::<Vec<_>>()
            .join(" ");
        let line = format!("{text} {citations}");
        let atomic = claim_verification::extract_atomic_claims(&line);
        if atomic.len() != 1
            || atomic[0].claim_type == claim_verification::ClaimType::ResearchSuggestion
            || atomic[0].evidence_ids != claim.evidence_ids
        {
            return Err(invalid("claim_not_atomic"));
        }
        rendered.push(line);
    }
    Ok(rendered.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn evidence(id: &str, kind: &str) -> EvidenceItem {
        EvidenceItem {
            id: id.to_string(),
            kind: kind.to_string(),
            snippet: "The ROSE problem imposes probabilistic EMR safety constraints.".to_string(),
            ..EvidenceItem::default()
        }
    }

    #[test]
    fn direct_schema_parses_and_renders_natural_markdown_with_bound_ids() {
        let raw = json!({
            "schemaVersion": SCHEMA_VERSION,
            "claims": [{
                "text": "ROSE addresses charging under probabilistic EMR safety constraints.",
                "evidenceIds": ["E2", "E1"]
            }],
            "insufficientEvidence": false
        })
        .to_string();
        let rendered =
            parse_validate_render(&raw, &[evidence("E1", "wiki"), evidence("E2", "paper")])
                .unwrap();
        assert_eq!(
            rendered,
            "ROSE addresses charging under probabilistic EMR safety constraints. [E2] [E1]"
        );
    }

    #[test]
    fn direct_schema_rejects_unknown_empty_graph_only_and_too_many_claims() {
        let sources = [evidence("E1", "paper"), evidence("E2", "graph")];
        for (name, value) in [
            (
                "unknown",
                json!({"schemaVersion":SCHEMA_VERSION,"claims":[{"text":"Supported factual claim.","evidenceIds":["E99"]}],"insufficientEvidence":false}),
            ),
            (
                "empty",
                json!({"schemaVersion":SCHEMA_VERSION,"claims":[{"text":"Supported factual claim.","evidenceIds":[]}],"insufficientEvidence":false}),
            ),
            (
                "graph",
                json!({"schemaVersion":SCHEMA_VERSION,"claims":[{"text":"Supported factual claim.","evidenceIds":["E2"]}],"insufficientEvidence":false}),
            ),
            (
                "too-many",
                json!({"schemaVersion":SCHEMA_VERSION,"claims":[
                    {"text":"Supported factual claim one.","evidenceIds":["E1"]},
                    {"text":"Supported factual claim two.","evidenceIds":["E1"]},
                    {"text":"Supported factual claim three.","evidenceIds":["E1"]},
                    {"text":"Supported factual claim four.","evidenceIds":["E1"]}
                ],"insufficientEvidence":false}),
            ),
        ] {
            assert!(
                parse_validate_render(&value.to_string(), &sources).is_err(),
                "{name}"
            );
        }
    }

    #[test]
    fn direct_schema_requires_one_atomic_fact_and_has_explicit_insufficient_form() {
        let source = [evidence("E1", "paper")];
        let compound = json!({
            "schemaVersion":SCHEMA_VERSION,
            "claims":[{"text":"ROSE handles EMR safety. It also optimizes everything.","evidenceIds":["E1"]}],
            "insufficientEvidence":false
        });
        assert!(parse_validate_render(&compound.to_string(), &source).is_err());
        let insufficient = json!({
            "schemaVersion":SCHEMA_VERSION,
            "claims":[],
            "insufficientEvidence":true
        });
        assert_eq!(
            parse_validate_render(&insufficient.to_string(), &source).unwrap(),
            super::super::grounding::NO_SUPPORTED_CLAIMS_NOTICE
        );
    }
}
