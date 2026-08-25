use super::{
    compact, CitationRepair, CitationValidation, EvidenceItem, MODEL_SUPPLEMENT_HEADING,
    MODEL_SUPPLEMENT_NOTICE, NO_EVIDENCE_NOTICE,
};
use std::collections::HashMap;

#[derive(Debug)]
struct CitationToken<'a> {
    id: &'a str,
    start: usize,
    end: usize,
}

struct MarkdownMasks {
    citation_hidden: Vec<bool>,
    claim_hidden: Vec<bool>,
}

fn is_escaped(bytes: &[u8], index: usize) -> bool {
    let mut backslashes = 0;
    let mut cursor = index;
    while cursor > 0 && bytes[cursor - 1] == b'\\' {
        backslashes += 1;
        cursor -= 1;
    }
    backslashes % 2 == 1
}

fn mark_range(mask: &mut [bool], start: usize, end: usize) {
    for hidden in &mut mask[start..end] {
        *hidden = true;
    }
}

fn find_delimiter(bytes: &[u8], start: usize, marker: u8, width: usize) -> Option<usize> {
    let delimiter = vec![marker; width];
    (start..=bytes.len().saturating_sub(width))
        .find(|&index| bytes[index..].starts_with(&delimiter))
}

fn find_closing_bracket(bytes: &[u8], hidden: &[bool], start: usize) -> Option<usize> {
    let mut depth = 1usize;
    let mut index = start;
    while index < bytes.len() {
        if hidden[index] || is_escaped(bytes, index) {
            index += 1;
            continue;
        }
        match bytes[index] {
            b'[' => depth += 1,
            b']' => {
                depth -= 1;
                if depth == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }
        index += 1;
    }
    None
}

fn find_closing_parenthesis(bytes: &[u8], start: usize) -> Option<usize> {
    let mut depth = 1usize;
    let mut index = start;
    while index < bytes.len() {
        if is_escaped(bytes, index) {
            index += 1;
            continue;
        }
        match bytes[index] {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }
        index += 1;
    }
    None
}

/// Mirrors the Markdown projection boundary used by the desktop renderer.
/// Code/math literals and both halves of existing Markdown links cannot own a
/// QA citation. Link destinations are additionally hidden from claim text.
fn markdown_masks(value: &str) -> MarkdownMasks {
    let bytes = value.as_bytes();
    let mut citation_hidden = vec![false; bytes.len()];
    let mut claim_hidden = vec![false; bytes.len()];

    // Backtick and dollar runs follow the frontend projection rule: a closed
    // run protects everything through the matching run, including newlines.
    let mut index = 0;
    while index < bytes.len() {
        let marker = bytes[index];
        if matches!(marker, b'`' | b'$') && !is_escaped(bytes, index) {
            let mut width = 1;
            while bytes.get(index + width) == Some(&marker) {
                width += 1;
            }
            if let Some(closing) = find_delimiter(bytes, index + width, marker, width) {
                let end = closing + width;
                mark_range(&mut citation_hidden, index, end);
                mark_range(&mut claim_hidden, index, end);
                index = end;
                continue;
            }
        }
        index += 1;
    }

    // Protect inline and reference-style Markdown links. The label remains
    // ordinary visible claim text, but citation-looking tokens inside it are
    // not QA evidence because the label already belongs to another link.
    index = 0;
    while index < bytes.len() {
        if bytes[index] != b'[' || citation_hidden[index] || is_escaped(bytes, index) {
            index += 1;
            continue;
        }
        let Some(label_end) = find_closing_bracket(bytes, &citation_hidden, index + 1) else {
            index += 1;
            continue;
        };
        let target_start = label_end + 1;
        let target_end = match bytes.get(target_start) {
            Some(b'(') => find_closing_parenthesis(bytes, target_start + 1),
            Some(b'[') => find_closing_bracket(bytes, &citation_hidden, target_start + 1),
            _ => None,
        };
        let Some(target_end) = target_end else {
            index += 1;
            continue;
        };
        let end = target_end + 1;
        mark_range(&mut citation_hidden, index, end);
        // Keep the target delimiters so a later per-claim citation scan still
        // recognizes the label as link-owned after the destination is hidden.
        mark_range(&mut claim_hidden, target_start + 1, target_end);
        index = end;
    }

    MarkdownMasks {
        citation_hidden,
        claim_hidden,
    }
}

fn citation_tokens(value: &str) -> Vec<CitationToken<'_>> {
    let bytes = value.as_bytes();
    let masks = markdown_masks(value);
    let mut tokens = Vec::new();
    let mut index = 0;
    while index + 3 < bytes.len() {
        if bytes[index] == b'['
            && bytes.get(index + 1) == Some(&b'E')
            && !masks.citation_hidden[index]
            && !is_escaped(bytes, index)
        {
            let mut end = index + 2;
            while end < bytes.len() && bytes[end].is_ascii_digit() {
                end += 1;
            }
            if end > index + 2 && bytes.get(end) == Some(&b']') {
                tokens.push(CitationToken {
                    id: &value[index + 1..end],
                    start: index,
                    end: end + 1,
                });
                index = end + 1;
                continue;
            }
        }
        index += 1;
    }
    tokens
}

