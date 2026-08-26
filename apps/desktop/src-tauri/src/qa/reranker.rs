use super::{compact, query_terms, Candidate};
use std::collections::{HashMap, HashSet};
use std::time::Instant;

const SEMANTIC_RERANK_LIMIT: usize = 80;
const SEMANTIC_CANDIDATE_CHARS: usize = 900;
const BASE_SCORE_WEIGHT: f64 = 0.70;
const CROSS_ENCODER_SCORE_WEIGHT: f64 = 0.30;
const CROSS_ENCODER_TOP_BONUS: f64 = 0.15;
const DOCUMENT_REPEAT_PENALTY: f64 = 0.06;

#[derive(Debug)]
pub struct RerankOutcome {
    pub candidates: Vec<Candidate>,
    pub reranker_version: String,
    pub fallback: bool,
    pub fallback_reason: String,
    pub batch_size: usize,
    pub batch_count: usize,
    pub model_max_length: usize,
    pub model_load_ms: u64,
    pub input_prepare_ms: u64,
    pub inference_ms: u64,
    pub average_input_tokens: usize,
}

#[derive(Debug)]
pub struct CrossEncoderScores {
    pub scores: Vec<f32>,
    pub batch_size: usize,
    pub batch_count: usize,
    pub model_max_length: usize,
    pub model_load_ms: u64,
    pub inference_ms: u64,
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
            batch_size: 0,
            batch_count: 0,
            model_max_length: 0,
            model_load_ms: 0,
            input_prepare_ms: 0,
            inference_ms: 0,
            average_input_tokens: 0,
        })
    }
}

pub type SemanticEmbedder<'a> = dyn Fn(Vec<String>) -> Result<Vec<Vec<f32>>, String> + 'a;

pub struct EmbeddingRescorer<'a> {
    embedder: &'a SemanticEmbedder<'a>,
}

impl<'a> EmbeddingRescorer<'a> {
    pub fn new(embedder: &'a SemanticEmbedder<'a>) -> Self {
        Self { embedder }
    }
}

impl Reranker for EmbeddingRescorer<'_> {
    fn name(&self) -> &'static str {
        "embedding-rescorer-v2"
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
                batch_size: 0,
                batch_count: 0,
                model_max_length: 0,
                model_load_ms: 0,
                input_prepare_ms: 0,
                inference_ms: 0,
                average_input_tokens: 0,
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
        let similarities = embeddings
            .iter()
            .skip(1)
            .map(|embedding| {
                cosine_similarity(query_embedding, embedding)
                    .ok_or_else(|| "reranker_unavailable: invalid_candidate_embedding".to_string())
            })
            .collect::<Result<Vec<_>, _>>()?;
        fuse_rankings(
            &mut deterministic,
            &similarities,
            explicit_paths,
            self.name(),
            "cosine",
        );
        Ok(RerankOutcome {
            candidates: deterministic,
            reranker_version: self.name().to_string(),
            fallback: false,
            fallback_reason: String::new(),
            batch_size: 0,
            batch_count: 0,
            model_max_length: 0,
            model_load_ms: 0,
            input_prepare_ms: 0,
            inference_ms: 0,
            average_input_tokens: 0,
        })
    }
}

pub type CrossEncoderScorer<'a> =
    dyn Fn(&str, Vec<String>) -> Result<CrossEncoderScores, String> + 'a;

pub struct CrossEncoderResearchReranker<'a> {
    scorer: &'a CrossEncoderScorer<'a>,
}

impl<'a> CrossEncoderResearchReranker<'a> {
    pub fn new(scorer: &'a CrossEncoderScorer<'a>) -> Self {
        Self { scorer }
    }
}

