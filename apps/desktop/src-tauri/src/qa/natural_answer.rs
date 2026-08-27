use super::{
    compact, normalize_unverified_answer, CitationRepair, CitationValidation, EvidenceItem,
};

pub const ANSWER_FORMAT: &str = "natural-markdown-v2";
pub const APPENDIX_HEADING: &str = "## 参考证据";

#[derive(Debug)]
pub struct NaturalAnswerResult {
    pub markdown: String,
    pub validation: CitationValidation,
    pub repair: CitationRepair,
}

fn strip_existing_appendix(value: &str) -> &str {
    value
        .find(APPENDIX_HEADING)
        .map(|index| &value[..index])
        .unwrap_or(value)
}

fn strip_evidence_tokens(value: &str) -> (String, Vec<String>) {
    let mut result = String::with_capacity(value.len());
    let mut removed = Vec::new();
    let bytes = value.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'[' && bytes.get(index + 1) == Some(&b'E') {
            let mut end = index + 2;
            while bytes.get(end).is_some_and(u8::is_ascii_digit) {
                end += 1;
            }
            if end > index + 2 && bytes.get(end) == Some(&b']') {
                removed.push(value[index + 1..end].to_string());
                index = end + 1;
                continue;
            }
        }
        let character = value[index..].chars().next().expect("valid UTF-8 boundary");
        result.push(character);
        index += character.len_utf8();
    }
    removed.sort();
    removed.dedup();
    (result, removed)
}

fn safe_link_target(target: &str) -> bool {
    let value = target.trim();
    value.starts_with("https://")
        || value.starts_with("http://")
        || value.starts_with('#')
        || (!value.contains(':')
            && !value.starts_with('/')
            && !value.starts_with('\\')
            && !value.split(['/', '\\']).any(|part| part == ".."))
}

fn sanitize_markdown_link_targets(value: &str) -> String {
    let mut result = value.to_string();
    let mut search_from = 0;
    while let Some(relative) = result[search_from..].find("](") {
        let target_start = search_from + relative + 2;
        let Some(relative_end) = result[target_start..].find(')') else {
            break;
        };
        let target_end = target_start + relative_end;
        if safe_link_target(&result[target_start..target_end]) {
            search_from = target_end + 1;
        } else {
            result.replace_range(target_start..target_end, "#blocked-link");
            search_from = target_start + "#blocked-link".len() + 1;
        }
    }
    result
}

fn redact_windows_absolute_paths(value: &str) -> String {
    let characters = value.char_indices().collect::<Vec<_>>();
    let mut result = String::with_capacity(value.len());
    let mut cursor = 0;
    let mut index = 0;
    while index < characters.len() {
        let (start, character) = characters[index];
        let drive_path = character.is_ascii_alphabetic()
            && characters
                .get(index + 1)
                .is_some_and(|(_, value)| *value == ':')
            && characters
                .get(index + 2)
                .is_some_and(|(_, value)| matches!(*value, '/' | '\\'));
        let unc_path = matches!(character, '/' | '\\')
            && characters
                .get(index + 1)
                .is_some_and(|(_, value)| *value == character)
            && (start == 0
                || value[..start]
                    .chars()
                    .last()
                    .is_some_and(char::is_whitespace));
        if !drive_path && !unc_path {
            index += 1;
            continue;
        }
        result.push_str(&value[cursor..start]);
        let mut end_index = index;
        while let Some((_, value)) = characters.get(end_index) {
            if value.is_whitespace() || matches!(*value, ')' | ']' | '>' | '，' | '。') {
                break;
            }
            end_index += 1;
        }
        cursor = characters
            .get(end_index)
            .map(|(offset, _)| *offset)
            .unwrap_or(value.len());
        result.push_str("[本地路径已隐藏]");
        index = end_index;
    }
    result.push_str(&value[cursor..]);
    result
}

