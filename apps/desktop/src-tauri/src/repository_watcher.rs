use notify::{
    event::{ModifyKind, RenameMode},
    Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher,
};
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeKind {
    Create,
    Modify,
    Remove,
    Rename,
    Other,
}

#[derive(Debug, Clone)]
pub struct IndexChange {
    pub path: PathBuf,
    pub previous_path: Option<PathBuf>,
    pub kind: ChangeKind,
    pub full_rebuild: bool,
    pub graph_refresh: bool,
}

pub fn classify_event(event: &Event, root: &Path) -> Vec<IndexChange> {
    if let EventKind::Modify(ModifyKind::Name(mode)) = event.kind {
        return classify_rename(event, root, mode);
    }

    let kind = match event.kind {
        EventKind::Create(_) => ChangeKind::Create,
        EventKind::Modify(_) => ChangeKind::Modify,
        EventKind::Remove(_) => ChangeKind::Remove,
        _ => ChangeKind::Other,
    };
    event
        .paths
        .iter()
        .filter_map(|path| {
            let classification = classify_path_for_event(path, root)?;
            Some(IndexChange {
                path: path.clone(),
                previous_path: None,
                kind: kind.clone(),
                full_rebuild: classification.full_rebuild,
                graph_refresh: classification.graph_refresh,
            })
        })
        .collect()
}

fn classify_rename(event: &Event, root: &Path, mode: RenameMode) -> Vec<IndexChange> {
    match mode {
        RenameMode::Both if event.paths.len() >= 2 => {
            let previous = event.paths[0].clone();
            let path = event.paths[1].clone();
            let previous_classification = classify_path_for_event(&previous, root);
            let current_classification = classify_path_for_event(&path, root);
            if previous_classification.is_none() && current_classification.is_none() {
                return vec![];
            }
            let full_rebuild = previous_classification
                .as_ref()
                .is_some_and(|classification| classification.full_rebuild)
                || current_classification
                    .as_ref()
                    .is_some_and(|classification| classification.full_rebuild);
            let graph_refresh = previous_classification
                .as_ref()
                .is_some_and(|classification| classification.graph_refresh)
                || current_classification
                    .as_ref()
                    .is_some_and(|classification| classification.graph_refresh);
            vec![IndexChange {
                path,
                previous_path: Some(previous),
                kind: ChangeKind::Rename,
                full_rebuild,
                graph_refresh,
            }]
        }
        RenameMode::From => event
            .paths
            .first()
            .and_then(|path| {
                classify_path_for_event(path, root).map(|classification| (path, classification))
            })
            .map(|(path, classification)| IndexChange {
                path: path.clone(),
                previous_path: None,
                kind: ChangeKind::Remove,
                full_rebuild: classification.full_rebuild,
                graph_refresh: classification.graph_refresh,
            })
            .into_iter()
            .collect(),
        RenameMode::To => event
            .paths
            .first()
            .and_then(|path| {
                classify_path_for_event(path, root).map(|classification| (path, classification))
            })
            .map(|(path, classification)| IndexChange {
                path: path.clone(),
                previous_path: None,
                kind: ChangeKind::Create,
                full_rebuild: classification.full_rebuild,
                graph_refresh: classification.graph_refresh,
            })
            .into_iter()
            .collect(),
        RenameMode::Any | RenameMode::Other if event.paths.len() >= 2 => {
            classify_rename(event, root, RenameMode::Both)
        }
        RenameMode::Any | RenameMode::Other | RenameMode::Both => event
            .paths
            .iter()
            .filter_map(|path| {
                let classification = classify_path_for_event(path, root)?;
                Some(IndexChange {
                    path: path.clone(),
                    previous_path: None,
                    kind: ChangeKind::Rename,
                    full_rebuild: classification.full_rebuild,
                    graph_refresh: classification.graph_refresh,
                })
            })
            .collect(),
    }
}

fn classify_path_for_event(path: &Path, root: &Path) -> Option<PathClassification> {
    let relative = path.strip_prefix(root).ok()?;
    if excluded(relative) {
        return None;
    }
    let classification = classify_path(relative);
    classification.interesting.then_some(classification)
}

struct PathClassification {
    interesting: bool,
    full_rebuild: bool,
    graph_refresh: bool,
}

fn classify_path(relative: &Path) -> PathClassification {
    let value = relative.to_string_lossy().replace('\\', "/");
    let wiki = value.starts_with("wiki/") && value.ends_with(".md");
    let schema =
        value.starts_with("schema/") && (value.ends_with(".md") || value.ends_with(".yaml"));
    let core = value.starts_with("raw/canonical/core-books")
        || value.contains("raw/canonical/algorithmic-game-theory/")
        || value.contains("raw/canonical/approximation-algorithms/");
    let graph = value == "graphify-out/graph.json";
    PathClassification {
        interesting: wiki || schema || core || graph,
        full_rebuild: schema || core,
        graph_refresh: graph,
    }
}

fn excluded(rel: &Path) -> bool {
    rel.components().any(|c| {
        matches!(
            c.as_os_str().to_str(),
            Some("node_modules" | "target" | "dist" | "logs" | "compile-backups")
        )
    }) || rel
        .file_name()
        .and_then(|n| n.to_str())
        .map(|n| n.starts_with('.') || n.ends_with('~'))
        .unwrap_or(false)
}