impl Reranker for CrossEncoderResearchReranker<'_> {
    fn name(&self) -> &'static str {
        "cross-encoder-research-v1"
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
                batch_size: 0,
                batch_count: 0,
                model_max_length: 0,
                model_load_ms: 0,
                input_prepare_ms: 0,
                inference_ms: 0,
                average_input_tokens: 0,
            });
        }
        let rerank_count = deterministic.len().min(SEMANTIC_RERANK_LIMIT);
        let input_started = Instant::now();
        let documents = deterministic
            .iter()
            .take(rerank_count)
            .map(|candidate| {
                compact(
                    &format!(
                        "{}\n{}\n{}",
                        candidate.title, candidate.source_location, candidate.snippet
                    ),
                    SEMANTIC_CANDIDATE_CHARS,
                )
            })
            .collect::<Vec<_>>();
        let average_input_tokens = if documents.is_empty() {
            0
        } else {
            documents
                .iter()
                .map(|document| super::context::estimate_tokens(document) as usize)
                .sum::<usize>()
                / documents.len()
        };
        let input_prepare_ms = input_started
            .elapsed()
            .as_millis()
            .min(u128::from(u64::MAX)) as u64;
        let execution = (self.scorer)(&compact(question, SEMANTIC_CANDIDATE_CHARS), documents)
            .map_err(|error| {
                if error.starts_with("QUESTION_CANCELLED") {
                    error
                } else {
                    stable_cross_encoder_error(&error)
                }
            })?;
        if execution.scores.len() != rerank_count
            || execution.scores.iter().any(|score| !score.is_finite())
        {
            return Err("cross_encoder_unavailable: invalid_score_count".to_string());
        }
        let scores = execution
            .scores
            .into_iter()
            .map(f64::from)
            .collect::<Vec<_>>();
        fuse_rankings(
            &mut deterministic,
            &scores,
            explicit_paths,
            self.name(),
            "cross_encoder",
        );
        Ok(RerankOutcome {
            candidates: deterministic,
            reranker_version: self.name().to_string(),
            fallback: false,
            fallback_reason: String::new(),
            batch_size: execution.batch_size,
            batch_count: execution.batch_count,
            model_max_length: execution.model_max_length,
            model_load_ms: execution.model_load_ms,
            input_prepare_ms,
            inference_ms: execution.inference_ms,
            average_input_tokens,
        })
    }
}

pub struct HybridResearchReranker<'a> {
    cross_encoder: CrossEncoderResearchReranker<'a>,
    embedding: EmbeddingRescorer<'a>,
    deterministic: DeterministicResearchReranker,
}

impl<'a> HybridResearchReranker<'a> {
    pub fn new(embedder: &'a SemanticEmbedder<'a>, scorer: &'a CrossEncoderScorer<'a>) -> Self {
        Self {
            cross_encoder: CrossEncoderResearchReranker::new(scorer),
            embedding: EmbeddingRescorer::new(embedder),
            deterministic: DeterministicResearchReranker,
        }
    }
}

impl Reranker for HybridResearchReranker<'_> {
    fn name(&self) -> &'static str {
        "hybrid-cross-encoder-research-v2"
    }

    fn rerank(
        &self,
        question: &str,
        candidates: Vec<Candidate>,
        explicit_paths: &HashSet<String>,
    ) -> Result<RerankOutcome, String> {
        let embedding_candidates = candidates.clone();
        let deterministic_candidates = candidates.clone();
        match self
            .cross_encoder
            .rerank(question, candidates, explicit_paths)
        {
            Ok(outcome) => Ok(outcome),
            Err(error) if error.starts_with("QUESTION_CANCELLED") => Err(error),
            Err(cross_error) => {
                match self
                    .embedding
                    .rerank(question, embedding_candidates, explicit_paths)
                {
                    Ok(mut outcome) => {
                        outcome.fallback = true;
                        outcome.fallback_reason =
                            stable_error_kind(&cross_error, "cross_encoder_unavailable");
                        for candidate in &mut outcome.candidates {
                            candidate
                                .retrieval_reason
                                .push_str("；cross_encoder_fallback=embedding_rescorer");
                        }
                        Ok(outcome)
                    }
                    Err(error) if error.starts_with("QUESTION_CANCELLED") => Err(error),
                    Err(embedding_error) => {
                        let mut outcome = self.deterministic.rerank(
                            question,
                            deterministic_candidates,
                            explicit_paths,
                        )?;
                        outcome.fallback = true;
                        outcome.fallback_reason = format!(
                            "{}+{}",
                            stable_error_kind(&cross_error, "cross_encoder_unavailable"),
                            stable_error_kind(&embedding_error, "reranker_unavailable")
                        );
                        for candidate in &mut outcome.candidates {
                            candidate.retrieval_reason.push_str(
                            "；cross_encoder_fallback=deterministic；embedding_rescorer_fallback=deterministic",
                        );
                        }
                        Ok(outcome)
                    }
                }
            }
        }
    }
}

