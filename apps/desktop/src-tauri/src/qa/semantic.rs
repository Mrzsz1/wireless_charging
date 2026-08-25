#![cfg_attr(test, allow(dead_code))]

use super::{check_cancelled, compact, Candidate};
use fastembed::{
    EmbeddingModel, InitOptions, RerankInitOptionsUserDefined, TextEmbedding, TextRerank,
    TokenizerFiles, UserDefinedRerankingModel,
};
use hf_hub::api::{sync::ApiBuilder, Progress};
use rusqlite::Connection;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{Cursor, Read, Write};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use walkdir::WalkDir;

pub(crate) const MODEL_NAME: &str = "Qdrant/paraphrase-multilingual-MiniLM-L12-v2-onnx-Q";
const MODEL_RETRY_DELAY: Duration = Duration::from_secs(30);
const MODEL_CACHE_FOLDER: &str = "models--Qdrant--paraphrase-multilingual-MiniLM-L12-v2-onnx-Q";
const MODEL_FILE: &str = "model_optimized.onnx";
const TOKENIZER_FILES: &[&str] = &[
    "tokenizer.json",
    "config.json",
    "special_tokens_map.json",
    "tokenizer_config.json",
];
const PROBE_DIMENSION: usize = 384;
const MAX_DOCUMENTS: usize = 8_000;
const MAX_EMBED_CHARS: usize = 2_400;
const EMBED_BATCH_SIZE: usize = 32;
const RESULT_LIMIT: usize = 30;
const MIN_COSINE_SIMILARITY: f32 = 0.60;
const MAX_DISTANCE_FROM_BEST: f32 = 0.14;
#[cfg(target_os = "windows")]
const ORT_RUNTIME_URL: &str =
    "https://github.com/microsoft/onnxruntime/releases/download/v1.20.1/onnxruntime-win-x64-1.20.1.zip";
#[cfg(target_os = "windows")]
const ORT_RUNTIME_SHA256: &str = "78d447051e48bd2e1e778bba378bec4ece11191c9e538cf7b2c4a4565e8f5581";
#[cfg(target_os = "windows")]
const ORT_ARCHIVE_LIMIT_BYTES: usize = 80 * 1024 * 1024;

static SEMANTIC_STATE: OnceLock<Mutex<SemanticState>> = OnceLock::new();
static CACHE_DIR_OVERRIDE: OnceLock<Mutex<Option<PathBuf>>> = OnceLock::new();
static CROSS_ENCODER_STATE: OnceLock<Mutex<CrossEncoderState>> = OnceLock::new();

#[derive(Default)]
struct CrossEncoderState {
    model_dir: Option<PathBuf>,
    model: Option<TextRerank>,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SemanticDeploymentStatus {
    pub state: String,
    pub model_name: String,
    pub cache_dir: String,
    pub default_cache_dir: String,
    pub runtime_ready: bool,
    pub model_files_ready: bool,
    pub tokenizer_ready: bool,
    pub partial_download_count: usize,
    pub total_bytes: u64,
    pub probe_dimension: usize,
    pub checked_at: String,
    pub diagnostic: String,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SemanticDownloadProgress {
    pub status: String,
    pub phase: String,
    pub file_name: String,
    pub downloaded_bytes: u64,
    pub total_bytes: u64,
    pub percent: f64,
    pub bytes_per_second: u64,
    pub message: String,
}

fn progress_event(
    status: &str,
    phase: &str,
    file_name: impl Into<String>,
    downloaded_bytes: u64,
    total_bytes: u64,
    started_at: Instant,
    message: impl Into<String>,
) -> SemanticDownloadProgress {
    let elapsed = started_at.elapsed().as_secs_f64();
    SemanticDownloadProgress {
        status: status.to_string(),
        phase: phase.to_string(),
        file_name: file_name.into(),
        downloaded_bytes,
        total_bytes,
        percent: if total_bytes > 0 {
            (downloaded_bytes.min(total_bytes) as f64 / total_bytes as f64) * 100.0
        } else {
            0.0
        },
        bytes_per_second: if elapsed > 0.0 {
            (downloaded_bytes as f64 / elapsed) as u64
        } else {
            0
        },
        message: message.into(),
    }
}

struct HubProgress<'a, F>
where
    F: FnMut(SemanticDownloadProgress),
{
    on_progress: &'a mut F,
    phase: &'static str,
    file_name: String,
    downloaded_bytes: u64,
    total_bytes: u64,
    started_at: Instant,
}

impl<F> Progress for HubProgress<'_, F>
where
    F: FnMut(SemanticDownloadProgress),
{
    fn init(&mut self, size: usize, _filename: &str) {
        self.downloaded_bytes = 0;
        self.total_bytes = size as u64;
        self.started_at = Instant::now();
        (self.on_progress)(progress_event(
            "downloading",
            self.phase,
            self.file_name.clone(),
            0,
            self.total_bytes,
            self.started_at,
            "开始下载",
        ));
    }

    fn update(&mut self, size: usize) {
        self.downloaded_bytes = self.downloaded_bytes.saturating_add(size as u64);
        (self.on_progress)(progress_event(
            "downloading",
            self.phase,
            self.file_name.clone(),
            self.downloaded_bytes,
            self.total_bytes,
            self.started_at,
            "正在下载",
        ));
    }

    fn finish(&mut self) {
        self.downloaded_bytes = self.total_bytes.max(self.downloaded_bytes);
        (self.on_progress)(progress_event(
            "complete",
            self.phase,
            self.file_name.clone(),
            self.downloaded_bytes,
            self.total_bytes,
            self.started_at,
            "下载完成",
        ));
    }
}

#[derive(Clone)]
struct SemanticDocument {
    key: String,
    kind: String,
    tier: String,
    title: String,
    body: String,
    page_id: String,
    page_type: String,
    source_path: String,
    wikilink: String,
    book_id: String,
    chapter_id: String,
    physical_page_start: Option<i64>,
    physical_page_end: Option<i64>,
    markdown_path: String,
    pdf_path: String,
    node_id: String,
    source_location: String,
}

impl SemanticDocument {
    fn passage(&self) -> String {
        let text = format!("{}\n{}", self.title, self.body);
        text.chars().take(MAX_EMBED_CHARS).collect()
    }

    fn candidate(&self, score: f32) -> Candidate {
        Candidate {
            kind: self.kind.clone(),
            tier: self.tier.clone(),
            title: self.title.clone(),
            snippet: compact(&self.body, if self.kind == "paper" { 1_200 } else { 480 }),
            score: score as f64,
            page_id: self.page_id.clone(),
            page_type: self.page_type.clone(),
            source_path: self.source_path.clone(),
            wikilink: self.wikilink.clone(),
            book_id: self.book_id.clone(),
            chapter_id: self.chapter_id.clone(),
            physical_page_start: self.physical_page_start,
            physical_page_end: self.physical_page_end,
            markdown_path: self.markdown_path.clone(),
            pdf_path: self.pdf_path.clone(),
            node_id: self.node_id.clone(),
            parent_block_id: String::new(),
            parent_context: String::new(),
            source_location: self.source_location.clone(),
            relation: String::new(),
            retrieval_reason: format!(
                "本地多语言语义向量命中；模型 {MODEL_NAME}；cosine={score:.3}"
            ),
        }
    }
}

struct CachedCorpus {
    repository_key: String,
    snapshot_id: String,
    documents: Vec<SemanticDocument>,
    embeddings: Vec<Vec<f32>>,
}

#[derive(Default)]
struct SemanticState {
    model: Option<TextEmbedding>,
    model_retry_after: Option<Instant>,
    corpus: Option<CachedCorpus>,
}

impl SemanticState {
    fn reset_for_cache_switch(&mut self) {
        self.model = None;
        self.corpus = None;
        self.model_retry_after = None;
    }
}

pub(crate) fn default_cache_dir() -> PathBuf {
    env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(env::temp_dir)
        .join("LunaWiki")
        .join("fastembed")
}

pub(crate) fn validate_cache_dir(path: &Path) -> Result<PathBuf, String> {
    if !path.is_absolute() {
        return Err("语义模型缓存目录必须是绝对路径".to_string());
    }
    if path.exists() && !path.is_dir() {
        return Err("语义模型缓存路径必须是目录".to_string());
    }
    fs::create_dir_all(path).map_err(|error| format!("创建语义模型缓存目录失败：{error}"))?;
    let resolved = path
        .canonicalize()
        .map_err(|error| format!("解析语义模型缓存目录失败：{error}"))?;
    let probe = resolved.join(format!(".lunawiki-write-probe-{}", Uuid::new_v4()));
    fs::write(&probe, b"semantic-cache-write-probe")
        .map_err(|error| format!("语义模型缓存目录不可写：{error}"))?;
    fs::remove_file(&probe).map_err(|error| format!("清理目录写入探针失败：{error}"))?;
    Ok(resolved)
}

pub(crate) fn configure_cache_dir(path: Option<PathBuf>) -> Result<PathBuf, String> {
    let effective = match path {
        Some(path) => validate_cache_dir(&path)?,
        None => validate_cache_dir(&default_cache_dir())?,
    };
    let state = SEMANTIC_STATE.get_or_init(|| Mutex::new(SemanticState::default()));
    let mut state = state
        .lock()
        .map_err(|_| "语义模型状态锁定失败".to_string())?;
    let cache_override = CACHE_DIR_OVERRIDE.get_or_init(|| Mutex::new(None));
    *cache_override
        .lock()
        .map_err(|_| "语义模型缓存目录状态锁定失败".to_string())? = Some(effective.clone());
    state.reset_for_cache_switch();
    if let Some(reranker) = CROSS_ENCODER_STATE.get() {
        let mut reranker = reranker
            .lock()
            .map_err(|_| "交叉编码器状态锁定失败".to_string())?;
        reranker.model = None;
        reranker.model_dir = None;
    }
    env::set_var("HF_HOME", &effective);
    Ok(effective)
}

pub(crate) fn effective_cache_dir() -> PathBuf {
    model_cache_dir()
}

fn checked_at() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .to_string()
}

fn runtime_path(cache_dir: &Path) -> PathBuf {
    cache_dir.join("onnxruntime-1.20.1").join("onnxruntime.dll")
}

fn model_repo_path(cache_dir: &Path) -> PathBuf {
    cache_dir.join(MODEL_CACHE_FOLDER)
}

fn snapshot_directories(cache_dir: &Path) -> Vec<PathBuf> {
    let root = model_repo_path(cache_dir).join("snapshots");
    let Ok(entries) = fs::read_dir(root) else {
        return Vec::new();
    };
    entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .collect()
}

fn complete_snapshot(cache_dir: &Path) -> Option<PathBuf> {
    snapshot_directories(cache_dir)
        .into_iter()
        .find(|snapshot| {
            snapshot.join(MODEL_FILE).is_file()
                && TOKENIZER_FILES
                    .iter()
                    .all(|file| snapshot.join(file).is_file())
        })
}

fn partial_download_count(cache_dir: &Path) -> usize {
    let repo = model_repo_path(cache_dir);
    if !repo.is_dir() {
        return 0;
    }
    WalkDir::new(repo)
        .follow_links(false)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| {
            entry.file_type().is_file()
                && entry
                    .path()
                    .extension()
                    .is_some_and(|value| value == "part")
        })
        .count()
}