pub struct RepositoryWatcher {
    _watcher: RecommendedWatcher,
    rx: Receiver<notify::Result<Event>>,
    root: PathBuf,
    last: Option<Instant>,
}
impl RepositoryWatcher {
    pub fn start(root: PathBuf) -> notify::Result<Self> {
        let (tx, rx) = mpsc::channel();
        let mut watcher = notify::recommended_watcher(move |res| {
            let _ = tx.send(res);
        })?;
        watcher.watch(&root, RecursiveMode::Recursive)?;
        Ok(Self {
            _watcher: watcher,
            rx,
            root,
            last: None,
        })
    }
    pub fn poll(&mut self) -> Vec<IndexChange> {
        if self
            .last
            .map(|t| t.elapsed() < Duration::from_millis(700))
            .unwrap_or(false)
        {
            return vec![];
        }
        let mut out = Vec::new();
        while let Ok(Ok(ev)) = self.rx.try_recv() {
            out.extend(classify_event(&ev, &self.root));
        }
        if !out.is_empty() {
            self.last = Some(Instant::now());
        }
        out
    }

    pub fn root(&self) -> &Path {
        &self.root
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use notify::event::{CreateKind, ModifyKind, RenameMode};
    #[test]
    fn excludes_dirs() {
        let e = Event::new(EventKind::Create(CreateKind::File))
            .add_path(PathBuf::from("/r/target/x.md"));
        assert!(classify_event(&e, Path::new("/r")).is_empty());
    }
    #[test]
    fn classifies_wiki_and_schema() {
        let e = Event::new(EventKind::Modify(ModifyKind::Any))
            .add_path(PathBuf::from("/r/wiki/a.md"))
            .add_path(PathBuf::from("/r/schema/x.md"));
        let c = classify_event(&e, Path::new("/r"));
        assert_eq!(c.len(), 2);
        assert!(c.iter().any(|x| x.full_rebuild));
    }

    #[test]
    fn pairs_rename_paths_and_classifies_yaml_schema() {
        let rename = Event::new(EventKind::Modify(ModifyKind::Name(RenameMode::Both)))
            .add_path(PathBuf::from("/r/wiki/a.md"))
            .add_path(PathBuf::from("/r/wiki/b.md"));
        let changes = classify_event(&rename, Path::new("/r"));
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].kind, ChangeKind::Rename);
        assert_eq!(
            changes[0].previous_path,
            Some(PathBuf::from("/r/wiki/a.md"))
        );
        assert_eq!(changes[0].path, PathBuf::from("/r/wiki/b.md"));

        let schema = Event::new(EventKind::Modify(ModifyKind::Any))
            .add_path(PathBuf::from("/r/schema/vocab.yaml"));
        let changes = classify_event(&schema, Path::new("/r"));
        assert_eq!(changes.len(), 1);
        assert!(changes[0].full_rebuild);
    }

    #[test]
    fn rename_from_and_to_keep_single_path_semantics() {
        let from = Event::new(EventKind::Modify(ModifyKind::Name(RenameMode::From)))
            .add_path(PathBuf::from("/r/wiki/a.md"));
        let from_changes = classify_event(&from, Path::new("/r"));
        assert_eq!(from_changes.len(), 1);
        assert_eq!(from_changes[0].kind, ChangeKind::Remove);
        assert_eq!(from_changes[0].path, PathBuf::from("/r/wiki/a.md"));

        let to = Event::new(EventKind::Modify(ModifyKind::Name(RenameMode::To)))
            .add_path(PathBuf::from("/r/wiki/b.md"));
        let to_changes = classify_event(&to, Path::new("/r"));
        assert_eq!(to_changes.len(), 1);
        assert_eq!(to_changes[0].kind, ChangeKind::Create);
        assert_eq!(to_changes[0].path, PathBuf::from("/r/wiki/b.md"));
    }

    #[test]
    fn rename_both_keeps_change_when_only_one_endpoint_is_interesting() {
        let rename = Event::new(EventKind::Modify(ModifyKind::Name(RenameMode::Both)))
            .add_path(PathBuf::from("/r/wiki/a.md"))
            .add_path(PathBuf::from("/r/raw/a.pdf"));
        let changes = classify_event(&rename, Path::new("/r"));
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].kind, ChangeKind::Rename);
        assert_eq!(
            changes[0].previous_path,
            Some(PathBuf::from("/r/wiki/a.md"))
        );
        assert!(!changes[0].full_rebuild);
    }

    #[test]
    fn rename_any_with_two_paths_is_paired_like_both() {
        let rename = Event::new(EventKind::Modify(ModifyKind::Name(RenameMode::Any)))
            .add_path(PathBuf::from("/r/wiki/a.md"))
            .add_path(PathBuf::from("/r/wiki/b.md"));
        let changes = classify_event(&rename, Path::new("/r"));
        assert_eq!(changes.len(), 1);
        assert_eq!(
            changes[0].previous_path,
            Some(PathBuf::from("/r/wiki/a.md"))
        );
        assert_eq!(changes[0].path, PathBuf::from("/r/wiki/b.md"));
    }
}
