use super::corpus::{ContentBlock, ContentGranularity, ContentRole, SourceLocator};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

const SEMANTIC_TARGET_CHARS: usize = 2_400;
const SEMANTIC_OVERLAP_CHARS: usize = 320;

#[derive(Debug, Clone)]
struct Heading {
    level: usize,
    title: String,
    line_index: usize,
}

#[derive(Debug, Clone)]
struct Section {
    heading: String,
    heading_path: Vec<String>,
    line_start: usize,
    line_end: usize,
    body: String,
}

#[derive(Debug, Clone)]
struct MarkdownUnit {
    line_start: usize,
    line_end: usize,
    text: String,
}

pub(crate) fn sha256_hex(value: impl AsRef<[u8]>) -> String {
    let mut digest = Sha256::new();
    digest.update(value.as_ref());
    format!("{:x}", digest.finalize())
}

fn normalized_heading(value: &str) -> String {
    value
        .chars()
        .filter(|character| character.is_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

fn block_id(
    document_id: &str,
    granularity: ContentGranularity,
    heading_path: &[String],
    ordinal: usize,
    content: &str,
) -> String {
    let identity = format!(
        "{}\n{}\n{}\n{}\n{}",
        document_id,
        granularity.as_str(),
        heading_path
            .iter()
            .map(|value| normalized_heading(value))
            .collect::<Vec<_>>()
            .join("/"),
        ordinal,
        sha256_hex(content)
    );
    format!("blk-{}", &sha256_hex(identity)[..24])
}

fn atx_heading(line: &str, line_index: usize) -> Option<Heading> {
    let trimmed = line.trim_start();
    let level = trimmed.chars().take_while(|value| *value == '#').count();
    if !(1..=6).contains(&level) {
        return None;
    }
    let title = trimmed
        .get(level..)?
        .strip_prefix(' ')?
        .trim()
        .trim_end_matches('#')
        .trim();
    (!title.is_empty()).then(|| Heading {
        level,
        title: title.to_string(),
        line_index,
    })
}

fn setext_level(line: &str) -> Option<usize> {
    let trimmed = line.trim();
    if trimmed.len() < 3 {
        return None;
    }
    if trimmed.chars().all(|value| value == '=') {
        Some(1)
    } else if trimmed.chars().all(|value| value == '-') {
        Some(2)
    } else {
        None
    }
}

fn frontmatter_end(lines: &[&str]) -> usize {
    if lines
        .first()
        .is_some_and(|line| line.trim_start_matches('\u{feff}').trim() == "---")
    {
        for (index, line) in lines.iter().enumerate().skip(1) {
            if line.trim() == "---" {
                return index + 1;
            }
        }
    }
    0
}

fn headings(lines: &[&str], start: usize) -> Vec<Heading> {
    let mut result = Vec::new();
    let mut in_fence = false;
    let mut index = start;
    while index < lines.len() {
        let trimmed = lines[index].trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_fence = !in_fence;
            index += 1;
            continue;
        }
        if !in_fence {
            if let Some(heading) = atx_heading(lines[index], index) {
                result.push(heading);
            } else if index + 1 < lines.len()
                && !lines[index].trim().is_empty()
                && setext_level(lines[index + 1]).is_some()
            {
                result.push(Heading {
                    level: setext_level(lines[index + 1]).unwrap_or(2),
                    title: lines[index].trim().to_string(),
                    line_index: index,
                });
                index += 1;
            }
        }
        index += 1;
    }
    result
}

fn sections(content: &str, fallback_title: &str) -> Vec<Section> {
    let normalized = content.strip_prefix('\u{feff}').unwrap_or(content);
    let lines = normalized.lines().collect::<Vec<_>>();
    let start = frontmatter_end(&lines);
    let found = headings(&lines, start);
    if found.is_empty() {
        let body = lines[start..].join("\n").trim().to_string();
        return (!body.is_empty())
            .then(|| Section {
                heading: fallback_title.to_string(),
                heading_path: vec![fallback_title.to_string()],
                line_start: start + 1,
                line_end: lines.len().max(start + 1),
                body,
            })
            .into_iter()
            .collect();
    }

    let mut stack: Vec<(usize, String)> = Vec::new();
    let mut result = Vec::new();
    if found[0].line_index > start {
        let body = lines[start..found[0].line_index]
            .join("\n")
            .trim()
            .to_string();
        if !body.is_empty() {
            result.push(Section {
                heading: fallback_title.to_string(),
                heading_path: vec![fallback_title.to_string()],
                line_start: start + 1,
                line_end: found[0].line_index,
                body,
            });
        }
    }
    for (position, heading) in found.iter().enumerate() {
        while stack
            .last()
            .is_some_and(|(level, _)| *level >= heading.level)
        {
            stack.pop();
        }
        stack.push((heading.level, heading.title.clone()));
        let heading_path = stack.iter().map(|(_, value)| value.clone()).collect();
        let end = found
            .get(position + 1)
            .map(|item| item.line_index)
            .unwrap_or(lines.len());
        let body = lines[heading.line_index..end].join("\n").trim().to_string();
        if !body.is_empty() {
            result.push(Section {
                heading: heading.title.clone(),
                heading_path,
                line_start: heading.line_index + 1,
                line_end: end.max(heading.line_index + 1),
                body,
            });
        }
    }
    result
}

fn role_for_heading(heading: &str) -> ContentRole {
    let value = heading.to_lowercase();
    let contains = |needles: &[&str]| needles.iter().any(|needle| value.contains(needle));
    if contains(&["abstract", "摘要", "tldr", "tl;dr"]) {
        ContentRole::Abstract
    } else if contains(&["motivation", "研究动机", "动机"]) {
        ContentRole::ResearchMotivation
    } else if contains(&[
        "background",
        "introduction",
        "研究背景",
        "背景",
        "引言",
        "引论",
    ]) {
        ContentRole::ResearchBackground
    } else if contains(&["objective", "research goal", "研究目的", "研究目标"]) {
        ContentRole::ResearchObjective
    } else if contains(&["related work", "literature review", "相关工作", "文献综述"]) {
        ContentRole::RelatedWork
    } else if contains(&[
        "problem formulation",
        "problem definition",
        "问题定义",
        "问题形式化",
    ]) {
        ContentRole::ProblemDefinition
    } else if contains(&["model", "system setting", "系统模型", "模型"]) {
        ContentRole::Model
    } else if contains(&["algorithm", "算法", "pseudo-code", "pseudocode"]) {
        ContentRole::Algorithm
    } else if contains(&["method", "approach", "方法", "方案"]) {
        ContentRole::Method
    } else if contains(&["proof", "证明"]) {
        ContentRole::Proof
    } else if contains(&["theory", "theorem", "理论", "定理"]) {
        ContentRole::Theory
    } else if contains(&["result", "finding", "结果", "发现"]) {
        ContentRole::Result
    } else if contains(&[
        "experiment",
        "evaluation",
        "simulation",
        "实验",
        "仿真",
        "评估",
    ]) {
        ContentRole::Experiment
    } else if contains(&["limitation", "threat", "局限", "限制", "失效"]) {
        ContentRole::Limitation
    } else if contains(&["conclusion", "结论", "总结"]) {
        ContentRole::Conclusion
    } else if contains(&["reference", "bibliography", "参考文献"]) {
        ContentRole::Reference
    } else {
        ContentRole::GeneralContent
    }
}

fn markdown_units(section: &Section) -> Vec<MarkdownUnit> {
    let lines = section.body.lines().collect::<Vec<_>>();
    let mut result = Vec::new();
    let mut current = Vec::new();
    let mut current_start = section.line_start;
    let mut in_fence = false;
    for (offset, line) in lines.iter().enumerate() {
        let line_number = section.line_start + offset;
        let trimmed = line.trim_start();
        if current.is_empty() {
            current_start = line_number;
        }
        current.push(*line);
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_fence = !in_fence;
        }
        if !in_fence && line.trim().is_empty() {
            let text = current.join("\n").trim().to_string();
            if !text.is_empty() {
                result.push(MarkdownUnit {
                    line_start: current_start,
                    line_end: line_number,
                    text,
                });
            }
            current.clear();
        }
    }
    if !current.is_empty() {
        let text = current.join("\n").trim().to_string();
        if !text.is_empty() {
            result.push(MarkdownUnit {
                line_start: current_start,
                line_end: section.line_end,
                text,
            });
        }
    }
    result
}

