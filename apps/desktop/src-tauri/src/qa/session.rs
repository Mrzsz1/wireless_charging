use super::{
    compact, now_string, repository_id, ChatMessage, ChatMessagePage, ChatSessionDetail,
    ChatSessionPage, ChatSessionSummary, CitationValidation, EvidenceItem, QaRunManifest,
    WaterlineSnapshot,
};
use rusqlite::{params, params_from_iter, Connection, OptionalExtension};
use std::collections::HashMap;
use std::path::Path;
use uuid::Uuid;

const CURSOR_SEPARATOR: char = '|';

fn encode_cursor(primary: &str, secondary: &str) -> String {
    format!("{primary}{CURSOR_SEPARATOR}{secondary}")
}

fn decode_cursor(value: Option<&str>) -> Result<(String, String), String> {
    let Some(value) = value.filter(|value| !value.trim().is_empty()) else {
        return Ok((String::new(), String::new()));
    };
    let (primary, secondary) = value
        .split_once(CURSOR_SEPARATOR)
        .ok_or_else(|| "会话分页 cursor 格式无效".to_string())?;
    if primary.is_empty()
        || secondary.is_empty()
        || primary.len() > 64
        || secondary.len() > 128
        || primary.chars().any(char::is_control)
        || secondary.chars().any(char::is_control)
    {
        return Err("会话分页 cursor 格式无效".to_string());
    }
    Ok((primary.to_string(), secondary.to_string()))
}

fn like_pattern(query: Option<&str>) -> String {
    let query = query.unwrap_or_default().trim().to_lowercase();
    if query.is_empty() {
        return String::new();
    }
    let escaped = query
        .replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_");
    format!("%{escaped}%")
}

pub fn create_session(
    connection: &Connection,
    root: &Path,
    title: &str,
) -> Result<ChatSessionSummary, String> {
    let id = Uuid::new_v4().to_string();
    let timestamp = now_string();
    let title = compact(title, 48);
    let title = if title.is_empty() {
        "新对话".to_string()
    } else {
        title
    };
    connection
        .execute(
            "INSERT INTO chat_sessions(id,repository_id,title,created_at,updated_at) VALUES(?1,?2,?3,?4,?5)",
            params![id, repository_id(root), title, timestamp, timestamp],
        )
        .map_err(|error| format!("创建问答会话失败：{error}"))?;
    Ok(ChatSessionSummary {
        id,
        title,
        created_at: timestamp.clone(),
        updated_at: timestamp,
        message_count: 0,
        last_message_preview: String::new(),
    })
}

pub fn list_sessions(
    connection: &Connection,
    root: &Path,
    limit: usize,
) -> Result<Vec<ChatSessionSummary>, String> {
    let mut statement = connection
        .prepare(
            "SELECT s.id,s.title,s.created_at,s.updated_at,
                    COUNT(m.id),
                    COALESCE((SELECT content FROM chat_messages lm WHERE lm.session_id=s.id ORDER BY lm.created_at DESC, lm.rowid DESC LIMIT 1),'')
             FROM chat_sessions s
             LEFT JOIN chat_messages m ON m.session_id=s.id
             WHERE s.repository_id=?1
             GROUP BY s.id
             ORDER BY s.updated_at DESC,s.id DESC
             LIMIT ?2",
        )
        .map_err(|error| format!("准备会话列表失败：{error}"))?;
    let rows = statement
        .query_map(
            params![repository_id(root), limit.clamp(1, 500) as i64],
            |row| {
                Ok(ChatSessionSummary {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    created_at: row.get(2)?,
                    updated_at: row.get(3)?,
                    message_count: row.get::<_, i64>(4)?.max(0) as usize,
                    last_message_preview: compact(&row.get::<_, String>(5)?, 80),
                })
            },
        )
        .map_err(|error| format!("查询会话列表失败：{error}"))?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("解析会话列表失败：{error}"))
}

pub fn rename_session(
    connection: &Connection,
    root: &Path,
    session_id: &str,
    title: &str,
) -> Result<(), String> {
    let changed = connection
        .execute(
            "UPDATE chat_sessions SET title=?3,updated_at=?4 WHERE id=?1 AND repository_id=?2",
            params![
                session_id,
                repository_id(root),
                compact(title, 80),
                now_string()
            ],
        )
        .map_err(|error| format!("重命名会话失败：{error}"))?;
    if changed == 0 {
        return Err("会话不存在或不属于当前知识库".to_string());
    }
    Ok(())
}

pub fn delete_session(
    connection: &Connection,
    root: &Path,
    session_id: &str,
) -> Result<(), String> {
    let changed = connection
        .execute(
            "DELETE FROM chat_sessions WHERE id=?1 AND repository_id=?2",
            params![session_id, repository_id(root)],
        )
        .map_err(|error| format!("删除会话失败：{error}"))?;
    if changed == 0 {
        return Err("会话不存在或不属于当前知识库".to_string());
    }
    Ok(())
}

