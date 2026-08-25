use super::coverage::{evaluate_coverage, CoverageAction};
use super::fusion::{reciprocal_rank_fusion, RankedChannel};
#[cfg(test)]
use super::reranker::RerankOutcome;
use super::reranker::{HybridResearchReranker, Reranker};
use super::retrieval_contract::RetrievalContract;
use super::source_resolver::{resolve_sources, SourceResolution};
use super::{
    candidate_key, candidate_matches_facet, check_cancelled, compact, fts_query,
    index_expansion_terms, query_terms, Candidate,
};
use rusqlite::{params_from_iter, types::Value as SqlValue, Connection, OptionalExtension};
use sha2::{Digest, Sha256};
use std::collections::{HashSet, VecDeque};
use std::path::Path;
use std::sync::atomic::AtomicBool;
use std::time::Instant;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChannelAttempt {
    pub name: String,
    pub kind: String,
    pub round: usize,
    pub status: String,
    pub error_kind: String,
    pub round_fingerprint: String,
    pub candidate_count: usize,
    pub duration_ms: u64,
}

#[derive(Debug)]
pub struct RetrievalOutcome {
    pub(super) candidates: Vec<Candidate>,
    pub sources: SourceResolution,
    pub attempts: Vec<ChannelAttempt>,
    pub covered_facets: HashSet<String>,
    pub candidate_gains: Vec<usize>,
    pub stop_reason: String,
    pub reranker_version: String,
    pub reranker_status: String,
    pub reranker_latency_ms: u64,
    pub reranker_fallback: bool,
    pub reranker_fallback_reason: String,
}

impl RetrievalOutcome {
    #[cfg(test)]
    pub fn candidate_summaries(&self) -> Vec<(String, String, String, String)> {
        self.candidates
            .iter()
            .map(|candidate| {
                (
                    candidate.kind.clone(),
                    candidate.title.clone(),
                    candidate.source_path.clone(),
                    candidate.relation.clone(),
                )
            })
            .collect()
    }
}

fn elapsed_ms(started: Instant) -> u64 {
    started.elapsed().as_millis().min(u128::from(u64::MAX)) as u64
}

fn table_available(connection: &Connection, name: &str) -> bool {
    connection
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE name=?1)",
            [name],
            |row| row.get::<_, bool>(0),
        )
        .unwrap_or(false)
}

pub fn corpus_v2_available(connection: &Connection) -> bool {
    table_available(connection, "documents_v2")
        && table_available(connection, "content_blocks_v2")
        && table_available(connection, "content_blocks_fts_v2")
        && connection
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM documents_v2 WHERE active=1)",
                [],
                |row| row.get::<_, bool>(0),
            )
            .unwrap_or(false)
}

fn classify_channel_result(
    name: &str,
    kind: &str,
    round: usize,
    started: Instant,
    round_fingerprint: &str,
    result: &Result<Vec<Candidate>, String>,
) -> ChannelAttempt {
    match result {
        Ok(candidates) => ChannelAttempt {
            name: name.to_string(),
            kind: kind.to_string(),
            round,
            status: if candidates.is_empty() {
                "attempted_zero_hit"
            } else {
                "succeeded_with_hits"
            }
            .to_string(),
            error_kind: String::new(),
            round_fingerprint: round_fingerprint.to_string(),
            candidate_count: candidates.len(),
            duration_ms: elapsed_ms(started),
        },
        Err(error) => ChannelAttempt {
            name: name.to_string(),
            kind: kind.to_string(),
            round,
            status: "degraded".to_string(),
            error_kind: error
                .split(':')
                .next()
                .unwrap_or("retrieval_error")
                .to_lowercase(),
            round_fingerprint: round_fingerprint.to_string(),
            candidate_count: 0,
            duration_ms: elapsed_ms(started),
        },
    }
}