fn char_slice(value: &str, start: usize, end: usize) -> String {
    value
        .chars()
        .skip(start)
        .take(end.saturating_sub(start))
        .collect()
}

fn push_semantic_group(
    result: &mut Vec<(usize, usize, String)>,
    current: &[MarkdownUnit],
    section: &Section,
) {
    if current.is_empty() {
        return;
    }
    result.push((
        current
            .first()
            .map(|item| item.line_start)
            .unwrap_or(section.line_start),
        current
            .last()
            .map(|item| item.line_end)
            .unwrap_or(section.line_end),
        current
            .iter()
            .map(|item| item.text.as_str())
            .collect::<Vec<_>>()
            .join("\n\n"),
    ));
}

fn semantic_chunks(section: &Section) -> Vec<(usize, usize, String)> {
    let units = markdown_units(section);
    if units.is_empty() {
        return Vec::new();
    }
    let mut result = Vec::new();
    let mut current: Vec<MarkdownUnit> = Vec::new();
    let mut current_chars = 0usize;
    for unit in units {
        let unit_chars = unit.text.chars().count() + 2;
        let is_fenced =
            unit.text.trim_start().starts_with("```") || unit.text.trim_start().starts_with("~~~");
        if unit_chars > SEMANTIC_TARGET_CHARS && !is_fenced {
            push_semantic_group(&mut result, &current, section);
            current.clear();
            current_chars = 0;
            let total_chars = unit.text.chars().count();
            let mut start = 0usize;
            while start < total_chars {
                let end = (start + SEMANTIC_TARGET_CHARS).min(total_chars);
                result.push((
                    unit.line_start,
                    unit.line_end,
                    char_slice(&unit.text, start, end),
                ));
                if end == total_chars {
                    break;
                }
                start = end.saturating_sub(SEMANTIC_OVERLAP_CHARS);
            }
            continue;
        }
        if !current.is_empty() && current_chars + unit_chars > SEMANTIC_TARGET_CHARS {
            push_semantic_group(&mut result, &current, section);
            let mut overlap = Vec::new();
            let mut overlap_chars = 0usize;
            for previous in current.iter().rev() {
                if overlap_chars >= SEMANTIC_OVERLAP_CHARS {
                    break;
                }
                overlap.push(previous.clone());
                overlap_chars += previous.text.chars().count() + 2;
            }
            overlap.reverse();
            current = overlap;
            current_chars = overlap_chars;
        }
        current_chars += unit_chars;
        current.push(unit);
    }
    push_semantic_group(&mut result, &current, section);
    result
}