fn parse_evidence_id(value: &str) -> Option<(&str, &str)> {
    let value = value.trim_start();
    let bytes = value.as_bytes();
    if bytes.first() != Some(&b'E') {
        return None;
    }
    let mut end = 1;
    while bytes.get(end).is_some_and(u8::is_ascii_digit) {
        end += 1;
    }
    (end > 1).then(|| (&value[..end], &value[end..]))
}

fn split_id_list(value: &str) -> Option<Vec<&str>> {
    let mut ids = Vec::new();
    for part in value.split([',', '，', '、']) {
        let (id, rest) = parse_evidence_id(part)?;
        if !rest.trim().is_empty() {
            return None;
        }
        ids.push(id);
    }
    (!ids.is_empty()).then_some(ids)
}

/// Converts only citation spellings that are provably equivalent to current
/// evidence IDs. It never invents an ID or changes factual prose.
fn normalize_citation_groups(answer: &str, evidence: &[EvidenceItem]) -> (String, usize) {
    let known = evidence
        .iter()
        .map(|item| item.id.as_str())
        .collect::<std::collections::HashSet<_>>();
    let masks = markdown_masks(answer);
    let bytes = answer.as_bytes();
    let mut output = String::with_capacity(answer.len());
    let mut copied_through = 0;
    let mut index = 0;
    let mut normalized = 0;

    while index < bytes.len() {
        if bytes[index] != b'[' || masks.citation_hidden[index] || is_escaped(bytes, index) {
            index += 1;
            continue;
        }
        let Some(closing) = find_closing_bracket(bytes, &masks.citation_hidden, index + 1) else {
            break;
        };
        let inside = &answer[index + 1..closing];
        if !inside.trim_start().starts_with('E') {
            index = closing + 1;
            continue;
        }

        let mut rendered_parts = Vec::new();
        let mut valid = true;
        for part in inside.split([';', '；']) {
            let trimmed = part.trim();
            if let Some(ids) = split_id_list(trimmed) {
                if ids.iter().all(|id| known.contains(id)) {
                    rendered_parts.push(
                        ids.into_iter()
                            .map(|id| format!("[{id}]"))
                            .collect::<Vec<_>>()
                            .join(" "),
                    );
                    continue;
                }
                valid = false;
                break;
            }
            let Some((id, rest)) = parse_evidence_id(trimmed) else {
                valid = false;
                break;
            };
            if !known.contains(id) {
                valid = false;
                break;
            }
            let rest = rest.trim_start();
            let location = rest
                .strip_prefix(',')
                .or_else(|| rest.strip_prefix('，'))
                .or_else(|| rest.strip_prefix('、'))
                .map(str::trim)
                .filter(|value| !value.is_empty());
            let Some(location) = location else {
                valid = false;
                break;
            };
            // A range-like tail is ambiguous and must remain invalid rather
            // than being interpreted as source-location prose.
            if location.starts_with('E') || location.contains("-E") {
                valid = false;
                break;
            }
            rendered_parts.push(format!("（{location}） [{id}]"));
        }
        if valid {
            let replacement = rendered_parts.join(" ");
            let original = &answer[index..=closing];
            if replacement != original {
                output.push_str(&answer[copied_through..index]);
                output.push_str(&replacement);
                copied_through = closing + 1;
                normalized += 1;
            }
        }
        index = closing + 1;
    }
    output.push_str(&answer[copied_through..]);
    (output, normalized)
}

