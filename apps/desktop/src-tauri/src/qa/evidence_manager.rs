use super::{candidate_similarity, compact, context, Candidate};
use std::collections::{HashMap, HashSet};

pub const EVIDENCE_MANAGER_VERSION: &str = "evidence-manager-v1";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EvidenceManagementReport {
    pub input_count: usize,
    pub deduplicated_count: usize,
    pub selected_count: usize,
    pub document_count: usize,
    pub parent_expansion_count: usize,
    pub estimated_tokens: u32,
}

#[derive(Debug)]
pub struct ManagedEvidence {
    pub candidates: Vec<Candidate>,
    pub report: EvidenceManagementReport,
}

pub fn manage(candidates: &[Candidate], maximum: usize) -> ManagedEvidence {
    let maximum = maximum.max(1);
    let input_count = candidates.len();
    let mut unique = deduplicate(candidates);
    let deduplicated_count = input_count.saturating_sub(unique.len());
    for candidate in &mut unique {
        let authority = authority_bonus(candidate);
        candidate
            .retrieval_reason
            .push_str(&format!("；evidence_authority={authority:.3}"));
    }
    unique.sort_by(|left, right| right.score.total_cmp(&left.score));
    let parent_candidates = unique.clone();
    let mut selected = Vec::<Candidate>::new();
    let mut remaining = unique;
    while selected.len() < maximum && !remaining.is_empty() {
        let best = remaining
            .iter()
            .enumerate()
            .filter(|(_, candidate)| {
                candidate.kind != "paper" || document_count(&selected, candidate) < 2
            })
            .filter(|(_, candidate)| kind_count_allowed(&selected, candidate, maximum))
            .map(|(index, candidate)| {
                let redundancy = selected
                    .iter()
                    .map(|chosen| candidate_similarity(candidate, chosen))
                    .fold(0.0, f64::max);
                // Authority is deliberately a tie-break-sized signal. Retrieval
                // relevance remains dominant so evidence packaging cannot erase
                // the frozen ranking baseline.
                (
                    index,
                    candidate.score - redundancy * 0.22 + authority_bonus(candidate) * 0.001,
                )
            })
            .max_by(|left, right| left.1.total_cmp(&right.1));
        let Some((index, _)) = best else {
            break;
        };
        let candidate = remaining.remove(index);
        selected.push(candidate);
    }

    let mut parent_expansion_count = 0;
    for candidate in &mut selected {
        if candidate.relation != "semantic_block_v2" {
            continue;
        }
        let key = document_key(candidate);
        let parent = parent_candidates
            .iter()
            .filter(|other| document_key(other) == key)
            .filter(|other| other.node_id != candidate.node_id)
            .filter(|other| other.relation == "content_block_v2")
            .max_by_key(|other| other.snippet.chars().count());
        let Some(parent) = parent else {
            continue;
        };
        if parent.snippet.trim().is_empty() || candidate.snippet.contains(parent.snippet.trim()) {
            continue;
        }
        candidate.snippet = compact(
            &format!("{} 上级上下文：{}", candidate.snippet, parent.snippet),
            1_600,
        );
        candidate
            .retrieval_reason
            .push_str("；parent_context_expanded=true");
        parent_expansion_count += 1;
    }
    let estimated_tokens = selected
        .iter()
        .map(|candidate| {
            context::estimate_tokens(&candidate.snippet)
                + context::estimate_tokens(&candidate.title)
                + 24
        })
        .sum();
    let document_count = selected
        .iter()
        .map(document_key)
        .collect::<HashSet<_>>()
        .len();
    ManagedEvidence {
        report: EvidenceManagementReport {
            input_count,
            deduplicated_count,
            selected_count: selected.len(),
            document_count,
            parent_expansion_count,
            estimated_tokens,
        },
        candidates: selected,
    }
}

