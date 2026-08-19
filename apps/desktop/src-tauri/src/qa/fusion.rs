use super::{candidate_key, Candidate};
use std::collections::{HashMap, HashSet};

const RRF_K: f64 = 60.0;

#[derive(Debug, Clone)]
pub struct RankedChannel {
    pub name: String,
    pub round: usize,
    pub candidates: Vec<Candidate>,
}

pub fn reciprocal_rank_fusion(
    mut channels: Vec<RankedChannel>,
    explicit_paths: &HashSet<String>,
) -> Vec<Candidate> {
    let mut fused: HashMap<String, (Candidate, f64, Vec<String>, usize)> = HashMap::new();
    for channel in &mut channels {
        channel
            .candidates
            .sort_by(|left, right| right.score.total_cmp(&left.score));
        for (index, candidate) in channel.candidates.iter().enumerate() {
            let key = candidate_key(candidate);
            let rank = index + 1;
            let mut contribution = 1.0 / (RRF_K + rank as f64);
            if explicit_paths.contains(&candidate.markdown_path)
                || explicit_paths.contains(&candidate.source_path)
            {
                contribution += 0.025;
            }
            if candidate.kind == "graph" || candidate.relation.contains("graph_only") {
                contribution *= 0.25;
            }
            if candidate.relation.contains("reference")
                || candidate.relation.contains("primary_fallback")
            {
                contribution *= 0.35;
            }
            if channel.round > 1 {
                contribution *= 1.0 - 0.08 * (channel.round.saturating_sub(1) as f64);
            }
            let entry = fused
                .entry(key)
                .or_insert_with(|| (candidate.clone(), 0.0, Vec::new(), channel.round));
            entry.1 += contribution;
            entry.2.push(format!("{}@{}", channel.name, rank));
            entry.3 = entry.3.min(channel.round);
            if candidate.score > entry.0.score {
                entry.0 = candidate.clone();
            }
        }
    }
    let mut candidates = fused
        .into_values()
        .map(|(mut candidate, score, origins, first_round)| {
            candidate.score = score;
            candidate.retrieval_reason.push_str(&format!(
                "；RRF={score:.5}；channels={}；originRound={first_round}",
                origins.join(",")
            ));
            candidate
        })
        .collect::<Vec<_>>();
    candidates.sort_by(|left, right| {
        right
            .score
            .total_cmp(&left.score)
            .then_with(|| candidate_key(left).cmp(&candidate_key(right)))
    });
    candidates
}

#[cfg(test)]
mod tests {
    use super::*;

    fn candidate(id: &str, score: f64) -> Candidate {
        Candidate {
            kind: "book".to_string(),
            tier: "theory".to_string(),
            title: id.to_string(),
            snippet: String::new(),
            score,
            page_id: String::new(),
            page_type: String::new(),
            source_path: format!("{id}.md"),
            wikilink: String::new(),
            book_id: "book".to_string(),
            chapter_id: String::new(),
            physical_page_start: None,
            physical_page_end: None,
            markdown_path: format!("{id}.md"),
            pdf_path: String::new(),
            node_id: id.to_string(),
            source_location: String::new(),
            relation: "content_block_v2".to_string(),
            retrieval_reason: String::new(),
        }
    }

    #[test]
    fn rrf_rewards_cross_channel_hits_without_comparing_raw_scores() {
        let fused = reciprocal_rank_fusion(
            vec![
                RankedChannel {
                    name: "fts".to_string(),
                    round: 1,
                    candidates: vec![candidate("shared", 99.0), candidate("fts", 98.0)],
                },
                RankedChannel {
                    name: "dense".to_string(),
                    round: 1,
                    candidates: vec![candidate("dense", 0.9), candidate("shared", 0.8)],
                },
            ],
            &HashSet::new(),
        );
        assert_eq!(fused[0].node_id, "shared");
        assert!(fused[0].retrieval_reason.contains("fts@1"));
        assert!(fused[0].retrieval_reason.contains("dense@2"));
    }
}