pub(super) fn extract_citation_ids(value: &str) -> Vec<String> {
    let mut cited = Vec::new();
    for token in citation_tokens(value) {
        if !cited.iter().any(|value| value == token.id) {
            cited.push(token.id.to_string());
        }
    }
    cited
}

fn claim_text_projection(value: &str) -> String {
    let masks = markdown_masks(value);
    let mut projected = String::with_capacity(value.len());
    for (index, character) in value.char_indices() {
        if masks.claim_hidden[index] {
            projected.push(if character == '\n' { '\n' } else { ' ' });
        } else {
            projected.push(character);
        }
    }
    projected
}

pub(super) fn claim_segments(answer: &str) -> Vec<String> {
    let projected = claim_text_projection(answer);
    let mut segments = Vec::new();
    let mut current = String::new();
    let characters = projected.char_indices().collect::<Vec<_>>();
    let mut index = 0;
    while index < characters.len() {
        let (byte_index, character) = characters[index];
        current.push(character);
        let period_boundary = character == '.'
            && characters
                .get(index + 1)
                .map_or(true, |(_, next)| next.is_whitespace());
        let line_boundary = character == '\n';
        let sentence_boundary =
            period_boundary || matches!(character, '。' | '！' | '？' | '!' | '?' | ';' | '；');
        if sentence_boundary {
            let boundary_end = byte_index + character.len_utf8();
            if let Some(suffix_end) = adjacent_citation_suffix_end(&projected, boundary_end) {
                current.push_str(&projected[boundary_end..suffix_end]);
                while index + 1 < characters.len() && characters[index + 1].0 < suffix_end {
                    index += 1;
                }
            }
        }
        if line_boundary || sentence_boundary {
            let segment = current.trim();
            if !segment.is_empty() {
                segments.push(segment.to_string());
            }
            current.clear();
        }
        index += 1;
    }
    let remainder = current.trim();
    if !remainder.is_empty() {
        segments.push(remainder.to_string());
    }
    segments
}

/// Returns the end of citation tokens immediately following sentence
/// punctuation on the same line. This keeps the common academic spelling
/// `claim. [E1] [E2]` in one structural claim without allowing a citation in a
/// later paragraph to support earlier prose.
fn adjacent_citation_suffix_end(value: &str, start: usize) -> Option<usize> {
    fn skip_horizontal_space(value: &str, cursor: &mut usize) {
        while let Some(character) = value[*cursor..].chars().next() {
            if matches!(character, ' ' | '\t' | '\r') {
                *cursor += character.len_utf8();
            } else {
                break;
            }
        }
    }

    let mut cursor = start;
    skip_horizontal_space(value, &mut cursor);
    let wrapper = value[cursor..].chars().next().and_then(|character| {
        let closing = match character {
            '(' => ')',
            '（' => '）',
            _ => return None,
        };
        cursor += character.len_utf8();
        Some(closing)
    });
    skip_horizontal_space(value, &mut cursor);

    let mut citation_count = 0;
    loop {
        let bytes = value.as_bytes();
        if bytes.get(cursor) != Some(&b'[') || bytes.get(cursor + 1) != Some(&b'E') {
            break;
        }
        let mut end = cursor + 2;
        while bytes.get(end).is_some_and(u8::is_ascii_digit) {
            end += 1;
        }
        if end == cursor + 2 || bytes.get(end) != Some(&b']') {
            break;
        }
        citation_count += 1;
        cursor = end + 1;
        skip_horizontal_space(value, &mut cursor);
    }
    if citation_count == 0 {
        return None;
    }
    if let Some(closing) = wrapper {
        if !value[cursor..].starts_with(closing) {
            return None;
        }
        cursor += closing.len_utf8();
    }
    Some(cursor)
}