fn embedding_text(
    canonical_title: &str,
    aliases: &[String],
    kind: &str,
    heading_path: &[String],
    role: ContentRole,
    content: &str,
) -> String {
    format!(
        "文档：{canonical_title}\n别名：{}\n类型：{kind}\n位置：{}\n角色：{}\n正文：{content}",
        aliases.join("；"),
        heading_path.join(" / "),
        role.as_str()
    )
}

#[allow(clippy::too_many_arguments)]
fn source_locator(
    document_id: &str,
    block_id: &str,
    heading_path: &[String],
    markdown_path: &str,
    line_start: usize,
    line_end: usize,
    content_hash: &str,
    snapshot_id: &str,
) -> SourceLocator {
    SourceLocator {
        document_id: document_id.to_string(),
        block_id: block_id.to_string(),
        heading_path: heading_path.to_vec(),
        markdown_path: markdown_path.to_string(),
        line_start: Some(line_start as i64),
        line_end: Some(line_end as i64),
        content_hash: content_hash.to_string(),
        snapshot_id: snapshot_id.to_string(),
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn parse_markdown(
    document_id: &str,
    kind: &str,
    canonical_title: &str,
    aliases: &[String],
    markdown_path: &str,
    content: &str,
    snapshot_id: &str,
    include_document: bool,
) -> Vec<ContentBlock> {
    let parsed_sections = sections(content, canonical_title);
    let mut blocks = Vec::new();
    let mut section_ids: HashMap<Vec<String>, String> = HashMap::new();
    if include_document {
        let headings = parsed_sections
            .iter()
            .map(|section| section.heading.clone())
            .collect::<Vec<_>>()
            .join("；");
        let excerpt = content
            .lines()
            .filter(|line| !line.trim().is_empty() && !line.trim_start().starts_with("---"))
            .take(80)
            .collect::<Vec<_>>()
            .join("\n");
        let document_content = format!("目录：{headings}\n\n{excerpt}");
        let content_hash = sha256_hex(&document_content);
        let heading_path = vec![canonical_title.to_string()];
        let id = block_id(
            document_id,
            ContentGranularity::Document,
            &heading_path,
            0,
            &document_content,
        );
        blocks.push(ContentBlock {
            id: id.clone(),
            document_id: document_id.to_string(),
            parent_block_id: None,
            granularity: ContentGranularity::Document,
            heading: canonical_title.to_string(),
            heading_path: heading_path.clone(),
            role: ContentRole::GeneralContent,
            ordinal: 0,
            line_start: 1,
            line_end: content.lines().count().max(1),
            markdown_path: markdown_path.to_string(),
            content: document_content.clone(),
            content_hash: content_hash.clone(),
            embedding_text: embedding_text(
                canonical_title,
                aliases,
                kind,
                &heading_path,
                ContentRole::GeneralContent,
                &document_content,
            ),
            snapshot_id: snapshot_id.to_string(),
            active: true,
            locator: source_locator(
                document_id,
                &id,
                &heading_path,
                markdown_path,
                1,
                content.lines().count().max(1),
                &content_hash,
                snapshot_id,
            ),
        });
    }

    for (ordinal, section) in parsed_sections.iter().enumerate() {
        let role = role_for_heading(&section.heading);
        let content_hash = sha256_hex(&section.body);
        let id = block_id(
            document_id,
            ContentGranularity::Section,
            &section.heading_path,
            ordinal,
            &section.body,
        );
        let parent_block_id = section
            .heading_path
            .get(..section.heading_path.len().saturating_sub(1))
            .and_then(|path| section_ids.get(path))
            .cloned()
            .or_else(|| blocks.first().map(|block| block.id.clone()));
        section_ids.insert(section.heading_path.clone(), id.clone());
        blocks.push(ContentBlock {
            id: id.clone(),
            document_id: document_id.to_string(),
            parent_block_id,
            granularity: ContentGranularity::Section,
            heading: section.heading.clone(),
            heading_path: section.heading_path.clone(),
            role,
            ordinal,
            line_start: section.line_start,
            line_end: section.line_end,
            markdown_path: markdown_path.to_string(),
            content: section.body.clone(),
            content_hash: content_hash.clone(),
            embedding_text: embedding_text(
                canonical_title,
                aliases,
                kind,
                &section.heading_path,
                role,
                &section.body,
            ),
            snapshot_id: snapshot_id.to_string(),
            active: true,
            locator: source_locator(
                document_id,
                &id,
                &section.heading_path,
                markdown_path,
                section.line_start,
                section.line_end,
                &content_hash,
                snapshot_id,
            ),
        });
        for (semantic_ordinal, (line_start, line_end, chunk)) in
            semantic_chunks(section).into_iter().enumerate()
        {
            let content_hash = sha256_hex(&chunk);
            let semantic_id = block_id(
                document_id,
                ContentGranularity::Semantic,
                &section.heading_path,
                semantic_ordinal,
                &chunk,
            );
            blocks.push(ContentBlock {
                id: semantic_id.clone(),
                document_id: document_id.to_string(),
                parent_block_id: Some(id.clone()),
                granularity: ContentGranularity::Semantic,
                heading: section.heading.clone(),
                heading_path: section.heading_path.clone(),
                role,
                ordinal: semantic_ordinal,
                line_start,
                line_end,
                markdown_path: markdown_path.to_string(),
                content: chunk.clone(),
                content_hash: content_hash.clone(),
                embedding_text: embedding_text(
                    canonical_title,
                    aliases,
                    kind,
                    &section.heading_path,
                    role,
                    &chunk,
                ),
                snapshot_id: snapshot_id.to_string(),
                active: true,
                locator: source_locator(
                    document_id,
                    &semantic_id,
                    &section.heading_path,
                    markdown_path,
                    line_start,
                    line_end,
                    &content_hash,
                    snapshot_id,
                ),
            });
        }
    }
    blocks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_heading_tree_roles_and_semantic_blocks() {
        let content = "---\ntitle: Demo\n---\n# Paper\n## 摘要\n简短摘要。\n\n## Model\n"
            .to_string()
            + &"模型正文。".repeat(900)
            + "\n\n## 实验结果\n结果正文。";
        let blocks = parse_markdown(
            "paper:demo",
            "paper",
            "Demo",
            &["示例".to_string()],
            "raw/demo.md",
            &content,
            "snapshot",
            true,
        );
        assert!(blocks
            .iter()
            .any(|block| block.role == ContentRole::Abstract));
        assert!(blocks.iter().any(|block| block.role == ContentRole::Model));
        assert!(blocks.iter().any(|block| block.role == ContentRole::Result));
        assert!(
            blocks
                .iter()
                .filter(|block| block.granularity == ContentGranularity::Semantic)
                .count()
                >= 3
        );
        assert!(blocks
            .iter()
            .all(|block| block.embedding_text.contains("文档：Demo")));
    }

    #[test]
    fn keeps_fenced_code_and_setext_headings_intact() {
        let content =
            "Title\n=====\n\n```rust\n# not a heading\n\nfn main() {}\n```\n\nMethod\n------\n正文";
        let blocks = parse_markdown(
            "wiki:demo",
            "wiki",
            "Demo",
            &[],
            "wiki/demo.md",
            content,
            "snapshot",
            true,
        );
        assert!(blocks.iter().any(|block| block.heading == "Title"));
        assert!(blocks.iter().any(|block| block.heading == "Method"));
        assert!(!blocks.iter().any(|block| block.heading == "not a heading"));
        assert!(blocks.iter().any(|block| block.content.contains("fn main")));
    }

    #[test]
    fn block_ids_do_not_depend_on_line_numbers() {
        let one = parse_markdown(
            "wiki:demo",
            "wiki",
            "Demo",
            &[],
            "wiki/demo.md",
            "# Demo\n\n## Model\nBody",
            "snapshot",
            true,
        );
        let two = parse_markdown(
            "wiki:demo",
            "wiki",
            "Demo",
            &[],
            "wiki/demo.md",
            "\n\n# Demo\n\n## Model\nBody",
            "snapshot",
            true,
        );
        let first = one
            .iter()
            .find(|block| {
                block.heading == "Model" && block.granularity == ContentGranularity::Section
            })
            .unwrap();
        let second = two
            .iter()
            .find(|block| {
                block.heading == "Model" && block.granularity == ContentGranularity::Section
            })
            .unwrap();
        assert_eq!(first.id, second.id);
        assert_ne!(first.line_start, second.line_start);
    }
}
