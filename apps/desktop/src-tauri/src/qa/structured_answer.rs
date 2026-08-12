use super::{
    compact, CitationValidation, EvidenceItem, MODEL_SUPPLEMENT_HEADING, MODEL_SUPPLEMENT_NOTICE,
};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};

const SCHEMA_VERSION: &str = "qa-structured-answer-v1";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct StructuredAnswer {
    schema_version: String,
    sections: Vec<StructuredSection>,
    #[serde(default)]
    supplement: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct StructuredSection {
    title: String,
    groups: Vec<StructuredGroup>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct StructuredGroup {
    #[serde(default)]
    label: String,
    claims: Vec<StructuredClaim>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct StructuredClaim {
    #[serde(default)]
    label: String,
    text: String,
    evidence_ids: Vec<String>,
}

pub struct StructuredRenderResult {
    pub markdown: String,
    pub validation: CitationValidation,
}

fn json_payload(value: &str) -> &str {
    let trimmed = value.trim();
    if !trimmed.starts_with("```") {
        return trimmed;
    }
    let after_open = trimmed
        .find('\n')
        .map_or(trimmed, |index| &trimmed[index + 1..]);
    after_open
        .strip_suffix("```")
        .map(str::trim)
        .unwrap_or(after_open)
}

fn source_label(item: &EvidenceItem) -> String {
    let kind = match item.kind.as_str() {
        "paper" => "论文原文",
        "book" => "书籍来源",
        "wiki" => "知识库来源",
        _ => "图谱提示",
    };
    let location = if !item.source_location.trim().is_empty() {
        item.source_location.trim().to_string()
    } else if item.kind == "book" {
        match (item.physical_page_start, item.physical_page_end) {
            (Some(start), Some(end)) => format!("PDF p.{start}–{end}"),
            (Some(start), None) => format!("PDF p.{start}"),
            _ => "可打开定位".to_string(),
        }
    } else if !item.wikilink.trim().is_empty() {
        "知识库页面".to_string()
    } else {
        "可打开定位".to_string()
    };
    format!("{kind} · {location}")
}

pub fn parse_validate_render(
    raw: &str,
    intent: &str,
    evidence: &[EvidenceItem],
) -> Result<StructuredRenderResult, String> {
    let answer: StructuredAnswer = serde_json::from_str(json_payload(raw))
        .map_err(|error| format!("结构化回答不是有效 JSON：{error}"))?;
    if answer.schema_version != SCHEMA_VERSION {
        return Err(format!("结构化回答版本无效：{}", answer.schema_version));
    }
    if answer.sections.is_empty() {
        return Err("结构化回答没有 sections".to_string());
    }
    let expected_titles = super::context::required_answer_sections(intent)
        .into_iter()
        .map(|title| title.trim_start_matches("## ").to_string())
        .collect::<Vec<_>>();
    let actual_titles = answer
        .sections
        .iter()
        .map(|section| {
            section
                .title
                .trim()
                .trim_start_matches('#')
                .trim()
                .to_string()
        })
        .collect::<Vec<_>>();
    if actual_titles != expected_titles {
        return Err(format!(
            "结构化回答章节应为 [{}]，实际为 [{}]",
            expected_titles.join("、"),
            actual_titles.join("、")
        ));
    }

    let known = evidence
        .iter()
        .map(|item| (item.id.as_str(), item))
        .collect::<HashMap<_, _>>();
    let mut cited_ids = Vec::new();
    let mut unknown_ids = Vec::new();
    let mut unsupported_claims = Vec::new();
    let mut graph_only_claims = Vec::new();
    let mut claim_count = 0;
    let mut cited_claim_count = 0;
    let mut markdown = String::new();

    for section in &answer.sections {
        let title = section.title.trim().trim_start_matches('#').trim();
        if title.is_empty() || section.groups.is_empty() {
            return Err("section title/groups 为空".to_string());
        }
        markdown.push_str("## ");
        markdown.push_str(title);
        markdown.push_str("\n\n");
        for group in &section.groups {
            if !group.label.trim().is_empty() {
                markdown.push_str("### ");
                markdown.push_str(&compact(group.label.trim(), 52));
                markdown.push_str("\n\n");
            }
            if group.claims.is_empty() {
                return Err(format!("章节“{title}”包含空分组"));
            }
            for claim in &group.claims {
                claim_count += 1;
                let text = claim.text.trim();
                if text.is_empty() {
                    unsupported_claims.push("空声明".to_string());
                    continue;
                }
                let mut claim_known = Vec::new();
                for id in &claim.evidence_ids {
                    if let Some(item) = known.get(id.as_str()) {
                        claim_known.push(*item);
                        if !cited_ids.contains(id) {
                            cited_ids.push(id.clone());
                        }
                    } else if !unknown_ids.contains(id) {
                        unknown_ids.push(id.clone());
                    }
                }
                let has_non_graph = claim_known.iter().any(|item| item.kind != "graph");
                let graph_only = !claim_known.is_empty() && !has_non_graph;
                if graph_only {
                    graph_only_claims.push(compact(text, 180));
                }
                if claim.evidence_ids.is_empty()
                    || claim
                        .evidence_ids
                        .iter()
                        .any(|id| !known.contains_key(id.as_str()))
                    || !has_non_graph
                {
                    unsupported_claims.push(compact(text, 180));
                } else {
                    cited_claim_count += 1;
                }
                markdown.push_str("- ");
                if !claim.label.trim().is_empty() {
                    markdown.push_str("**");
                    markdown.push_str(claim.label.trim().trim_end_matches([':', '：']));
                    markdown.push_str("：** ");
                }
                markdown.push_str(text);
                for id in &claim.evidence_ids {
                    markdown.push(' ');
                    markdown.push_str(&format!("[{id}]"));
                }
                markdown.push('\n');
            }
            markdown.push('\n');
        }
    }

    markdown.push_str("## 参考证据\n\n");
    let cited_set = cited_ids.iter().collect::<HashSet<_>>();
    for item in evidence.iter().filter(|item| cited_set.contains(&item.id)) {
        markdown.push_str(&format!("- [{}] · {}\n", item.id, source_label(item)));
    }

    let model_supplement_claims = answer
        .supplement
        .iter()
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .map(|value| value.to_string())
        .collect::<Vec<_>>();
    if model_supplement_claims.iter().any(|claim| {
        claim.contains("[E")
            || claim.contains("[[")
            || claim.contains("原文第")
            || claim.contains("PDF p.")
    }) {
        return Err("模型补充包含证据编号或库内定位".to_string());
    }
    if !model_supplement_claims.is_empty() {
        markdown.push_str("\n");
        markdown.push_str(MODEL_SUPPLEMENT_HEADING);
        markdown.push_str("\n\n");
        markdown.push_str(MODEL_SUPPLEMENT_NOTICE);
        markdown.push_str("\n\n");
        for claim in &model_supplement_claims {
            markdown.push_str("- ");
            markdown.push_str(claim);
            markdown.push('\n');
        }
    }

    let syntax_valid = unknown_ids.is_empty();
    let coverage_valid =
        claim_count > 0 && cited_claim_count == claim_count && unsupported_claims.is_empty();
    let supported = syntax_valid && coverage_valid;
    let mixed = supported && !model_supplement_claims.is_empty();
    Ok(StructuredRenderResult {
        markdown,
        validation: CitationValidation {
            citation_precision: if cited_ids.is_empty() && unknown_ids.is_empty() {
                0.0
            } else {
                cited_ids.len() as f64 / (cited_ids.len() + unknown_ids.len()) as f64
            },
            has_citations: !cited_ids.is_empty(),
            supported,
            grounding_status: if mixed {
                "mixed"
            } else if supported {
                "supported"
            } else {
                "invalid"
            }
            .to_string(),
            zero_evidence: false,
            claim_count,
            cited_claim_count,
            citation_coverage: if claim_count == 0 {
                0.0
            } else {
                cited_claim_count as f64 / claim_count as f64
            },
            cited_ids,
            unknown_ids,
            unsupported_claims,
            graph_only_claims,
            syntax_valid,
            coverage_valid,
            entailment_checked: false,
            model_supplement_claim_count: model_supplement_claims.len(),
            model_supplement_claims,
        },
    })
}

pub fn invalid_validation(error: String) -> CitationValidation {
    CitationValidation {
        grounding_status: "invalid".to_string(),
        claim_count: 1,
        unsupported_claims: vec![compact(&error, 180)],
        syntax_valid: false,
        ..CitationValidation::default()
    }
}