pub(super) fn is_factual_claim(segment: &str) -> bool {
    let trimmed = segment
        .trim()
        .trim_start_matches(|character: char| {
            character.is_ascii_digit()
                || matches!(
                    character,
                    '#' | '-' | '*' | '+' | '.' | ')' | '（' | '）' | ' '
                )
        })
        .trim();
    if trimmed.is_empty()
        || trimmed.starts_with(NO_EVIDENCE_NOTICE)
        || [
            "库水位",
            "当前库水位",
            "年份范围",
            "本轮证据",
            "已召回以下",
            "当前处于离线证据模式",
            "当前处于证据浏览模式",
            "当前证据不足以核验该陈述",
            "当前证据与该陈述存在冲突",
        ]
        .iter()
        .any(|prefix| trimmed.starts_with(prefix))
        || [
            "库内直接证据",
            "相似模型",
            "可迁移算法",
            "核心书籍理论基础",
            "库内尚未覆盖",
            "结论",
            "模型与适用前提",
            "证据综合",
            "方法或比较",
            "边界、冲突与未覆盖项",
            "库水位与复现信息",
        ]
        .iter()
        .any(|heading| trimmed.trim_end_matches([':', '：']) == *heading)
        || (trimmed.ends_with(':') || trimmed.ends_with('：'))
            && extract_citation_ids(trimmed).is_empty()
    {
        return false;
    }
    let without_citations = remove_citation_tokens(trimmed);
    let information_length = without_citations
        .chars()
        .filter(|character| character.is_alphanumeric())
        .count();
    if is_markdown_table_row(trimmed) {
        information_length >= 2
    } else {
        information_length >= 6
    }
}

fn is_markdown_table_row(segment: &str) -> bool {
    segment.trim().matches('|').count() >= 2
}

fn is_markdown_table_separator(segment: &str) -> bool {
    let trimmed = segment.trim();
    is_markdown_table_row(trimmed)
        && trimmed.contains("---")
        && trimmed
            .chars()
            .all(|character| matches!(character, '|' | '-' | ':' | ' ' | '\t' | '\r' | '\n'))
}

fn remove_citation_tokens(value: &str) -> String {
    let mut result = String::with_capacity(value.len());
    let mut copied_through = 0;
    for token in citation_tokens(value) {
        result.push_str(&value[copied_through..token.start]);
        copied_through = token.end;
    }
    result.push_str(&value[copied_through..]);
    result
}

fn claim_bounds(value: &str, token_start: usize, token_end: usize) -> (usize, usize) {
    let masks = markdown_masks(value);
    let is_boundary = |index: usize, character: char| {
        if character == '\n' {
            return true;
        }
        if masks.claim_hidden[index] {
            return false;
        }
        matches!(character, '。' | '！' | '？' | '!' | '?' | ';' | '；')
            || character == '.'
                && value[index + character.len_utf8()..]
                    .chars()
                    .next()
                    .map_or(true, char::is_whitespace)
    };
    let mut start = 0;
    for (index, character) in value[..token_start].char_indices() {
        if is_boundary(index, character) {
            start = index + character.len_utf8();
        }
    }
    let mut end = value.len();
    for (relative_index, character) in value[token_end..].char_indices() {
        let index = token_end + relative_index;
        if is_boundary(index, character) {
            end = index + character.len_utf8();
            break;
        }
    }
    (start, end)
}

/// Removes an unknown citation token only when its own claim already carries a
/// known, non-Graphify citation. It never adds a citation or changes claim text.
pub fn repair_unknown_citations(
    answer: &str,
    evidence: &[EvidenceItem],
) -> (String, CitationRepair) {
    let (normalized_answer, normalized_citation_groups) =
        normalize_citation_groups(answer, evidence);
    let answer = normalized_answer.as_str();
    let known = evidence
        .iter()
        .map(|item| (item.id.as_str(), item.kind.as_str()))
        .collect::<HashMap<_, _>>();
    let mut output = String::with_capacity(answer.len());
    let mut removed = Vec::new();
    let mut copied_through = 0;
    for token in citation_tokens(answer) {
        output.push_str(&answer[copied_through..token.start]);
        if !known.contains_key(token.id) {
            let (claim_start, claim_end) = claim_bounds(answer, token.start, token.end);
            let has_valid_non_graph = extract_citation_ids(&answer[claim_start..claim_end])
                .iter()
                .any(|claim_id| {
                    known
                        .get(claim_id.as_str())
                        .is_some_and(|kind| *kind != "graph")
                });
            if has_valid_non_graph {
                if !removed.iter().any(|value| value == token.id) {
                    removed.push(token.id.to_string());
                }
                copied_through = token.end;
                continue;
            }
        }
        output.push_str(&answer[token.start..token.end]);
        copied_through = token.end;
    }
    output.push_str(&answer[copied_through..]);
    let repair = CitationRepair {
        applied: !removed.is_empty() || normalized_citation_groups > 0,
        removed_unknown_ids: removed,
        normalized_citation_groups,
    };
    (output, repair)
}

