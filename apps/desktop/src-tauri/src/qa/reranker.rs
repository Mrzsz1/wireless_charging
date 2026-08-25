use super::{compact, query_terms, Candidate};
use std::collections::HashSet;

const SEMANTIC_RERANK_LIMIT: usize = 80;
const SEMANTIC_CANDIDATE_CHARS: usize = 1_200;
const SEMANTIC_WEIGHT: f64 = 0.75;

#[derive(Debug)]
pub struct RerankOutcome {
    pub candidates: Vec<Candidate>,
    pub reranker_version: String,
    pub fallback: bool,
    pub fallback_reason: String,
}

pub trait Reranker {
    fn name(&self) -> &'static str;
    fn rerank(
        &self,
        question: &str,
        candidates: Vec<Candidate>,
        explicit_paths: &HashSet<String>,
    ) -> Result<RerankOutcome, String>;
}

pub struct DeterministicResearchReranker;

impl Reranker for DeterministicResearchReranker {
    fn name(&self) -> &'static str {
        "deterministic-research-v2"
    }

    fn rerank(
        &self,
        question: &str,
        mut candidates: Vec<Candidate>,
        explicit_paths: &HashSet<String>,
    ) -> Result<RerankOutcome, String> {
        let terms = query_terms(question);
        for candidate in &mut candidates {
            let haystack = format!(
                "{} {} {}",
                candidate.title, candidate.source_location, candidate.snippet
            )
            .to_lowercase();
            let overlap = terms
                .iter()
                .filter(|term| haystack.contains(term.as_str()))
                .count();
            let explicit = explicit_paths.contains(&candidate.markdown_path)
                || explicit_paths.contains(&candidate.source_path);
            let mut adjustment = (overlap.min(6) as f64) * 0.06;
            if explicit {
                adjustment += 0.08;
            }
            if candidate.relation.contains("reference") {
                adjustment -= 0.08;
            }
            if candidate.kind == "graph" || candidate.relation.contains("graph_only") {
                adjustment -= 0.12;
            }
            if candidate.relation.contains("primary_fallback") {
                adjustment -= 0.07;
            }
            candidate.score += adjustment;
            candidate.retrieval_reason.push_str(&format!(
                "；reranker={} overlap={} explicit={} adjustment={adjustment:.3}",
                self.name(),
                overlap,
                explicit
            ));
        }
        candidates.sort_by(|left, right| right.score.total_cmp(&left.score));
        Ok(RerankOutcome {
            candidates,
            reranker_version: self.name().to_string(),
            fallback: false,
            fallback_reason: String::new(),
        })
    }
}

pub type SemanticEmbedder<'a> = dyn Fn(Vec<String>) -> Result<Vec<Vec<f32>>, String> + 'a;

pub struct SemanticResearchReranker<'a> {
    embedder: &'a SemanticEmbedder<'a>,
}

impl<'a> SemanticResearchReranker<'a> {
    pub fn new(embedder: &'a SemanticEmbedder<'a>) -> Self {
        Self { embedder }
    }
}

impl Reranker for SemanticResearchReranker<'_> {
    fn name(&self) -> &'static str {
        "semantic-research-v1"
    }

    fn rerank(
        &self,
        question: &str,
        candidates: Vec<Candidate>,
        explicit_paths: &HashSet<String>,
    ) -> Result<RerankOutcome, String> {
        let mut deterministic = DeterministicResearchReranker
            .rerank(question, candidates, explicit_paths)?
            .candidates;
        if deterministic.is_empty() {
            return Ok(RerankOutcome {
                candidates: deterministic,
                reranker_version: self.name().to_string(),
                fallback: false,
                fallback_reason: String::new(),
            });
        }
        let semantic_count = deterministic.len().min(SEMANTIC_RERANK_LIMIT);
        let mut inputs = Vec::with_capacity(semantic_count + 1);
        inputs.push(compact(question, SEMANTIC_CANDIDATE_CHARS));
        inputs.extend(deterministic.iter().take(semantic_count).map(|candidate| {
            compact(
                &format!(
                    "{}\n{}\n{}",
                    candidate.title, candidate.source_location, candidate.snippet
                ),
                SEMANTIC_CANDIDATE_CHARS,
            )
        }));
        let embeddings = (self.embedder)(inputs).map_err(|error| {
            if error.starts_with("QUESTION_CANCELLED") {
                error
            } else {
                stable_reranker_error(&error)
            }
        })?;
        if embeddings.len() != semantic_count + 1 {
            return Err("reranker_unavailable: embedding_count".to_string());
        }
        let query_embedding = &embeddings[0];
        if query_embedding.is_empty() || query_embedding.iter().any(|value| !value.is_finite()) {
            return Err("reranker_unavailable: invalid_query_embedding".to_string());
        }
        for (candidate, embedding) in deterministic
            .iter_mut()
            .take(semantic_count)
            .zip(embeddings.iter().skip(1))
        {
            let similarity = cosine_similarity(query_embedding, embedding)
                .ok_or_else(|| "reranker_unavailable: invalid_candidate_embedding".to_string())?;
            let semantic_bonus = similarity.clamp(0.0, 1.0) * SEMANTIC_WEIGHT;
            candidate.score += semantic_bonus;
            candidate.retrieval_reason.push_str(&format!(
                "；reranker={} cosine={similarity:.4} semantic_bonus={semantic_bonus:.4}",
                self.name()
            ));
        }
        deterministic.sort_by(|left, right| right.score.total_cmp(&left.score));
        Ok(RerankOutcome {
            candidates: deterministic,
            reranker_version: self.name().to_string(),
            fallback: false,
            fallback_reason: String::new(),
        })
    }
}

