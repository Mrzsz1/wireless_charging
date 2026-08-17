use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashSet;

pub const QUERY_PLAN_VERSION: &str = "qa-query-plan-v1";
const ANSWER_PROFILES: &[&str] = &["solve", "novelty", "relationship", "literature"];
const EVIDENCE_KINDS: &[&str] = &["wiki", "paper", "book"];
const MAX_FACETS: usize = 6;
const MAX_QUERIES_PER_FACET: usize = 4;
const MAX_TOTAL_QUERIES: usize = 16;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct QueryFacet {
    pub id: String,
    pub label: String,
    pub required: bool,
    pub search_queries: Vec<String>,
    pub preferred_kinds: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct QueryPlan {
    pub schema_version: String,
    pub answer_profile: String,
    pub restated_question: String,
    pub facets: Vec<QueryFacet>,
    pub required_kinds: Vec<String>,
    pub minimum_evidence: usize,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryPlanningCandidate {
    pub kind: String,
    pub page_type: String,
    pub title: String,
    pub excerpt: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryPlanningInput {
    pub resolved_question: String,
    pub baseline_candidates: Vec<QueryPlanningCandidate>,
}

impl QueryPlan {
    pub fn fallback(question: &str) -> Self {
        Self {
            schema_version: QUERY_PLAN_VERSION.to_string(),
            answer_profile: "solve".to_string(),
            restated_question: question.trim().to_string(),
            facets: vec![QueryFacet {
                id: "answer".to_string(),
                label: "回答问题".to_string(),
                required: true,
                search_queries: Vec::new(),
                preferred_kinds: Vec::new(),
            }],
            required_kinds: Vec::new(),
            minimum_evidence: 4,
        }
    }

    fn normalize(mut self, original_question: &str) -> Result<Self, String> {
        if self.schema_version != QUERY_PLAN_VERSION {
            return Err("QUERY_PLAN_INVALID: schemaVersion 不受支持".to_string());
        }
        if !ANSWER_PROFILES.contains(&self.answer_profile.as_str()) {
            return Err("QUERY_PLAN_INVALID: answerProfile 不受支持".to_string());
        }
        self.restated_question = bounded_text(&self.restated_question, 500);
        if self.restated_question.is_empty() {
            self.restated_question = bounded_text(original_question, 500);
        }
        if self.facets.is_empty() || self.facets.len() > MAX_FACETS {
            return Err("QUERY_PLAN_INVALID: facets 数量必须为 1–6".to_string());
        }
        let mut facet_ids = HashSet::new();
        let mut total_queries = 0;
        for facet in &mut self.facets {
            facet.id = facet.id.trim().to_lowercase();
            if facet.id.is_empty()
                || facet.id.len() > 40
                || !facet.id.chars().all(|character| {
                    character.is_ascii_alphanumeric() || matches!(character, '_' | '-')
                })
                || !facet_ids.insert(facet.id.clone())
            {
                return Err("QUERY_PLAN_INVALID: facet id 非法或重复".to_string());
            }
            facet.label = bounded_text(&facet.label, 80);
            if facet.label.is_empty() {
                return Err("QUERY_PLAN_INVALID: facet label 不能为空".to_string());
            }
            if facet.search_queries.len() > MAX_QUERIES_PER_FACET {
                return Err("QUERY_PLAN_INVALID: 单个 facet 查询过多".to_string());
            }
            let mut seen_queries = HashSet::new();
            facet.search_queries = facet
                .search_queries
                .iter()
                .map(|query| bounded_text(query, 180))
                .filter(|query| !query.is_empty() && seen_queries.insert(query.to_lowercase()))
                .collect();
            total_queries += facet.search_queries.len();
            if total_queries > MAX_TOTAL_QUERIES {
                return Err("QUERY_PLAN_INVALID: 扩展查询总数过多".to_string());
            }
            facet.preferred_kinds = normalized_kinds(&facet.preferred_kinds)?;
        }
        self.required_kinds = normalized_kinds(&self.required_kinds)?;
        if !(2..=12).contains(&self.minimum_evidence) {
            return Err("QUERY_PLAN_INVALID: minimumEvidence 必须为 2–12".to_string());
        }
        Ok(self)
    }
}

fn normalized_kinds(values: &[String]) -> Result<Vec<String>, String> {
    let mut seen = HashSet::new();
    let mut normalized = Vec::new();
    for value in values {
        let value = value.trim().to_lowercase();
        if !EVIDENCE_KINDS.contains(&value.as_str()) {
            return Err("QUERY_PLAN_INVALID: evidence kind 不受支持".to_string());
        }
        if seen.insert(value.clone()) {
            normalized.push(value);
        }
    }
    Ok(normalized)
}

fn bounded_text(value: &str, maximum: usize) -> String {
    value
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .take(maximum)
        .collect()
}

pub fn parse_query_plan(raw: &str, original_question: &str) -> Result<QueryPlan, String> {
    serde_json::from_str::<QueryPlan>(raw.trim())
        .map_err(|error| format!("QUERY_PLAN_INVALID: JSON 解析失败：{error}"))?
        .normalize(original_question)
}

pub fn query_plan_schema() -> Value {
    json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "type": "object",
        "additionalProperties": false,
        "required": ["schemaVersion", "answerProfile", "restatedQuestion", "facets", "requiredKinds", "minimumEvidence"],
        "properties": {
            "schemaVersion": {"type": "string", "const": QUERY_PLAN_VERSION},
            "answerProfile": {"type": "string", "enum": ANSWER_PROFILES},
            "restatedQuestion": {"type": "string", "minLength": 2, "maxLength": 500},
            "facets": {
                "type": "array", "minItems": 1, "maxItems": MAX_FACETS,
                "items": {
                    "type": "object", "additionalProperties": false,
                    "required": ["id", "label", "required", "searchQueries", "preferredKinds"],
                    "properties": {
                        "id": {"type": "string", "pattern": "^[a-z0-9_-]{1,40}$"},
                        "label": {"type": "string", "minLength": 1, "maxLength": 80},
                        "required": {"type": "boolean"},
                        "searchQueries": {"type": "array", "maxItems": MAX_QUERIES_PER_FACET, "items": {"type": "string", "minLength": 2, "maxLength": 180}},
                        "preferredKinds": {"type": "array", "uniqueItems": true, "items": {"type": "string", "enum": EVIDENCE_KINDS}}
                    }
                }
            },
            "requiredKinds": {"type": "array", "uniqueItems": true, "items": {"type": "string", "enum": EVIDENCE_KINDS}},
            "minimumEvidence": {"type": "integer", "minimum": 2, "maximum": 12}
        }
    })
}

pub fn query_plan_prompt(input: &QueryPlanningInput) -> String {
    let input_json = serde_json::to_string(input).unwrap_or_else(|_| "{}".to_string());
    format!(
        "你是科研知识库检索规划器。只输出符合 Provider JSON Schema 的 QueryPlan，不输出 Markdown。\n\
         目标：理解用户真正要回答的多个证据面，选择最合适的 answerProfile，并为每个开放 Facet 提供少量中英检索表达。\n\
         规则：不要预先回答问题；不要编造论文名；searchQueries 是检索表达而非结论；Facet ID 使用简短英文；只把回答不可缺少的面标 required；requiredKinds 仅在问题确实要求特定来源类型时填写。\n\
         baselineCandidates 只是首轮召回摘要，不代表事实成立，也不能限制你识别未知问题类型。\n\
         输入 JSON：{input_json}"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_closes_root_and_facet_objects() {
        let schema = query_plan_schema();
        assert_eq!(schema["additionalProperties"], false);
        assert_eq!(
            schema["properties"]["facets"]["items"]["additionalProperties"],
            false
        );
    }

    #[test]
    fn parser_accepts_open_facets_and_rejects_unknown_fields() {
        let raw = json!({
            "schemaVersion": QUERY_PLAN_VERSION,
            "answerProfile": "literature",
            "restatedQuestion": "哪些工作建立了干涉模型",
            "facets": [{
                "id": "modeling",
                "label": "建模方法",
                "required": true,
                "searchQueries": ["wave interference model", "波干涉 建模"],
                "preferredKinds": ["paper"]
            }],
            "requiredKinds": ["paper"],
            "minimumEvidence": 3
        });
        let plan = parse_query_plan(&raw.to_string(), "原问题").unwrap();
        assert_eq!(plan.facets[0].id, "modeling");
        let invalid = raw
            .as_object()
            .unwrap()
            .iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .chain([("extra".to_string(), json!(true))])
            .collect::<serde_json::Map<_, _>>();
        assert!(parse_query_plan(&Value::Object(invalid).to_string(), "原问题").is_err());
    }

    #[test]
    fn parser_rejects_unbounded_or_unsupported_values() {
        let invalid = json!({
            "schemaVersion": QUERY_PLAN_VERSION,
            "answerProfile": "hardcoded-new-type",
            "restatedQuestion": "问题",
            "facets": [],
            "requiredKinds": ["web"],
            "minimumEvidence": 99
        });
        assert!(parse_query_plan(&invalid.to_string(), "问题").is_err());
    }
}