pub fn validate_citations(answer: &str, evidence: &[EvidenceItem]) -> CitationValidation {
    let known = evidence
        .iter()
        .map(|item| (item.id.as_str(), item.kind.as_str()))
        .collect::<HashMap<_, _>>();
    let cited = extract_citation_ids(answer);
    let unknown_ids = cited
        .iter()
        .filter(|id| !known.contains_key(id.as_str()))
        .cloned()
        .collect::<Vec<_>>();
    let valid = cited.len().saturating_sub(unknown_ids.len());
    let has_citations = !cited.is_empty();
    let zero_evidence = evidence.is_empty();
    let unverified =
        zero_evidence && answer.starts_with(NO_EVIDENCE_NOTICE) && unknown_ids.is_empty();
    let supplement_start = answer.find(MODEL_SUPPLEMENT_HEADING);
    let supplement_notice_present =
        supplement_start.is_some_and(|start| answer[start..].contains(MODEL_SUPPLEMENT_NOTICE));
    let factual_claims = |value: &str| {
        let segments = claim_segments(value);
        segments
            .iter()
            .enumerate()
            .filter(|(index, segment)| {
                !(is_markdown_table_separator(segment)
                    || is_markdown_table_row(segment)
                        && segments
                            .get(index + 1)
                            .is_some_and(|next| is_markdown_table_separator(next)))
                    && is_factual_claim(segment)
            })
            .map(|(_, segment)| segment.clone())
            .collect::<Vec<_>>()
    };
    let claims = if zero_evidence {
        Vec::new()
    } else if let Some(start) = supplement_start {
        let mut claims = factual_claims(&answer[..start])
            .into_iter()
            .map(|claim| (claim, false))
            .collect::<Vec<_>>();
        claims.extend(
            factual_claims(&answer[start + MODEL_SUPPLEMENT_HEADING.len()..])
                .into_iter()
                .map(|claim| (claim, true)),
        );
        claims
    } else {
        factual_claims(answer)
            .into_iter()
            .map(|claim| (claim, false))
            .collect()
    };
    let mut cited_claim_count = 0;
    let mut unsupported_claims = Vec::new();
    let mut graph_only_claims = Vec::new();
    let mut model_supplement_claims = Vec::new();
    for (claim, is_supplement) in &claims {
        let claim_ids = extract_citation_ids(claim);
        let known_kinds = claim_ids
            .iter()
            .filter_map(|id| known.get(id.as_str()).copied())
            .collect::<Vec<_>>();
        let claim_has_unknown = claim_ids.iter().any(|id| !known.contains_key(id.as_str()));
        let graph_only = !known_kinds.is_empty() && known_kinds.iter().all(|kind| *kind == "graph");
        if *is_supplement {
            if claim.trim().trim_start_matches('>').trim()
                == MODEL_SUPPLEMENT_NOTICE.trim_start_matches('>').trim()
            {
                continue;
            }
            if supplement_notice_present && claim_ids.is_empty() {
                model_supplement_claims.push(compact(claim, 180));
            } else {
                unsupported_claims.push(compact(claim, 180));
            }
            continue;
        }
        if graph_only {
            graph_only_claims.push(compact(claim, 180));
        }
        if !claim_has_unknown && !graph_only && !known_kinds.is_empty() {
            cited_claim_count += 1;
        } else {
            unsupported_claims.push(compact(claim, 180));
        }
    }
    let claim_count = claims
        .iter()
        .filter(|(_, is_supplement)| !is_supplement)
        .count();
    let citation_coverage = if claim_count == 0 {
        0.0
    } else {
        cited_claim_count as f64 / claim_count as f64
    };
    let syntax_valid = unknown_ids.is_empty();
    let coverage_valid =
        claim_count > 0 && cited_claim_count == claim_count && unsupported_claims.is_empty();
    let mixed =
        !zero_evidence && syntax_valid && coverage_valid && !model_supplement_claims.is_empty();
    let supported = !zero_evidence && syntax_valid && coverage_valid;
    CitationValidation {
        cited_ids: cited.clone(),
        unknown_ids: unknown_ids.clone(),
        citation_precision: if cited.is_empty() {
            0.0
        } else {
            valid as f64 / cited.len() as f64
        },
        has_citations,
        supported,
        grounding_status: if mixed {
            "mixed"
        } else if supported {
            "supported"
        } else if unverified {
            "unverified"
        } else {
            "invalid"
        }
        .to_string(),
        zero_evidence,
        claim_count,
        cited_claim_count,
        citation_coverage,
        unsupported_claims,
        graph_only_claims,
        syntax_valid,
        coverage_valid,
        entailment_checked: false,
        model_supplement_claim_count: model_supplement_claims.len(),
        model_supplement_claims,
        appendix_integrity: false,
        appendix_evidence_ids: Vec::new(),
    }
}