pub struct HybridResearchReranker<'a> {
    semantic: SemanticResearchReranker<'a>,
    deterministic: DeterministicResearchReranker,
}

impl<'a> HybridResearchReranker<'a> {
    pub fn new(embedder: &'a SemanticEmbedder<'a>) -> Self {
        Self {
            semantic: SemanticResearchReranker::new(embedder),
            deterministic: DeterministicResearchReranker,
        }
    }
}

impl Reranker for HybridResearchReranker<'_> {
    fn name(&self) -> &'static str {
        "hybrid-semantic-research-v1"
    }

    fn rerank(
        &self,
        question: &str,
        candidates: Vec<Candidate>,
        explicit_paths: &HashSet<String>,
    ) -> Result<RerankOutcome, String> {
        let fallback_candidates = candidates.clone();
        match self.semantic.rerank(question, candidates, explicit_paths) {
            Ok(mut outcome) => {
                outcome.reranker_version = self.name().to_string();
                Ok(outcome)
            }
            Err(error) if error.starts_with("QUESTION_CANCELLED") => Err(error),
            Err(error) => {
                let mut outcome =
                    self.deterministic
                        .rerank(question, fallback_candidates, explicit_paths)?;
                outcome.reranker_version = self.name().to_string();
                outcome.fallback = true;
                outcome.fallback_reason = error
                    .split(':')
                    .next()
                    .unwrap_or("reranker_unavailable")
                    .to_lowercase();
                for candidate in &mut outcome.candidates {
                    candidate
                        .retrieval_reason
                        .push_str("；semantic_reranker_fallback=deterministic");
                }
                Ok(outcome)
            }
        }
    }
}

fn stable_reranker_error(error: &str) -> String {
    let kind = error
        .split(':')
        .next()
        .unwrap_or("semantic_unavailable")
        .trim()
        .to_ascii_lowercase();
    format!("reranker_unavailable: {kind}")
}