fn tree_size(path: &Path) -> u64 {
    if !path.exists() {
        return 0;
    }
    WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .filter_map(|entry| entry.metadata().ok().map(|metadata| metadata.len()))
        .sum()
}

fn directory_size(cache_dir: &Path) -> u64 {
    [
        cache_dir.join("onnxruntime-1.20.1"),
        model_repo_path(cache_dir),
        cache_dir.join("vectors"),
    ]
    .iter()
    .map(|path| tree_size(path))
    .sum()
}

#[derive(Default)]
struct DeploymentComponents {
    runtime_ready: bool,
    model_files_ready: bool,
    tokenizer_ready: bool,
    partial_download_count: usize,
    probe_dimension: usize,
}

fn deployment_status(
    cache_dir: &Path,
    state: &str,
    components: DeploymentComponents,
    diagnostic: impl Into<String>,
) -> SemanticDeploymentStatus {
    SemanticDeploymentStatus {
        state: state.to_string(),
        model_name: MODEL_NAME.to_string(),
        cache_dir: cache_dir.to_string_lossy().to_string(),
        default_cache_dir: default_cache_dir().to_string_lossy().to_string(),
        runtime_ready: components.runtime_ready,
        model_files_ready: components.model_files_ready,
        tokenizer_ready: components.tokenizer_ready,
        partial_download_count: components.partial_download_count,
        total_bytes: directory_size(cache_dir),
        probe_dimension: components.probe_dimension,
        checked_at: checked_at(),
        diagnostic: diagnostic.into(),
    }
}

fn initialize_model(cache_dir: &Path) -> Result<TextEmbedding, String> {
    let runtime = runtime_path(cache_dir);
    if !runtime.is_file() {
        return Err("缺少 ONNX Runtime".to_string());
    }
    env::set_var("ORT_DYLIB_PATH", runtime);
    env::set_var("HF_HOME", cache_dir);
    let options = InitOptions::new(EmbeddingModel::ParaphraseMLMiniLML12V2Q)
        .with_cache_dir(cache_dir.to_path_buf())
        .with_max_length(512)
        .with_show_download_progress(false);
    match catch_unwind(AssertUnwindSafe(|| TextEmbedding::try_new(options))) {
        Ok(Ok(model)) => Ok(model),
        Ok(Err(error)) => Err(format!("语义模型加载失败：{error}")),
        Err(_) => Err("语义模型加载发生异常".to_string()),
    }
}

