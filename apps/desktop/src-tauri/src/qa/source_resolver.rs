use super::retrieval_contract::RetrievalContract;
use rusqlite::Connection;
use serde::Serialize;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ResolvedSource {
    pub requested_name: String,
    pub document_id: String,
    pub kind: String,
    pub canonical_title: String,
    pub markdown_path: String,
    pub matched_alias: String,
    pub exact: bool,
}

#[derive(Debug, Clone, Default, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SourceResolution {
    pub constrained: bool,
    pub resolved: Vec<ResolvedSource>,
    pub gaps: Vec<String>,
}

impl SourceResolution {
    pub fn document_ids(&self) -> Vec<String> {
        self.resolved
            .iter()
            .map(|source| source.document_id.clone())
            .collect()
    }

    pub fn markdown_paths(&self) -> HashSet<String> {
        self.resolved
            .iter()
            .map(|source| source.markdown_path.clone())
            .collect()
    }

    pub fn resolved_kinds(&self) -> Vec<String> {
        let mut seen = HashSet::new();
        self.resolved
            .iter()
            .map(|source| source.kind.clone())
            .filter(|kind| seen.insert(kind.clone()))
            .collect()
    }
}

#[derive(Debug, Clone)]
struct AliasRow {
    document_id: String,
    kind: String,
    canonical_title: String,
    markdown_path: String,
    alias: String,
    normalized_alias: String,
}

