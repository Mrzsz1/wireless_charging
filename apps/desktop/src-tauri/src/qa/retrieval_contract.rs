use super::research_query_context::ResearchQueryContext;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashSet;

pub const RETRIEVAL_CONTRACT_VERSION: &str = "qa-retrieval-contract-v2";
const EVIDENCE_KINDS: &[&str] = &["wiki", "paper", "book"];
const MAX_FACETS: usize = 8;
const MAX_QUERIES_PER_FACET: usize = 4;
const MAX_TOTAL_QUERIES: usize = 20;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RetrievalScope {
    pub mode: String,
    pub explicit_sources: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RetrievalBudget {
    pub max_rounds: usize,
    pub max_queries: usize,
    pub max_candidates: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RetrievalFacet {
    pub id: String,
    pub label: String,
    pub required: bool,
    pub search_queries: Vec<String>,
    pub preferred_kinds: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RetrievalContract {
    pub schema_version: String,
    pub scope: RetrievalScope,
    pub concepts: Vec<String>,
    pub aliases: Vec<String>,
    pub related_problems: Vec<String>,
    pub facets: Vec<RetrievalFacet>,
    pub requested_kinds: Vec<String>,
    pub must_attempt_kinds: Vec<String>,
    pub budget: RetrievalBudget,
    /// Transitional ranking-only compatibility. This field is never accepted
    /// from Provider JSON and is absent from the native schema.
    #[serde(skip, default)]
    pub legacy_ranking_profile: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RetrievalPlanningCandidate {
    pub kind: String,
    pub page_type: String,
    pub title: String,
    pub excerpt: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RetrievalPlanningInput {
    pub resolved_question: String,
    #[serde(default)]
    pub research_context: ResearchQueryContext,
    pub baseline_candidates: Vec<RetrievalPlanningCandidate>,
}

impl RetrievalContract {
    pub fn fallback(question: &str) -> Self {
        let question = question.trim().to_string();
        let explicit_sources = explicit_sources_from_question(&question);
        Self {
            schema_version: RETRIEVAL_CONTRACT_VERSION.to_string(),
            scope: RetrievalScope {
                mode: if explicit_sources.is_empty() {
                    "open"
                } else {
                    "sources"
                }
                .to_string(),
                explicit_sources,
            },
            concepts: vec![question.clone()],
            aliases: Vec::new(),
            related_problems: Vec::new(),
            facets: vec![RetrievalFacet {
                id: "question".to_string(),
                label: "完整研究问题".to_string(),
                required: true,
                search_queries: vec![question],
                preferred_kinds: Vec::new(),
            }],
            requested_kinds: EVIDENCE_KINDS
                .iter()
                .map(|value| (*value).to_string())
                .collect(),
            must_attempt_kinds: EVIDENCE_KINDS
                .iter()
                .map(|value| (*value).to_string())
                .collect(),
            budget: RetrievalBudget {
                max_rounds: 1,
                max_queries: 8,
                max_candidates: 120,
            },
            legacy_ranking_profile: "solve".to_string(),
        }
    }

    fn normalize(mut self, original_question: &str) -> Result<Self, String> {
        if self.schema_version != RETRIEVAL_CONTRACT_VERSION {
            return Err("RETRIEVAL_CONTRACT_INVALID: schemaVersion 不受支持".to_string());
        }
        if !matches!(self.scope.mode.as_str(), "open" | "sources") {
            return Err("RETRIEVAL_CONTRACT_INVALID: scope.mode 不受支持".to_string());
        }
        self.scope.explicit_sources = normalized_texts(&self.scope.explicit_sources, 12, 240);
        if self.scope.mode == "sources" && self.scope.explicit_sources.is_empty() {
            return Err("RETRIEVAL_CONTRACT_INVALID: sources scope 必须包含显式来源".to_string());
        }
        self.concepts = normalized_texts(&self.concepts, 12, 2_000);
        if self.concepts.is_empty() {
            self.concepts.push(original_question.trim().to_string());
        }
        self.aliases = normalized_texts(&self.aliases, 16, 160);
        self.related_problems = normalized_texts(&self.related_problems, 12, 240);
        self.requested_kinds = normalized_kinds(&self.requested_kinds)?;
        self.must_attempt_kinds = normalized_kinds(&self.must_attempt_kinds)?;
        if self.requested_kinds.is_empty() {
            self.requested_kinds = EVIDENCE_KINDS
                .iter()
                .map(|value| (*value).to_string())
                .collect();
        }
        if self.must_attempt_kinds.is_empty() {
            self.must_attempt_kinds = self.requested_kinds.clone();
        }
        if self
            .must_attempt_kinds
            .iter()
            .any(|kind| !self.requested_kinds.contains(kind))
        {
            return Err(
                "RETRIEVAL_CONTRACT_INVALID: mustAttemptKinds 必须属于 requestedKinds".to_string(),
            );
        }
        if self.facets.is_empty() || self.facets.len() > MAX_FACETS {
            return Err("RETRIEVAL_CONTRACT_INVALID: facets 数量必须为 1–8".to_string());
        }
        let mut facet_ids = HashSet::new();
        let mut query_count = 0usize;
        for facet in &mut self.facets {
            facet.id = facet.id.trim().to_ascii_lowercase();
            if facet.id.is_empty()
                || facet.id.len() > 48
                || !facet.id.chars().all(|character| {
                    character.is_ascii_alphanumeric() || matches!(character, '_' | '-')
                })
                || !facet_ids.insert(facet.id.clone())
            {
                return Err("RETRIEVAL_CONTRACT_INVALID: facet id 非法或重复".to_string());
            }
            facet.label = bounded_text(&facet.label, 120);
            if facet.label.is_empty() {
                return Err("RETRIEVAL_CONTRACT_INVALID: facet label 不能为空".to_string());
            }
            facet.search_queries =
                normalized_texts(&facet.search_queries, MAX_QUERIES_PER_FACET, 240);
            query_count += facet.search_queries.len();
            facet.preferred_kinds = normalized_kinds(&facet.preferred_kinds)?;
        }
        if query_count > MAX_TOTAL_QUERIES {
            return Err("RETRIEVAL_CONTRACT_INVALID: 扩展查询总数过多".to_string());
        }
        if !(1..=3).contains(&self.budget.max_rounds)
            || !(1..=MAX_TOTAL_QUERIES).contains(&self.budget.max_queries)
            || !(20..=400).contains(&self.budget.max_candidates)
        {
            return Err("RETRIEVAL_CONTRACT_INVALID: budget 超出允许范围".to_string());
        }
        Ok(self)
    }
}

fn explicit_sources_from_question(question: &str) -> Vec<String> {
    let mut sources = Vec::new();
    let mut start = None;
    for (index, character) in question.char_indices() {
        if character == '《' {
            start = Some(index + character.len_utf8());
        } else if character == '》' {
            if let Some(begin) = start.take() {
                let value = question[begin..index].trim();
                if !value.is_empty() {
                    sources.push(value.to_string());
                }
            }
        }
    }
    normalized_texts(&sources, 12, 240)
}

fn normalized_kinds(values: &[String]) -> Result<Vec<String>, String> {
    let mut seen = HashSet::new();
    let mut normalized = Vec::new();
    for value in values {
        let value = value.trim().to_ascii_lowercase();
        if !EVIDENCE_KINDS.contains(&value.as_str()) {
            return Err("RETRIEVAL_CONTRACT_INVALID: evidence kind 不受支持".to_string());
        }
        if seen.insert(value.clone()) {
            normalized.push(value);
        }
    }
    Ok(normalized)
}

fn normalized_texts(values: &[String], maximum_items: usize, maximum_chars: usize) -> Vec<String> {
    let mut seen = HashSet::new();
    values
        .iter()
        .map(|value| bounded_text(value, maximum_chars))
        .filter(|value| !value.is_empty() && seen.insert(value.to_lowercase()))
        .take(maximum_items)
        .collect()
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

pub fn parse_retrieval_contract(
    raw: &str,
    original_question: &str,
) -> Result<RetrievalContract, String> {
    serde_json::from_str::<RetrievalContract>(raw.trim())
        .map_err(|error| format!("RETRIEVAL_CONTRACT_INVALID: JSON 解析失败：{error}"))?
        .normalize(original_question)
}

fn kind_array_schema() -> Value {
    json!({"type":"array","uniqueItems":true,"items":{"type":"string","enum":EVIDENCE_KINDS}})
}

pub fn retrieval_contract_schema() -> Value {
    json!({
        "$schema":"https://json-schema.org/draft/2020-12/schema","type":"object","additionalProperties":false,
        "required":["schemaVersion","scope","concepts","aliases","relatedProblems","facets","requestedKinds","mustAttemptKinds","budget"],
        "properties":{
            "schemaVersion":{"type":"string","const":RETRIEVAL_CONTRACT_VERSION},
            "scope":{"type":"object","additionalProperties":false,"required":["mode","explicitSources"],"properties":{"mode":{"type":"string","enum":["open","sources"]},"explicitSources":{"type":"array","maxItems":12,"items":{"type":"string","minLength":1,"maxLength":240}}}},
            "concepts":{"type":"array","minItems":1,"maxItems":12,"items":{"type":"string","minLength":1,"maxLength":2000}},
            "aliases":{"type":"array","maxItems":16,"items":{"type":"string","minLength":1,"maxLength":160}},
            "relatedProblems":{"type":"array","maxItems":12,"items":{"type":"string","minLength":1,"maxLength":240}},
            "facets":{"type":"array","minItems":1,"maxItems":MAX_FACETS,"items":{"type":"object","additionalProperties":false,"required":["id","label","required","searchQueries","preferredKinds"],"properties":{"id":{"type":"string","pattern":"^[a-z0-9_-]{1,48}$"},"label":{"type":"string","minLength":1,"maxLength":120},"required":{"type":"boolean"},"searchQueries":{"type":"array","maxItems":MAX_QUERIES_PER_FACET,"items":{"type":"string","minLength":1,"maxLength":240}},"preferredKinds":kind_array_schema()}}},
            "requestedKinds":kind_array_schema(),"mustAttemptKinds":kind_array_schema(),
            "budget":{"type":"object","additionalProperties":false,"required":["maxRounds","maxQueries","maxCandidates"],"properties":{"maxRounds":{"type":"integer","minimum":1,"maximum":3},"maxQueries":{"type":"integer","minimum":1,"maximum":MAX_TOTAL_QUERIES},"maxCandidates":{"type":"integer","minimum":20,"maximum":400}}}
        },
        "examples":[retrieval_contract_example()]
    })
}

pub fn retrieval_contract_example() -> Value {
    json!({
        "schemaVersion":RETRIEVAL_CONTRACT_VERSION,
        "scope":{"mode":"sources","explicitSources":["示例书名（仅演示格式）"]},
        "concepts":["示例研究问题的完整概念（仅演示格式）"],"aliases":["example concept"],"relatedProblems":["example related problem"],
        "facets":[{"id":"modeling","label":"示例模型与方法","required":true,"searchQueries":["example model method","示例 模型 方法"],"preferredKinds":["book"]}],
        "requestedKinds":["book"],"mustAttemptKinds":["book"],"budget":{"maxRounds":3,"maxQueries":8,"maxCandidates":120}
    })
}

pub fn retrieval_contract_prompt(input: &RetrievalPlanningInput) -> String {
    let input_json = serde_json::to_string(input).unwrap_or_else(|_| "{}".to_string());
    let example = serde_json::to_string_pretty(&retrieval_contract_example())
        .unwrap_or_else(|_| "{}".to_string());
    format!(
        "你是科研知识库检索规划器。只输出符合 Provider 原生 JSON Schema 的 RetrievalContract，不输出 Markdown 或答案。\n\
         规划检索范围、显式来源、概念、同义表达、相关问题、证据面与预算。不要输出 answerProfile，不要输出 minimumEvidence，也不要判断事实真假或证据是否充分。\n\
         explicitSources 只放用户明确指定的书名、论文名或可审计来源；不得编造来源。requestedKinds 是允许尝试的来源类型，mustAttemptKinds 是必须实际尝试并记录状态的类型。\n\
         researchContext 是应用当前消息 StatePatch 后的最新研究状态投影。开放方法搜索必须保留其中的目标、关键约束和参数；excludedMethods 不得作为首选推荐，但不能据此硬过滤有价值的比较证据。\n\
         searchQueries 需要保留问题尾部概念，可给少量中英表达；补查最多两轮，因此 maxRounds 最大为 3。baselineCandidates 只是候选摘要，不能限制未知术语。\n\
         完整格式示例（所有内容仅演示结构，严禁复制事实）：\n{example}\n输入 JSON：{input_json}"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_has_native_contract_without_answer_profile_or_minimum_evidence() {
        let schema = retrieval_contract_schema();
        let rendered = schema.to_string();
        assert!(!rendered.contains("answerProfile"));
        assert!(!rendered.contains("minimumEvidence"));
        assert_eq!(schema["additionalProperties"], false);
        assert_eq!(schema["properties"]["scope"]["additionalProperties"], false);
        assert!(rendered.contains("examples"));
    }

    #[test]
    fn fallback_preserves_complete_unicode_question_and_explicit_book() {
        let question = "前置说明很长，但是最终核心问题是《近似算法》是否包含移动路径规划";
        let contract = RetrievalContract::fallback(question);
        assert_eq!(contract.concepts, [question]);
        assert_eq!(contract.scope.mode, "sources");
        assert_eq!(contract.scope.explicit_sources, ["近似算法"]);
    }

    #[test]
    fn parser_rejects_unknown_fields_and_unbounded_budget() {
        let mut value = retrieval_contract_example();
        value["extra"] = json!(true);
        assert!(parse_retrieval_contract(&value.to_string(), "问题").is_err());
        let mut value = retrieval_contract_example();
        value["budget"]["maxRounds"] = json!(4);
        assert!(parse_retrieval_contract(&value.to_string(), "问题").is_err());
    }
}