pub(crate) fn check_deployment(cache_dir: &Path) -> SemanticDeploymentStatus {
    if cache_dir.exists() && !cache_dir.is_dir() {
        return deployment_status(
            cache_dir,
            "error",
            DeploymentComponents::default(),
            "配置的语义模型缓存路径不是目录",
        );
    }
    if cache_dir.is_dir() && fs::read_dir(cache_dir).is_err() {
        return deployment_status(
            cache_dir,
            "error",
            DeploymentComponents::default(),
            "语义模型缓存目录不可读",
        );
    }
    let runtime_ready = runtime_path(cache_dir).is_file();
    let snapshots = snapshot_directories(cache_dir);
    let model_files_ready = snapshots
        .iter()
        .any(|snapshot| snapshot.join(MODEL_FILE).is_file());
    let tokenizer_ready = snapshots.iter().any(|snapshot| {
        TOKENIZER_FILES
            .iter()
            .all(|file| snapshot.join(file).is_file())
    });
    let partial_count = partial_download_count(cache_dir);
    if !cache_dir.is_dir() {
        return deployment_status(
            cache_dir,
            "missing",
            DeploymentComponents {
                partial_download_count: partial_count,
                ..Default::default()
            },
            "缓存目录尚不存在",
        );
    }
    if !runtime_ready || complete_snapshot(cache_dir).is_none() {
        let state = if partial_count > 0 {
            "partial"
        } else {
            "missing"
        };
        let diagnostic = if partial_count > 0 {
            format!("检测到 {partial_count} 个未完成下载文件")
        } else if !runtime_ready {
            "缺少 ONNX Runtime".to_string()
        } else {
            "缺少完整的语义模型或 tokenizer 文件".to_string()
        };
        return deployment_status(
            cache_dir,
            state,
            DeploymentComponents {
                runtime_ready,
                model_files_ready,
                tokenizer_ready,
                partial_download_count: partial_count,
                ..Default::default()
            },
            diagnostic,
        );
    }

    match initialize_model(cache_dir).and_then(|model| {
        let vectors = model
            .embed(
                vec!["LunaWiki semantic deployment probe".to_string()],
                Some(1),
            )
            .map_err(|error| format!("语义模型探针推理失败：{error}"))?;
        let vector = vectors
            .first()
            .ok_or_else(|| "语义模型探针没有返回向量".to_string())?;
        if vector.len() != PROBE_DIMENSION || vector.iter().any(|value| !value.is_finite()) {
            return Err(format!(
                "语义模型探针维度或数值无效：dimension={}",
                vector.len()
            ));
        }
        Ok(vector.len())
    }) {
        Ok(dimension) => deployment_status(
            cache_dir,
            "ready",
            DeploymentComponents {
                runtime_ready: true,
                model_files_ready: true,
                tokenizer_ready: true,
                partial_download_count: partial_count,
                probe_dimension: dimension,
            },
            if partial_count > 0 {
                "模型探针通过；目录中另有未完成的非必需下载文件"
            } else {
                "模型文件、tokenizer、ONNX Runtime 与 384 维探针均通过"
            },
        ),
        Err(error) => deployment_status(
            cache_dir,
            "invalid",
            DeploymentComponents {
                runtime_ready: true,
                model_files_ready,
                tokenizer_ready,
                partial_download_count: partial_count,
                ..Default::default()
            },
            error,
        ),
    }
}

fn quarantine_for_repair(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Ok(());
    }
    let file_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| "语义模型缓存目录名称无效".to_string())?;
    let backup = path.with_file_name(format!(
        "{file_name}.invalid-{}-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
        Uuid::new_v4()
    ));
    fs::rename(path, backup).map_err(|error| format!("隔离损坏缓存失败：{error}"))
}

fn cached_snapshot_file(cache_dir: &Path, file_name: &str) -> Option<PathBuf> {
    snapshot_directories(cache_dir)
        .into_iter()
        .map(|snapshot| snapshot.join(file_name))
        .find(|path| path.is_file())
}

fn download_model_files_with_progress<F>(
    cache_dir: &Path,
    on_progress: &mut F,
) -> Result<(), String>
where
    F: FnMut(SemanticDownloadProgress),
{
    env::set_var("HF_HOME", cache_dir);
    let mut builder = ApiBuilder::new()
        .with_cache_dir(cache_dir.to_path_buf())
        .with_progress(false);
    if let Ok(endpoint) = env::var("HF_ENDPOINT") {
        builder = builder.with_endpoint(endpoint);
    }
    let repository = builder
        .build()
        .map_err(|error| format!("初始化模型下载客户端失败：{error}"))?
        .model(MODEL_NAME.to_string());
    for file_name in std::iter::once(MODEL_FILE).chain(TOKENIZER_FILES.iter().copied()) {
        let phase = if file_name == MODEL_FILE {
            "model"
        } else {
            "tokenizer"
        };
        if let Some(path) = cached_snapshot_file(cache_dir, file_name) {
            let size = path.metadata().map(|metadata| metadata.len()).unwrap_or(0);
            let started = Instant::now();
            on_progress(progress_event(
                "skipped",
                phase,
                file_name,
                size,
                size,
                started,
                "已缓存，无需下载",
            ));
            continue;
        }
        repository
            .download_with_progress(
                file_name,
                HubProgress {
                    on_progress,
                    phase,
                    file_name: file_name.to_string(),
                    downloaded_bytes: 0,
                    total_bytes: 0,
                    started_at: Instant::now(),
                },
            )
            .map_err(|error| format!("下载 {file_name} 失败：{error}"))?;
    }
    Ok(())
}

pub(crate) fn repair_deployment_with_progress<F>(
    cache_dir: &Path,
    mut on_progress: F,
) -> Result<SemanticDeploymentStatus, String>
where
    F: FnMut(SemanticDownloadProgress),
{
    let mut last_progress = None;
    let result = {
        let mut emit = |progress: SemanticDownloadProgress| {
            last_progress = Some(progress.clone());
            on_progress(progress);
        };
        (|| {
            let cache_dir = validate_cache_dir(cache_dir)?;
            configure_cache_dir(Some(cache_dir.clone()))?;
            let before = check_deployment(&cache_dir);
            if before.state == "invalid" {
                quarantine_for_repair(&model_repo_path(&cache_dir))?;
                quarantine_for_repair(&cache_dir.join("onnxruntime-1.20.1"))?;
                configure_cache_dir(Some(cache_dir.clone()))?;
            }
            prepare_onnx_runtime_with_progress(&cache_dir, &mut emit)
                .map_err(|error| format!("ONNX Runtime 下载或初始化失败：{error}"))?;
            download_model_files_with_progress(&cache_dir, &mut emit)?;
            emit(progress_event(
                "verifying",
                "inference",
                "model initialization",
                0,
                0,
                Instant::now(),
                "正在加载模型",
            ));
            let state = SEMANTIC_STATE.get_or_init(|| Mutex::new(SemanticState::default()));
            let mut state = state
                .lock()
                .map_err(|_| "语义模型状态锁定失败".to_string())?;
            let model = initialize_model(&cache_dir)
                .map_err(|error| format!("语义模型下载或初始化失败：{error}"))?;
            state.model = Some(model);
            state.model_retry_after = None;
            drop(state);
            emit(progress_event(
                "verifying",
                "inference",
                "384-dimensional probe",
                0,
                0,
                Instant::now(),
                "正在执行推理探针",
            ));
            let status = check_deployment(&cache_dir);
            if status.state != "ready" {
                return Err(status.diagnostic);
            }
            emit(progress_event(
                "complete",
                "inference",
                "deployment",
                1,
                1,
                Instant::now(),
                "语义模型部署完成",
            ));
            Ok(status)
        })()
    };
    if let Err(error) = &result {
        let failed = last_progress
            .clone()
            .map(|progress| SemanticDownloadProgress {
                status: "failed".to_string(),
                message: error.clone(),
                ..progress
            })
            .unwrap_or_else(|| {
                progress_event(
                    "failed",
                    "inference",
                    "deployment",
                    0,
                    0,
                    Instant::now(),
                    error,
                )
            });
        on_progress(failed);
    }
    result
}

pub(crate) fn copy_cache(source: &Path, target: &Path) -> Result<PathBuf, String> {
    if !source.is_dir() {
        return Err("当前语义模型缓存目录不存在，无法复制".to_string());
    }
    let source = source
        .canonicalize()
        .map_err(|error| format!("解析当前缓存目录失败：{error}"))?;
    let target = validate_cache_dir(target)?;
    if source == target || source.starts_with(&target) || target.starts_with(&source) {
        return Err("源缓存目录与目标目录不得相同或相互嵌套".to_string());
    }
    for component in ["onnxruntime-1.20.1", MODEL_CACHE_FOLDER, "vectors"] {
        let component_source = source.join(component);
        if !component_source.exists() {
            continue;
        }
        for entry in WalkDir::new(&component_source).follow_links(false) {
            let entry = entry.map_err(|error| format!("遍历缓存目录失败：{error}"))?;
            let relative = entry
                .path()
                .strip_prefix(&source)
                .map_err(|error| format!("计算缓存相对路径失败：{error}"))?;
            if entry
                .path()
                .extension()
                .is_some_and(|value| value == "lock")
            {
                continue;
            }
            let destination = target.join(relative);
            if entry.file_type().is_dir() {
                fs::create_dir_all(&destination)
                    .map_err(|error| format!("创建目标缓存子目录失败：{error}"))?;
                continue;
            }
            if !entry.path().is_file() {
                continue;
            }
            if let Some(parent) = destination.parent() {
                fs::create_dir_all(parent)
                    .map_err(|error| format!("创建目标缓存目录失败：{error}"))?;
            }
            let temporary = destination.with_file_name(format!(
                ".{}.copying-{}",
                destination
                    .file_name()
                    .and_then(|value| value.to_str())
                    .unwrap_or("cache-file"),
                Uuid::new_v4()
            ));
            fs::copy(entry.path(), &temporary)
                .map_err(|error| format!("复制语义模型缓存失败：{error}"))?;
            if destination.exists() {
                fs::remove_file(&destination)
                    .map_err(|error| format!("替换目标缓存文件失败：{error}"))?;
            }
            fs::rename(&temporary, &destination)
                .map_err(|error| format!("提交目标缓存文件失败：{error}"))?;
        }
    }
    Ok(target)
}