fn deduplicate(candidates: &[Candidate]) -> Vec<Candidate> {
    let mut best = HashMap::<String, Candidate>::new();
    for candidate in candidates {
        let key = if !candidate.node_id.trim().is_empty() {
            format!("{}|{}", candidate.relation, candidate.node_id)
        } else {
            format!(
                "{}|{}|{}|{}",
                candidate.kind,
                document_key(candidate),
                candidate.relation,
                candidate.title.trim().to_lowercase()
            )
        };
        match best.get(&key) {
            Some(existing) if existing.score >= candidate.score => {}
            _ => {
                best.insert(key, candidate.clone());
            }
        }
    }
    best.into_values().collect()
}

fn authority_bonus(candidate: &Candidate) -> f64 {
    match candidate.tier.as_str() {
        "primary_source" => 0.12,
        "theory" => 0.09,
        "direct" => 0.07,
        "transferable_method" => 0.05,
        "similar_model" => 0.02,
        "graph_hint" => -0.18,
        _ => 0.0,
    }
}

fn document_key(candidate: &Candidate) -> String {
    if !candidate.page_id.is_empty() {
        format!("{}:{}", candidate.kind, candidate.page_id)
    } else if !candidate.book_id.is_empty() {
        format!("book:{}", candidate.book_id)
    } else if !candidate.markdown_path.is_empty() {
        candidate.markdown_path.replace('\\', "/").to_lowercase()
    } else {
        candidate.source_path.replace('\\', "/").to_lowercase()
    }
}

fn document_count(selected: &[Candidate], candidate: &Candidate) -> usize {
    let key = document_key(candidate);
    selected
        .iter()
        .filter(|chosen| document_key(chosen) == key)
        .count()
}

fn kind_count_allowed(selected: &[Candidate], candidate: &Candidate, maximum: usize) -> bool {
    let count = selected
        .iter()
        .filter(|chosen| chosen.kind == candidate.kind)
        .count();
    let cap = match candidate.kind.as_str() {
        "paper" => (maximum / 2).max(1),
        "book" => (maximum / 4).max(1),
        "graph" => (maximum / 5).max(1),
        _ => maximum,
    };
    count < cap
}

#[cfg(test)]
mod tests {
    use super::*;

    fn candidate(kind: &str, tier: &str, document: &str, title: &str, score: f64) -> Candidate {
        Candidate {
            kind: kind.into(),
            tier: tier.into(),
            title: title.into(),
            snippet: title.repeat(4),
            score,
            page_id: document.into(),
            page_type: String::new(),
            source_path: format!("{document}.md"),
            wikilink: String::new(),
            book_id: String::new(),
            chapter_id: String::new(),
            physical_page_start: None,
            physical_page_end: None,
            markdown_path: format!("{document}.md"),
            pdf_path: String::new(),
            node_id: title.into(),
            source_location: String::new(),
            relation: "content_block_v2".into(),
            retrieval_reason: String::new(),
        }
    }

    #[test]
    fn manager_deduplicates_balances_documents_and_prefers_authority() {
        let managed = manage(
            &[
                candidate("paper", "primary_source", "a", "same", 0.4),
                candidate("paper", "primary_source", "a", "same", 0.3),
                candidate("paper", "primary_source", "a", "second", 0.39),
                candidate("paper", "primary_source", "a", "third", 0.38),
                candidate("wiki", "direct", "b", "wiki", 0.37),
                candidate("book", "theory", "c", "book", 0.36),
            ],
            4,
        );
        assert_eq!(managed.report.deduplicated_count, 1);
        assert_eq!(managed.candidates.len(), 4);
        assert_eq!(managed.report.document_count, 3);
        assert!(managed
            .candidates
            .iter()
            .any(|candidate| candidate.tier == "primary_source"));
    }

    #[test]
    fn semantic_block_receives_available_parent_context() {
        let mut child = candidate("paper", "primary_source", "a", "child", 0.5);
        child.relation = "semantic_block_v2".into();
        child.snippet = "child detail".into();
        let mut parent = candidate("paper", "primary_source", "a", "parent", 0.4);
        parent.snippet = "broader section context".into();
        let managed = manage(&[child, parent], 2);
        assert_eq!(managed.report.parent_expansion_count, 1);
        assert!(managed
            .candidates
            .iter()
            .any(|candidate| candidate.snippet.contains("broader section context")));
    }
}
