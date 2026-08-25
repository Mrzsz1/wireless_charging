use super::Candidate;
use rusqlite::Connection;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::UNIX_EPOCH;

const CANCEL_CHECK_INTERVAL: usize = 64;

#[derive(Clone)]
struct GraphNeighbor {
    label: String,
    relation: String,
}

#[derive(Clone)]
struct GraphSearchNode {
    id: String,
    label: String,
    source_path: String,
    source_location: String,
    community: String,
    community_name: String,
    neighbors: Vec<GraphNeighbor>,
    node_haystack: String,
    relation_haystack: String,
    neighbor_haystack: String,
}

#[derive(Clone, Default)]
struct GraphSearchIndex {
    nodes: Vec<GraphSearchNode>,
    token_to_nodes: HashMap<String, Vec<usize>>,
    source_to_nodes: HashMap<String, Vec<usize>>,
}

#[derive(Clone, PartialEq, Eq)]
struct GraphCacheKey {
    path: PathBuf,
    length: u64,
    modified_nanos: u128,
}

#[derive(Default)]
struct GraphCache {
    key: Option<GraphCacheKey>,
    index: Option<Arc<GraphSearchIndex>>,
}

#[derive(Default)]
pub(super) struct GraphCandidateResult {
    pub candidates: Vec<Candidate>,
    pub scanned_nodes: usize,
    pub cancel_check_count: usize,
}

#[derive(Clone)]
struct IndexedPage {
    id: String,
    page_type: String,
    title: String,
}

static GRAPH_CACHE: OnceLock<Mutex<GraphCache>> = OnceLock::new();

fn normalized_source_path(source_file: &str) -> String {
    let normalized = source_file.replace('\\', "/");
    if normalized.starts_with("wiki/") && normalized.ends_with(".md") {
        normalized
    } else if normalized.contains("/wiki/") && normalized.ends_with(".md") {
        normalized
            .split_once("/wiki/")
            .map(|(_, suffix)| format!("wiki/{suffix}"))
            .unwrap_or_default()
    } else {
        String::new()
    }
}

fn token_keys(value: &str) -> HashSet<String> {
    let lower = value.to_lowercase();
    let mut keys = HashSet::new();
    for token in lower
        .split(|character: char| {
            !character.is_alphanumeric() && character != '-' && character != '_'
        })
        .filter(|token| token.chars().count() >= 2)
    {
        keys.insert(token.to_string());
        let chinese = token
            .chars()
            .filter(|character| ('\u{4e00}'..='\u{9fff}').contains(character))
            .collect::<Vec<_>>();
        for width in [3_usize, 4] {
            for window in chinese.windows(width) {
                keys.insert(window.iter().collect());
            }
        }
    }
    keys
}

fn graph_cache_key(graph_path: &Path) -> Option<GraphCacheKey> {
    let metadata = fs::metadata(graph_path).ok()?;
    let modified_nanos = metadata
        .modified()
        .ok()?
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    Some(GraphCacheKey {
        path: graph_path.to_path_buf(),
        length: metadata.len(),
        modified_nanos,
    })
}

