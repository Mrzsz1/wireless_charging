use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::Path;

use super::{
    graph_edge, graph_node, graph_payload, qa, resolve_page_summary, summary_from_row, PageSummary,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResearchTrailRequest {
    pub kind: String,
    pub page_id: Option<String>,
    pub text: Option<String>,
    pub evidence_limit: Option<usize>,
    pub method_limit: Option<usize>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResearchContextAnchor {
    pub kind: String,
    pub context_key: String,
    pub title: String,
    pub subtitle: String,
    pub page_id: String,
    pub graph_node_id: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResearchTrailItem {
    pub id: String,
    pub kind: String,
    pub rank: usize,
    pub title: String,
    pub snippet: String,
    pub score: f64,
    pub relation: String,
    pub retrieval_reason: String,
    pub page_id: String,
    pub page_type: String,
    pub source_path: String,
    pub wikilink: String,
    pub book_id: String,
    pub chapter_id: String,
    pub physical_page_start: Option<i64>,
    pub physical_page_end: Option<i64>,
    pub markdown_path: String,
    pub pdf_path: String,
    pub node_id: String,
    pub source_location: String,
    pub graph_path: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResearchTrailResponse {
    pub anchor: ResearchContextAnchor,
    pub evidence: Vec<ResearchTrailItem>,
    pub methods: Vec<ResearchTrailItem>,
    pub degraded_channels: Vec<String>,
    pub generated_at: String,
}

fn normalized_text(value: &str) -> String {
    value
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

fn fnv1a(value: &str) -> String {
    let mut hash = 0x811c9dc5_u32;
    for byte in value.as_bytes() {
        hash ^= u32::from(*byte);
        hash = hash.wrapping_mul(0x01000193);
    }
    format!("{hash:08x}")
}

fn normalized_score(score: f64) -> f64 {
    if !score.is_finite() || score <= 0.0 {
        0.0
    } else if score <= 1.0 {
        score
    } else {
        score / (1.0 + score)
    }
}

fn page_item(page: &PageSummary, score: f64, relation: &str, reason: &str) -> ResearchTrailItem {
    ResearchTrailItem {
        id: format!("wiki:{}:{relation}", page.id),
        kind: "wiki".to_string(),
        rank: 0,
        title: page.title.clone(),
        snippet: qa::compact(&page.summary, 180),
        score: score.clamp(0.0, 1.0),
        relation: relation.to_string(),
        retrieval_reason: reason.to_string(),
        page_id: page.id.clone(),
        page_type: page.page_type.clone(),
        source_path: page.source_path.clone(),
        wikilink: format!("[[{}]]", page.id.trim_end_matches(".md")),
        book_id: String::new(),
        chapter_id: String::new(),
        physical_page_start: None,
        physical_page_end: None,
        markdown_path: String::new(),
        pdf_path: String::new(),
        node_id: String::new(),
        source_location: page.source_path.clone(),
        graph_path: Vec::new(),
    }
}

fn evidence_item(item: qa::EvidenceItem) -> ResearchTrailItem {
    ResearchTrailItem {
        id: item.id,
        kind: item.kind,
        rank: item.rank,
        title: item.title,
        snippet: item.snippet,
        score: normalized_score(item.score),
        relation: item.relation,
        retrieval_reason: item.retrieval_reason,
        page_id: item.page_id,
        page_type: item.page_type,
        source_path: item.source_path,
        wikilink: item.wikilink,
        book_id: item.book_id,
        chapter_id: item.chapter_id,
        physical_page_start: item.physical_page_start,
        physical_page_end: item.physical_page_end,
        markdown_path: item.markdown_path,
        pdf_path: item.pdf_path,
        node_id: item.node_id,
        source_location: item.source_location,
        graph_path: Vec::new(),
    }
}

fn item_key(item: &ResearchTrailItem) -> String {
    if !item.page_id.is_empty() {
        format!("wiki:{}", item.page_id)
    } else if !item.chapter_id.is_empty() {
        format!("book:{}", item.chapter_id)
    } else if !item.node_id.is_empty() {
        format!("graph:{}", item.node_id)
    } else {
        format!("{}:{}", item.kind, item.title.to_lowercase())
    }
}

fn relation_priority(relation: &str) -> usize {
    match relation {
        "outgoing_link" => 0,
        "backlink" => 1,
        "graph_neighbor" => 2,
        "wiki_fts" | "book_fts" => 3,
        "field_overlap" => 4,
        _ => 5,
    }
}

fn merge_and_rank(items: Vec<ResearchTrailItem>) -> Vec<ResearchTrailItem> {
    let mut merged: HashMap<String, ResearchTrailItem> = HashMap::new();
    for item in items {
        let key = item_key(&item);
        match merged.get_mut(&key) {
            Some(existing) => {
                if item.score > existing.score {
                    let previous_reason = existing.retrieval_reason.clone();
                    *existing = item;
                    if !previous_reason.is_empty()
                        && !existing.retrieval_reason.contains(&previous_reason)
                    {
                        existing.retrieval_reason =
                            format!("{}；{}", existing.retrieval_reason, previous_reason);
                    }
                } else if !item.retrieval_reason.is_empty()
                    && !existing.retrieval_reason.contains(&item.retrieval_reason)
                {
                    existing.retrieval_reason =
                        format!("{}；{}", existing.retrieval_reason, item.retrieval_reason);
                }
            }
            None => {
                merged.insert(key, item);
            }
        }
    }
    let mut values = merged.into_values().collect::<Vec<_>>();
    values.sort_by(|left, right| {
        right
            .score
            .total_cmp(&left.score)
            .then_with(|| {
                relation_priority(&left.relation).cmp(&relation_priority(&right.relation))
            })
            .then_with(|| left.title.to_lowercase().cmp(&right.title.to_lowercase()))
            .then_with(|| left.id.cmp(&right.id))
    });
    for (index, value) in values.iter_mut().enumerate() {
        value.rank = index + 1;
    }
    values
}

fn select_diverse(items: &[ResearchTrailItem], limit: usize) -> Vec<ResearchTrailItem> {
    let limit = limit.clamp(1, 20);
    let mut selected = items.iter().take(limit).cloned().collect::<Vec<_>>();
    for required_kind in ["wiki", "book"] {
        if selected.iter().any(|item| item.kind == required_kind) {
            continue;
        }
        if let Some(item) = items.iter().find(|item| item.kind == required_kind) {
            if selected.len() >= limit {
                selected.pop();
            }
            selected.push(item.clone());
        }
    }
    let mut selected = merge_and_rank(selected);
    selected.truncate(limit);
    selected
}

fn path_matches(source_path: &str, graph_source: &str) -> bool {
    let page = source_path.replace('\\', "/").to_lowercase();
    let graph = graph_source
        .replace('\\', "/")
        .trim_start_matches("./")
        .to_lowercase();
    !graph.is_empty() && (page.ends_with(&graph) || graph.ends_with(&page))
}

fn page_for_graph_source(
    connection: &Connection,
    graph_source: &str,
) -> Result<Option<PageSummary>, String> {
    if let Some(page) = resolve_page_summary(connection, graph_source)? {
        return Ok(Some(page));
    }
    let mut statement = connection
        .prepare("SELECT id,page_type,title,year,summary,source_path,modified_at,status,epistemic,method_family FROM pages ORDER BY id")
        .map_err(|error| format!("准备 Graphify 来源映射失败：{error}"))?;
    let rows = statement
        .query_map([], summary_from_row)
        .map_err(|error| format!("执行 Graphify 来源映射失败：{error}"))?;
    for row in rows {
        let page = row.map_err(|error| format!("读取 Graphify 来源映射失败：{error}"))?;
        if path_matches(&page.source_path, graph_source) {
            return Ok(Some(page));
        }
    }
    Ok(None)
}

fn page_graph_neighbors(
    connection: &Connection,
    root: &Path,
    anchor: &PageSummary,
) -> Result<(String, Vec<ResearchTrailItem>), String> {
    let payload = graph_payload(root)?;
    let nodes = payload
        .get("nodes")
        .and_then(|value| value.as_array())
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .map(|value| graph_node(&value))
        .collect::<Vec<_>>();
    let anchor_node = nodes
        .iter()
        .find(|node| path_matches(&anchor.source_path, &node.source_file))
        .map(|node| node.id.clone())
        .unwrap_or_default();
    if anchor_node.is_empty() {
        return Ok((String::new(), Vec::new()));
    }
    let mut neighbors = HashMap::new();
    for edge_value in payload
        .get("links")
        .and_then(|value| value.as_array())
        .cloned()
        .unwrap_or_default()
    {
        let edge = graph_edge(&edge_value);
        let neighbor_id = if edge.source == anchor_node {
            Some(edge.target.clone())
        } else if edge.target == anchor_node {
            Some(edge.source.clone())
        } else {
            None
        };
        if let Some(neighbor_id) = neighbor_id {
            neighbors
                .entry(neighbor_id)
                .or_insert((edge.relation, edge.weight));
        }
    }
    let mut items = Vec::new();
    for node in nodes {
        let Some((relation, weight)) = neighbors.get(&node.id) else {
            continue;
        };
        let reason = format!("Graphify 一跳关系：{relation}");
        if let Some(page) = page_for_graph_source(connection, &node.source_file)? {
            if page.id == anchor.id {
                continue;
            }
            let mut item = page_item(
                &page,
                (0.68 + weight.min(1.0) * 0.12).min(0.82),
                "graph_neighbor",
                &reason,
            );
            item.node_id = node.id.clone();
            item.source_location = node.source_location.clone();
            item.graph_path = vec![anchor_node.clone(), node.id.clone()];
            items.push(item);
        } else {
            items.push(ResearchTrailItem {
                id: format!("graph:{}", node.id),
                kind: "graph".to_string(),
                rank: 0,
                title: node.label.clone(),
                snippet: qa::compact(&node.source_location, 180),
                score: (0.62 + weight.min(1.0) * 0.1).min(0.76),
                relation: "graph_neighbor".to_string(),
                retrieval_reason: reason,
                page_id: String::new(),
                page_type: node.node_type.clone(),
                source_path: node.source_file.clone(),
                wikilink: String::new(),
                book_id: String::new(),
                chapter_id: String::new(),
                physical_page_start: None,
                physical_page_end: None,
                markdown_path: String::new(),
                pdf_path: String::new(),
                node_id: node.id.clone(),
                source_location: node.source_location.clone(),
                graph_path: vec![anchor_node.clone(), node.id],
            });
        }
    }
    Ok((anchor_node, items))
}

fn direct_page_relations(
    connection: &Connection,
    anchor: &PageSummary,
) -> Result<Vec<ResearchTrailItem>, String> {
    let mut items = Vec::new();
    let mut statement = connection
        .prepare("SELECT target FROM wikilinks WHERE source_id=?1 ORDER BY target")
        .map_err(|error| format!("读取出链失败：{error}"))?;
    let targets = statement
        .query_map([&anchor.id], |row| row.get::<_, String>(0))
        .map_err(|error| format!("读取出链失败：{error}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("读取出链失败：{error}"))?;
    for target in targets {
        if let Some(page) = resolve_page_summary(connection, &target)? {
            if page.id != anchor.id {
                items.push(page_item(
                    &page,
                    0.98,
                    "outgoing_link",
                    "当前页面正文直接引用",
                ));
            }
        }
    }

    let mut statement = connection
        .prepare("SELECT source_id,target FROM wikilinks ORDER BY source_id,target")
        .map_err(|error| format!("读取反链失败：{error}"))?;
    let links = statement
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })
        .map_err(|error| format!("读取反链失败：{error}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("读取反链失败：{error}"))?;
    for (source_id, target) in links {
        let points_to_anchor = resolve_page_summary(connection, &target)?
            .map(|page| page.id == anchor.id)
            .unwrap_or(false);
        if points_to_anchor {
            if let Some(page) = resolve_page_summary(connection, &source_id)? {
                if page.id != anchor.id {
                    items.push(page_item(&page, 0.9, "backlink", "该页面反向引用当前页面"));
                }
            }
        }
    }
    Ok(items)
}

fn method_candidates(
    connection: &Connection,
    query: &str,
) -> Result<Vec<ResearchTrailItem>, String> {
    let terms = qa::query_terms(query);
    let expression = qa::fts_query(&terms);
    if expression.is_empty() {
        return Ok(Vec::new());
    }
    let mut statement = connection
        .prepare(
            "SELECT p.id,p.page_type,p.title,p.year,p.summary,p.source_path,p.modified_at,p.status,p.epistemic,p.method_family,\
                    snippet(pages_fts,2,'','…','…',28), bm25(pages_fts,8.0,1.0,4.0) \
             FROM pages_fts JOIN pages p ON p.id=pages_fts.page_id \
             WHERE pages_fts MATCH ?1 AND p.page_type='method' \
             ORDER BY bm25(pages_fts,8.0,1.0,4.0) LIMIT 24",
        )
        .map_err(|error| format!("准备方法检索失败：{error}"))?;
    let rows = statement
        .query_map([expression], |row| {
            let page = PageSummary {
                id: row.get(0)?,
                page_type: row.get(1)?,
                title: row.get(2)?,
                year: row.get(3)?,
                summary: row.get(4)?,
                source_path: row.get(5)?,
                modified_at: row.get(6)?,
                status: row.get(7)?,
                epistemic: row.get(8)?,
                method_family: row.get(9)?,
            };
            let snippet: String = row.get(10)?;
            let rank: f64 = row.get(11)?;
            Ok((page, snippet, rank))
        })
        .map_err(|error| format!("执行方法检索失败：{error}"))?;
    let mut items = Vec::new();
    for row in rows {
        let (page, snippet, rank) =
            row.map_err(|error| format!("读取方法检索结果失败：{error}"))?;
        let raw_score = if rank < 0.0 {
            -rank
        } else {
            1.0 / (1.0 + rank)
        };
        let mut item = page_item(
            &page,
            (normalized_score(raw_score) + 0.06).min(1.0),
            "field_overlap",
            "方法页与当前研究上下文关键词重合",
        );
        if !snippet.trim().is_empty() {
            item.snippet = qa::compact(&snippet, 180);
        }
        items.push(item);
    }
    Ok(items)
}

fn book_channel_available(connection: &Connection) -> Result<bool, String> {
    connection
        .query_row("SELECT COUNT(*) FROM book_chapters", [], |row| {
            row.get::<_, i64>(0)
        })
        .map(|count| count > 0)
        .map_err(|error| format!("读取书籍索引状态失败：{error}"))
}

pub fn prepare(
    connection: &Connection,
    root: &Path,
    request: ResearchTrailRequest,
) -> Result<ResearchTrailResponse, String> {
    let kind = request.kind.trim().to_lowercase();
    if !matches!(kind.as_str(), "page" | "question" | "search") {
        return Err("研究脉络上下文类型必须是 page、question 或 search".to_string());
    }
    let evidence_limit = request.evidence_limit.unwrap_or(5).clamp(1, 20);
    let method_limit = request.method_limit.unwrap_or(4).clamp(1, 12);
    let mut degraded_channels = Vec::new();
    if !book_channel_available(connection)? {
        degraded_channels.push("books".to_string());
    }

    let mut direct = Vec::new();
    let (anchor_page, query, mut anchor) = if kind == "page" {
        let page_id = request.page_id.as_deref().unwrap_or("").trim();
        let page = resolve_page_summary(connection, page_id)?
            .ok_or_else(|| format!("未找到研究脉络锚点页面：{page_id}"))?;
        let query = format!("{} {} {}", page.title, page.summary, page.method_family);
        direct.extend(direct_page_relations(connection, &page)?);
        (
            Some(page.clone()),
            query,
            ResearchContextAnchor {
                kind: kind.clone(),
                context_key: format!("page:{}", page.id),
                title: page.title.clone(),
                subtitle: format!("{} · {}", page.page_type, page.year),
                page_id: page.id.clone(),
                graph_node_id: String::new(),
            },
        )
    } else {
        let text = request.text.as_deref().unwrap_or("").trim();
        if text.chars().count() < 2 {
            return Err("研究问题或搜索词至少需要两个字符".to_string());
        }
        let normalized = normalized_text(text);
        (
            None,
            text.to_string(),
            ResearchContextAnchor {
                kind: kind.clone(),
                context_key: format!("{}:{}", kind, fnv1a(&normalized)),
                title: qa::compact(text, 72),
                subtitle: if kind == "question" {
                    "基于当前提问的证据检索".to_string()
                } else {
                    "基于当前搜索词的证据检索".to_string()
                },
                page_id: String::new(),
                graph_node_id: String::new(),
            },
        )
    };

    let qa_context = qa::prepare_question(connection, root, &query, 30)?;
    direct.extend(qa_context.evidence.into_iter().map(evidence_item));
    if let Some(page) = anchor_page.as_ref() {
        match page_graph_neighbors(connection, root, page) {
            Ok((node_id, graph_items)) => {
                anchor.graph_node_id = node_id;
                direct.extend(graph_items);
            }
            Err(_) => degraded_channels.push("graph".to_string()),
        }
    } else if graph_payload(root).is_err() {
        degraded_channels.push("graph".to_string());
    }

    let anchor_id = anchor.page_id.clone();
    direct.retain(|item| item.page_id.is_empty() || item.page_id != anchor_id);
    let ranked = merge_and_rank(direct);
    let evidence = select_diverse(&ranked, evidence_limit);

    let mut methods = ranked
        .iter()
        .filter(|item| item.page_type == "method")
        .cloned()
        .collect::<Vec<_>>();
    methods.extend(method_candidates(connection, &query)?);
    methods.retain(|item| item.page_id.is_empty() || item.page_id != anchor_id);
    let mut methods = merge_and_rank(methods);
    methods.truncate(method_limit);
    for (index, item) in methods.iter_mut().enumerate() {
        item.rank = index + 1;
    }

    let mut seen_degraded = HashSet::new();
    degraded_channels.retain(|channel| seen_degraded.insert(channel.clone()));
    Ok(ResearchTrailResponse {
        anchor,
        evidence,
        methods,
        degraded_channels,
        generated_at: qa_context.generated_at,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_hash_is_stable_after_whitespace_normalization() {
        assert_eq!(
            fnv1a(&normalized_text("  Wireless   Charging  ")),
            fnv1a(&normalized_text("wireless charging"))
        );
    }

    #[test]
    fn merge_prefers_stronger_relation_without_losing_reason() {
        let page = PageSummary {
            id: "methods/demo.md".to_string(),
            page_type: "method".to_string(),
            title: "Demo".to_string(),
            year: "2026".to_string(),
            summary: "summary".to_string(),
            source_path: "wiki/methods/demo.md".to_string(),
            modified_at: String::new(),
            status: String::new(),
            epistemic: String::new(),
            method_family: String::new(),
        };
        let ranked = merge_and_rank(vec![
            page_item(&page, 0.5, "wiki_fts", "全文命中"),
            page_item(&page, 0.9, "outgoing_link", "正文直接引用"),
        ]);
        assert_eq!(ranked.len(), 1);
        assert_eq!(ranked[0].relation, "outgoing_link");
        assert!(ranked[0].retrieval_reason.contains("全文命中"));
        assert_eq!(ranked[0].rank, 1);
    }

    #[test]
    fn page_context_returns_direct_method_instead_of_catalog_fallback() {
        let root = tempfile::tempdir().expect("fixture root");
        let connection = Connection::open_in_memory().expect("sqlite");
        crate::db_schema(&connection).expect("schema");
        for (id, page_type, title, summary) in [
            (
                "sources/source-a.md",
                "source",
                "Source A",
                "wireless scheduling model",
            ),
            (
                "methods/method-a.md",
                "method",
                "Method A",
                "wireless scheduling approximation",
            ),
            (
                "methods/unrelated.md",
                "method",
                "Unrelated",
                "unrelated inventory model",
            ),
        ] {
            connection.execute(
                "INSERT INTO pages(id,page_type,title,year,summary,body,source_path,modified_at,status,epistemic,method_family,scenario,objectives,constraints,frontmatter) VALUES(?1,?2,?3,'2026',?4,?4,?5,'1','','','','','','','{}')",
                rusqlite::params![id, page_type, title, summary, format!("wiki/{id}")],
            ).expect("page");
            connection.execute(
                "INSERT INTO pages_fts(page_id,title,body,keywords) VALUES(?1,?2,?3,'wireless scheduling')",
                rusqlite::params![id, title, summary],
            ).expect("fts");
        }
        connection.execute(
            "INSERT INTO wikilinks(source_id,target) VALUES('sources/source-a.md','methods/method-a')",
            [],
        ).expect("link");
        let response = prepare(
            &connection,
            root.path(),
            ResearchTrailRequest {
                kind: "page".to_string(),
                page_id: Some("sources/source-a.md".to_string()),
                text: None,
                evidence_limit: Some(5),
                method_limit: Some(4),
            },
        )
        .expect("research trail");
        assert_eq!(response.anchor.context_key, "page:sources/source-a.md");
        assert_eq!(response.evidence[0].page_id, "methods/method-a.md");
        assert_eq!(response.evidence[0].relation, "outgoing_link");
        assert!(response
            .methods
            .iter()
            .any(|item| item.page_id == "methods/method-a.md"));
        assert!(response.degraded_channels.contains(&"books".to_string()));
        assert!(response.degraded_channels.contains(&"graph".to_string()));
    }
}