fn candidate_from_block(
    row: &rusqlite::Row<'_>,
    raw_score: f64,
    relation_override: Option<&str>,
) -> rusqlite::Result<Candidate> {
    let block_id: String = row.get(0)?;
    let document_id: String = row.get(1)?;
    let kind: String = row.get(2)?;
    let canonical_title: String = row.get(3)?;
    let heading: String = row.get(4)?;
    let role: String = row.get(5)?;
    let markdown_path: String = row.get(7)?;
    let line_start = row.get::<_, Option<i64>>(8)?;
    let line_end = row.get::<_, Option<i64>>(9)?;
    let content: String = row.get(10)?;
    let source_id = document_id
        .split_once(':')
        .map(|(_, value)| value)
        .unwrap_or(&document_id)
        .to_string();
    let relation = relation_override.map(str::to_string).unwrap_or_else(|| {
        if role == "references" {
            "reference_only".to_string()
        } else {
            "content_block_v2".to_string()
        }
    });
    Ok(Candidate {
        kind: kind.clone(),
        tier: match (kind.as_str(), role.as_str()) {
            ("paper", _) => "primary_source",
            ("book", _) => "theory",
            (_, "method" | "algorithm") => "transferable_method",
            _ => "direct",
        }
        .to_string(),
        title: if heading.trim().is_empty() {
            canonical_title
        } else {
            format!("{canonical_title} · {heading}")
        },
        snippet: compact(&content, if kind == "paper" { 1_200 } else { 640 }),
        score: raw_score,
        page_id: if matches!(kind.as_str(), "wiki" | "paper") {
            source_id.clone()
        } else {
            String::new()
        },
        page_type: if kind == "paper" {
            "source".to_string()
        } else {
            String::new()
        },
        source_path: markdown_path.clone(),
        wikilink: if matches!(kind.as_str(), "wiki" | "paper") {
            format!("[[{source_id}]]")
        } else {
            String::new()
        },
        book_id: if kind == "book" {
            source_id
        } else {
            String::new()
        },
        chapter_id: String::new(),
        physical_page_start: None,
        physical_page_end: None,
        markdown_path,
        pdf_path: String::new(),
        node_id: block_id,
        parent_block_id: String::new(),
        parent_context: String::new(),
        source_location: match (line_start, line_end) {
            (Some(start), Some(end)) if kind == "paper" => {
                format!("{heading} · 原文第 {start}–{end} 行")
            }
            (Some(start), Some(end)) => format!("{heading} · Markdown 第 {start}–{end} 行"),
            _ => heading,
        },
        relation,
        retrieval_reason: format!("Markdown ContentBlock v2；role={role}"),
    })
}

fn content_block_fts(
    connection: &Connection,
    terms: &[String],
    kinds: &[String],
    document_ids: &[String],
    limit: usize,
    relation_override: Option<&str>,
) -> Result<Vec<Candidate>, String> {
    if terms.is_empty() {
        return Ok(Vec::new());
    }
    let query = fts_query(terms);
    if query.is_empty() {
        return Ok(Vec::new());
    }
    let mut sql = String::from(
        "SELECT b.id,b.document_id,d.kind,d.canonical_title,b.heading,b.role,b.granularity,b.markdown_path,b.line_start,b.line_end,b.content,
                bm25(content_blocks_fts_v2,0.0,0.0,8.0,5.0,4.0,3.0,1.0)
         FROM content_blocks_fts_v2
         JOIN content_blocks_v2 b ON b.id=content_blocks_fts_v2.block_id
         JOIN documents_v2 d ON d.id=b.document_id
         WHERE content_blocks_fts_v2 MATCH ? AND b.active=1 AND d.active=1
           AND b.granularity IN ('section','semantic')",
    );
    let mut parameters = vec![SqlValue::Text(query)];
    if !kinds.is_empty() {
        sql.push_str(&format!(
            " AND d.kind IN ({})",
            vec!["?"; kinds.len()].join(",")
        ));
        parameters.extend(kinds.iter().cloned().map(SqlValue::Text));
    }
    if !document_ids.is_empty() {
        sql.push_str(&format!(
            " AND b.document_id IN ({})",
            vec!["?"; document_ids.len()].join(",")
        ));
        parameters.extend(document_ids.iter().cloned().map(SqlValue::Text));
    }
    sql.push_str(" ORDER BY bm25(content_blocks_fts_v2,0.0,0.0,8.0,5.0,4.0,3.0,1.0),b.document_id,b.ordinal LIMIT ?");
    parameters.push(SqlValue::Integer(limit.clamp(1, 160) as i64));
    let mut statement = connection
        .prepare(&sql)
        .map_err(|error| format!("fts_prepare: {error}"))?;
    let rows = statement
        .query_map(params_from_iter(parameters), |row| {
            let rank: f64 = row.get(11)?;
            candidate_from_block(row, 1.0 / (1.0 + rank.abs()), relation_override)
        })
        .map_err(|error| format!("fts_query: {error}"))?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("fts_decode: {error}"))
}