pub fn trusted_context(answer: &str, grounding_status: &str) -> String {
    if !matches!(
        grounding_status,
        "supported" | "mixed" | "partially_supported"
    ) {
        return String::new();
    }
    let boundary = [
        answer.find(MODEL_SUPPLEMENT_HEADING),
        answer.find(super::natural_answer::APPENDIX_HEADING),
    ]
    .into_iter()
    .flatten()
    .min();
    let verified = boundary.map(|start| &answer[..start]).unwrap_or(answer);
    remove_citation_tokens(verified).trim().to_string()
}

pub fn normalize_unverified_answer(answer: &str) -> String {
    let mut body = answer.trim().to_string();
    let mut search_from = 0;
    while let Some(relative_start) = body[search_from..].find("[E") {
        let start = search_from + relative_start;
        let suffix = &body[start + 2..];
        let digits = suffix
            .chars()
            .take_while(|character| character.is_ascii_digit())
            .count();
        if digits == 0 || suffix.chars().nth(digits) != Some(']') {
            search_from = start + 2;
            continue;
        }
        body.replace_range(start..start + digits + 3, "[无来源]");
        search_from = start + "[无来源]".len();
    }
    search_from = 0;
    while let Some(relative_start) = body[search_from..].find("[[") {
        let start = search_from + relative_start;
        let Some(relative_end) = body[start + 2..].find("]]") else {
            search_from = start + 2;
            continue;
        };
        let end = start + 2 + relative_end;
        let label = body[start + 2..end]
            .split_once('|')
            .map(|(_, label)| label)
            .unwrap_or(&body[start + 2..end])
            .to_string();
        let replacement = format!("{label}（无来源）");
        body.replace_range(start..end + 2, &replacement);
        search_from = start + replacement.len();
    }
    if body.starts_with(NO_EVIDENCE_NOTICE) {
        body
    } else if body.is_empty() {
        NO_EVIDENCE_NOTICE.to_string()
    } else {
        format!("{NO_EVIDENCE_NOTICE}\n\n{body}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn evidence(id: &str) -> EvidenceItem {
        EvidenceItem {
            id: id.to_string(),
            kind: "wiki".to_string(),
            tier: "direct".to_string(),
            title: "Evidence".to_string(),
            snippet: "Supported statement".to_string(),
            score: 1.0,
            rank: 1,
            page_id: "source.md".to_string(),
            page_type: "source".to_string(),
            source_path: "wiki/sources/source.md".to_string(),
            wikilink: "[[source]]".to_string(),
            book_id: String::new(),
            chapter_id: String::new(),
            physical_page_start: None,
            physical_page_end: None,
            markdown_path: String::new(),
            pdf_path: String::new(),
            node_id: String::new(),
            source_location: String::new(),
            relation: String::new(),
            retrieval_reason: String::new(),
            locator: None,
        }
    }

    #[test]
    fn citation_extraction_ignores_markdown_literal_and_link_regions() {
        let answer = concat!(
            "Visible [E1]. `inline [E2]`. $math [E3]$.\n",
            "```rust\nlet citation = \"[E4]\";\n```\n",
            "$$block [E5]$$. \\[E6]. ",
            "[E7](https://example.test/[E8]) ",
            "[label [E9]](https://example.test/[E10]) Visible [E11].",
        );

        assert_eq!(extract_citation_ids(answer), vec!["E1", "E11"]);
    }

    #[test]
    fn validation_excludes_code_and_math_from_claims_and_citations() {
        let answer = concat!(
            "Supported factual statement [E1].\n",
            "```rust\nlet example = \"[E99]\";\n```\n",
            "The notation is $x_[E98]$ [E1].\n",
            "Another supported statement [E1].",
        );
        let validation = validate_citations(answer, &[evidence("E1")]);

        assert!(validation.supported, "{validation:?}");
        assert_eq!(validation.cited_ids, vec!["E1"]);
        assert!(validation.unknown_ids.is_empty());
        assert_eq!(validation.claim_count, 3);
        assert_eq!(validation.cited_claim_count, 3);
    }

    #[test]
    fn grouped_citations_and_locations_are_canonicalized_before_claim_splitting() {
        let answer = concat!(
            "两篇论文共同研究波干扰下的并发充电[E1；E5]。\n",
            "非线性叠加模型见结论部分[E5，7 CONCLUSION · 原文第 478–481 行]。",
        );
        let (normalized, repair) =
            repair_unknown_citations(answer, &[evidence("E1"), evidence("E5")]);

        assert_eq!(repair.normalized_citation_groups, 2);
        assert!(repair.applied);
        assert_eq!(
            normalized,
            concat!(
                "两篇论文共同研究波干扰下的并发充电[E1] [E5]。\n",
                "非线性叠加模型见结论部分（7 CONCLUSION · 原文第 478–481 行） [E5]。",
            )
        );
        let validation = validate_citations(&normalized, &[evidence("E1"), evidence("E5")]);
        assert!(validation.supported, "{validation:?}");
        assert_eq!(validation.claim_count, 2);
    }

    #[test]
    fn canonicalization_is_fail_closed_for_unknown_ranges_and_markdown_literals() {
        let answer = concat!(
            "Unknown [E1；E99]. Range [E1-E5]. ",
            "`literal [E1；E5]` and [label [E1；E5]](https://example.test).",
        );
        let (normalized, repair) =
            repair_unknown_citations(answer, &[evidence("E1"), evidence("E5")]);

        assert_eq!(normalized, answer);
        assert_eq!(repair.normalized_citation_groups, 0);
        assert!(!repair.applied);
    }

    #[test]
    fn markdown_links_do_not_support_claims() {
        let linked_label = validate_citations(
            "This factual statement uses [E1](https://example.test).",
            &[evidence("E1")],
        );
        assert!(!linked_label.supported);
        assert!(linked_label.cited_ids.is_empty());
        assert_eq!(linked_label.claim_count, 1);
        assert_eq!(linked_label.cited_claim_count, 0);

        let linked_target = validate_citations(
            "This factual statement links [documentation](https://example.test/[E1]).",
            &[evidence("E1")],
        );
        assert!(!linked_target.supported);
        assert!(linked_target.cited_ids.is_empty());
        assert_eq!(linked_target.claim_count, 1);
        assert_eq!(linked_target.cited_claim_count, 0);

        let mixed = validate_citations(
            concat!(
                "Supported statement [E1] links [E99](https://example.test). ",
                "Another supported statement [E1] links ",
                "[documentation](https://example.test/[E98]).",
            ),
            &[evidence("E1")],
        );
        assert!(mixed.supported, "{mixed:?}");
        assert_eq!(mixed.cited_ids, vec!["E1"]);
        assert!(mixed.unknown_ids.is_empty());
    }

    #[test]
    fn repair_requires_visible_evidence_in_the_same_claim() {
        let answer = concat!(
            "Literal `known [E1]` does not authorize unknown [E9]. ",
            "Math $known [E1]$ does not authorize unknown [E6]. ",
            "Escaped \\[E1] does not authorize unknown [E3]. ",
            "Linked label [E1](https://example.test) does not authorize [E5]. ",
            "Linked target [docs](https://example.test/[E1]) does not authorize [E4]. ",
            "Visible evidence [E1] authorizes removal [E8]. ",
            "Existing link [E7](https://example.test/[E1]) stays unchanged.",
        );
        let (repaired, repair) = repair_unknown_citations(answer, &[evidence("E1")]);

        assert!(repair.applied);
        assert_eq!(repair.removed_unknown_ids, vec!["E8"]);
        assert!(repaired.contains("[E9]"));
        assert!(repaired.contains("[E6]"));
        assert!(repaired.contains("[E3]"));
        assert!(repaired.contains("[E5]"));
        assert!(repaired.contains("[E4]"));
        assert!(!repaired.contains("removal [E8]"));
        assert!(repaired.contains("[E7](https://example.test/[E1])"));
    }

    #[test]
    fn mixed_answer_keeps_verified_claims_and_isolates_model_supplement() {
        let answer = format!(
            "## 结论\n当前库支持这个结论 [E1]。\n\n{MODEL_SUPPLEMENT_HEADING}\n{MODEL_SUPPLEMENT_NOTICE}\n模型推测该方向还可能使用自适应算法。"
        );
        let validation = validate_citations(&answer, &[evidence("E1")]);

        assert!(validation.supported, "{validation:?}");
        assert_eq!(validation.grounding_status, "mixed");
        assert_eq!(validation.claim_count, 1);
        assert_eq!(validation.cited_claim_count, 1);
        assert_eq!(validation.model_supplement_claim_count, 1);
        assert!(validation.unsupported_claims.is_empty());
        let trusted = trusted_context(&answer, &validation.grounding_status);
        assert!(trusted.contains("当前库支持这个结论"));
        assert!(!trusted.contains("[E1]"));
        assert!(!trusted.contains("自适应算法"));
        assert!(!trusted.contains(MODEL_SUPPLEMENT_HEADING));
    }

    #[test]
    fn repeated_claim_text_before_and_after_boundary_is_classified_by_section() {
        let answer = format!(
            "重复的结论内容 [E1]。\n\n{MODEL_SUPPLEMENT_HEADING}\n{MODEL_SUPPLEMENT_NOTICE}\n重复的结论内容。"
        );
        let validation = validate_citations(&answer, &[evidence("E1")]);

        assert_eq!(validation.grounding_status, "mixed", "{validation:?}");
        assert_eq!(validation.claim_count, 1);
        assert_eq!(validation.model_supplement_claim_count, 1);
    }

    #[test]
    fn supplement_requires_exact_notice_and_must_not_use_citations() {
        let missing_notice =
            format!("可验证结论 [E1]。\n\n{MODEL_SUPPLEMENT_HEADING}\n未经验证的模型结论。");
        assert_eq!(
            validate_citations(&missing_notice, &[evidence("E1")]).grounding_status,
            "invalid"
        );
        let cited_supplement = format!(
            "可验证结论 [E1]。\n\n{MODEL_SUPPLEMENT_HEADING}\n{MODEL_SUPPLEMENT_NOTICE}\n模型结论 [E1]。"
        );
        assert_eq!(
            validate_citations(&cited_supplement, &[evidence("E1")]).grounding_status,
            "invalid"
        );
    }

    #[test]
    fn citations_immediately_after_sentence_punctuation_support_the_previous_claim() {
        for answer in [
            "论文直接研究波干扰。[E1]",
            "论文直接研究波干扰。 [E1] [E2]",
            "论文直接研究波干扰。（[E1]）",
            "论文直接研究波干扰。([E1])",
        ] {
            let validation = validate_citations(answer, &[evidence("E1"), evidence("E2")]);
            assert!(validation.supported, "{answer}: {validation:?}");
            assert_eq!(validation.claim_count, 1);
            assert_eq!(validation.cited_claim_count, 1);
        }
    }

    #[test]
    fn citation_in_a_later_paragraph_does_not_support_the_previous_claim() {
        let validation =
            validate_citations("论文直接研究波干扰。\n\n参考来源：[E1]", &[evidence("E1")]);

        assert!(!validation.supported);
        assert_eq!(validation.cited_claim_count, 0);
        assert!(!validation.unsupported_claims.is_empty());
    }
}
