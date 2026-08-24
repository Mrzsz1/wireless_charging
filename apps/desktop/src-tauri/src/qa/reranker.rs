use super::{query_terms, Candidate};
use std::collections::HashSet;

pub trait Reranker {
    fn name(&self) -> &'static str;
    fn rerank(
        &self,
        question: &str,
        candidates: Vec<Candidate>,
        explicit_paths: &HashSet<String>,
    ) -> Result<Vec<Candidate>, String>;
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
    ) -> Result<Vec<Candidate>, String> {
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
        Ok(candidates)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn candidate(title: &str, relation: &str, path: &str) -> Candidate {
        Candidate {
            kind: "book".into(),
            tier: "theory".into(),
            title: title.into(),
            snippet: title.into(),
            score: 0.1,
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
                    candidate("References TSP", "reference_only", "refs.md"),
                    candidate("TSP path planning algorithm", "content_block_v2", "tsp.md"),
                ],
                &explicit,
            )
            .unwrap();
        assert_eq!(ranked[0].source_path, "tsp.md");
    }
}