fn title_alias_candidates(
    connection: &Connection,
    sources: &SourceResolution,
) -> Result<Vec<Candidate>, String> {
    let mut candidates = Vec::new();
    for source in &sources.resolved {
        let candidate = connection.query_row(
            "SELECT b.id,b.document_id,d.kind,d.canonical_title,b.heading,b.role,b.granularity,b.markdown_path,b.line_start,b.line_end,b.content
             FROM documents_v2 d JOIN content_blocks_v2 b ON b.document_id=d.id
             WHERE d.id=?1 AND d.active=1 AND b.active=1
             ORDER BY CASE b.granularity WHEN 'document' THEN 0 WHEN 'section' THEN 1 ELSE 2 END,b.ordinal LIMIT 1",
            [&source.document_id],
            |row| candidate_from_block(row, if source.exact { 1.0 } else { 0.8 }, Some("exact_source_title")),
        ).optional().map_err(|error| format!("title_alias: {error}"))?;
        if let Some(mut candidate) = candidate {
            candidate
                .retrieval_reason
                .push_str(&format!("；alias={}", source.matched_alias));
            candidates.push(candidate);
        }
    }
    Ok(candidates)
}

fn effective_kinds(contract: &RetrievalContract, sources: &SourceResolution) -> Vec<String> {
    if sources.constrained && !sources.resolved.is_empty() {
        let resolved = sources.resolved_kinds();
        contract
            .requested_kinds
            .iter()
            .filter(|kind| resolved.contains(*kind))
            .cloned()
            .collect()
    } else {
        contract.requested_kinds.clone()
    }
}

fn distinct_terms(queries: &[String]) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut terms = Vec::new();
    for query in queries {
        let mut query_values = query_terms(query);
        let chinese = query
            .chars()
            .filter(|character| ('\u{4e00}'..='\u{9fff}').contains(character))
            .collect::<Vec<_>>();
        for width in [4_usize, 3, 2] {
            if chinese.len() >= width {
                query_values.extend(
                    chinese
                        .windows(width)
                        .rev()
                        .take(4)
                        .map(|window| window.iter().collect::<String>()),
                );
            }
        }
        terms.extend(
            query_values
                .into_iter()
                .filter(|term| seen.insert(term.clone())),
        );
    }
    if terms.len() > 96 {
        let tail = terms.split_off(terms.len() - 48);
        terms.truncate(48);
        terms.extend(tail);
    }
    terms
}

fn round_fingerprint(queries: &[String]) -> String {
    let mut digest = Sha256::new();
    for query in queries {
        digest.update((query.chars().count() as u64).to_le_bytes());
        digest.update(query.as_bytes());
        digest.update([0]);
    }
    format!("sha256:{:x}", digest.finalize())
}

fn covered_facets(contract: &RetrievalContract, candidates: &[Candidate]) -> HashSet<String> {
    contract
        .facets
        .iter()
        .filter(|facet| {
            candidates
                .iter()
                .any(|candidate| candidate_matches_facet(candidate, facet))
        })
        .map(|facet| facet.id.clone())
        .collect()
}

fn section_identity(candidate: &Candidate) -> String {
    format!(
        "{}|{}|{}|{}",
        candidate.kind,
        candidate.markdown_path.replace('\\', "/").to_lowercase(),
        candidate.title.trim().to_lowercase(),
        candidate.relation
    )
}