fn visible_text_projection(value: &str) -> (String, Vec<String>) {
    let raw_body = strip_existing_appendix(value).trim();
    let (body, removed_ids) = strip_evidence_tokens(raw_body);
    let visible = redact_windows_absolute_paths(&sanitize_markdown_link_targets(body.trim()))
        .trim()
        .to_string();
    (visible, removed_ids)
}

pub(crate) fn project_visible_text(value: &str) -> String {
    visible_text_projection(value).0
}

fn escaped_label(value: &str) -> String {
    value
        .replace('[', "［")
        .replace(']', "］")
        .replace('(', "（")
        .replace(')', "）")
        .replace('\n', " ")
}

fn short_label(item: &EvidenceItem) -> String {
    let canonical = item.title.split(" · ").next().unwrap_or(&item.title).trim();
    let heading = item
        .locator
        .as_ref()
        .and_then(|locator| locator.heading_path.last())
        .map(String::as_str)
        .filter(|value| !value.trim().is_empty() && value.trim() != canonical);
    let title = heading
        .map(|heading| format!("{canonical} · {heading}"))
        .unwrap_or_else(|| canonical.to_string());
    escaped_label(&compact(&title, 64))
}

fn kind_label(kind: &str) -> &'static str {
    match kind {
        "paper" => "论文",
        "book" => "书籍",
        "wiki" => "知识库",
        _ => "来源",
    }
}

fn appendix(evidence: &[EvidenceItem]) -> (String, Vec<String>) {
    let links = evidence
        .iter()
        .filter(|item| item.kind != "graph")
        .filter(|item| {
            item.locator.as_ref().is_some_and(|locator| {
                !locator.document_id.trim().is_empty() && !locator.markdown_path.trim().is_empty()
            })
        })
        .collect::<Vec<_>>();
    if links.is_empty() {
        return (String::new(), Vec::new());
    }
    let mut markdown = format!("\n\n{APPENDIX_HEADING}\n\n");
    let mut ids = Vec::with_capacity(links.len());
    for item in links {
        ids.push(item.id.clone());
        markdown.push_str(&format!(
            "- [{} · {}](evidence:{})\n",
            kind_label(&item.kind),
            short_label(item),
            item.id
        ));
    }
    (markdown, ids)
}

