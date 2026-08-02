use notify::{event::ModifyKind, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
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
    pub kind: ChangeKind,
    pub full_rebuild: bool,
    pub graph_refresh: bool,
}

pub fn classify_event(event: &Event, root: &Path) -> Vec<IndexChange> {
    let kind = match event.kind {
        EventKind::Create(_) => ChangeKind::Create,
        EventKind::Modify(ModifyKind::Name(_)) => ChangeKind::Rename,
        EventKind::Modify(_) => ChangeKind::Modify,
        EventKind::Remove(_) => ChangeKind::Remove,
        _ => ChangeKind::Other,
    };
    event
        .paths
        .iter()
        .filter_map(|p| {
            let rel = p.strip_prefix(root).ok()?;
            if excluded(rel) {
                return None;
            }
            let s = rel.to_string_lossy().replace('\\', "/");
            let wiki = s.starts_with("wiki/") && s.ends_with(".md");
            let schema = s.starts_with("schema/") && s.ends_with(".md");
            let core = s.starts_with("raw/canonical/core-books");
            let graph = s == "graphify-out/graph.json";
            if !(wiki || schema || core || graph) {
                return None;
            }
            Some(IndexChange {
                path: p.clone(),
                kind: kind.clone(),
                full_rebuild: schema || core,
                graph_refresh: graph,
            })
        })
        .collect()
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
    use notify::event::{CreateKind, ModifyKind};
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
}
