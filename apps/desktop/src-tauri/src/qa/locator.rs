use super::corpus::SourceLocator;
use rusqlite::{Connection, OptionalExtension};
use serde::Serialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ResolvedSourceLocation {
    pub document_id: String,
    pub block_id: String,
    pub markdown_path: String,
    pub heading_path: Vec<String>,
    pub line_start: Option<i64>,
    pub line_end: Option<i64>,
    pub matched_by: String,
    pub content_hash_matches: bool,
    pub degraded_reason: String,
}

fn safe_repository_file(root: &Path, relative: &str) -> Result<PathBuf, String> {
    let normalized = relative.replace('\\', "/");
    if normalized.is_empty()
        || normalized.starts_with('/')
        || normalized.starts_with("//")
        || normalized.split('/').any(|part| part == "..")
        || (normalized.len() > 1 && normalized.as_bytes().get(1) == Some(&b':'))
    {
        return Err("SOURCE_LOCATOR_INVALID: Markdown 路径越界".to_string());
    }
    let root = root
        .canonicalize()
        .map_err(|error| format!("SOURCE_LOCATOR_INVALID: 知识库路径无效：{error}"))?;
    let path = root
        .join(normalized)
        .canonicalize()
        .map_err(|error| format!("SOURCE_LOCATOR_MISSING: Markdown 文件不存在：{error}"))?;
    if !path.starts_with(&root) || !path.is_file() {
        return Err("SOURCE_LOCATOR_INVALID: Markdown 文件必须位于知识库内".to_string());
    }
    Ok(path)
}

fn existing_relative(root: &Path, relative: &str) -> Result<String, String> {
    let path = safe_repository_file(root, relative)?;
    let root = root
        .canonicalize()
        .map_err(|error| format!("SOURCE_LOCATOR_INVALID: {error}"))?;
    path.strip_prefix(root)
        .map(|value| value.to_string_lossy().replace('\\', "/"))
        .map_err(|error| format!("SOURCE_LOCATOR_INVALID: {error}"))
}

type BlockRow = (
    String,
    String,
    String,
    String,
    Option<i64>,
    Option<i64>,
    String,
);

fn block_by_id(
    connection: &Connection,
    locator: &SourceLocator,
) -> Result<Option<BlockRow>, String> {
    connection
        .query_row(
            "SELECT id,document_id,markdown_path,heading_path_json,line_start,line_end,content_hash
             FROM content_blocks_v2 WHERE id=?1 AND document_id=?2 AND active=1",
            [&locator.block_id, &locator.document_id],
            |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                    row.get(6)?,
                ))
            },
        )
        .optional()
        .map_err(|error| format!("读取 Markdown block locator 失败：{error}"))
}

fn block_by_heading(
    connection: &Connection,
    locator: &SourceLocator,
) -> Result<Option<BlockRow>, String> {
    let heading = serde_json::to_string(&locator.heading_path).unwrap_or_else(|_| "[]".to_string());
    connection
        .query_row(
            "SELECT id,document_id,markdown_path,heading_path_json,line_start,line_end,content_hash
             FROM content_blocks_v2
             WHERE document_id=?1 AND heading_path_json=?2 AND active=1
             ORDER BY CASE granularity WHEN 'section' THEN 0 WHEN 'semantic' THEN 1 ELSE 2 END, ordinal
             LIMIT 1",
            [&locator.document_id, &heading],
            |row| Ok((row.get(0)?,row.get(1)?,row.get(2)?,row.get(3)?,row.get(4)?,row.get(5)?,row.get(6)?)),
        )
        .optional()
        .map_err(|error| format!("按标题读取 Markdown locator 失败：{error}"))
}

fn resolved_from_block(
    root: &Path,
    locator: &SourceLocator,
    row: BlockRow,
    matched_by: &str,
) -> Result<ResolvedSourceLocation, String> {
    let heading_path = serde_json::from_str(&row.3).unwrap_or_default();
    Ok(ResolvedSourceLocation {
        document_id: row.1,
        block_id: row.0,
        markdown_path: existing_relative(root, &row.2)?,
        heading_path,
        line_start: row.4,
        line_end: row.5,
        matched_by: matched_by.to_string(),
        content_hash_matches: row.6 == locator.content_hash,
        degraded_reason: if matched_by == "block" && row.6 == locator.content_hash {
            String::new()
        } else {
            "原内容位置已变化，已使用稳定标题路径定位".to_string()
        },
    })
}