fn parse_graph_index(payload: &Value) -> GraphSearchIndex {
    let nodes = payload
        .get("nodes")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let links = payload
        .get("links")
        .or_else(|| payload.get("edges"))
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let labels = nodes
        .iter()
        .filter_map(|node| {
            Some((
                node.get("id")?.as_str()?.to_string(),
                node.get("label")
                    .or_else(|| node.get("name"))?
                    .as_str()?
                    .to_string(),
            ))
        })
        .collect::<HashMap<_, _>>();
    let mut adjacency: HashMap<String, Vec<GraphNeighbor>> = HashMap::new();
    for link in links {
        let Some(source) = link.get("source").and_then(Value::as_str) else {
            continue;
        };
        let Some(target) = link.get("target").and_then(Value::as_str) else {
            continue;
        };
        let relation = link
            .get("relation")
            .and_then(Value::as_str)
            .unwrap_or("related_to")
            .to_string();
        adjacency
            .entry(source.to_string())
            .or_default()
            .push(GraphNeighbor {
                label: labels
                    .get(target)
                    .cloned()
                    .unwrap_or_else(|| target.to_string()),
                relation: relation.clone(),
            });
        adjacency
            .entry(target.to_string())
            .or_default()
            .push(GraphNeighbor {
                label: labels
                    .get(source)
                    .cloned()
                    .unwrap_or_else(|| source.to_string()),
                relation,
            });
    }

    let mut index = GraphSearchIndex::default();
    for node in nodes {
        let id = node
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        let label = node
            .get("label")
            .or_else(|| node.get("name"))
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        let description = node
            .get("description")
            .or_else(|| node.get("summary"))
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        let source_file = node
            .get("source_file")
            .or_else(|| node.get("sourceFile"))
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        let source_path = normalized_source_path(&source_file);
        let source_location = node
            .get("source_location")
            .or_else(|| node.get("sourceLocation"))
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        let community = node
            .get("community")
            .map(|value| {
                value
                    .as_str()
                    .map(str::to_string)
                    .unwrap_or_else(|| value.to_string())
            })
            .unwrap_or_default();
        let community_name = node
            .get("community_name")
            .or_else(|| node.get("communityName"))
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        let neighbors = adjacency.remove(&id).unwrap_or_default();
        let relation_haystack = neighbors
            .iter()
            .map(|neighbor| neighbor.relation.as_str())
            .collect::<Vec<_>>()
            .join(" ")
            .to_lowercase();
        let neighbor_haystack = neighbors
            .iter()
            .map(|neighbor| neighbor.label.as_str())
            .collect::<Vec<_>>()
            .join(" ")
            .to_lowercase();
        let node_haystack = format!(
            "{label} {description} {source_file} {source_location} {community} {community_name}"
        )
        .to_lowercase();
        let node_index = index.nodes.len();
        for key in token_keys(&format!(
            "{node_haystack} {relation_haystack} {neighbor_haystack}"
        )) {
            index
                .token_to_nodes
                .entry(key)
                .or_default()
                .push(node_index);
        }
        if !source_path.is_empty() {
            index
                .source_to_nodes
                .entry(source_path.clone())
                .or_default()
                .push(node_index);
        }
        index.nodes.push(GraphSearchNode {
            id,
            label,
            source_path,
            source_location,
            community,
            community_name,
            neighbors,
            node_haystack,
            relation_haystack,
            neighbor_haystack,
        });
    }
    for node_ids in index.token_to_nodes.values_mut() {
        node_ids.sort_unstable();
        node_ids.dedup();
    }
    index
}

fn load_graph_index(root: &Path) -> Option<Arc<GraphSearchIndex>> {
    let graph_path = root.join("graphify-out/graph.json");
    let key = graph_cache_key(&graph_path)?;
    let cache = GRAPH_CACHE.get_or_init(|| Mutex::new(GraphCache::default()));
    if let Ok(cache) = cache.lock() {
        if cache.key.as_ref() == Some(&key) {
            return cache.index.clone();
        }
    }
    let content = fs::read_to_string(&graph_path).ok()?;
    let payload = serde_json::from_str::<Value>(&content).ok()?;
    let index = Arc::new(parse_graph_index(&payload));
    if let Ok(mut cache) = cache.lock() {
        cache.key = Some(key);
        cache.index = Some(index.clone());
    }
    Some(index)
}

fn indexed_pages(connection: &Connection, root: &Path) -> HashMap<String, IndexedPage> {
    let Ok(mut statement) = connection.prepare(
        "SELECT id,page_type,title,replace(source_path,'\\','/') FROM pages WHERE source_path<>''",
    ) else {
        return HashMap::new();
    };
    let Ok(rows) = statement.query_map([], |row| {
        Ok((
            row.get::<_, String>(3)?,
            IndexedPage {
                id: row.get(0)?,
                page_type: row.get(1)?,
                title: row.get(2)?,
            },
        ))
    }) else {
        return HashMap::new();
    };
    rows.flatten()
        .filter(|(source_path, _)| root.join(source_path).is_file())
        .collect()
}

