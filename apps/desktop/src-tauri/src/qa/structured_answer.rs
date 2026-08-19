use super::{
    compact, CitationValidation, EvidenceItem, MODEL_SUPPLEMENT_HEADING, MODEL_SUPPLEMENT_NOTICE,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};

const SCHEMA_VERSION: &str = super::context::ANSWER_SCHEMA_VERSION;

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
    #[serde(default)]
    id: String,
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
    role: String,
    #[serde(default)]
    label: String,
    text: String,
    evidence_ids: Vec<String>,
}

#[derive(Debug)]
pub struct StructuredRenderResult {
    pub markdown: String,
    pub validation: CitationValidation,
    pub roles: Vec<String>,
}

pub fn complete_example(intent: &str) -> Value {
    let sections = super::context::required_answer_section_contract(intent);
    let roles = super::context::required_answer_role_contract(intent);
    let section_count = sections.len().max(1);
    let fallback_role = roles.first().copied();
    let rendered_sections = sections
        .iter()
        .enumerate()
        .map(|(section_index, section)| {
            let mut assigned_roles = roles
                .iter()
                .enumerate()
                .filter(|(role_index, _)| role_index % section_count == section_index)
                .map(|(_, role)| *role)
                .collect::<Vec<_>>();
            if assigned_roles.is_empty() {
                if let Some(role) = fallback_role {
                    assigned_roles.push(role);
                }
            }
            let claims = assigned_roles
                .into_iter()
                .map(|role| {
                    json!({
                        "role": role.id,
                        "label": format!("示例：{}", role.title),
                        "text": format!("示例占位：这里填写由当前证据支持的“{}”完整陈述。", role.title),
                        "evidenceIds": ["E1"]
                    })
                })
                .collect::<Vec<_>>();
            json!({
                "id": section.id,
                "title": section.title,
                "groups": [{
                    "label": format!("示例分组 {}", section_index + 1),
                    "claims": claims
                }]
            })
        })
        .collect::<Vec<_>>();
    json!({
        "schemaVersion": SCHEMA_VERSION,
        "sections": rendered_sections,
        "supplement": []
    })
}