pub(crate) fn resolve(
    connection: &Connection,
    root: &Path,
    locator: &SourceLocator,
) -> Result<ResolvedSourceLocation, String> {
    if locator.document_id.trim().is_empty() {
        return Err("SOURCE_LOCATOR_INVALID: documentId 不能为空".to_string());
    }
    if let Some(row) = block_by_id(connection, locator)? {
        return resolved_from_block(root, locator, row, "block");
    }
    if !locator.heading_path.is_empty() {
        if let Some(row) = block_by_heading(connection, locator)? {
            return resolved_from_block(root, locator, row, "heading");
        }
    }
    if let Ok(relative) = existing_relative(root, &locator.markdown_path) {
        if locator.line_start.is_some() || locator.line_end.is_some() {
            return Ok(ResolvedSourceLocation {
                document_id: locator.document_id.clone(),
                block_id: String::new(),
                markdown_path: relative,
                heading_path: locator.heading_path.clone(),
                line_start: locator.line_start,
                line_end: locator.line_end,
                matched_by: "line".to_string(),
                content_hash_matches: false,
                degraded_reason: "稳定内容块已变化，已降级到原行号范围".to_string(),
            });
        }
    }
    let document_path = connection
        .query_row(
            "SELECT markdown_path FROM documents_v2 WHERE id=?1 AND active=1",
            [&locator.document_id],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|error| format!("读取 Markdown 文档 locator 失败：{error}"))?
        .ok_or_else(|| "SOURCE_LOCATOR_MISSING: 来源文档已不在当前索引".to_string())?;
    Ok(ResolvedSourceLocation {
        document_id: locator.document_id.clone(),
        block_id: String::new(),
        markdown_path: existing_relative(root, &document_path)?,
        heading_path: Vec::new(),
        line_start: None,
        line_end: None,
        matched_by: "document".to_string(),
        content_hash_matches: false,
        degraded_reason: "原内容块和标题均已变化，已打开来源文档".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn resolves_block_then_heading_and_rejects_traversal() {
        let directory = tempdir().unwrap();
        let root = directory.path();
        fs::create_dir_all(root.join("wiki")).unwrap();
        fs::write(root.join("wiki/demo.md"), "# Demo\n\n## Model\nBody").unwrap();
        let connection = Connection::open_in_memory().unwrap();
        connection.execute_batch(
            "CREATE TABLE documents_v2(id TEXT PRIMARY KEY,markdown_path TEXT,active INTEGER);
             CREATE TABLE content_blocks_v2(id TEXT PRIMARY KEY,document_id TEXT,markdown_path TEXT,heading_path_json TEXT,line_start INTEGER,line_end INTEGER,content_hash TEXT,granularity TEXT,ordinal INTEGER,active INTEGER);",
        ).unwrap();
        connection
            .execute(
                "INSERT INTO documents_v2 VALUES('wiki:demo','wiki/demo.md',1)",
                [],
            )
            .unwrap();
        connection.execute(
            "INSERT INTO content_blocks_v2 VALUES('b1','wiki:demo','wiki/demo.md',?1,3,4,'hash','section',0,1)",
            [serde_json::to_string(&vec!["Demo", "Model"]).unwrap()],
        ).unwrap();
        let locator = SourceLocator {
            document_id: "wiki:demo".to_string(),
            block_id: "b1".to_string(),
            heading_path: vec!["Demo".to_string(), "Model".to_string()],
            markdown_path: "wiki/demo.md".to_string(),
            line_start: Some(3),
            line_end: Some(4),
            content_hash: "hash".to_string(),
            snapshot_id: "s".to_string(),
        };
        assert_eq!(
            resolve(&connection, root, &locator).unwrap().matched_by,
            "block"
        );
        connection
            .execute("DELETE FROM content_blocks_v2 WHERE id='b1'", [])
            .unwrap();
        connection.execute(
            "INSERT INTO content_blocks_v2 VALUES('b2','wiki:demo','wiki/demo.md',?1,3,4,'new','section',0,1)",
            [serde_json::to_string(&locator.heading_path).unwrap()],
        ).unwrap();
        assert_eq!(
            resolve(&connection, root, &locator).unwrap().matched_by,
            "heading"
        );
        let mut unsafe_locator = locator;
        unsafe_locator.block_id.clear();
        unsafe_locator.heading_path.clear();
        unsafe_locator.markdown_path = "../outside.md".to_string();
        connection.execute("DELETE FROM documents_v2", []).unwrap();
        assert!(resolve(&connection, root, &unsafe_locator).is_err());
    }
}