pub(super) fn semantic_candidates(
    connection: &Connection,
    root: &Path,
    question: &str,
    cancelled: Option<&AtomicBool>,
) -> Result<Vec<Candidate>, String> {
    check_cancelled(cancelled)?;
    match super::vector_sync::semantic_candidates_v2(connection, root, question, cancelled) {
        Ok(v2) if !v2.is_empty() => return Ok(v2),
        Err(error) if error.starts_with("QUESTION_CANCELLED") => return Err(error),
        Ok(_) | Err(_) => {}
    }
    let state = SEMANTIC_STATE.get_or_init(|| Mutex::new(SemanticState::default()));
    let Ok(mut state) = state.lock() else {
        return Ok(Vec::new());
    };
    match state.search(connection, root, question, cancelled) {
        Ok(candidates) => Ok(candidates),
        Err(SemanticFailure::Cancelled) => Err("QUESTION_CANCELLED: 用户停止了问答".to_string()),
        Err(SemanticFailure::Unavailable) => Ok(Vec::new()),
    }
}

pub(super) fn semantic_candidates_filtered(
    connection: &Connection,
    root: &Path,
    question: &str,
    kinds: &[String],
    document_ids: &[String],
    cancelled: Option<&AtomicBool>,
) -> Result<Vec<Candidate>, String> {
    check_cancelled(cancelled)?;
    match super::vector_sync::semantic_candidates_v2_filtered(
        connection,
        root,
        question,
        kinds,
        document_ids,
        cancelled,
    ) {
        Ok(candidates) => Ok(candidates),
        Err(error) if error.starts_with("QUESTION_CANCELLED") => Err(error),
        Err(_) => Ok(Vec::new()),
    }
}

pub(super) fn embed_texts(
    texts: Vec<String>,
    cancelled: Option<&AtomicBool>,
) -> Result<Vec<Vec<f32>>, String> {
    if texts.is_empty() {
        return Ok(Vec::new());
    }
    check_cancelled(cancelled)?;
    let state = SEMANTIC_STATE.get_or_init(|| Mutex::new(SemanticState::default()));
    let mut state = state
        .lock()
        .map_err(|_| "SEMANTIC_UNAVAILABLE: 语义模型状态锁定失败".to_string())?;
    state.ensure_model().map_err(|failure| match failure {
        SemanticFailure::Cancelled => "QUESTION_CANCELLED: 用户停止了问答".to_string(),
        SemanticFailure::Unavailable => {
            "SEMANTIC_UNAVAILABLE: 本地语义模型尚未部署或初始化失败".to_string()
        }
    })?;
    let model = state
        .model
        .as_mut()
        .ok_or_else(|| "SEMANTIC_UNAVAILABLE: 本地语义模型不可用".to_string())?;
    let mut embeddings = Vec::with_capacity(texts.len());
    for batch in texts.chunks(EMBED_BATCH_SIZE) {
        check_cancelled(cancelled)?;
        let values = model
            .embed(batch.to_vec(), Some(EMBED_BATCH_SIZE))
            .map_err(|_| "SEMANTIC_UNAVAILABLE: 本地向量推理失败".to_string())?;
        if values.len() != batch.len() {
            return Err("SEMANTIC_UNAVAILABLE: 本地向量推理结果数量异常".to_string());
        }
        if values.iter().any(|vector| {
            vector.len() != PROBE_DIMENSION || vector.iter().any(|value| !value.is_finite())
        }) {
            return Err("SEMANTIC_UNAVAILABLE: 本地向量推理结果无效".to_string());
        }
        embeddings.extend(values);
    }
    Ok(embeddings)
}