pub fn get_session(
    connection: &Connection,
    root: &Path,
    session_id: &str,
) -> Result<ChatSessionDetail, String> {
    let session = session_summary(connection, root, session_id)?;
    let mut statement = connection
        .prepare(
            "SELECT id,session_id,role,content,status,created_at,error_code,error_message,waterline,provider,model,request_id,citation_validation,run_manifest
             FROM chat_messages WHERE session_id=?1 ORDER BY created_at,rowid",
        )
        .map_err(|error| format!("准备历史消息查询失败：{error}"))?;
    let rows = statement
        .query_map([session_id], |row| {
            let waterline_json: String = row.get(8)?;
            Ok(ChatMessage {
                id: row.get(0)?,
                session_id: row.get(1)?,
                role: row.get(2)?,
                content: row.get(3)?,
                status: row.get(4)?,
                created_at: row.get(5)?,
                error_code: row.get(6)?,
                error_message: row.get(7)?,
                waterline: serde_json::from_str(&waterline_json).ok(),
                provider: row.get(9)?,
                model: row.get(10)?,
                request_id: row.get(11)?,
                citation_validation: serde_json::from_str(&row.get::<_, String>(12)?).ok(),
                run_manifest: serde_json::from_str::<QaRunManifest>(&row.get::<_, String>(13)?)
                    .ok(),
                evidence: Vec::new(),
            })
        })
        .map_err(|error| format!("查询历史消息失败：{error}"))?;
    let mut messages = rows
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("解析历史消息失败：{error}"))?;
    let message_ids = messages
        .iter()
        .map(|message| message.id.clone())
        .collect::<Vec<_>>();
    let mut evidence = evidence_for_messages(connection, &message_ids)?;
    for message in &mut messages {
        message.evidence = evidence.remove(&message.id).unwrap_or_default();
    }
    Ok(ChatSessionDetail { session, messages })
}

pub(super) fn list_sessions_page(
    connection: &Connection,
    root: &Path,
    cursor: Option<&str>,
    query: Option<&str>,
    limit: usize,
) -> Result<ChatSessionPage, String> {
    let (cursor_updated_at, cursor_id) = decode_cursor(cursor)?;
    let pattern = like_pattern(query);
    let limit = limit.clamp(1, 100);
    let mut statement = connection
        .prepare(
            "SELECT s.id,s.title,s.created_at,s.updated_at,
                    COUNT(m.id),
                    COALESCE((SELECT content FROM chat_messages lm WHERE lm.session_id=s.id ORDER BY lm.created_at DESC, lm.rowid DESC LIMIT 1),'')
             FROM chat_sessions s
             LEFT JOIN chat_messages m ON m.session_id=s.id
             WHERE s.repository_id=?1
               AND (?2='' OR s.updated_at<?2 OR (s.updated_at=?2 AND s.id<?3))
               AND (?4='' OR lower(s.title) LIKE ?4 ESCAPE '\\'
                    OR EXISTS(SELECT 1 FROM chat_messages sm WHERE sm.session_id=s.id AND lower(sm.content) LIKE ?4 ESCAPE '\\'))
             GROUP BY s.id
             ORDER BY s.updated_at DESC,s.id DESC
             LIMIT ?5",
        )
        .map_err(|error| format!("准备会话分页失败：{error}"))?;
    let rows = statement
        .query_map(
            params![
                repository_id(root),
                cursor_updated_at,
                cursor_id,
                pattern,
                (limit + 1) as i64
            ],
            |row| {
                Ok(ChatSessionSummary {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    created_at: row.get(2)?,
                    updated_at: row.get(3)?,
                    message_count: row.get::<_, i64>(4)?.max(0) as usize,
                    last_message_preview: compact(&row.get::<_, String>(5)?, 80),
                })
            },
        )
        .map_err(|error| format!("查询会话分页失败：{error}"))?;
    let mut items = rows
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("解析会话分页失败：{error}"))?;
    let has_more = items.len() > limit;
    items.truncate(limit);
    let next_cursor = if has_more {
        items
            .last()
            .map(|item| encode_cursor(&item.updated_at, &item.id))
    } else {
        None
    };
    Ok(ChatSessionPage { items, next_cursor })
}

fn session_summary(
    connection: &Connection,
    root: &Path,
    session_id: &str,
) -> Result<ChatSessionSummary, String> {
    connection
        .query_row(
            "SELECT s.id,s.title,s.created_at,s.updated_at,COUNT(m.id),
                    COALESCE((SELECT content FROM chat_messages lm WHERE lm.session_id=s.id ORDER BY lm.created_at DESC,lm.rowid DESC LIMIT 1),'')
             FROM chat_sessions s LEFT JOIN chat_messages m ON m.session_id=s.id
             WHERE s.id=?1 AND s.repository_id=?2 GROUP BY s.id",
            params![session_id, repository_id(root)],
            |row| {
                Ok(ChatSessionSummary {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    created_at: row.get(2)?,
                    updated_at: row.get(3)?,
                    message_count: row.get::<_, i64>(4)?.max(0) as usize,
                    last_message_preview: compact(&row.get::<_, String>(5)?, 80),
                })
            },
        )
        .optional()
        .map_err(|error| format!("读取会话失败：{error}"))?
        .ok_or_else(|| "会话不存在或不属于当前知识库".to_string())
}