pub fn provider_output_schema(intent: &str, evidence: &[EvidenceItem]) -> Value {
    let section_contract = super::context::required_answer_section_contract(intent);
    let section_ids = section_contract
        .iter()
        .map(|section| section.id)
        .collect::<Vec<_>>();
    let section_titles = section_contract
        .iter()
        .map(|section| section.title)
        .collect::<Vec<_>>();
    let roles = super::context::required_answer_role_contract(intent)
        .into_iter()
        .map(|role| role.id)
        .collect::<Vec<_>>();
    let evidence_ids = evidence
        .iter()
        .map(|item| item.id.as_str())
        .collect::<Vec<_>>();

    json!({
        "type": "object",
        "additionalProperties": false,
        "required": ["schemaVersion", "sections", "supplement"],
        "properties": {
            "schemaVersion": {
                "type": "string",
                "enum": [SCHEMA_VERSION]
            },
            "sections": {
                "type": "array",
                "items": {
                    "type": "object",
                    "additionalProperties": false,
                    "required": ["id", "title", "groups"],
                    "properties": {
                        "id": {
                            "type": "string",
                            "enum": section_ids
                        },
                        "title": {
                            "type": "string",
                            "enum": section_titles
                        },
                        "groups": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "additionalProperties": false,
                                "required": ["label", "claims"],
                                "properties": {
                                    "label": {"type": "string"},
                                    "claims": {
                                        "type": "array",
                                        "items": {
                                            "type": "object",
                                            "additionalProperties": false,
                                            "required": ["role", "label", "text", "evidenceIds"],
                                            "properties": {
                                                "role": {
                                                    "type": "string",
                                                    "enum": roles
                                                },
                                                "label": {"type": "string"},
                                                "text": {"type": "string"},
                                                "evidenceIds": {
                                                    "type": "array",
                                                    "items": {
                                                        "type": "string",
                                                        "enum": evidence_ids
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "supplement": {
                "type": "array",
                "items": {"type": "string"}
            }
        }
    })
}

fn legacy_role(intent: &str, label: &str) -> Option<&'static str> {
    let label = label.trim().trim_end_matches([':', '：']);
    let aliases: &[(&str, &[&str])] = match intent {
        "literature" => &[
            ("paper_title", &["论文标题"]),
            ("question_relevance", &["与问题的关系"]),
            (
                "model_or_method",
                &[
                    "模型或方法",
                    "模型与方法",
                    "模型",
                    "方法",
                    "求解方法",
                    "优化方法",
                    "控制对象",
                    "动态输入",
                    "复杂度处理",
                    "安全判据",
                    "概率化建模",
                    "部署变量",
                    "优化目标",
                    "信号叠加",
                ],
            ),
            (
                "evidence_boundary",
                &["证据边界", "模型边界", "本轮证据范围"],
            ),
            ("source_location", &["来源定位", "复现定位"]),
        ],
        "novelty" => &[
            ("coverage_matrix", &["覆盖矩阵"]),
            ("covered_topics", &["已覆盖主题"]),
            ("evidence_gap", &["证据缺口"]),
            ("knowledge_boundary", &["当前知识库边界"]),
        ],
        "relationship" => &[
            ("common_object", &["共同对象"]),
            ("assumptions", &["假设"]),
            ("objectives", &["目标"]),
            ("constraints", &["约束"]),
            ("algorithm_mechanism", &["算法机制"]),
            ("guarantees", &["保证"]),
            ("cost", &["代价"]),
            ("applicable_scenario", &["适用场景"]),
        ],
        _ => &[
            ("research_object", &["研究对象"]),
            ("variables", &["变量"]),
            ("objective", &["目标函数"]),
            ("constraints", &["约束"]),
            ("solution_steps", &["求解步骤"]),
            ("guarantee", &["可证明保证"]),
            ("failure_boundary", &["失效边界"]),
        ],
    };
    let exact = aliases
        .iter()
        .find_map(|(role, labels)| labels.contains(&label).then_some(*role));
    if exact.is_some() {
        return exact;
    }
    if intent == "literature" && label.ends_with("边界") {
        return Some("evidence_boundary");
    }
    if intent == "literature" && label.ends_with("定位") {
        return Some("source_location");
    }
    None
}

fn normalized_title(value: &str) -> String {
    value.trim().trim_start_matches('#').trim().to_string()
}

fn normalize_sections(
    intent: &str,
    sections: Vec<StructuredSection>,
) -> Result<Vec<(super::context::AnswerSectionContract, StructuredSection)>, String> {
    let expected = super::context::required_answer_section_contract(intent);
    let has_ids = sections.iter().any(|section| !section.id.trim().is_empty());
    if has_ids && sections.iter().any(|section| section.id.trim().is_empty()) {
        return Err("结构化回答 sections 不得混用有 id 和无 id 的章节".to_string());
    }

    if has_ids {
        let actual_ids = sections
            .iter()
            .map(|section| section.id.trim().to_string())
            .collect::<Vec<_>>();
        let expected_ids = expected
            .iter()
            .map(|section| section.id.to_string())
            .collect::<Vec<_>>();
        if actual_ids != expected_ids {
            return Err(format!(
                "结构化回答章节 id 应为 {}，实际为 {}",
                serde_json::to_string(&expected_ids).unwrap_or_else(|_| "[]".to_string()),
                serde_json::to_string(&actual_ids).unwrap_or_else(|_| "[]".to_string())
            ));
        }
        for (contract, section) in expected.iter().zip(&sections) {
            let title = normalized_title(&section.title);
            if !title.is_empty() && title != contract.title {
                return Err(format!(
                    "结构化回答章节 {} 的 title 应为 {:?}，实际为 {:?}",
                    contract.id, contract.title, title
                ));
            }
        }
        return Ok(expected.into_iter().zip(sections).collect());
    }

    let mut compatible = Vec::with_capacity(sections.len());
    let mut sections = sections.into_iter().peekable();
    while let Some(mut section) = sections.next() {
        let title = normalized_title(&section.title);
        if intent == "literature"
            && title == "主题"
            && sections
                .peek()
                .is_some_and(|next| normalized_title(&next.title) == "模型与方法")
        {
            let next = sections.next().expect("peeked section must exist");
            section.groups.extend(next.groups);
            section.title = "主题、模型与方法".to_string();
        }
        compatible.push(section);
    }

    let actual_titles = compatible
        .iter()
        .map(|section| normalized_title(&section.title))
        .collect::<Vec<_>>();
    let expected_titles = expected
        .iter()
        .map(|section| section.title.to_string())
        .collect::<Vec<_>>();
    if actual_titles != expected_titles {
        return Err(format!(
            "结构化回答章节 title 应为 {}，实际为 {}",
            serde_json::to_string(&expected_titles).unwrap_or_else(|_| "[]".to_string()),
            serde_json::to_string(&actual_titles).unwrap_or_else(|_| "[]".to_string())
        ));
    }
    Ok(expected.into_iter().zip(compatible).collect())
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
    let sections = normalize_sections(intent, answer.sections)?;

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
    let mut roles = Vec::new();
    let mut markdown = String::new();
    let allowed_roles = super::context::required_answer_role_contract(intent)
        .into_iter()
        .map(|role| role.id)
        .collect::<HashSet<_>>();

    for (contract, section) in &sections {
        let title = contract.title;
        if section.groups.is_empty() {
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
                let role = if claim.role.trim().is_empty() {
                    legacy_role(intent, &claim.label)
                } else {
                    let role = claim.role.trim();
                    if !allowed_roles.contains(role) {
                        return Err(format!("结构化回答包含未知 claim role：{role}"));
                    }
                    Some(role)
                };
                if let Some(role) = role {
                    if !roles.iter().any(|existing| existing == role) {
                        roles.push(role.to_string());
                    }
                }
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
        markdown.push('\n');
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
        roles,
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

pub fn invalid_validation(error: &str) -> CitationValidation {
    CitationValidation {
        grounding_status: "invalid".to_string(),
        claim_count: 0,
        unsupported_claims: vec![compact(error, 180)],
        syntax_valid: false,
        ..CitationValidation::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn evidence(id: &str) -> EvidenceItem {
        EvidenceItem {
            id: id.to_string(),
            kind: "wiki".to_string(),
            tier: "direct".to_string(),
            title: "Fixture".to_string(),
            snippet: "Supported fixture claim".to_string(),
            score: 1.0,
            rank: 1,
            page_id: "fixture".to_string(),
            page_type: "source".to_string(),
            source_path: "wiki/sources/fixture.md".to_string(),
            wikilink: "[[fixture]]".to_string(),
            book_id: String::new(),
            chapter_id: String::new(),
            physical_page_start: None,
            physical_page_end: None,
            markdown_path: String::new(),
            pdf_path: String::new(),
            node_id: String::new(),
            source_location: "原文第1–2行".to_string(),
            relation: String::new(),
            retrieval_reason: String::new(),
        }
    }

    fn group(text: &str) -> serde_json::Value {
        json!({
            "label": "fixture",
            "claims": [{"label": "claim", "text": text, "evidenceIds": ["E1"]}]
        })
    }

    #[test]
    fn generated_examples_cover_every_intent_contract_and_parse() {
        for intent in ["literature", "novelty", "relationship", "solve"] {
            let raw = complete_example(intent).to_string();
            let result = parse_validate_render(&raw, intent, &[evidence("E1")]).unwrap();
            assert!(result.validation.supported, "intent={intent}");
            let required_roles = super::super::context::required_answer_role_contract(intent);
            for role in required_roles {
                assert!(
                    result.roles.iter().any(|observed| observed == role.id),
                    "intent={intent}, missing role={}",
                    role.id
                );
            }
        }
    }

    #[test]
    fn provider_schema_closes_every_object_and_limits_dynamic_values() {
        let schema = provider_output_schema("literature", &[evidence("E1"), evidence("E2")]);
        assert_eq!(schema.get("additionalProperties"), Some(&json!(false)));
        assert_eq!(
            schema.pointer("/properties/sections/items/additionalProperties"),
            Some(&json!(false))
        );
        assert_eq!(
            schema
                .pointer("/properties/sections/items/properties/groups/items/additionalProperties"),
            Some(&json!(false))
        );
        assert_eq!(
            schema.pointer("/properties/sections/items/properties/groups/items/properties/claims/items/additionalProperties"),
            Some(&json!(false))
        );
        let roles = schema
            .pointer("/properties/sections/items/properties/groups/items/properties/claims/items/properties/role/enum")
            .and_then(Value::as_array)
            .unwrap();
        assert!(roles.contains(&json!("model_or_method")));
        let evidence_ids = schema
            .pointer("/properties/sections/items/properties/groups/items/properties/claims/items/properties/evidenceIds/items/enum")
            .and_then(Value::as_array)
            .unwrap();
        assert_eq!(evidence_ids, &[json!("E1"), json!("E2")]);
    }

    #[test]
    fn canonical_section_ids_render_backend_titles() {
        let sections = super::super::context::required_answer_section_contract("literature")
            .into_iter()
            .map(|section| {
                json!({
                    "id": section.id,
                    "title": section.title,
                    "groups": [group(section.title)]
                })
            })
            .collect::<Vec<_>>();
        let raw = json!({
            "schemaVersion": SCHEMA_VERSION,
            "sections": sections,
            "supplement": []
        })
        .to_string();

        let result = parse_validate_render(&raw, "literature", &[evidence("E1")]).unwrap();
        assert!(result.validation.supported);
        assert!(result.markdown.contains("## 主题、模型与方法"));
    }

    #[test]
    fn legacy_literature_topic_and_methods_sections_are_merged() {
        let raw = json!({
            "schemaVersion": SCHEMA_VERSION,
            "sections": [
                {"title": "结论", "groups": [group("结论")]},
                {"title": "库内相关论文", "groups": [group("论文")]},
                {"title": "主题", "groups": [group("主题")]},
                {"title": "模型与方法", "groups": [group("模型")]},
                {"title": "边界与复现信息", "groups": [group("边界")]}
            ],
            "supplement": []
        })
        .to_string();

        let result = parse_validate_render(&raw, "literature", &[evidence("E1")]).unwrap();
        assert!(result.validation.supported);
        assert_eq!(result.validation.claim_count, 5);
        assert_eq!(result.markdown.matches("## 主题、模型与方法").count(), 1);
        assert!(!result.markdown.contains("## 主题\n"));
        assert!(!result.markdown.contains("## 模型与方法\n"));
    }

    #[test]
    fn invalid_section_ids_report_explicit_contract_arrays() {
        let raw = json!({
            "schemaVersion": SCHEMA_VERSION,
            "sections": [{"id": "wrong", "title": "结论", "groups": [group("结论")]}],
            "supplement": []
        })
        .to_string();
        let error = parse_validate_render(&raw, "literature", &[evidence("E1")]).unwrap_err();
        assert!(error.contains("章节 id 应为"));
        assert!(error.contains("topic_methods"));
        assert!(error.contains("wrong"));
    }

    #[test]
    fn explicit_roles_allow_natural_labels_and_reject_unknown_roles() {
        let sections = super::super::context::required_answer_section_contract("literature")
            .into_iter()
            .enumerate()
            .map(|(index, section)| {
                let role = if index == 0 {
                    "model_or_method"
                } else {
                    "paper_title"
                };
                json!({
                    "id": section.id,
                    "title": section.title,
                    "groups": [{
                        "label": "fixture",
                        "claims": [{
                            "role": role,
                            "label": "完全自由的展示标签",
                            "text": "有证据的自然语言内容",
                            "evidenceIds": ["E1"]
                        }]
                    }]
                })
            })
            .collect::<Vec<_>>();
        let raw = json!({
            "schemaVersion": SCHEMA_VERSION,
            "sections": sections,
            "supplement": []
        })
        .to_string();
        let result = parse_validate_render(&raw, "literature", &[evidence("E1")]).unwrap();
        assert!(result.roles.contains(&"model_or_method".to_string()));
        assert!(result.markdown.contains("完全自由的展示标签"));

        let invalid = raw.replacen("model_or_method", "invented_role", 1);
        let error = parse_validate_render(&invalid, "literature", &[evidence("E1")]).unwrap_err();
        assert!(error.contains("未知 claim role：invented_role"));
    }

    #[test]
    fn legacy_labels_map_to_roles_without_scanning_claim_prose() {
        assert_eq!(
            legacy_role("literature", "求解方法"),
            Some("model_or_method")
        );
        assert_eq!(
            legacy_role("literature", "GAIN边界"),
            Some("evidence_boundary")
        );
        assert_eq!(
            legacy_role("literature", "ROSE定位"),
            Some("source_location")
        );
        assert_eq!(legacy_role("literature", "任意说明"), None);
    }
}