fn cross_encoder_model_dir() -> PathBuf {
    env::var_os("QA_RERANKER_MODEL_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| model_cache_dir().join("reranker-bge-base"))
}

fn read_reranker_file(model_dir: &Path, name: &str) -> Result<Vec<u8>, String> {
    fs::read(model_dir.join(name)).map_err(|_| format!("CROSS_ENCODER_UNAVAILABLE: missing_{name}"))
}

fn cross_encoder_artifacts(model_dir: &Path) -> Result<(PathBuf, TokenizerFiles), String> {
    if !model_dir.is_absolute() || !model_dir.is_dir() {
        return Err("CROSS_ENCODER_UNAVAILABLE: model_directory_missing".to_string());
    }
    let onnx = [
        model_dir.join("model.onnx"),
        model_dir.join("onnx/model.onnx"),
    ]
    .into_iter()
    .find(|path| path.is_file())
    .ok_or_else(|| "CROSS_ENCODER_UNAVAILABLE: model_file_missing".to_string())?;
    if onnx.metadata().map(|metadata| metadata.len()).unwrap_or(0) < 16 {
        return Err("CROSS_ENCODER_UNAVAILABLE: model_file_invalid".to_string());
    }
    let tokenizer_file = read_reranker_file(model_dir, "tokenizer.json")?;
    let config_file = read_reranker_file(model_dir, "config.json")?;
    let special_tokens_map_file = read_reranker_file(model_dir, "special_tokens_map.json")?;
    let tokenizer_config_file = read_reranker_file(model_dir, "tokenizer_config.json")?;
    for bytes in [
        &tokenizer_file,
        &config_file,
        &special_tokens_map_file,
        &tokenizer_config_file,
    ] {
        serde_json::from_slice::<serde_json::Value>(bytes)
            .map_err(|_| "CROSS_ENCODER_UNAVAILABLE: tokenizer_file_invalid".to_string())?;
    }
    Ok((
        onnx,
        TokenizerFiles {
            tokenizer_file,
            config_file,
            special_tokens_map_file,
            tokenizer_config_file,
        },
    ))
}

fn initialize_cross_encoder(model_dir: &Path) -> Result<TextRerank, String> {
    initialize_cross_encoder_with_runtime(model_dir, &runtime_path(&model_cache_dir()))
}

fn initialize_cross_encoder_with_runtime(
    model_dir: &Path,
    runtime: &Path,
) -> Result<TextRerank, String> {
    let (onnx, tokenizer_files) = cross_encoder_artifacts(model_dir)?;
    if !runtime.is_file() {
        return Err("CROSS_ENCODER_UNAVAILABLE: runtime_missing".to_string());
    }
    env::set_var("ORT_DYLIB_PATH", runtime);
    let model = UserDefinedRerankingModel::new(onnx, tokenizer_files);
    match catch_unwind(AssertUnwindSafe(|| {
        TextRerank::try_new_from_user_defined(model, RerankInitOptionsUserDefined::default())
    })) {
        Ok(Ok(model)) => Ok(model),
        Ok(Err(_)) => Err("CROSS_ENCODER_UNAVAILABLE: model_invalid".to_string()),
        Err(_) => Err("CROSS_ENCODER_UNAVAILABLE: model_initialization_panic".to_string()),
    }
}

pub(super) fn rerank_texts(
    query: &str,
    documents: Vec<String>,
    cancelled: Option<&AtomicBool>,
) -> Result<Vec<f32>, String> {
    if documents.is_empty() {
        return Ok(Vec::new());
    }
    check_cancelled(cancelled)?;
    let model_dir = cross_encoder_model_dir();
    let state = CROSS_ENCODER_STATE.get_or_init(|| Mutex::new(CrossEncoderState::default()));
    let mut state = state
        .lock()
        .map_err(|_| "CROSS_ENCODER_UNAVAILABLE: state_lock".to_string())?;
    if state.model_dir.as_ref() != Some(&model_dir) {
        state.model = None;
        state.model_dir = Some(model_dir.clone());
    }
    if state.model.is_none() {
        state.model = Some(initialize_cross_encoder(&model_dir)?);
    }
    check_cancelled(cancelled)?;
    let results = state
        .model
        .as_ref()
        .ok_or_else(|| "CROSS_ENCODER_UNAVAILABLE: model_missing".to_string())?
        .rerank(query.to_string(), documents.clone(), false, Some(16))
        .map_err(|_| "CROSS_ENCODER_UNAVAILABLE: inference_failed".to_string())?;
    let mut scores = vec![f32::NEG_INFINITY; documents.len()];
    for result in results {
        if result.index >= scores.len() || !result.score.is_finite() {
            return Err("CROSS_ENCODER_UNAVAILABLE: invalid_result".to_string());
        }
        scores[result.index] = result.score;
    }
    if scores.iter().any(|score| !score.is_finite()) {
        return Err("CROSS_ENCODER_UNAVAILABLE: incomplete_result".to_string());
    }
    Ok(scores)
}

#[derive(Debug)]
enum SemanticFailure {
    Cancelled,
    Unavailable,
}

impl SemanticState {
    fn search(
        &mut self,
        connection: &Connection,
        root: &Path,
        question: &str,
        cancelled: Option<&AtomicBool>,
    ) -> Result<Vec<Candidate>, SemanticFailure> {
        ensure_not_cancelled(cancelled)?;
        let repository_key = repository_key(root);
        let snapshot_id = super::context::index_snapshot_id(connection, root);
        let cache_matches = self.corpus.as_ref().is_some_and(|corpus| {
            corpus.repository_key == repository_key && corpus.snapshot_id == snapshot_id
        });
        if !cache_matches {
            let documents = load_documents(connection).map_err(|_| SemanticFailure::Unavailable)?;
            if documents.is_empty() {
                self.corpus = Some(CachedCorpus {
                    repository_key,
                    snapshot_id,
                    documents,
                    embeddings: Vec::new(),
                });
                return Ok(Vec::new());
            }
            let persisted =
                load_persisted_embeddings(&repository_key, &snapshot_id).unwrap_or_default();
            let mut embeddings = vec![Vec::new(); documents.len()];
            let missing = documents
                .iter()
                .enumerate()
                .filter_map(|(index, document)| {
                    if let Some(vector) = persisted.get(&document.key) {
                        embeddings[index] = vector.clone();
                        None
                    } else {
                        Some(index)
                    }
                })
                .collect::<Vec<_>>();
            if !missing.is_empty() {
                self.ensure_model()?;
                let model = self.model.as_mut().ok_or(SemanticFailure::Unavailable)?;
                for indices in missing.chunks(EMBED_BATCH_SIZE) {
                    ensure_not_cancelled(cancelled)?;
                    let passages = indices
                        .iter()
                        .map(|index| documents[*index].passage())
                        .collect::<Vec<_>>();
                    let batch = model
                        .embed(passages, Some(EMBED_BATCH_SIZE))
                        .map_err(|_| SemanticFailure::Unavailable)?;
                    if batch.len() != indices.len() {
                        return Err(SemanticFailure::Unavailable);
                    }
                    for (index, vector) in indices.iter().zip(batch) {
                        embeddings[*index] = vector;
                    }
                }
                persist_embeddings(&repository_key, &snapshot_id, &documents, &embeddings)
                    .map_err(|_| SemanticFailure::Unavailable)?;
            }
            if embeddings.iter().any(Vec::is_empty) {
                return Err(SemanticFailure::Unavailable);
            }
            self.corpus = Some(CachedCorpus {
                repository_key,
                snapshot_id,
                documents,
                embeddings,
            });
        }

        self.ensure_model()?;
        ensure_not_cancelled(cancelled)?;
        let query = question.trim().to_string();
        let query_embedding = self
            .model
            .as_mut()
            .ok_or(SemanticFailure::Unavailable)?
            .embed(vec![query], Some(1))
            .map_err(|_| SemanticFailure::Unavailable)?
            .pop()
            .ok_or(SemanticFailure::Unavailable)?;
        ensure_not_cancelled(cancelled)?;
        let corpus = self.corpus.as_ref().ok_or(SemanticFailure::Unavailable)?;
        Ok(rank_candidates(
            &corpus.documents,
            &corpus.embeddings,
            &query_embedding,
            RESULT_LIMIT,
        ))
    }

    fn ensure_model(&mut self) -> Result<(), SemanticFailure> {
        if self.model.is_some() {
            return Ok(());
        }
        if self
            .model_retry_after
            .is_some_and(|retry_after| retry_after > Instant::now())
        {
            return Err(SemanticFailure::Unavailable);
        }
        let cache_dir = model_cache_dir();
        if !runtime_path(&cache_dir).is_file() || complete_snapshot(&cache_dir).is_none() {
            self.defer_model_retry();
            return Err(SemanticFailure::Unavailable);
        }
        match initialize_model(&cache_dir) {
            Ok(model) => {
                self.model = Some(model);
                self.model_retry_after = None;
                Ok(())
            }
            Err(_) => {
                self.defer_model_retry();
                Err(SemanticFailure::Unavailable)
            }
        }
    }

    fn defer_model_retry(&mut self) {
        self.model_retry_after = Some(Instant::now() + MODEL_RETRY_DELAY);
    }
}

#[cfg(target_os = "windows")]
fn prepare_onnx_runtime_with_progress<F>(
    cache_dir: &Path,
    on_progress: &mut F,
) -> Result<(), String>
where
    F: FnMut(SemanticDownloadProgress),
{
    let runtime_path = cache_dir.join("onnxruntime-1.20.1").join("onnxruntime.dll");
    if runtime_path.is_file() {
        let size = runtime_path
            .metadata()
            .map(|metadata| metadata.len())
            .unwrap_or(0);
        on_progress(progress_event(
            "skipped",
            "runtime",
            "onnxruntime.dll",
            size,
            size,
            Instant::now(),
            "已缓存，无需下载",
        ));
    } else {
        let mut response = reqwest::blocking::Client::builder()
            .connect_timeout(std::time::Duration::from_secs(15))
            .timeout(std::time::Duration::from_secs(180))
            .build()
            .map_err(|error| format!("创建下载客户端失败：{error}"))?
            .get(ORT_RUNTIME_URL)
            .send()
            .and_then(reqwest::blocking::Response::error_for_status)
            .map_err(|error| format!("请求 ONNX Runtime 失败：{error}"))?;
        if response
            .content_length()
            .is_some_and(|length| length as usize > ORT_ARCHIVE_LIMIT_BYTES)
        {
            return Err("ONNX Runtime 下载包超过安全上限".to_string());
        }
        let total_bytes = response.content_length().unwrap_or(0);
        let started_at = Instant::now();
        let mut archive_bytes = Vec::with_capacity(total_bytes as usize);
        let mut buffer = [0_u8; 64 * 1024];
        on_progress(progress_event(
            "downloading",
            "runtime",
            "onnxruntime-1.20.1.zip",
            0,
            total_bytes,
            started_at,
            "正在下载 ONNX Runtime",
        ));
        loop {
            let count = response
                .read(&mut buffer)
                .map_err(|error| format!("读取 ONNX Runtime 下载流失败：{error}"))?;
            if count == 0 {
                break;
            }
            archive_bytes.extend_from_slice(&buffer[..count]);
            if archive_bytes.len() > ORT_ARCHIVE_LIMIT_BYTES {
                return Err("ONNX Runtime 下载包超过安全上限".to_string());
            }
            on_progress(progress_event(
                "downloading",
                "runtime",
                "onnxruntime-1.20.1.zip",
                archive_bytes.len() as u64,
                total_bytes,
                started_at,
                "正在下载 ONNX Runtime",
            ));
        }
        on_progress(progress_event(
            "complete",
            "runtime",
            "onnxruntime-1.20.1.zip",
            archive_bytes.len() as u64,
            total_bytes.max(archive_bytes.len() as u64),
            started_at,
            "ONNX Runtime 下载完成",
        ));
        if archive_bytes.len() > ORT_ARCHIVE_LIMIT_BYTES
            || format!("{:x}", Sha256::digest(&archive_bytes)) != ORT_RUNTIME_SHA256
        {
            return Err("ONNX Runtime 下载包校验失败".to_string());
        }
        let mut archive = zip::ZipArchive::new(Cursor::new(archive_bytes))
            .map_err(|error| format!("解析 ONNX Runtime 压缩包失败：{error}"))?;
        let entry_index = (0..archive.len())
            .find(|index| {
                archive
                    .by_index(*index)
                    .map(|entry| entry.name().ends_with("/lib/onnxruntime.dll"))
                    .unwrap_or(false)
            })
            .ok_or_else(|| "ONNX Runtime 压缩包缺少 DLL".to_string())?;
        let mut entry = archive
            .by_index(entry_index)
            .map_err(|error| format!("读取 ONNX Runtime DLL 失败：{error}"))?;
        if entry.size() > 32 * 1024 * 1024 {
            return Err("ONNX Runtime DLL 超过安全上限".to_string());
        }
        let parent = runtime_path
            .parent()
            .ok_or_else(|| "ONNX Runtime 目标目录无效".to_string())?;
        std::fs::create_dir_all(parent)
            .map_err(|error| format!("创建 ONNX Runtime 目录失败：{error}"))?;
        let temporary = runtime_path.with_extension("dll.tmp");
        let mut output = std::fs::File::create(&temporary)
            .map_err(|error| format!("创建 ONNX Runtime 临时文件失败：{error}"))?;
        let mut copied = 0_u64;
        let mut buffer = [0_u8; 64 * 1024];
        loop {
            let count = entry
                .read(&mut buffer)
                .map_err(|error| format!("解压 ONNX Runtime 失败：{error}"))?;
            if count == 0 {
                break;
            }
            copied += count as u64;
            if copied > 32 * 1024 * 1024 {
                let _ = std::fs::remove_file(&temporary);
                return Err("ONNX Runtime DLL 超过安全上限".to_string());
            }
            output
                .write_all(&buffer[..count])
                .map_err(|error| format!("写入 ONNX Runtime 失败：{error}"))?;
        }
        output
            .flush()
            .map_err(|error| format!("刷新 ONNX Runtime 文件失败：{error}"))?;
        std::fs::rename(&temporary, &runtime_path)
            .map_err(|error| format!("提交 ONNX Runtime 文件失败：{error}"))?;
    }
    env::set_var("ORT_DYLIB_PATH", &runtime_path);
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn prepare_onnx_runtime_with_progress<F>(
    _cache_dir: &Path,
    on_progress: &mut F,
) -> Result<(), String>
where
    F: FnMut(SemanticDownloadProgress),
{
    // Linux/macOS packages may provide the ONNX Runtime dynamic library. The
    // semantic channel remains fail-soft when it is absent.
    on_progress(progress_event(
        "skipped",
        "runtime",
        "system ONNX Runtime",
        0,
        0,
        Instant::now(),
        "使用系统 ONNX Runtime",
    ));
    Ok(())
}

fn ensure_not_cancelled(cancelled: Option<&AtomicBool>) -> Result<(), SemanticFailure> {
    check_cancelled(cancelled).map_err(|_| SemanticFailure::Cancelled)
}

fn model_cache_dir() -> PathBuf {
    CACHE_DIR_OVERRIDE
        .get_or_init(|| Mutex::new(None))
        .lock()
        .ok()
        .and_then(|path| path.clone())
        .unwrap_or_else(default_cache_dir)
}

fn repository_key(root: &Path) -> String {
    let normalized = root
        .canonicalize()
        .unwrap_or_else(|_| root.to_path_buf())
        .to_string_lossy()
        .replace('\\', "/")
        .to_lowercase();
    format!("{:x}", Sha256::digest(normalized.as_bytes()))
}

fn table_exists(connection: &Connection, table: &str) -> rusqlite::Result<bool> {
    connection.query_row(
        "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type IN ('table','view') AND name=?1)",
        [table],
        |row| row.get(0),
    )
}

fn load_documents(connection: &Connection) -> rusqlite::Result<Vec<SemanticDocument>> {
    let mut documents = Vec::new();
    if table_exists(connection, "pages")? {
        let mut statement = connection.prepare(
            "SELECT id,page_type,title,body,source_path FROM pages ORDER BY id LIMIT ?1",
        )?;
        let rows = statement.query_map([MAX_DOCUMENTS as i64], |row| {
            let page_id: String = row.get(0)?;
            let page_type: String = row.get(1)?;
            Ok(SemanticDocument {
                key: format!("wiki:{page_id}"),
                kind: "wiki".to_string(),
                tier: if page_type == "method" {
                    "transferable_method".to_string()
                } else {
                    "direct".to_string()
                },
                title: row.get(2)?,
                body: row.get(3)?,
                page_id: page_id.clone(),
                page_type,
                source_path: row.get(4)?,
                wikilink: format!("[[{}]]", page_id.trim_end_matches(".md")),
                book_id: String::new(),
                chapter_id: String::new(),
                physical_page_start: None,
                physical_page_end: None,
                markdown_path: String::new(),
                pdf_path: String::new(),
                node_id: String::new(),
                source_location: String::new(),
            })
        })?;
        documents.extend(rows.collect::<rusqlite::Result<Vec<_>>>()?);
    }
    if documents.len() < MAX_DOCUMENTS && table_exists(connection, "paper_sections")? {
        let remaining = MAX_DOCUMENTS - documents.len();
        let mut statement = connection.prepare(
            "SELECT id,page_id,title,section_title,source_path,pdf_path,line_start,line_end,body
             FROM paper_sections ORDER BY page_id,line_start LIMIT ?1",
        )?;
        let rows = statement.query_map([remaining as i64], |row| {
            let section_id: String = row.get(0)?;
            let page_id: String = row.get(1)?;
            let paper_title: String = row.get(2)?;
            let section_title: String = row.get(3)?;
            let source_path: String = row.get(4)?;
            let line_start: i64 = row.get(6)?;
            let line_end: i64 = row.get(7)?;
            Ok(SemanticDocument {
                key: format!("paper:{section_id}"),
                kind: "paper".to_string(),
                tier: "primary_source".to_string(),
                title: format!("{paper_title} · {section_title}"),
                body: row.get(8)?,
                page_id: page_id.clone(),
                page_type: "source".to_string(),
                source_path: source_path.clone(),
                wikilink: format!("[[{page_id}]]"),
                book_id: String::new(),
                chapter_id: String::new(),
                physical_page_start: None,
                physical_page_end: None,
                markdown_path: source_path,
                pdf_path: row.get(5)?,
                node_id: section_id,
                source_location: format!("{section_title} · 原文第 {line_start}–{line_end} 行"),
            })
        })?;
        documents.extend(rows.collect::<rusqlite::Result<Vec<_>>>()?);
    }
    if documents.len() < MAX_DOCUMENTS
        && table_exists(connection, "book_chapters")?
        && table_exists(connection, "book_chapters_fts")?
        && table_exists(connection, "books")?
    {
        let remaining = MAX_DOCUMENTS - documents.len();
        let mut statement = connection.prepare(
            "SELECT c.id,c.book_id,b.title,c.title,f.body,c.markdown_path,c.pdf_path,
                    c.physical_page_start,c.physical_page_end
             FROM book_chapters c
             JOIN book_chapters_fts f ON f.chapter_id=c.id
             JOIN books b ON b.id=c.book_id
             ORDER BY c.book_id,c.chapter_number LIMIT ?1",
        )?;
        let rows = statement.query_map([remaining as i64], |row| {
            let chapter_id: String = row.get(0)?;
            let book_id: String = row.get(1)?;
            let book_title: String = row.get(2)?;
            let chapter_title: String = row.get(3)?;
            let markdown_path: String = row.get(5)?;
            Ok(SemanticDocument {
                key: format!("book:{chapter_id}"),
                kind: "book".to_string(),
                tier: "theory".to_string(),
                title: format!("{book_title} · {chapter_title}"),
                body: row.get(4)?,
                page_id: String::new(),
                page_type: String::new(),
                source_path: markdown_path.clone(),
                wikilink: String::new(),
                book_id,
                chapter_id,
                physical_page_start: row.get(7)?,
                physical_page_end: row.get(8)?,
                markdown_path,
                pdf_path: row.get(6)?,
                node_id: String::new(),
                source_location: String::new(),
            })
        })?;
        documents.extend(rows.collect::<rusqlite::Result<Vec<_>>>()?);
    }
    Ok(documents)
}

fn load_persisted_embeddings(
    repository_key: &str,
    snapshot_id: &str,
) -> std::io::Result<HashMap<String, Vec<f32>>> {
    let bytes = std::fs::read(vector_cache_path(repository_key, snapshot_id))?;
    let mut cursor = Cursor::new(bytes);
    let mut magic = [0_u8; 8];
    cursor.read_exact(&mut magic)?;
    if &magic != b"LUNAVEC1" {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "invalid semantic vector cache",
        ));
    }
    let count = read_u32(&mut cursor)? as usize;
    if count > MAX_DOCUMENTS {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "semantic vector cache count exceeds limit",
        ));
    }
    let mut embeddings = HashMap::new();
    for _ in 0..count {
        let key_length = read_u32(&mut cursor)? as usize;
        let dimensions = read_u32(&mut cursor)? as usize;
        if key_length == 0 || key_length > 1_024 || dimensions == 0 || dimensions > 2_048 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "semantic vector cache record exceeds limit",
            ));
        }
        let mut key = vec![0_u8; key_length];
        cursor.read_exact(&mut key)?;
        let key = String::from_utf8(key).map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "semantic vector cache key is not UTF-8",
            )
        })?;
        let mut vector_bytes = vec![0_u8; dimensions * 4];
        cursor.read_exact(&mut vector_bytes)?;
        embeddings.insert(key, decode_embedding(&vector_bytes));
    }
    Ok(embeddings)
}