fn strip_resolved_source_names(value: &str, sources: &SourceResolution) -> String {
    let mut result = value.to_string();
    if let Some(index) = result.find("相关实体：") {
        result.truncate(index);
    }
    for source in &sources.resolved {
        for name in [&source.matched_alias, &source.canonical_title] {
            if !name.trim().is_empty() {
                result = result.replace(name, " ");
            }
        }
    }
    result
        .replace(['《', '》'], " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn run_retrieval(
    connection: &Connection,
    root: &Path,
    question: &str,
    contract: &RetrievalContract,
    cancelled: Option<&AtomicBool>,
) -> Result<RetrievalOutcome, String> {
    let embedder = |texts: Vec<String>| super::semantic::embed_texts(texts, cancelled);
    let cross_encoder = |query: &str, documents: Vec<String>| {
        super::semantic::rerank_texts(query, documents, cancelled)
    };
    let reranker = HybridResearchReranker::new(&embedder, &cross_encoder);
    run_retrieval_with_reranker(connection, root, question, contract, cancelled, &reranker)
}

fn run_retrieval_with_reranker(
    connection: &Connection,
    root: &Path,
    question: &str,
    contract: &RetrievalContract,
    cancelled: Option<&AtomicBool>,
    reranker: &dyn Reranker,
) -> Result<RetrievalOutcome, String> {
    check_cancelled(cancelled)?;
    let sources = match resolve_sources(connection, question, contract) {
        Ok(sources) => sources,
        Err(error) => {
            return Ok(RetrievalOutcome {
                candidates: Vec::new(),
                sources: SourceResolution::default(),
                attempts: vec![ChannelAttempt {
                    name: "source-resolver".to_string(),
                    kind: "source".to_string(),
                    round: 1,
                    status: "degraded".to_string(),
                    error_kind: error
                        .split(':')
                        .next()
                        .unwrap_or("source_resolver_error")
                        .to_lowercase(),
                    round_fingerprint: round_fingerprint(&[question.to_string()]),
                    candidate_count: 0,
                    duration_ms: 0,
                }],
                covered_facets: HashSet::new(),
                candidate_gains: vec![0],
                stop_reason: "source_resolver_degraded".to_string(),
                reranker_version: reranker.name().to_string(),
                reranker_status: "not_run".to_string(),
                reranker_latency_ms: 0,
                reranker_fallback: false,
                reranker_fallback_reason: String::new(),
            });
        }
    };
    if sources.constrained && sources.resolved.is_empty() && !sources.gaps.is_empty() {
        return Ok(RetrievalOutcome {
            candidates: Vec::new(),
            sources,
            attempts: vec![ChannelAttempt {
                name: "source-resolver".to_string(),
                kind: "source".to_string(),
                round: 1,
                status: "attempted_zero_hit".to_string(),
                error_kind: String::new(),
                round_fingerprint: round_fingerprint(&[question.to_string()]),
                candidate_count: 0,
                duration_ms: 0,
            }],
            covered_facets: HashSet::new(),
            candidate_gains: vec![0],
            stop_reason: "unresolved_explicit_source".to_string(),
            reranker_version: reranker.name().to_string(),
            reranker_status: "not_run".to_string(),
            reranker_latency_ms: 0,
            reranker_fallback: false,
            reranker_fallback_reason: String::new(),
        });
    }
    let document_ids = if sources.constrained {
        sources.document_ids()
    } else {
        Vec::new()
    };
    let explicit_paths = sources.markdown_paths();
    let kinds = effective_kinds(contract, &sources);
    let mut coverage_contract = contract.clone();
    if sources.constrained && !sources.resolved.is_empty() {
        coverage_contract.requested_kinds = kinds.clone();
        coverage_contract.must_attempt_kinds = kinds.clone();
    }
    let mut attempts = ["wiki", "paper", "book"]
        .into_iter()
        .filter(|kind| !kinds.iter().any(|requested| requested == *kind))
        .map(|kind| ChannelAttempt {
            name: "content-fts".to_string(),
            kind: kind.to_string(),
            round: 1,
            status: "not_requested".to_string(),
            error_kind: String::new(),
            round_fingerprint: round_fingerprint(&[question.to_string()]),
            candidate_count: 0,
            duration_ms: 0,
        })
        .collect::<Vec<_>>();
    let mut all_channels = Vec::new();
    let mut attempted_kinds = HashSet::new();
    let mut candidate_gains = Vec::new();
    let mut known_keys = HashSet::new();
    let mut queries_used = 0usize;
    let mut stop_reason = String::new();
    let mut reranker_version = reranker.name().to_string();
    let mut reranker_status = "not_run".to_string();
    let mut reranker_latency_ms = 0_u64;
    let mut reranker_fallback = false;
    let mut reranker_fallback_reason = String::new();
    let mut queue = VecDeque::new();
    let content_question = strip_resolved_source_names(question, &sources);
    let rerank_query = std::iter::once(content_question.clone())
        .chain(
            contract
                .concepts
                .iter()
                .map(|value| strip_resolved_source_names(value, &sources)),
        )
        .chain(contract.aliases.iter().cloned())
        .chain(contract.related_problems.iter().cloned())
        .chain(
            contract
                .facets
                .iter()
                .flat_map(|facet| facet.search_queries.iter().cloned()),
        )
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .take(4_000)
        .collect::<String>();
    let mut base = vec![content_question];
    base.extend(
        contract
            .concepts
            .iter()
            .map(|value| strip_resolved_source_names(value, &sources)),
    );
    base.extend(contract.aliases.clone());
    base.extend(contract.related_problems.clone());
    if !sources.constrained {
        base.extend(
            sources
                .resolved
                .iter()
                .flat_map(|source| [source.canonical_title.clone(), source.matched_alias.clone()]),
        );
    }
    queue.push_back(base);
    queue.push_back(
        contract
            .facets
            .iter()
            .flat_map(|facet| facet.search_queries.clone())
            .collect(),
    );

    let mut current_candidates = Vec::new();
    for round in 1..=contract.budget.max_rounds.min(3) {
        check_cancelled(cancelled)?;
        let queries = queue.pop_front().unwrap_or_default();
        let remaining_query_budget = contract.budget.max_queries.saturating_sub(queries_used);
        let queries = queries
            .into_iter()
            .filter(|query| !query.trim().is_empty())
            .take(remaining_query_budget)
            .collect::<Vec<_>>();
        queries_used += queries.len();
        let round_round_fingerprint = round_fingerprint(&queries);
        let terms = distinct_terms(&queries);
        let _semantic_query = if queries.is_empty() {
            question.to_string()
        } else {
            queries.join(" \n")
        };

        if round == 1 {
            let started = Instant::now();
            let result = title_alias_candidates(connection, &sources);
            attempts.push(classify_channel_result(
                "title-alias",
                "source",
                round,
                started,
                &round_round_fingerprint,
                &result,
            ));
            if let Ok(candidates) = result {
                all_channels.push(RankedChannel {
                    name: "title-alias".to_string(),
                    round,
                    candidates,
                });
            }
        }

        for kind in &kinds {
            attempted_kinds.insert(kind.clone());
            let started = Instant::now();
            let fts = content_block_fts(
                connection,
                &terms,
                std::slice::from_ref(kind),
                &document_ids,
                48,
                None,
            );
            let fts_channel = if document_ids.is_empty() {
                "content-fts"
            } else {
                "metadata-filtered-fts"
            };
            attempts.push(classify_channel_result(
                fts_channel,
                kind,
                round,
                started,
                &round_round_fingerprint,
                &fts,
            ));
            if let Ok(candidates) = fts {
                all_channels.push(RankedChannel {
                    name: format!("{fts_channel}-{kind}"),
                    round,
                    candidates,
                });
            }
        }

        let dense_started = Instant::now();
        #[cfg(not(test))]
        let dense = super::semantic::semantic_candidates_filtered(
            connection,
            root,
            &_semantic_query,
            &kinds,
            &document_ids,
            cancelled,
        );
        #[cfg(test)]
        let dense: Result<Vec<Candidate>, String> = Ok(Vec::new());
        attempts.push(classify_channel_result(
            "dense",
            "mixed",
            round,
            dense_started,
            &round_round_fingerprint,
            &dense,
        ));
        if let Ok(candidates) = dense {
            all_channels.push(RankedChannel {
                name: "dense".to_string(),
                round,
                candidates,
            });
        }

        let graph_started = Instant::now();
        let graph = super::graph::graph_candidates(connection, root, &terms, cancelled).and_then(
            |result| {
                let graph_document_ids = result
                    .candidates
                    .into_iter()
                    .filter_map(|candidate| {
                        (!candidate.page_id.is_empty())
                            .then(|| format!("wiki:{}", candidate.page_id))
                    })
                    .collect::<HashSet<_>>()
                    .into_iter()
                    .collect::<Vec<_>>();
                if graph_document_ids.is_empty() {
                    Ok(Vec::new())
                } else {
                    content_block_fts(
                        connection,
                        &terms,
                        &kinds,
                        &graph_document_ids,
                        24,
                        Some("graph_mapped_content"),
                    )
                }
            },
        );
        attempts.push(classify_channel_result(
            "graph-mapped",
            "mixed",
            round,
            graph_started,
            &round_round_fingerprint,
            &graph,
        ));
        if let Ok(candidates) = graph {
            all_channels.push(RankedChannel {
                name: "graph-mapped".to_string(),
                round,
                candidates,
            });
        }

        current_candidates = reciprocal_rank_fusion(all_channels.clone(), &explicit_paths);
        let fallback_candidates = current_candidates.clone();
        let reranker_started = Instant::now();
        current_candidates =
            match reranker.rerank(&rerank_query, current_candidates, &explicit_paths) {
                Ok(outcome) => {
                    let duration_ms = elapsed_ms(reranker_started);
                    reranker_latency_ms = reranker_latency_ms.saturating_add(duration_ms);
                    reranker_version = outcome.reranker_version;
                    if outcome.fallback {
                        reranker_status = "degraded".to_string();
                        reranker_fallback = true;
                        reranker_fallback_reason = outcome.fallback_reason.clone();
                    } else if !reranker_fallback {
                        reranker_status = "succeeded".to_string();
                    }
                    attempts.push(ChannelAttempt {
                        name: "reranker".to_string(),
                        kind: "mixed".to_string(),
                        round,
                        status: if outcome.fallback {
                            "degraded"
                        } else {
                            "succeeded_with_hits"
                        }
                        .to_string(),
                        error_kind: outcome.fallback_reason,
                        round_fingerprint: round_round_fingerprint.clone(),
                        candidate_count: outcome.candidates.len(),
                        duration_ms,
                    });
                    outcome.candidates
                }
                Err(error) => {
                    if error.starts_with("QUESTION_CANCELLED") {
                        return Err(error);
                    }
                    let duration_ms = elapsed_ms(reranker_started);
                    reranker_latency_ms = reranker_latency_ms.saturating_add(duration_ms);
                    reranker_status = "degraded".to_string();
                    reranker_fallback = true;
                    reranker_fallback_reason = error
                        .split(':')
                        .next()
                        .unwrap_or("reranker_unavailable")
                        .to_lowercase();
                    attempts.push(ChannelAttempt {
                        name: "reranker".to_string(),
                        kind: "mixed".to_string(),
                        round,
                        status: "degraded".to_string(),
                        error_kind: error
                            .split(':')
                            .next()
                            .unwrap_or("reranker_unavailable")
                            .to_lowercase(),
                        round_fingerprint: round_round_fingerprint.clone(),
                        candidate_count: fallback_candidates.len(),
                        duration_ms,
                    });
                    fallback_candidates
                }
            };
        let mut seen_sections = HashSet::new();
        current_candidates.retain(|candidate| seen_sections.insert(section_identity(candidate)));
        current_candidates.truncate(contract.budget.max_candidates);
        let after = current_candidates
            .iter()
            .map(candidate_key)
            .collect::<HashSet<_>>();
        let gain = after.difference(&known_keys).count();
        known_keys = after;
        candidate_gains.push(gain);
        let facets = covered_facets(contract, &current_candidates);
        match evaluate_coverage(
            &coverage_contract,
            &sources,
            &attempted_kinds,
            &facets,
            round,
            gain,
            queries_used,
        ) {
            CoverageAction::Stop(reason) => {
                stop_reason = reason.to_string();
                break;
            }
            CoverageAction::Continue { .. } => {
                if round == 2 && queue.is_empty() {
                    let known_terms = terms.into_iter().collect::<HashSet<_>>();
                    queue.push_back(index_expansion_terms(&current_candidates, &known_terms));
                }
            }
        }
    }
    if stop_reason.is_empty() {
        stop_reason = "max_rounds".to_string();
    }
    let facets = covered_facets(contract, &current_candidates);
    Ok(RetrievalOutcome {
        candidates: current_candidates,
        sources,
        attempts,
        covered_facets: facets,
        candidate_gains,
        stop_reason,
        reranker_version,
        reranker_status,
        reranker_latency_ms,
        reranker_fallback,
        reranker_fallback_reason,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::qa::corpus;
    use std::sync::atomic::AtomicBool;

    struct FailingReranker;

    impl Reranker for FailingReranker {
        fn name(&self) -> &'static str {
            "failing-fixture"
        }

        fn rerank(
            &self,
            _question: &str,
            _candidates: Vec<Candidate>,
            _explicit_paths: &HashSet<String>,
        ) -> Result<RerankOutcome, String> {
            Err("reranker_unavailable: fixture".to_string())
        }
    }

    #[test]
    fn source_constrained_fts_never_leaks_another_document() {
        let connection = Connection::open_in_memory().unwrap();
        corpus::db_schema(&connection).unwrap();
        connection.execute("INSERT INTO documents_v2(id,kind,canonical_title,markdown_path,content_hash,snapshot_id,active) VALUES('book:a','book','Approximation Algorithms','a.md','h','s',1),('book:b','book','Other Book','b.md','h','s',1)", []).unwrap();
        connection.execute("INSERT INTO document_aliases_v2(document_id,alias,normalized_alias) VALUES('book:a','近似算法','近似算法')", []).unwrap();
        for (id, doc, title) in [
            ("a-tsp", "book:a", "Euclidean TSP path planning"),
            ("b-tsp", "book:b", "Unrelated TSP reference"),
        ] {
            connection.execute("INSERT INTO content_blocks_v2(id,document_id,granularity,heading,heading_path_json,role,ordinal,markdown_path,content,content_hash,embedding_text,locator_json,snapshot_id,active) VALUES(?1,?2,'semantic',?3,'[]','algorithm',1,?4,?3,'h',?3,'{}','s',1)", rusqlite::params![id,doc,title,format!("{}.md",doc)]).unwrap();
            connection.execute("INSERT INTO content_blocks_fts_v2(block_id,document_id,canonical_title,aliases,heading_path,role,content) VALUES(?1,?2,?3,'','','algorithm',?3)", rusqlite::params![id,doc,title]).unwrap();
        }
        let contract = RetrievalContract::fallback("《近似算法》中有没有移动路径规划");
        let outcome = run_retrieval(
            &connection,
            Path::new("."),
            "《近似算法》中有没有 TSP path planning",
            &contract,
            None,
        )
        .unwrap();
        assert!(!outcome.candidates.is_empty());
        assert!(outcome
            .candidates
            .iter()
            .all(|candidate| candidate.source_path.starts_with("book:a")));
        assert!(outcome
            .attempts
            .iter()
            .any(|attempt| attempt.kind == "book" && attempt.name == "metadata-filtered-fts"));
        assert!(outcome
            .attempts
            .iter()
            .any(|attempt| { attempt.kind == "paper" && attempt.status == "not_requested" }));
    }

    #[test]
    fn v2_query_builder_preserves_tail_concepts_after_long_prefixes() {
        let terms = distinct_terms(&[
            "前面包含很多很多研究背景与范围限定但最终真正需要检索的是移动路径规划".to_string(),
        ]);
        assert!(
            terms
                .iter()
                .any(|term| term == "移动路径" || term == "路径规划"),
            "{terms:?}"
        );
    }

    #[test]
    fn open_contract_audits_paper_and_book_even_when_they_have_zero_hits() {
        let connection = Connection::open_in_memory().unwrap();
        corpus::db_schema(&connection).unwrap();
        for (id, kind) in [("wiki:w", "wiki"), ("paper:p", "paper"), ("book:b", "book")] {
            connection.execute("INSERT INTO documents_v2(id,kind,canonical_title,markdown_path,content_hash,snapshot_id,active) VALUES(?1,?2,?1,?3,'h','s',1)", rusqlite::params![id,kind,format!("{id}.md")]).unwrap();
        }
        let mut contract = RetrievalContract::fallback("从未出现的新术语 omega-tail");
        contract.budget.max_rounds = 3;
        let outcome = run_retrieval(
            &connection,
            Path::new("."),
            "从未出现的新术语 omega-tail",
            &contract,
            None,
        )
        .unwrap();
        for kind in ["paper", "book"] {
            assert!(
                outcome
                    .attempts
                    .iter()
                    .any(|attempt| attempt.name == "content-fts"
                        && attempt.kind == kind
                        && attempt.status == "attempted_zero_hit"),
                "{:#?}",
                outcome.attempts
            );
        }
        assert!(outcome.attempts.iter().all(|attempt| attempt.round <= 3));
        assert!(matches!(
            outcome.stop_reason.as_str(),
            "no_novel_candidates" | "max_rounds" | "all_requested_surfaces_attempted"
        ));
    }

    #[test]
    fn unresolved_explicit_source_never_falls_back_to_unrelated_documents() {
        let connection = Connection::open_in_memory().unwrap();
        corpus::db_schema(&connection).unwrap();
        connection.execute("INSERT INTO documents_v2(id,kind,canonical_title,markdown_path,content_hash,snapshot_id,active) VALUES('paper:other','paper','Other Paper','other.md','h','s',1)", []).unwrap();
        let contract = RetrievalContract::fallback("《Missing Paper》如何建模");
        let outcome = run_retrieval(
            &connection,
            Path::new("."),
            "《Missing Paper》如何建模",
            &contract,
            None,
        )
        .unwrap();
        assert!(outcome.candidates.is_empty());
        assert_eq!(outcome.stop_reason, "unresolved_explicit_source");
        assert!(!outcome.sources.gaps.is_empty());
    }

    #[test]
    fn required_facet_releases_provider_expansion_in_a_bounded_second_round() {
        let connection = Connection::open_in_memory().unwrap();
        corpus::db_schema(&connection).unwrap();
        connection.execute("INSERT INTO documents_v2(id,kind,canonical_title,markdown_path,content_hash,snapshot_id,active) VALUES('book:tsp','book','Approximation Algorithms','book.md','h','s',1)", []).unwrap();
        connection.execute("INSERT INTO content_blocks_v2(id,document_id,granularity,heading,heading_path_json,role,ordinal,markdown_path,content,content_hash,embedding_text,locator_json,snapshot_id,active) VALUES('tsp-section','book:tsp','semantic','Euclidean TSP','[]','algorithm',1,'book.md','Euclidean TSP approximation and tour construction','h','Euclidean TSP approximation and tour construction','{}','s',1)", []).unwrap();
        connection.execute("INSERT INTO content_blocks_fts_v2(block_id,document_id,canonical_title,aliases,heading_path,role,content) VALUES('tsp-section','book:tsp','Approximation Algorithms','','Euclidean TSP','algorithm','Euclidean TSP approximation and tour construction')", []).unwrap();

        let mut contract = RetrievalContract::fallback("这个主题有哪些相关内容");
        contract.budget.max_rounds = 3;
        contract
            .facets
            .push(super::super::retrieval_contract::RetrievalFacet {
                id: "path-planning".to_string(),
                label: "移动路径规划".to_string(),
                required: true,
                search_queries: vec!["Euclidean TSP".to_string()],
                preferred_kinds: vec!["book".to_string()],
            });
        let outcome = run_retrieval(
            &connection,
            Path::new("."),
            "这个主题有哪些相关内容",
            &contract,
            None,
        )
        .unwrap();

        assert!(outcome.attempts.iter().any(|attempt| {
            attempt.round == 2 && attempt.kind == "book" && attempt.status == "succeeded_with_hits"
        }));
        assert!(outcome
            .candidates
            .iter()
            .any(|candidate| candidate.title.contains("Euclidean TSP")));
        assert!(outcome.attempts.iter().all(|attempt| attempt.round <= 3));
    }

    #[test]
    fn reranker_failure_keeps_fused_candidates_and_records_degradation() {
        let connection = Connection::open_in_memory().unwrap();
        corpus::db_schema(&connection).unwrap();
        connection.execute("INSERT INTO documents_v2(id,kind,canonical_title,markdown_path,content_hash,snapshot_id,active) VALUES('book:tsp','book','Approximation Algorithms','book.md','h','s',1)", []).unwrap();
        connection.execute("INSERT INTO content_blocks_v2(id,document_id,granularity,heading,heading_path_json,role,ordinal,markdown_path,content,content_hash,embedding_text,locator_json,snapshot_id,active) VALUES('tsp','book:tsp','semantic','Euclidean TSP','[]','algorithm',1,'book.md','Euclidean TSP tour','h','Euclidean TSP tour','{}','s',1)", []).unwrap();
        connection.execute("INSERT INTO content_blocks_fts_v2(block_id,document_id,canonical_title,aliases,heading_path,role,content) VALUES('tsp','book:tsp','Approximation Algorithms','','Euclidean TSP','algorithm','Euclidean TSP tour')", []).unwrap();
        let outcome = run_retrieval_with_reranker(
            &connection,
            Path::new("."),
            "Euclidean TSP",
            &RetrievalContract::fallback("Euclidean TSP"),
            None,
            &FailingReranker,
        )
        .unwrap();
        assert!(!outcome.candidates.is_empty());
        assert!(outcome.attempts.iter().any(|attempt| {
            attempt.name == "reranker"
                && attempt.status == "degraded"
                && attempt.error_kind == "reranker_unavailable"
        }));
        assert_eq!(outcome.reranker_version, "failing-fixture");
        assert_eq!(outcome.reranker_status, "degraded");
        assert!(outcome.reranker_fallback);
        assert_eq!(outcome.reranker_fallback_reason, "reranker_unavailable");
    }

    #[test]
    fn v2_retrieval_honors_cancellation_before_channels_run() {
        let connection = Connection::open_in_memory().unwrap();
        corpus::db_schema(&connection).unwrap();
        let cancelled = AtomicBool::new(true);
        let result = run_retrieval(
            &connection,
            Path::new("."),
            "问题",
            &RetrievalContract::fallback("问题"),
            Some(&cancelled),
        );
        assert!(result.unwrap_err().starts_with("QUESTION_CANCELLED"));
    }

    #[test]
    fn failed_channel_is_audited_as_degraded_without_exposing_query_text() {
        let result: Result<Vec<Candidate>, String> =
            Err("semantic_unavailable: fixture failure".to_string());
        let fingerprint = round_fingerprint(&["sensitive research question".to_string()]);
        let attempt =
            classify_channel_result("dense", "mixed", 1, Instant::now(), &fingerprint, &result);
        assert_eq!(attempt.status, "degraded");
        assert_eq!(attempt.error_kind, "semantic_unavailable");
        assert!(attempt.round_fingerprint.starts_with("sha256:"));
        assert!(!attempt.round_fingerprint.contains("sensitive"));
    }

    #[test]
    fn section_identity_collapses_section_and_semantic_duplicates() {
        let row = |id: &str| Candidate {
            kind: "paper".into(),
            tier: "primary_source".into(),
            title: "Paper · Motivation".into(),
            snippet: String::new(),
            score: 1.0,
            page_id: "sources/paper".into(),
            page_type: "source".into(),
            source_path: "raw/paper/full.md".into(),
            wikilink: String::new(),
            book_id: String::new(),
            chapter_id: String::new(),
            physical_page_start: None,
            physical_page_end: None,
            markdown_path: "raw/paper/full.md".into(),
            pdf_path: String::new(),
            node_id: id.into(),
            parent_block_id: String::new(),
            parent_context: String::new(),
            source_location: String::new(),
            relation: "content_block_v2".into(),
            retrieval_reason: String::new(),
        };
        assert_eq!(
            section_identity(&row("section")),
            section_identity(&row("semantic"))
        );
    }
}