pub fn render(answer: &str, evidence: &[EvidenceItem]) -> Result<NaturalAnswerResult, String> {
    let raw_body = strip_existing_appendix(answer).trim();
    if raw_body.is_empty() && !evidence.is_empty() {
        return Err("回答正文为空".to_string());
    }
    if raw_body.chars().count() > 200_000 {
        return Err("回答正文超过安全长度上限".to_string());
    }
    let (body, removed_ids) = visible_text_projection(answer);
    let known_ids = evidence
        .iter()
        .map(|item| item.id.as_str())
        .collect::<std::collections::HashSet<_>>();
    let removed_unknown_ids = removed_ids
        .into_iter()
        .filter(|id| !known_ids.contains(id.as_str()))
        .collect::<Vec<_>>();
    if evidence.is_empty() {
        let markdown = normalize_unverified_answer(&body);
        return Ok(NaturalAnswerResult {
            markdown,
            validation: CitationValidation {
                grounding_status: "unverified".to_string(),
                zero_evidence: true,
                syntax_valid: true,
                coverage_valid: true,
                entailment_checked: false,
                appendix_integrity: true,
                ..CitationValidation::default()
            },
            repair: CitationRepair {
                applied: !removed_unknown_ids.is_empty(),
                removed_unknown_ids,
                normalized_citation_groups: 0,
            },
        });
    }
    let (appendix, appendix_evidence_ids) = appendix(evidence);
    let appendix_integrity = !appendix_evidence_ids.is_empty();
    Ok(NaturalAnswerResult {
        markdown: format!("{}{appendix}", body.trim()),
        validation: CitationValidation {
            supported: false,
            grounding_status: "unverified".to_string(),
            zero_evidence: !appendix_integrity,
            syntax_valid: true,
            coverage_valid: false,
            entailment_checked: false,
            heuristic_verification_checked: false,
            appendix_integrity,
            appendix_evidence_ids,
            ..CitationValidation::default()
        },
        repair: CitationRepair {
            applied: !removed_unknown_ids.is_empty(),
            removed_unknown_ids,
            normalized_citation_groups: 0,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::qa::corpus::SourceLocator;

    fn evidence(id: &str) -> EvidenceItem {
        EvidenceItem {
            id: id.to_string(),
            kind: "book".to_string(),
            tier: "theory".to_string(),
            title: "Approximation Algorithms · Euclidean TSP".to_string(),
            snippet: "TSP".to_string(),
            score: 1.0,
            rank: 1,
            page_id: String::new(),
            page_type: String::new(),
            source_path: String::new(),
            wikilink: String::new(),
            book_id: "approximation-algorithms".to_string(),
            chapter_id: "chapter-3".to_string(),
            physical_page_start: None,
            physical_page_end: None,
            markdown_path: "raw/canonical/core-books/approximation-algorithms/chapter-3.md"
                .to_string(),
            pdf_path: String::new(),
            node_id: "block-tsp".to_string(),
            source_location: "Euclidean TSP".to_string(),
            relation: "content_block_v2".to_string(),
            retrieval_reason: String::new(),
            locator: Some(SourceLocator {
                document_id: "book:approximation-algorithms".to_string(),
                block_id: "block-tsp".to_string(),
                heading_path: vec!["Euclidean TSP".to_string()],
                markdown_path: "raw/canonical/core-books/approximation-algorithms/chapter-3.md"
                    .to_string(),
                line_start: Some(10),
                line_end: Some(20),
                content_hash: "hash".to_string(),
                snapshot_id: "snapshot".to_string(),
            }),
        }
    }

    #[test]
    fn natural_markdown_gets_backend_owned_short_appendix() {
        let rendered = render("直接回答，不需要固定章节。", &[evidence("E1")]).unwrap();
        assert!(rendered.markdown.starts_with("直接回答"));
        assert!(rendered.markdown.contains(APPENDIX_HEADING));
        assert!(rendered
            .markdown
            .contains("[书籍 · Approximation Algorithms · Euclidean TSP](evidence:E1)"));
        assert!(rendered.validation.appendix_integrity);
    }

    #[test]
    fn provider_citations_and_unsafe_links_are_sanitized_before_appendix() {
        let rendered = render(
            "普通回答 [E99]。[伪路径](file:///C:/secret.txt)，不要显示 C:\\private\\note.md",
            &[evidence("E1")],
        )
        .unwrap();
        assert!(!rendered.markdown.contains("E99"));
        assert!(!rendered.markdown.contains("secret.txt"));
        assert!(!rendered.markdown.contains("private"));
        assert!(rendered.markdown.contains("本地路径已隐藏"));
        assert!(rendered.markdown.contains("#blocked-link"));
        assert_eq!(rendered.repair.removed_unknown_ids, vec!["E99"]);
    }

    #[test]
    fn visible_projection_is_the_exact_render_body_transformation() {
        let raw = "Synthetic [link](file:///C:/secret.txt) C:\\private\\note.md [E1]";
        let projected = project_visible_text(raw);
        let rendered = render(raw, &[evidence("E1")]).unwrap();
        let rendered_body = rendered
            .markdown
            .split_once(&format!("\n\n{APPENDIX_HEADING}"))
            .map(|(body, _)| body)
            .unwrap();

        assert_eq!(
            projected,
            "Synthetic [link](#blocked-link) [本地路径已隐藏]"
        );
        assert_eq!(rendered_body, projected);
        assert_eq!(
            rendered.markdown,
            "Synthetic [link](#blocked-link) [本地路径已隐藏]\n\n## 参考证据\n\n- [书籍 · Approximation Algorithms · Euclidean TSP](evidence:E1)\n"
        );
    }
}