fn evidence_for_messages(
    connection: &Connection,
    message_ids: &[String],
) -> Result<HashMap<String, Vec<EvidenceItem>>, String> {
    if message_ids.is_empty() {
        return Ok(HashMap::new());
    }
    let placeholders = std::iter::repeat("?")
        .take(message_ids.len())
        .collect::<Vec<_>>()
        .join(",");
    let sql = format!(
        "SELECT message_id,payload FROM chat_evidence WHERE message_id IN ({placeholders}) ORDER BY message_id,rank"
    );
    let mut statement = connection
        .prepare(&sql)
        .map_err(|error| format!("准备批量历史证据查询失败：{error}"))?;
    let rows = statement
        .query_map(params_from_iter(message_ids.iter()), |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })
        .map_err(|error| format!("查询批量历史证据失败：{error}"))?;
    let mut evidence = HashMap::<String, Vec<EvidenceItem>>::new();
    for row in rows {
        let (message_id, payload) =
            row.map_err(|error| format!("读取批量历史证据失败：{error}"))?;
        if let Ok(item) = serde_json::from_str::<EvidenceItem>(&payload) {
            evidence.entry(message_id).or_default().push(item);
        }
    }
    Ok(evidence)
}

pub(super) fn get_session_page(
    connection: &Connection,
    root: &Path,
    session_id: &str,
    before: Option<&str>,
    limit: usize,
) -> Result<ChatMessagePage, String> {
    let session = session_summary(connection, root, session_id)?;
    let (cursor_created_at, cursor_rowid) = decode_cursor(before)?;
    let cursor_rowid = if cursor_rowid.is_empty() {
        i64::MAX
    } else {
        cursor_rowid
            .parse::<i64>()
            .map_err(|_| "消息分页 cursor 格式无效".to_string())?
    };
    let limit = limit.clamp(1, 200);
    let mut statement = connection
        .prepare(
            "SELECT rowid,id,session_id,role,content,status,created_at,error_code,error_message,waterline,provider,model,request_id,citation_validation,run_manifest
             FROM chat_messages
             WHERE session_id=?1
               AND (?2='' OR created_at<?2 OR (created_at=?2 AND rowid<?3))
             ORDER BY created_at DESC,rowid DESC
             LIMIT ?4",
        )
        .map_err(|error| format!("准备消息分页失败：{error}"))?;
    let rows = statement
        .query_map(
            params![
                session_id,
                cursor_created_at,
                cursor_rowid,
                (limit + 1) as i64
            ],
            |row| {
                let waterline_json: String = row.get(9)?;
                Ok((
                    row.get::<_, i64>(0)?,
                    ChatMessage {
                        id: row.get(1)?,
                        session_id: row.get(2)?,
                        role: row.get(3)?,
                        content: row.get(4)?,
                        status: row.get(5)?,
                        created_at: row.get(6)?,
                        error_code: row.get(7)?,
                        error_message: row.get(8)?,
                        waterline: serde_json::from_str::<WaterlineSnapshot>(&waterline_json).ok(),
                        provider: row.get(10)?,
                        model: row.get(11)?,
                        request_id: row.get(12)?,
                        citation_validation: serde_json::from_str::<CitationValidation>(
                            &row.get::<_, String>(13)?,
                        )
                        .ok(),
                        run_manifest: serde_json::from_str::<QaRunManifest>(
                            &row.get::<_, String>(14)?,
                        )
                        .ok(),
                        evidence: Vec::new(),
                    },
                ))
            },
        )
        .map_err(|error| format!("查询消息分页失败：{error}"))?;
    let mut rows = rows
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("解析消息分页失败：{error}"))?;
    let has_more = rows.len() > limit;
    rows.truncate(limit);
    let next_cursor = if has_more {
        rows.last()
            .map(|(rowid, message)| encode_cursor(&message.created_at, &rowid.to_string()))
    } else {
        None
    };
    let message_ids = rows
        .iter()
        .map(|(_, message)| message.id.clone())
        .collect::<Vec<_>>();
    let mut evidence = evidence_for_messages(connection, &message_ids)?;
    let mut messages = rows
        .into_iter()
        .map(|(_, mut message)| {
            message.evidence = evidence.remove(&message.id).unwrap_or_default();
            message
        })
        .collect::<Vec<_>>();
    messages.reverse();
    Ok(ChatMessagePage {
        session,
        messages,
        next_cursor,
    })
}

#[cfg(test)]
pub(super) fn decode_test_cursor(value: &str) -> Result<(String, String), String> {
    decode_cursor(Some(value))
}
