use super::retrieval_contract::RetrievalContract;
use super::source_resolver::SourceResolution;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoverageAction {
    Stop(&'static str),
    Continue {
        gaps: Vec<String>,
        allowed_kinds: Vec<String>,
    },
}

pub fn evaluate_coverage(
    contract: &RetrievalContract,
    sources: &SourceResolution,
    attempted_kinds: &HashSet<String>,
    covered_facets: &HashSet<String>,
    round: usize,
    unique_gain: usize,
    queries_used: usize,
) -> CoverageAction {
    if round >= contract.budget.max_rounds || round >= 3 {
        return CoverageAction::Stop("max_rounds");
    }
    if queries_used >= contract.budget.max_queries {
        return CoverageAction::Stop("query_budget_exhausted");
    }
    if round > 1 && unique_gain == 0 {
        return CoverageAction::Stop("no_novel_candidates");
    }
    let mut gaps = sources.gaps.clone();
    let mut allowed_kinds = contract
        .must_attempt_kinds
        .iter()
        .filter(|kind| !attempted_kinds.contains(*kind))
        .cloned()
        .collect::<Vec<_>>();
    for facet in contract.facets.iter().filter(|facet| facet.required) {
        if !covered_facets.contains(&facet.id) {
            gaps.push(facet.id.clone());
            for kind in &facet.preferred_kinds {
                if !allowed_kinds.contains(kind) {
                    allowed_kinds.push(kind.clone());
                }
            }
        }
    }
    if gaps.is_empty() && allowed_kinds.is_empty() {
        CoverageAction::Stop("all_requested_surfaces_attempted")
    } else if sources.constrained && sources.resolved.is_empty() {
        CoverageAction::Stop("unresolved_explicit_source")
    } else {
        CoverageAction::Continue {
            gaps,
            allowed_kinds,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weak_wiki_hits_do_not_stop_before_book_and_paper_attempts() {
        let mut contract = RetrievalContract::fallback("有没有文献或者哪本书涉及移动路径规划");
        contract.budget.max_rounds = 3;
        let attempted = HashSet::from(["wiki".to_string()]);
        let action = evaluate_coverage(
            &contract,
            &SourceResolution::default(),
            &attempted,
            &HashSet::new(),
            1,
            4,
            1,
        );
        let CoverageAction::Continue { allowed_kinds, .. } = action else {
            panic!("must continue")
        };
        assert!(allowed_kinds.contains(&"paper".to_string()));
        assert!(allowed_kinds.contains(&"book".to_string()));
    }
}