fn cosine_similarity(left: &[f32], right: &[f32]) -> Option<f64> {
    if left.len() != right.len() || left.is_empty() {
        return None;
    }
    let mut dot = 0.0_f64;
    let mut left_norm = 0.0_f64;
    let mut right_norm = 0.0_f64;
    for (left, right) in left.iter().zip(right) {
        if !left.is_finite() || !right.is_finite() {
            return None;
        }
        let left = f64::from(*left);
        let right = f64::from(*right);
        dot += left * right;
        left_norm += left * left;
        right_norm += right * right;
    }
    let denominator = left_norm.sqrt() * right_norm.sqrt();
    (denominator > f64::EPSILON)
        .then_some(dot / denominator)
        .filter(|value| value.is_finite())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn candidate(title: &str, relation: &str, path: &str, score: f64) -> Candidate {
        Candidate {
            kind: "book".into(),
            tier: "theory".into(),
            title: title.into(),
            snippet: title.into(),
            score,
            page_id: String::new(),
            page_type: String::new(),
            source_path: path.into(),
            wikilink: String::new(),
            book_id: "book".into(),
            chapter_id: String::new(),
            physical_page_start: None,
            physical_page_end: None,
            markdown_path: path.into(),
            pdf_path: String::new(),
            node_id: title.into(),
            source_location: String::new(),
            relation: relation.into(),
            retrieval_reason: String::new(),
        }
    }

    #[test]
    fn explicit_relevant_body_stays_above_reference_only_candidate() {
        let explicit = HashSet::from(["tsp.md".to_string()]);
        let ranked = DeterministicResearchReranker
            .rerank(
                "移动路径规划 TSP",
                vec![
                    candidate("References TSP", "reference_only", "refs.md", 0.1),
                    candidate(
                        "TSP path planning algorithm",
                        "content_block_v2",
                        "tsp.md",
                        0.1,
                    ),
                ],
                &explicit,
            )
            .unwrap();
        assert_eq!(ranked.candidates[0].source_path, "tsp.md");
    }

    #[test]
    fn semantic_reranker_improves_recall_at_five_mrr_and_ndcg_without_losing_recall_at_twenty() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../../evals/semantic_reranker_cases.json"
        ))
        .expect("semantic reranker fixture");
        assert_eq!(fixture["schemaVersion"], "qa-semantic-reranker-cases-v1");
        assert_eq!(
            fixture["datasetRole"],
            "model_independent_reranker_regression"
        );
        let cases = fixture["cases"].as_array().expect("cases");
        assert_eq!(cases.len(), fixture["caseCount"].as_u64().unwrap() as usize);
        assert!(cases.len() >= 10);
        let mut deterministic_recall5 = 0.0;
        let mut semantic_recall5 = 0.0;
        let mut deterministic_recall20 = 0.0;
        let mut semantic_recall20 = 0.0;
        let mut deterministic_mrr = 0.0;
        let mut semantic_mrr = 0.0;
        let mut deterministic_ndcg = 0.0;
        let mut semantic_ndcg = 0.0;

        for case in cases {
            let query = case["query"].as_str().expect("query");
            let target_title = case["targetTitle"].as_str().expect("target title");
            let mut candidates = (0..5)
                .map(|index| {
                    candidate(
                        &format!("lexical noise {index}"),
                        "content_block_v2",
                        &format!("noise-{index}.md"),
                        0.60 - index as f64 * 0.05,
                    )
                })
                .collect::<Vec<_>>();
            candidates.push(candidate(
                target_title,
                "content_block_v2",
                "relevant.md",
                0.01,
            ));
            let deterministic = DeterministicResearchReranker
                .rerank(query, candidates.clone(), &HashSet::new())
                .unwrap()
                .candidates;
            let embedder = |texts: Vec<String>| {
                Ok(texts
                    .into_iter()
                    .enumerate()
                    .map(|(index, text)| {
                        if index == 0 || text.contains(target_title) {
                            vec![1.0, 0.0]
                        } else {
                            vec![0.0, 1.0]
                        }
                    })
                    .collect())
            };
            let semantic = SemanticResearchReranker::new(&embedder)
                .rerank(query, candidates, &HashSet::new())
                .unwrap()
                .candidates;
            let rank = |values: &[Candidate]| {
                values
                    .iter()
                    .position(|candidate| candidate.source_path == "relevant.md")
                    .map(|index| index + 1)
                    .unwrap_or(usize::MAX)
            };
            let deterministic_rank = rank(&deterministic);
            let semantic_rank = rank(&semantic);
            deterministic_recall5 += f64::from(deterministic_rank <= 5);
            semantic_recall5 += f64::from(semantic_rank <= 5);
            deterministic_recall20 += f64::from(deterministic_rank <= 20);
            semantic_recall20 += f64::from(semantic_rank <= 20);
            deterministic_mrr += 1.0 / deterministic_rank as f64;
            semantic_mrr += 1.0 / semantic_rank as f64;
            deterministic_ndcg += 1.0 / (deterministic_rank as f64 + 1.0).log2();
            semantic_ndcg += 1.0 / (semantic_rank as f64 + 1.0).log2();
        }
        let count = cases.len() as f64;
        assert!(semantic_recall5 / count > deterministic_recall5 / count);
        assert!(semantic_mrr / count > deterministic_mrr / count);
        assert!(semantic_ndcg / count > deterministic_ndcg / count);
        assert_eq!(semantic_recall20 / count, deterministic_recall20 / count);
        assert_eq!(semantic_recall20 / count, 1.0);
    }

    #[test]
    fn hybrid_reranker_falls_back_to_deterministic_with_stable_reason() {
        let embedder =
            |_texts: Vec<String>| Err("SEMANTIC_UNAVAILABLE: fixture model missing".to_string());
        let outcome = HybridResearchReranker::new(&embedder)
            .rerank(
                "TSP",
                vec![candidate(
                    "TSP algorithm",
                    "content_block_v2",
                    "tsp.md",
                    0.1,
                )],
                &HashSet::new(),
            )
            .unwrap();
        assert!(outcome.fallback);
        assert_eq!(outcome.fallback_reason, "reranker_unavailable");
        assert_eq!(outcome.reranker_version, "hybrid-semantic-research-v1");
        assert!(outcome.candidates[0]
            .retrieval_reason
            .contains("semantic_reranker_fallback=deterministic"));
    }

    #[test]
    fn hybrid_reranker_never_converts_cancellation_into_fallback() {
        let embedder = |_texts: Vec<String>| Err("QUESTION_CANCELLED: 用户停止了问答".to_string());
        let error = HybridResearchReranker::new(&embedder)
            .rerank(
                "TSP",
                vec![candidate(
                    "TSP algorithm",
                    "content_block_v2",
                    "tsp.md",
                    0.1,
                )],
                &HashSet::new(),
            )
            .unwrap_err();
        assert!(error.starts_with("QUESTION_CANCELLED"));
    }
}