pub fn normalize_source_name(value: &str) -> String {
    value
        .chars()
        .filter(|character| character.is_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

fn alias_rows(connection: &Connection) -> Result<Vec<AliasRow>, String> {
    let mut statement = connection
        .prepare(
            "SELECT d.id,d.kind,d.canonical_title,d.markdown_path,d.canonical_title,
                    lower(replace(replace(replace(replace(d.canonical_title,' ',''),'-',''),'_',''),'·',''))
             FROM documents_v2 d WHERE d.active=1
             UNION ALL
             SELECT d.id,d.kind,d.canonical_title,d.markdown_path,a.alias,a.normalized_alias
             FROM document_aliases_v2 a JOIN documents_v2 d ON d.id=a.document_id
             WHERE d.active=1",
        )
        .map_err(|error| format!("准备来源别名解析失败：{error}"))?;
    let rows = statement
        .query_map([], |row| {
            let alias: String = row.get(4)?;
            let normalized: String = row.get(5)?;
            Ok(AliasRow {
                document_id: row.get(0)?,
                kind: row.get(1)?,
                canonical_title: row.get(2)?,
                markdown_path: row.get(3)?,
                alias: alias.clone(),
                normalized_alias: if normalized.trim().is_empty() {
                    normalize_source_name(&alias)
                } else {
                    normalize_source_name(&normalized)
                },
            })
        })
        .map_err(|error| format!("读取来源别名失败：{error}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("解析来源别名失败：{error}"))?;
    Ok(rows)
}

pub fn resolve_sources(
    connection: &Connection,
    question: &str,
    contract: &RetrievalContract,
) -> Result<SourceResolution, String> {
    let rows = alias_rows(connection)?;
    let mut requested = contract.scope.explicit_sources.clone();
    let mut resolution_material = vec![question.to_string()];
    resolution_material.extend(contract.concepts.clone());
    resolution_material.extend(contract.aliases.clone());
    resolution_material.extend(contract.related_problems.clone());
    resolution_material.extend(
        contract
            .facets
            .iter()
            .flat_map(|facet| facet.search_queries.clone()),
    );
    let question_normalized = normalize_source_name(&resolution_material.join(" "));
    if requested.is_empty() {
        let mut discovered = rows
            .iter()
            .filter(|row| row.normalized_alias.chars().count() >= 3)
            .filter(|row| {
                let document_slug = row
                    .document_id
                    .split_once(':')
                    .map(|(_, value)| normalize_source_name(value))
                    .unwrap_or_default();
                question_normalized.contains(&row.normalized_alias)
                    || (document_slug.chars().count() >= 3
                        && question_normalized.contains(&document_slug))
            })
            .map(|row| (row.normalized_alias.chars().count(), row.alias.clone()))
            .collect::<Vec<_>>();
        discovered.sort_by(|left, right| right.0.cmp(&left.0).then_with(|| left.1.cmp(&right.1)));
        let mut seen = HashSet::new();
        requested.extend(
            discovered
                .into_iter()
                .map(|(_, alias)| alias)
                .filter(|alias| seen.insert(normalize_source_name(alias)))
                .take(8),
        );
    }

    let mut by_normalized: HashMap<&str, Vec<&AliasRow>> = HashMap::new();
    for row in &rows {
        by_normalized
            .entry(&row.normalized_alias)
            .or_default()
            .push(row);
    }
    let explicitly_constrained =
        contract.scope.mode == "sources" || !contract.scope.explicit_sources.is_empty();
    let mut resolution = SourceResolution {
        constrained: explicitly_constrained,
        ..SourceResolution::default()
    };
    let mut seen_documents = HashSet::new();
    for requested_name in requested {
        let normalized = normalize_source_name(&requested_name);
        let exact_matches = by_normalized
            .get(normalized.as_str())
            .cloned()
            .unwrap_or_default();
        let exact = !exact_matches.is_empty();
        let mut matches = if !exact {
            rows.iter()
                .filter(|row| {
                    normalized.chars().count() >= 3
                        && (row.normalized_alias.contains(&normalized)
                            || normalized.contains(&row.normalized_alias))
                })
                .max_by_key(|row| row.normalized_alias.chars().count())
                .into_iter()
                .collect::<Vec<_>>()
        } else {
            exact_matches
        };
        if explicitly_constrained && matches.iter().any(|row| row.kind != "wiki") {
            matches.retain(|row| row.kind != "wiki");
        }
        if matches.is_empty() {
            if explicitly_constrained {
                resolution.gaps.push(requested_name);
            }
            continue;
        }
        for row in matches {
            if seen_documents.insert(row.document_id.clone()) {
                resolution.resolved.push(ResolvedSource {
                    requested_name: requested_name.clone(),
                    document_id: row.document_id.clone(),
                    kind: row.kind.clone(),
                    canonical_title: row.canonical_title.clone(),
                    markdown_path: row.markdown_path.clone(),
                    matched_alias: row.alias.clone(),
                    exact,
                });
            }
        }
    }
    Ok(resolution)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::qa::retrieval_contract::RetrievalContract;

    fn connection() -> Connection {
        let connection = Connection::open_in_memory().unwrap();
        connection.execute_batch(
            "CREATE TABLE documents_v2(id TEXT PRIMARY KEY,kind TEXT,canonical_title TEXT,markdown_path TEXT,active INTEGER);
             CREATE TABLE document_aliases_v2(document_id TEXT,alias TEXT,normalized_alias TEXT);",
        ).unwrap();
        connection.execute("INSERT INTO documents_v2 VALUES('book:approx','book','Approximation Algorithms','raw/canonical/approximation-algorithms/chapters/front.md',1)", []).unwrap();
        connection
            .execute(
                "INSERT INTO document_aliases_v2 VALUES('book:approx','近似算法','近似算法')",
                [],
            )
            .unwrap();
        connection
    }

    #[test]
    fn resolves_explicit_book_alias_and_records_unresolved_gap() {
        let connection = connection();
        let mut contract = RetrievalContract::fallback("《近似算法》中的移动路径规划");
        contract
            .scope
            .explicit_sources
            .push("不存在的书".to_string());
        let resolution = resolve_sources(&connection, "问题", &contract).unwrap();
        assert_eq!(resolution.document_ids(), ["book:approx"]);
        assert_eq!(resolution.resolved[0].kind, "book");
        assert_eq!(resolution.gaps, ["不存在的书"]);
    }

    #[test]
    fn discovers_auditable_alias_without_question_specific_code() {
        let connection = connection();
        let contract = RetrievalContract::fallback("有没有文献或者哪本书涉及移动路径规划");
        let resolution = resolve_sources(&connection, "我想查近似算法相关内容", &contract).unwrap();
        assert_eq!(resolution.document_ids(), ["book:approx"]);
        assert!(!resolution.constrained);
    }

    #[test]
    fn planner_source_slug_resolves_the_canonical_document() {
        let connection = connection();
        connection.execute("INSERT INTO documents_v2 VALUES('paper:sources/src-guo-concurrent-ccsp','paper','Concurrently Wireless Charging Sensor Networks with Efficient Scheduling','raw/guo.md',1)", []).unwrap();
        let mut contract = RetrievalContract::fallback("破坏性干涉开关调度");
        contract.facets[0].search_queries = vec!["sources src guo concurrent ccsp".to_string()];
        let resolution = resolve_sources(&connection, "破坏性干涉开关调度", &contract).unwrap();
        assert!(
            resolution
                .resolved
                .iter()
                .any(|source| source.document_id == "paper:sources/src-guo-concurrent-ccsp"),
            "{resolution:?}"
        );
    }
}