fn candidate_node_indices(
    index: &GraphSearchIndex,
    pages: &HashMap<String, IndexedPage>,
    terms: &[String],
) -> Vec<usize> {
    let mut candidates = HashSet::new();
    let mut requires_full_scan = false;
    for term in terms {
        let mut term_indexed = false;
        for key in token_keys(term) {
            if let Some(node_ids) = index.token_to_nodes.get(&key) {
                candidates.extend(node_ids.iter().copied());
                term_indexed = true;
            }
        }
        let lower = term.to_lowercase();
        for (source_path, page) in pages {
            if page.title.to_lowercase().contains(&lower) {
                if let Some(node_ids) = index.source_to_nodes.get(source_path) {
                    candidates.extend(node_ids.iter().copied());
                    term_indexed = true;
                }
            }
        }
        // The scoring contract uses substring matching. If any query term has
        // no exact token/title index entry, scanning only the other terms'
        // union could silently drop a substring hit on another node.
        requires_full_scan |= !term_indexed;
    }
    if requires_full_scan || candidates.is_empty() && !terms.is_empty() {
        return (0..index.nodes.len())
            .filter(|node_index| pages.contains_key(&index.nodes[*node_index].source_path))
            .collect();
    }
    let mut candidates = candidates
        .into_iter()
        .filter(|node_index| pages.contains_key(&index.nodes[*node_index].source_path))
        .collect::<Vec<_>>();
    candidates.sort_unstable();
    candidates
}

pub(super) fn graph_candidates(
    connection: &Connection,
    root: &Path,
    terms: &[String],
    cancelled: Option<&AtomicBool>,
) -> Result<GraphCandidateResult, String> {
    let Some(index) = load_graph_index(root) else {
        return Ok(GraphCandidateResult::default());
    };
    let pages = indexed_pages(connection, root);
    let candidate_indices = candidate_node_indices(&index, &pages, terms);
    let mut result = GraphCandidateResult::default();
    for (position, node_index) in candidate_indices.into_iter().enumerate() {
        if position % CANCEL_CHECK_INTERVAL == 0 {
            result.cancel_check_count += 1;
            if cancelled.is_some_and(|flag| flag.load(Ordering::SeqCst)) {
                return Err("QUESTION_CANCELLED: 用户停止了问答".to_string());
            }
        }
        result.scanned_nodes += 1;
        let node = &index.nodes[node_index];
        let Some(page) = pages.get(&node.source_path) else {
            continue;
        };
        let node_hits = terms
            .iter()
            .filter(|term| {
                node.node_haystack.contains(term.as_str())
                    || page.title.to_lowercase().contains(term.as_str())
            })
            .count();
        let relation_hits = terms
            .iter()
            .filter(|term| node.relation_haystack.contains(term.as_str()))
            .count();
        let neighbor_hits = terms
            .iter()
            .filter(|term| node.neighbor_haystack.contains(term.as_str()))
            .count();
        if node_hits + relation_hits + neighbor_hits == 0 {
            continue;
        }
        let node_id = if node.id.is_empty() {
            node.label.clone()
        } else {
            node.id.clone()
        };
        let neighbors = node
            .neighbors
            .iter()
            .take(4)
            .map(|neighbor| format!("{}→{}", neighbor.relation, neighbor.label))
            .collect::<Vec<_>>();
        result.candidates.push(Candidate {
            kind: "graph".to_string(),
            tier: "graph_hint".to_string(),
            title: node.label.clone(),
            snippet: if neighbors.is_empty() {
                "Graphify 关系候选；需回到 Wiki 正文核验。".to_string()
            } else {
                format!("Graphify 一跳关系：{}", neighbors.join("；"))
            },
            score: 0.15
                + node_hits as f64 * 0.05
                + relation_hits as f64 * 0.08
                + neighbor_hits as f64 * 0.07
                + (!neighbors.is_empty()) as usize as f64 * 0.04,
            page_id: page.id.clone(),
            page_type: page.page_type.clone(),
            source_path: node.source_path.clone(),
            wikilink: format!("[[{}]]", page.id),
            book_id: String::new(),
            chapter_id: String::new(),
            physical_page_start: None,
            physical_page_end: None,
            markdown_path: String::new(),
            pdf_path: String::new(),
            node_id,
            parent_block_id: String::new(),
            parent_context: String::new(),
            source_location: node.source_location.clone(),
            relation: if relation_hits > 0 {
                "graph_relation".to_string()
            } else if neighbor_hits > 0 {
                "graph_neighbor".to_string()
            } else if neighbors.is_empty() {
                "graph_node".to_string()
            } else {
                "graph_one_hop".to_string()
            },
            retrieval_reason: format!(
                "Graphify nodeHits={node_hits} relationHits={relation_hits} neighborHits={neighbor_hits}；community={} {}；一跳关系 {}；已回链 Wiki，仅作关系提示",
                node.community,
                node.community_name,
                neighbors.join("、")
            ),
        });
    }
    result
        .candidates
        .sort_by(|left, right| right.score.total_cmp(&left.score));
    result.candidates.truncate(5);
    Ok(result)
}