fn persist_embeddings(
    repository_key: &str,
    snapshot_id: &str,
    documents: &[SemanticDocument],
    embeddings: &[Vec<f32>],
) -> std::io::Result<()> {
    if documents.len() != embeddings.len() || documents.len() > MAX_DOCUMENTS {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "semantic vector cache shape mismatch",
        ));
    }
    let path = vector_cache_path(repository_key, snapshot_id);
    let parent = path.parent().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "invalid vector cache path",
        )
    })?;
    std::fs::create_dir_all(parent)?;
    let temporary = path.with_extension("bin.tmp");
    let mut output = std::fs::File::create(&temporary)?;
    output.write_all(b"LUNAVEC1")?;
    output.write_all(&(documents.len() as u32).to_le_bytes())?;
    for (document, embedding) in documents.iter().zip(embeddings) {
        let key = document.key.as_bytes();
        if key.is_empty() || key.len() > 1_024 || embedding.is_empty() || embedding.len() > 2_048 {
            let _ = std::fs::remove_file(&temporary);
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "semantic vector cache record exceeds limit",
            ));
        }
        output.write_all(&(key.len() as u32).to_le_bytes())?;
        output.write_all(&(embedding.len() as u32).to_le_bytes())?;
        output.write_all(key)?;
        output.write_all(&encode_embedding(embedding))?;
    }
    output.flush()?;
    std::fs::rename(temporary, path)
}