fn fuse_rankings(
    candidates: &mut [Candidate],
    secondary_scores: &[f64],
    explicit_paths: &HashSet<String>,
    provider_name: &str,
    metric_label: &str,
) {
    let base_scores = candidates
        .iter()
        .map(|candidate| candidate.score)
        .collect::<Vec<_>>();
    let normalized_base = normalize_scores(&base_scores);
    let normalized_secondary = normalize_scores(secondary_scores);
    for (base_rank, candidate) in candidates.iter_mut().enumerate() {
        let base_score = base_scores[base_rank];
        let base_component = normalized_base[base_rank];
        let secondary_component = normalized_secondary.get(base_rank).copied().unwrap_or(0.0);
        let mut fused = BASE_SCORE_WEIGHT * base_component
            + CROSS_ENCODER_SCORE_WEIGHT * secondary_component
            + CROSS_ENCODER_TOP_BONUS * secondary_component.powi(4);
        let explicit = explicit_paths.contains(&candidate.markdown_path)
            || explicit_paths.contains(&candidate.source_path);
        if explicit {
            fused += 0.020;
        }
        if candidate.relation.contains("reference") {
            fused -= 0.015;
        }
        if candidate.kind == "graph" || candidate.relation.contains("graph_only") {
            fused -= 0.030;
        }
        if candidate.relation.contains("primary_fallback") {
            fused -= 0.012;
        }
        let score = secondary_scores.get(base_rank).copied().unwrap_or_default();
        candidate.score = fused;
        candidate.retrieval_reason.push_str(&format!(
            "；reranker={provider_name} {metric_label}={score:.4} base_score={base_score:.6} base_rank={} base_weight={BASE_SCORE_WEIGHT:.2} provider_weight={CROSS_ENCODER_SCORE_WEIGHT:.2} provider_top_bonus={CROSS_ENCODER_TOP_BONUS:.2} fused_score={fused:.6}",
            base_rank + 1,
        ));
    }
    candidates.sort_by(|left, right| right.score.total_cmp(&left.score));
    let mut document_counts = HashMap::<String, usize>::new();
    for candidate in candidates.iter_mut() {
        let key = candidate_document_key(candidate);
        let repeats = *document_counts.get(&key).unwrap_or(&0);
        let penalty = DOCUMENT_REPEAT_PENALTY * repeats.min(4) as f64;
        candidate.score -= penalty;
        candidate.retrieval_reason.push_str(&format!(
            "；document_repeat_count={repeats} document_repeat_penalty={penalty:.3}"
        ));
        document_counts.insert(key, repeats + 1);
    }
    candidates.sort_by(|left, right| right.score.total_cmp(&left.score));
}

fn candidate_document_key(candidate: &Candidate) -> String {
    let identity = if !candidate.page_id.trim().is_empty() {
        candidate.page_id.as_str()
    } else if !candidate.source_path.trim().is_empty() {
        candidate.source_path.split('#').next().unwrap_or_default()
    } else {
        candidate
            .markdown_path
            .split('#')
            .next()
            .unwrap_or_default()
    };
    format!(
        "{}:{}",
        candidate.kind,
        identity.replace('\\', "/").to_lowercase()
    )
}