fn vector_cache_path(repository_key: &str, snapshot_id: &str) -> PathBuf {
    let snapshot_key = snapshot_id.trim_start_matches("sha256:");
    model_cache_dir()
        .join("vectors")
        .join(repository_key)
        .join(format!(
            "{snapshot_key}-paraphrase-multilingual-minilm-q.bin"
        ))
}

fn read_u32(reader: &mut impl Read) -> std::io::Result<u32> {
    let mut bytes = [0_u8; 4];
    reader.read_exact(&mut bytes)?;
    Ok(u32::from_le_bytes(bytes))
}

fn encode_embedding(vector: &[f32]) -> Vec<u8> {
    vector
        .iter()
        .flat_map(|value| value.to_le_bytes())
        .collect()
}

fn decode_embedding(bytes: &[u8]) -> Vec<f32> {
    if bytes.is_empty() || bytes.len() % 4 != 0 {
        return Vec::new();
    }
    bytes
        .chunks_exact(4)
        .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .collect()
}

fn cosine_similarity(left: &[f32], right: &[f32]) -> Option<f32> {
    if left.is_empty() || left.len() != right.len() {
        return None;
    }
    let mut dot = 0.0_f32;
    let mut left_norm = 0.0_f32;
    let mut right_norm = 0.0_f32;
    for (left, right) in left.iter().zip(right) {
        dot += left * right;
        left_norm += left * left;
        right_norm += right * right;
    }
    let denominator = left_norm.sqrt() * right_norm.sqrt();
    (denominator > f32::EPSILON)
        .then_some(dot / denominator)
        .filter(|score| score.is_finite())
}

fn rank_candidates(
    documents: &[SemanticDocument],
    embeddings: &[Vec<f32>],
    query: &[f32],
    limit: usize,
) -> Vec<Candidate> {
    let mut ranked = documents
        .iter()
        .zip(embeddings)
        .filter_map(|(document, embedding)| {
            cosine_similarity(query, embedding).map(|score| (document, score))
        })
        .collect::<Vec<_>>();
    ranked.sort_by(|left, right| right.1.total_cmp(&left.1));
    let Some(best) = ranked.first().map(|item| item.1) else {
        return Vec::new();
    };
    let minimum = MIN_COSINE_SIMILARITY.max(best - MAX_DISTANCE_FROM_BEST);
    ranked
        .into_iter()
        .filter(|(_, score)| *score >= minimum)
        .take(limit)
        .map(|(document, score)| document.candidate(score))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn document(key: &str, title: &str) -> SemanticDocument {
        SemanticDocument {
            key: key.to_string(),
            kind: "wiki".to_string(),
            tier: "direct".to_string(),
            title: title.to_string(),
            body: title.to_string(),
            page_id: format!("{key}.md"),
            page_type: "method".to_string(),
            source_path: format!("wiki/{key}.md"),
            wikilink: format!("[[{key}]]"),
            book_id: String::new(),
            chapter_id: String::new(),
            physical_page_start: None,
            physical_page_end: None,
            markdown_path: String::new(),
            pdf_path: String::new(),
            node_id: String::new(),
            source_location: String::new(),
        }
    }

    #[test]
    fn embedding_binary_round_trip_is_exact() {
        let vector = vec![0.125, -0.5, 1.0, 3.25];
        assert_eq!(decode_embedding(&encode_embedding(&vector)), vector);
    }

    #[test]
    fn semantic_ranking_uses_cosine_and_similarity_floor() {
        let documents = vec![document("related", "相关"), document("noise", "噪声")];
        let embeddings = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let ranked = rank_candidates(&documents, &embeddings, &[0.98, 0.02], 10);
        assert_eq!(ranked.len(), 1);
        assert_eq!(ranked[0].page_id, "related.md");
        assert!(ranked[0].retrieval_reason.contains("cosine="));
    }

    #[test]
    fn corpus_loader_degrades_when_optional_tables_are_absent() {
        let connection = Connection::open_in_memory().unwrap();
        connection
            .execute_batch(
                "CREATE TABLE pages(id TEXT PRIMARY KEY,page_type TEXT,title TEXT,body TEXT,source_path TEXT);
                 INSERT INTO pages VALUES('method.md','method','Method','semantic body','wiki/method.md');",
            )
            .unwrap();
        let documents = load_documents(&connection).unwrap();
        assert_eq!(documents.len(), 1);
        assert_eq!(documents[0].key, "wiki:method.md");
    }

    #[test]
    fn failed_model_initialization_uses_bounded_retry_instead_of_permanent_disable() {
        let mut state = SemanticState::default();
        state.defer_model_retry();
        assert!(state
            .model_retry_after
            .is_some_and(|retry_after| retry_after > Instant::now()));
        state.model_retry_after = Some(Instant::now() - Duration::from_secs(1));
        assert!(!state
            .model_retry_after
            .is_some_and(|retry_after| retry_after > Instant::now()));
    }

    #[test]
    fn deployment_check_distinguishes_missing_and_partial_without_loading_a_model() {
        let cache = tempfile::tempdir().unwrap();
        let missing = check_deployment(cache.path());
        assert_eq!(missing.state, "missing");
        let part = model_repo_path(cache.path())
            .join("blobs")
            .join("model.part");
        fs::create_dir_all(part.parent().unwrap()).unwrap();
        fs::write(part, b"partial").unwrap();
        let partial = check_deployment(cache.path());
        assert_eq!(partial.state, "partial");
        assert_eq!(partial.partial_download_count, 1);
        assert_eq!(partial.probe_dimension, 0);
    }

    #[test]
    fn deployment_check_requires_runtime_model_and_tokenizer_from_one_snapshot() {
        let cache = tempfile::tempdir().unwrap();
        let snapshot = model_repo_path(cache.path())
            .join("snapshots")
            .join("fixture");
        fs::create_dir_all(&snapshot).unwrap();
        fs::write(snapshot.join(MODEL_FILE), b"fixture").unwrap();
        for file in TOKENIZER_FILES {
            fs::write(snapshot.join(file), b"{}").unwrap();
        }
        let status = check_deployment(cache.path());
        assert_eq!(status.state, "missing");
        assert!(status.model_files_ready);
        assert!(status.tokenizer_ready);
        assert!(!status.runtime_ready);
    }

    #[test]
    fn cache_copy_is_non_destructive_and_skips_locks() {
        let source = tempfile::tempdir().unwrap();
        let target_root = tempfile::tempdir().unwrap();
        let target = target_root.path().join("copied-cache");
        let blobs = model_repo_path(source.path()).join("blobs");
        fs::create_dir_all(&blobs).unwrap();
        fs::write(blobs.join("model.bin"), b"model").unwrap();
        fs::write(blobs.join("model.lock"), b"lock").unwrap();
        let copied = copy_cache(source.path(), &target).unwrap();
        let copied_blobs = model_repo_path(&copied).join("blobs");
        assert_eq!(fs::read(copied_blobs.join("model.bin")).unwrap(), b"model");
        assert!(!copied_blobs.join("model.lock").exists());
        assert!(blobs.join("model.bin").is_file());
    }

    #[test]
    fn semantic_state_reset_drops_retry_and_corpus_for_a_cache_switch() {
        let mut state = SemanticState {
            model: None,
            model_retry_after: Some(Instant::now() + Duration::from_secs(30)),
            corpus: Some(CachedCorpus {
                repository_key: "repository".to_string(),
                snapshot_id: "snapshot".to_string(),
                documents: Vec::new(),
                embeddings: Vec::new(),
            }),
        };
        state.reset_for_cache_switch();
        assert!(state.model.is_none());
        assert!(state.model_retry_after.is_none());
        assert!(state.corpus.is_none());
    }

    #[test]
    fn hub_progress_reports_real_byte_percentages_and_completion() {
        let mut events = Vec::new();
        {
            let mut callback = |event| events.push(event);
            let mut progress = HubProgress {
                on_progress: &mut callback,
                phase: "model",
                file_name: MODEL_FILE.to_string(),
                downloaded_bytes: 0,
                total_bytes: 0,
                started_at: Instant::now(),
            };
            progress.init(100, MODEL_FILE);
            progress.update(25);
            progress.update(25);
            progress.finish();
        }
        assert_eq!(events[1].downloaded_bytes, 25);
        assert_eq!(events[1].percent, 25.0);
        assert_eq!(events[2].downloaded_bytes, 50);
        assert_eq!(events.last().unwrap().status, "complete");
        assert_eq!(events.last().unwrap().percent, 100.0);
    }

    #[test]
    fn cached_snapshot_detection_never_needs_a_network_probe() {
        let cache = tempfile::tempdir().unwrap();
        let snapshot = model_repo_path(cache.path()).join("snapshots/fixture");
        fs::create_dir_all(&snapshot).unwrap();
        fs::write(snapshot.join(MODEL_FILE), b"fixture-model").unwrap();
        assert!(cached_snapshot_file(cache.path(), MODEL_FILE).is_some());
        assert!(cached_snapshot_file(cache.path(), "tokenizer.json").is_none());
    }

    #[test]
    fn cross_encoder_requires_predeployed_local_artifacts() {
        let missing = tempfile::tempdir().unwrap().path().join("missing-reranker");
        let error = cross_encoder_artifacts(&missing).unwrap_err();
        assert_eq!(error, "CROSS_ENCODER_UNAVAILABLE: model_directory_missing");
        assert!(!missing.exists());
    }

    #[test]
    fn corrupt_cross_encoder_artifacts_fail_before_runtime_initialization() {
        let model = tempfile::tempdir().unwrap();
        fs::write(model.path().join("model.onnx"), b"corrupt").unwrap();
        for file in [
            "tokenizer.json",
            "config.json",
            "special_tokens_map.json",
            "tokenizer_config.json",
        ] {
            fs::write(model.path().join(file), b"not-json").unwrap();
        }
        let error = cross_encoder_artifacts(model.path()).unwrap_err();
        assert_eq!(error, "CROSS_ENCODER_UNAVAILABLE: model_file_invalid");
    }

    #[test]
    fn cross_encoder_cancellation_precedes_model_loading() {
        let cancelled = AtomicBool::new(true);
        let error =
            rerank_texts("query", vec!["document".to_string()], Some(&cancelled)).unwrap_err();
        assert!(error.starts_with("QUESTION_CANCELLED"));
    }
}