fn normalize_scores(scores: &[f64]) -> Vec<f64> {
    if scores.is_empty() {
        return Vec::new();
    }
    let minimum = scores.iter().copied().fold(f64::INFINITY, f64::min);
    let maximum = scores.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let range = maximum - minimum;
    if !minimum.is_finite() || !maximum.is_finite() || range <= f64::EPSILON {
        return vec![0.5; scores.len()];
    }
    scores
        .iter()
        .map(|score| ((score - minimum) / range).clamp(0.0, 1.0))
        .collect()
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

fn stable_cross_encoder_error(error: &str) -> String {
    let kind = stable_error_kind(error, "cross_encoder_unavailable");
    format!("cross_encoder_unavailable: {kind}")
}

fn stable_error_kind(error: &str, fallback: &str) -> String {
    error
        .split(':')
        .next()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(fallback)
        .to_ascii_lowercase()
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
            parent_block_id: String::new(),
            parent_context: String::new(),
            source_location: String::new(),
            relation: relation.into(),
            retrieval_reason: String::new(),
        }
    }

    fn cross_scores(scores: Vec<f32>) -> CrossEncoderScores {
        CrossEncoderScores {
            batch_size: scores.len(),
            batch_count: usize::from(!scores.is_empty()),
            model_max_length: 512,
            model_load_ms: 0,
            inference_ms: 0,
            scores,
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
    fn cross_encoder_reranker_improves_recall_at_five_mrr_and_ndcg_without_losing_recall_at_twenty()
    {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../../evals/semantic_reranker_cases.json"
        ))
        .expect("semantic reranker fixture");
        assert_eq!(
            fixture["schemaVersion"],
            "qa-cross-encoder-reranker-cases-v2"
        );
        assert_eq!(
            fixture["datasetRole"],
            "model_independent_cross_encoder_regression"
        );
        let cases = fixture["cases"].as_array().expect("cases");
        assert_eq!(cases.len(), fixture["caseCount"].as_u64().unwrap() as usize);
        assert!(cases.len() >= 10);
        let mut deterministic_recall5 = 0.0;
        let mut cross_encoder_recall5 = 0.0;
        let mut deterministic_recall20 = 0.0;
        let mut cross_encoder_recall20 = 0.0;
        let mut deterministic_mrr = 0.0;
        let mut cross_encoder_mrr = 0.0;
        let mut deterministic_ndcg = 0.0;
        let mut cross_encoder_ndcg = 0.0;

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
            let scorer = |_query: &str, documents: Vec<String>| {
                let mut scores = (0..documents.len())
                    .map(|index| 0.05 + index as f32 * 0.01)
                    .collect::<Vec<_>>();
                if let Some(last) = scores.last_mut() {
                    *last = 0.95;
                }
                Ok(cross_scores(scores))
            };
            let cross_encoder = CrossEncoderResearchReranker::new(&scorer)
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
            let cross_encoder_rank = rank(&cross_encoder);
            deterministic_recall5 += f64::from(deterministic_rank <= 5);
            cross_encoder_recall5 += f64::from(cross_encoder_rank <= 5);
            deterministic_recall20 += f64::from(deterministic_rank <= 20);
            cross_encoder_recall20 += f64::from(cross_encoder_rank <= 20);
            deterministic_mrr += 1.0 / deterministic_rank as f64;
            cross_encoder_mrr += 1.0 / cross_encoder_rank as f64;
            deterministic_ndcg += 1.0 / (deterministic_rank as f64 + 1.0).log2();
            cross_encoder_ndcg += 1.0 / (cross_encoder_rank as f64 + 1.0).log2();
        }
        let count = cases.len() as f64;
        assert!(cross_encoder_recall5 / count > deterministic_recall5 / count);
        assert!(cross_encoder_mrr / count > deterministic_mrr / count);
        assert!(cross_encoder_ndcg / count > deterministic_ndcg / count);
        assert_eq!(
            cross_encoder_recall20 / count,
            deterministic_recall20 / count
        );
        assert_eq!(cross_encoder_recall20 / count, 1.0);
    }

    #[test]
    fn hybrid_reranker_falls_back_to_deterministic_with_stable_reason() {
        let embedder =
            |_texts: Vec<String>| Err("SEMANTIC_UNAVAILABLE: fixture model missing".to_string());
        let scorer = |_query: &str, _documents: Vec<String>| {
            Err("CROSS_ENCODER_UNAVAILABLE: fixture model missing".to_string())
        };
        let outcome = HybridResearchReranker::new(&embedder, &scorer)
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
        assert_eq!(
            outcome.fallback_reason,
            "cross_encoder_unavailable+reranker_unavailable"
        );
        assert_eq!(outcome.reranker_version, "deterministic-research-v2");
        assert!(outcome.candidates[0]
            .retrieval_reason
            .contains("cross_encoder_fallback=deterministic"));
    }

    #[test]
    fn hybrid_reranker_never_converts_cancellation_into_fallback() {
        let embedder = |_texts: Vec<String>| Ok(vec![vec![1.0, 0.0]]);
        let scorer = |_query: &str, _documents: Vec<String>| {
            Err("QUESTION_CANCELLED: 用户停止了问答".to_string())
        };
        let error = HybridResearchReranker::new(&embedder, &scorer)
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

    #[test]
    fn cross_encoder_success_is_distinct_from_embedding_fallback() {
        let embedder =
            |_texts: Vec<String>| Err("SEMANTIC_UNAVAILABLE: should not be called".to_string());
        let scorer = |_query: &str, documents: Vec<String>| {
            Ok(cross_scores(
                (0..documents.len()).map(|index| index as f32).collect(),
            ))
        };
        let outcome = HybridResearchReranker::new(&embedder, &scorer)
            .rerank(
                "TSP",
                vec![
                    candidate("noise", "content_block_v2", "noise.md", 0.4),
                    candidate("TSP", "content_block_v2", "tsp.md", 0.1),
                ],
                &HashSet::new(),
            )
            .unwrap();
        assert!(!outcome.fallback);
        assert_eq!(outcome.reranker_version, "cross-encoder-research-v1");
        assert_eq!(outcome.batch_size, 2);
        assert!(outcome.candidates[0]
            .retrieval_reason
            .contains("fused_score="));
    }

    #[test]
    fn embedding_fallback_is_explicitly_named_and_audited() {
        let embedder = |texts: Vec<String>| {
            Ok(texts
                .into_iter()
                .enumerate()
                .map(|(index, _)| {
                    if index == 0 || index == 2 {
                        vec![1.0, 0.0]
                    } else {
                        vec![0.0, 1.0]
                    }
                })
                .collect())
        };
        let scorer = |_query: &str, _documents: Vec<String>| {
            Err("CROSS_ENCODER_UNAVAILABLE: model missing".to_string())
        };
        let outcome = HybridResearchReranker::new(&embedder, &scorer)
            .rerank(
                "TSP",
                vec![
                    candidate("noise", "content_block_v2", "noise.md", 0.4),
                    candidate("TSP", "content_block_v2", "tsp.md", 0.1),
                ],
                &HashSet::new(),
            )
            .unwrap();
        assert!(outcome.fallback);
        assert_eq!(outcome.reranker_version, "embedding-rescorer-v2");
        assert_eq!(outcome.fallback_reason, "cross_encoder_unavailable");
        assert!(outcome.candidates[0]
            .retrieval_reason
            .contains("cross_encoder_fallback=embedding_rescorer"));
    }
}
