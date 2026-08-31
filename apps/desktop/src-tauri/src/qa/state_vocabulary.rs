use super::state_mutation::{
    ParameterValue, ResearchStatePatch, StateAction, StateField, StateValue,
};
use rusqlite::{params, Connection, OptionalExtension, Transaction};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::{Arc, Mutex, OnceLock};
use uuid::Uuid;

pub const STATE_VOCABULARY_VERSION: &str = "qa-state-vocabulary-v1";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum VocabularyKind {
    Objective,
    Constraint,
    Assumption,
    Method,
    Parameter,
}

impl VocabularyKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Objective => "objective",
            Self::Constraint => "constraint",
            Self::Assumption => "assumption",
            Self::Method => "method",
            Self::Parameter => "parameter",
        }
    }

    pub fn from_state_field(field: StateField) -> Self {
        match field {
            StateField::Objective => Self::Objective,
            StateField::Constraint => Self::Constraint,
            StateField::Assumption => Self::Assumption,
            StateField::Method => Self::Method,
            StateField::Parameter => Self::Parameter,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ParameterValueKind {
    Integer,
    Float,
    Boolean,
    Text,
    Enum,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParameterSpec {
    pub value_kind: ParameterValueKind,
    #[serde(default)]
    pub unit: Option<String>,
    #[serde(default)]
    pub minimum: Option<f64>,
    #[serde(default)]
    pub maximum: Option<f64>,
    #[serde(default)]
    pub enum_values: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VocabularyOrigin {
    BuiltIn,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StateFieldDefinition {
    pub id: String,
    pub kind: VocabularyKind,
    pub label: String,
    pub description: String,
    pub aliases: Vec<String>,
    pub examples: Vec<String>,
    #[serde(default)]
    pub parameter_spec: Option<ParameterSpec>,
    pub origin: VocabularyOrigin,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StateVocabularyRegistry {
    pub schema_version: String,
    pub revision: u64,
    pub fields: Vec<StateFieldDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CustomStateFieldInput {
    pub kind: VocabularyKind,
    pub label: String,
    pub description: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default)]
    pub examples: Vec<String>,
    #[serde(default)]
    pub parameter_spec: Option<ParameterSpec>,
}

static REGISTRY_CACHE: OnceLock<Mutex<HashMap<(String, u64), Arc<StateVocabularyRegistry>>>> =
    OnceLock::new();

fn registry_cache() -> &'static Mutex<HashMap<(String, u64), Arc<StateVocabularyRegistry>>> {
    REGISTRY_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AllowedStateField {
    pub id: String,
    pub kind: VocabularyKind,
    pub label: String,
    pub description: String,
    pub aliases: Vec<String>,
    pub examples: Vec<String>,
    pub parameter_spec: Option<ParameterSpec>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VocabularyValidationStats {
    pub unknown_id_count: usize,
    pub kind_mismatch_count: usize,
    pub disabled_field_count: usize,
    pub value_type_mismatch_count: usize,
    pub value_out_of_range_count: usize,
}

fn field(
    id: &str,
    kind: VocabularyKind,
    label: &str,
    description: &str,
    aliases: &[&str],
    examples: &[&str],
) -> StateFieldDefinition {
    StateFieldDefinition {
        id: id.to_string(),
        kind,
        label: label.to_string(),
        description: description.to_string(),
        aliases: aliases.iter().map(|value| (*value).to_string()).collect(),
        examples: examples.iter().map(|value| (*value).to_string()).collect(),
        parameter_spec: None,
        origin: VocabularyOrigin::BuiltIn,
        enabled: true,
    }
}

fn parameter(
    id: &str,
    label: &str,
    description: &str,
    aliases: &[&str],
    value_kind: ParameterValueKind,
    unit: Option<&str>,
    minimum: Option<f64>,
) -> StateFieldDefinition {
    let mut definition = field(
        id,
        VocabularyKind::Parameter,
        label,
        description,
        aliases,
        &[],
    );
    definition.parameter_spec = Some(ParameterSpec {
        value_kind,
        unit: unit.map(str::to_string),
        minimum,
        maximum: None,
        enum_values: Vec::new(),
    });
    definition
}

pub fn built_in_fields() -> Vec<StateFieldDefinition> {
    use VocabularyKind::{Assumption, Constraint, Method, Objective};
    vec![
        field(
            "minimize_dead_nodes",
            Objective,
            "最小化死亡节点",
            "减少能量耗尽的传感器节点数量。",
            &["最小化死亡节点", "减少死亡节点", "minimize dead nodes"],
            &[],
        ),
        field(
            "maximize_network_lifetime",
            Objective,
            "最大化网络寿命",
            "延长无线传感器网络的有效运行寿命。",
            &["最大化网络寿命", "maximize network lifetime"],
            &[],
        ),
        field(
            "minimize_delay",
            Objective,
            "最小化延迟",
            "降低调度或服务完成延迟。",
            &["最小化延迟", "降低延迟", "minimize delay"],
            &[],
        ),
        field(
            "minimize_travel_distance",
            Objective,
            "最小化行驶距离",
            "降低移动充电设备的总行驶距离。",
            &["最小化距离", "最小化行驶距离", "minimize distance"],
            &[],
        ),
        field(
            "time_windows",
            Constraint,
            "时间窗",
            "服务或访问必须发生在指定时间窗口内。",
            &["时间窗", "time window"],
            &[],
        ),
        field(
            "deadlines",
            Constraint,
            "截止期限",
            "任务或充电服务必须在截止时间前完成。",
            &["deadline", "截止时间", "最后期限"],
            &[],
        ),
        field(
            "battery_capacity",
            Constraint,
            "电池容量约束",
            "节点或设备受有限电池容量约束。",
            &["容量约束", "电池容量", "battery capacity"],
            &[],
        ),
        field(
            "obstacle_avoidance",
            Constraint,
            "障碍规避",
            "移动路径必须避开障碍物。",
            &["障碍物", "避障", "obstacle"],
            &[],
        ),
        field(
            "multi_vehicle_coordination",
            Constraint,
            "多移动充电设备协同",
            "存在多个移动充电车、移动充电器或移动供能设备，需要进行协同调度。",
            &[
                "多车协同",
                "多辆车",
                "多充电车",
                "多个移动充电车",
                "多辆移动充电车",
                "多个移动充电器",
                "多套移动供能设备",
                "multiple mobile chargers",
                "multi vehicle",
            ],
            &["多个移动充电器", "多套移动供能设备共同调度"],
        ),
        field(
            "transmission_loss",
            Constraint,
            "传输损耗约束",
            "通信或能量传输存在损耗。",
            &["传输损耗", "packet loss", "transmission loss"],
            &[],
        ),
        field(
            "stationary_sensor_nodes",
            Assumption,
            "静态传感器节点",
            "传感器节点的位置在规划期间保持不变。",
            &["节点静态", "静态节点", "stationary nodes"],
            &[],
        ),
        field(
            "single_mobile_charger",
            Assumption,
            "单移动充电设备",
            "系统只有一个移动充电设备。",
            &["单充电车", "single charger"],
            &[],
        ),
        field(
            "connected_graph",
            Assumption,
            "连通图",
            "网络拓扑保持连通。",
            &["连通图", "connected graph"],
            &[],
        ),
        field(
            "particle_swarm_optimization",
            Method,
            "粒子群优化",
            "使用粒子群优化方法求解。",
            &["pso", "粒子群"],
            &[],
        ),
        field(
            "adaptive_large_neighborhood_search",
            Method,
            "自适应大邻域搜索",
            "使用自适应大邻域搜索方法求解。",
            &["alns", "自适应大邻域"],
            &[],
        ),
        field(
            "mixed_integer_linear_programming",
            Method,
            "混合整数线性规划",
            "使用混合整数线性规划建模或求解。",
            &["milp", "混合整数线性规划"],
            &[],
        ),
        field(
            "genetic_algorithm",
            Method,
            "遗传算法",
            "使用遗传算法求解。",
            &["遗传算法", "genetic algorithm", " ga "],
            &[],
        ),
        field(
            "reinforcement_learning",
            Method,
            "强化学习",
            "使用强化学习方法求解。",
            &["强化学习", "reinforcement learning"],
            &[],
        ),
        field(
            "dynamic_programming",
            Method,
            "动态规划",
            "使用动态规划方法求解。",
            &["动态规划", "dynamic programming"],
            &[],
        ),
        field(
            "greedy",
            Method,
            "贪心方法",
            "使用贪心策略求解。",
            &["贪心", "greedy"],
            &[],
        ),
        parameter(
            "mobile_charger_count",
            "移动充电设备数量",
            "参与调度的移动充电车、移动充电器或移动供能设备数量。",
            &[
                "移动充电车",
                "充电车数量",
                "移动充电器数量",
                "mobile charger",
            ],
            ParameterValueKind::Integer,
            None,
            Some(1.0),
        ),
        parameter(
            "node_count",
            "节点数量",
            "无线传感器网络中的节点数量。",
            &["节点数", "sensor nodes", "node count"],
            ParameterValueKind::Integer,
            None,
            Some(1.0),
        ),
        parameter(
            "battery_capacity",
            "电池容量",
            "节点或设备的电池容量数值。",
            &["电池容量", "battery capacity"],
            ParameterValueKind::Float,
            None,
            Some(0.0),
        ),
        parameter(
            "charger_capacity",
            "充电器容量",
            "充电器可用的容量数值。",
            &["充电器容量", "charger capacity"],
            ParameterValueKind::Float,
            None,
            Some(0.0),
        ),
        parameter(
            "deadline",
            "截止时间",
            "任务截止时间参数。",
            &["deadline", "截止时间"],
            ParameterValueKind::Float,
            Some("minute"),
            Some(0.0),
        ),
        parameter(
            "time_horizon",
            "时间范围",
            "规划或仿真的时间范围。",
            &["时间范围", "time horizon"],
            ParameterValueKind::Float,
            Some("minute"),
            Some(0.0),
        ),
        parameter(
            "charger_speed",
            "充电车速度",
            "移动充电设备的行驶速度。",
            &["充电车速度", "charger speed"],
            ParameterValueKind::Float,
            Some("m/s"),
            Some(0.0),
        ),
        parameter(
            "transmission_loss",
            "传输损耗",
            "通信或能量传输的损耗参数。",
            &["传输损耗", "transmission loss"],
            ParameterValueKind::Float,
            None,
            Some(0.0),
        ),
        parameter(
            "energy_threshold",
            "能量阈值",
            "触发服务或告警的剩余能量阈值。",
            &["能量阈值", "energy threshold"],
            ParameterValueKind::Float,
            None,
            Some(0.0),
        ),
    ]
}

impl Default for StateVocabularyRegistry {
    fn default() -> Self {
        Self {
            schema_version: STATE_VOCABULARY_VERSION.to_string(),
            revision: 0,
            fields: built_in_fields(),
        }
    }
}

impl StateVocabularyRegistry {
    pub fn merged(revision: u64, custom_fields: Vec<StateFieldDefinition>) -> Self {
        let mut fields = built_in_fields();
        fields.extend(custom_fields);
        fields.sort_by(|left, right| left.id.cmp(&right.id));
        Self {
            schema_version: STATE_VOCABULARY_VERSION.to_string(),
            revision,
            fields,
        }
    }

    pub fn field(&self, id: &str) -> Option<&StateFieldDefinition> {
        self.fields.iter().find(|field| field.id == id)
    }

    pub fn enabled_custom_count(&self) -> usize {
        self.fields
            .iter()
            .filter(|field| field.origin == VocabularyOrigin::Custom && field.enabled)
            .count()
    }

    pub fn hash(&self) -> String {
        let bytes = serde_json::to_vec(self).unwrap_or_default();
        format!("{:x}", Sha256::digest(bytes))
    }

    pub fn allowed_state_fields(&self) -> Vec<AllowedStateField> {
        self.fields
            .iter()
            .filter(|field| field.enabled)
            .map(|field| AllowedStateField {
                id: field.id.clone(),
                kind: field.kind,
                label: field.label.clone(),
                description: field.description.clone(),
                aliases: field.aliases.iter().take(6).cloned().collect(),
                examples: field.examples.iter().take(3).cloned().collect(),
                parameter_spec: field.parameter_spec.clone(),
            })
            .collect()
    }

    /// Returns every field whose canonical ID, label, or alias appears verbatim in
    /// the normalized text. Callers must treat more than one result as ambiguous.
    pub fn exact_matches(&self, text: &str, kind: VocabularyKind) -> Vec<&StateFieldDefinition> {
        let normalized = text.trim().to_lowercase();
        let mut matches = self
            .fields
            .iter()
            .filter(|field| field.enabled && field.kind == kind)
            .filter(|field| {
                normalized.contains(&field.id.to_lowercase())
                    || normalized.contains(&field.label.to_lowercase())
                    || field.aliases.iter().any(|alias| {
                        let alias = alias.trim().to_lowercase();
                        !alias.is_empty() && normalized.contains(&alias)
                    })
            })
            .collect::<Vec<_>>();
        matches.sort_by(|left, right| left.id.cmp(&right.id));
        matches.dedup_by(|left, right| left.id == right.id);
        matches
    }

    pub fn active_definitions<'a>(
        &'a self,
        active: impl IntoIterator<Item = &'a str>,
    ) -> Vec<&'a StateFieldDefinition> {
        let active = active.into_iter().collect::<HashSet<_>>();
        self.fields
            .iter()
            .filter(|field| active.contains(field.id.as_str()))
            .collect()
    }
}

fn vocabulary_kind_from_str(value: &str) -> Result<VocabularyKind, String> {
    match value {
        "objective" => Ok(VocabularyKind::Objective),
        "constraint" => Ok(VocabularyKind::Constraint),
        "assumption" => Ok(VocabularyKind::Assumption),
        "method" => Ok(VocabularyKind::Method),
        "parameter" => Ok(VocabularyKind::Parameter),
        _ => Err("STATE_VOCABULARY_STORAGE_INVALID_KIND".to_string()),
    }
}

fn bounded_trimmed(
    values: Vec<String>,
    maximum_items: usize,
    maximum_chars: usize,
    error_code: &str,
) -> Result<Vec<String>, String> {
    if values.len() > maximum_items {
        return Err(error_code.to_string());
    }
    let mut seen = HashSet::new();
    let mut output = Vec::new();
    for value in values {
        let value = value.trim().to_string();
        if value.is_empty() || value.chars().count() > maximum_chars {
            return Err(error_code.to_string());
        }
        if seen.insert(value.to_lowercase()) {
            output.push(value);
        }
    }
    Ok(output)
}

fn normalize_input(mut input: CustomStateFieldInput) -> Result<CustomStateFieldInput, String> {
    input.label = input.label.trim().to_string();
    input.description = input.description.trim().to_string();
    if !(2..=80).contains(&input.label.chars().count()) {
        return Err("STATE_VOCABULARY_LABEL_INVALID".to_string());
    }
    if !(5..=500).contains(&input.description.chars().count()) {
        return Err("STATE_VOCABULARY_DESCRIPTION_INVALID".to_string());
    }
    input.aliases = bounded_trimmed(input.aliases, 16, 120, "STATE_VOCABULARY_ALIASES_INVALID")?;
    input.examples = bounded_trimmed(input.examples, 8, 200, "STATE_VOCABULARY_EXAMPLES_INVALID")?;
    match (input.kind, &mut input.parameter_spec) {
        (VocabularyKind::Parameter, Some(spec)) => {
            spec.unit = spec
                .unit
                .take()
                .map(|value| value.trim().chars().take(32).collect::<String>())
                .filter(|value| !value.is_empty());
            spec.enum_values = bounded_trimmed(
                std::mem::take(&mut spec.enum_values),
                32,
                120,
                "STATE_VOCABULARY_ENUM_INVALID",
            )?;
            if spec
                .minimum
                .zip(spec.maximum)
                .is_some_and(|(min, max)| min > max)
                || (spec.value_kind == ParameterValueKind::Enum && spec.enum_values.is_empty())
            {
                return Err("STATE_VOCABULARY_PARAMETER_SPEC_INVALID".to_string());
            }
        }
        (VocabularyKind::Parameter, None) => {
            return Err("STATE_VOCABULARY_PARAMETER_SPEC_REQUIRED".to_string())
        }
        (_, Some(_)) => return Err("STATE_VOCABULARY_PARAMETER_SPEC_UNEXPECTED".to_string()),
        (_, None) => {}
    }
    Ok(input)
}

fn current_revision(connection: &Connection, repository_id: &str) -> Result<u64, String> {
    connection
        .query_row(
            "SELECT revision FROM qa_state_vocabulary_meta WHERE repository_id=?1",
            params![repository_id],
            |row| row.get::<_, i64>(0),
        )
        .optional()
        .map_err(|error| format!("STATE_VOCABULARY_STORAGE_READ_FAILED: {error}"))
        .map(|revision| revision.unwrap_or(0).max(0) as u64)
}

fn row_to_definition(
    field_id: String,
    kind: String,
    label: String,
    description: String,
    aliases_json: String,
    examples_json: String,
    parameter_spec_json: String,
    enabled: bool,
) -> Result<StateFieldDefinition, String> {
    let kind = vocabulary_kind_from_str(&kind)?;
    let aliases = serde_json::from_str::<Vec<String>>(&aliases_json)
        .map_err(|error| format!("STATE_VOCABULARY_STORAGE_INVALID_ALIASES: {error}"))?;
    let examples = serde_json::from_str::<Vec<String>>(&examples_json)
        .map_err(|error| format!("STATE_VOCABULARY_STORAGE_INVALID_EXAMPLES: {error}"))?;
    let parameter_spec = if parameter_spec_json.trim().is_empty()
        || parameter_spec_json.trim() == "{}"
        || parameter_spec_json.trim() == "null"
    {
        None
    } else {
        Some(
            serde_json::from_str::<ParameterSpec>(&parameter_spec_json).map_err(|error| {
                format!("STATE_VOCABULARY_STORAGE_INVALID_PARAMETER_SPEC: {error}")
            })?,
        )
    };
    Ok(StateFieldDefinition {
        id: field_id,
        kind,
        label,
        description,
        aliases,
        examples,
        parameter_spec,
        origin: VocabularyOrigin::Custom,
        enabled,
    })
}

fn load_custom_fields(
    connection: &Connection,
    repository_id: &str,
) -> Result<Vec<StateFieldDefinition>, String> {
    let mut statement = connection
        .prepare(
            "SELECT field_id,kind,label,description,aliases_json,examples_json,parameter_spec_json,enabled
             FROM qa_state_vocabulary_fields WHERE repository_id=?1 ORDER BY field_id",
        )
        .map_err(|error| format!("STATE_VOCABULARY_STORAGE_READ_FAILED: {error}"))?;
    let rows = statement
        .query_map(params![repository_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, String>(6)?,
                row.get::<_, bool>(7)?,
            ))
        })
        .map_err(|error| format!("STATE_VOCABULARY_STORAGE_READ_FAILED: {error}"))?;
    let mut fields = Vec::new();
    for row in rows {
        let row = row.map_err(|error| format!("STATE_VOCABULARY_STORAGE_READ_FAILED: {error}"))?;
        fields.push(row_to_definition(
            row.0, row.1, row.2, row.3, row.4, row.5, row.6, row.7,
        )?);
    }
    Ok(fields)
}

pub fn load_state_vocabulary(
    connection: &Connection,
    repository_id: &str,
) -> Result<StateVocabularyRegistry, String> {
    let revision = current_revision(connection, repository_id)?;
    let cache_key = (repository_id.to_string(), revision);
    if let Ok(cache) = registry_cache().lock() {
        if let Some(registry) = cache.get(&cache_key) {
            return Ok((**registry).clone());
        }
    }
    let registry =
        StateVocabularyRegistry::merged(revision, load_custom_fields(connection, repository_id)?);
    if let Ok(mut cache) = registry_cache().lock() {
        cache.insert(cache_key, Arc::new(registry.clone()));
    }
    log::info!(
        "feature=state_vocabulary stage=load_success operation_id={} revision={} custom_active_count={}",
        vocabulary_operation_id(repository_id),
        registry.revision,
        registry.enabled_custom_count()
    );
    Ok(registry)
}

pub fn list_state_vocabulary(
    connection: &Connection,
    repository_id: &str,
) -> Result<StateVocabularyRegistry, String> {
    load_state_vocabulary(connection, repository_id)
}

fn vocabulary_operation_id(repository_id: &str) -> String {
    let digest = Sha256::digest(repository_id.as_bytes());
    format!("{:x}", digest)[..16].to_string()
}

fn invalidate_registry_cache(repository_id: &str) {
    if let Ok(mut cache) = registry_cache().lock() {
        cache.retain(|(cached_repository, _), _| cached_repository != repository_id);
    }
}

fn field_is_referenced(
    transaction: &Transaction<'_>,
    repository_id: &str,
    field_id: &str,
) -> Result<bool, String> {
    transaction
        .query_row(
            "SELECT EXISTS(
               SELECT 1 FROM qa_session_research_state
               WHERE repository_id=?1 AND instr(state_json, ?2) > 0
               UNION ALL
               SELECT 1 FROM qa_message_state_patches
               WHERE repository_id=?1 AND instr(patch_json, ?2) > 0
             )",
            params![repository_id, field_id],
            |row| row.get::<_, bool>(0),
        )
        .map_err(|error| format!("STATE_VOCABULARY_REFERENCE_CHECK_FAILED: {error}"))
}

fn validate_alias_conflicts(
    registry: &StateVocabularyRegistry,
    input: &CustomStateFieldInput,
    excluded_id: Option<&str>,
) -> Result<(), String> {
    let candidate_terms = std::iter::once(&input.label)
        .chain(input.aliases.iter())
        .map(|value| value.trim().to_lowercase())
        .collect::<HashSet<_>>();
    let collision = registry.fields.iter().any(|field| {
        field.enabled
            && field.kind == input.kind
            && excluded_id != Some(field.id.as_str())
            && std::iter::once(&field.label)
                .chain(field.aliases.iter())
                .map(|value| value.trim().to_lowercase())
                .any(|value| candidate_terms.contains(&value))
    });
    if collision {
        Err("STATE_VOCABULARY_ALIAS_CONFLICT".to_string())
    } else {
        Ok(())
    }
}

fn bump_revision(transaction: &Transaction<'_>, repository_id: &str) -> Result<u64, String> {
    let now = super::now_string();
    transaction
        .execute(
            "INSERT INTO qa_state_vocabulary_meta(repository_id,revision,updated_at)
             VALUES(?1,1,?2)
             ON CONFLICT(repository_id) DO UPDATE SET
               revision=revision+1, updated_at=excluded.updated_at",
            params![repository_id, now],
        )
        .map_err(|error| format!("STATE_VOCABULARY_REVISION_WRITE_FAILED: {error}"))?;
    transaction
        .query_row(
            "SELECT revision FROM qa_state_vocabulary_meta WHERE repository_id=?1",
            params![repository_id],
            |row| row.get::<_, i64>(0),
        )
        .map(|value| value.max(0) as u64)
        .map_err(|error| format!("STATE_VOCABULARY_REVISION_READ_FAILED: {error}"))
}

fn custom_definition(field_id: String, input: &CustomStateFieldInput) -> StateFieldDefinition {
    StateFieldDefinition {
        id: field_id,
        kind: input.kind,
        label: input.label.clone(),
        description: input.description.clone(),
        aliases: input.aliases.clone(),
        examples: input.examples.clone(),
        parameter_spec: input.parameter_spec.clone(),
        origin: VocabularyOrigin::Custom,
        enabled: true,
    }
}

pub fn create_custom_state_field(
    connection: &Connection,
    repository_id: &str,
    input: CustomStateFieldInput,
) -> Result<StateFieldDefinition, String> {
    let operation_id = Uuid::new_v4().to_string();
    log::info!("feature=state_vocabulary_crud stage=create_start operation_id={operation_id}");
    let input = normalize_input(input).map_err(|error| {
        log::error!("feature=state_vocabulary_crud stage=create_failed operation_id={operation_id} error_code={error}");
        error
    })?;
    let registry = load_state_vocabulary(connection, repository_id)?;
    validate_alias_conflicts(&registry, &input, None)?;
    if registry.enabled_custom_count() >= 64 {
        return Err("STATE_VOCABULARY_CUSTOM_LIMIT_REACHED".to_string());
    }
    let field_id = format!(
        "custom:{}:{}",
        input.kind.as_str(),
        Uuid::new_v4().hyphenated()
    );
    let now = super::now_string();
    let transaction = connection
        .unchecked_transaction()
        .map_err(|error| format!("STATE_VOCABULARY_TRANSACTION_FAILED: {error}"))?;
    transaction
        .execute(
            "INSERT INTO qa_state_vocabulary_fields(
               repository_id,field_id,kind,label,description,aliases_json,examples_json,
               parameter_spec_json,enabled,created_at,updated_at
             ) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,1,?9,?9)",
            params![
                repository_id,
                field_id,
                input.kind.as_str(),
                input.label,
                input.description,
                serde_json::to_string(&input.aliases).unwrap_or_else(|_| "[]".to_string()),
                serde_json::to_string(&input.examples).unwrap_or_else(|_| "[]".to_string()),
                input
                    .parameter_spec
                    .as_ref()
                    .and_then(|spec| serde_json::to_string(spec).ok())
                    .unwrap_or_else(|| "{}".to_string()),
                now,
            ],
        )
        .map_err(|error| format!("STATE_VOCABULARY_CREATE_FAILED: {error}"))?;
    let revision = bump_revision(&transaction, repository_id)?;
    transaction
        .commit()
        .map_err(|error| format!("STATE_VOCABULARY_COMMIT_FAILED: {error}"))?;
    invalidate_registry_cache(repository_id);
    log::info!("feature=state_vocabulary_crud stage=create_complete operation_id={operation_id} revision={revision}");
    Ok(custom_definition(field_id, &input))
}

pub fn update_custom_state_field(
    connection: &Connection,
    repository_id: &str,
    field_id: &str,
    input: CustomStateFieldInput,
) -> Result<StateFieldDefinition, String> {
    let operation_id = Uuid::new_v4().to_string();
    log::info!("feature=state_vocabulary_crud stage=update_start operation_id={operation_id}");
    let input = normalize_input(input)?;
    let registry = load_state_vocabulary(connection, repository_id)?;
    let existing = registry
        .field(field_id)
        .filter(|field| field.origin == VocabularyOrigin::Custom)
        .ok_or_else(|| "STATE_VOCABULARY_FIELD_NOT_FOUND".to_string())?;
    if existing.kind != input.kind {
        return Err("STATE_VOCABULARY_KIND_IMMUTABLE".to_string());
    }
    validate_alias_conflicts(&registry, &input, Some(field_id))?;
    let transaction = connection
        .unchecked_transaction()
        .map_err(|error| format!("STATE_VOCABULARY_TRANSACTION_FAILED: {error}"))?;
    if field_is_referenced(&transaction, repository_id, field_id)?
        && existing.parameter_spec.as_ref().map(|spec| spec.value_kind)
            != input.parameter_spec.as_ref().map(|spec| spec.value_kind)
    {
        return Err("STATE_VOCABULARY_VALUE_TYPE_IMMUTABLE".to_string());
    }
    let now = super::now_string();
    transaction
        .execute(
            "UPDATE qa_state_vocabulary_fields SET label=?3,description=?4,aliases_json=?5,
               examples_json=?6,parameter_spec_json=?7,updated_at=?8
             WHERE repository_id=?1 AND field_id=?2",
            params![
                repository_id,
                field_id,
                input.label,
                input.description,
                serde_json::to_string(&input.aliases).unwrap_or_else(|_| "[]".to_string()),
                serde_json::to_string(&input.examples).unwrap_or_else(|_| "[]".to_string()),
                input
                    .parameter_spec
                    .as_ref()
                    .and_then(|spec| serde_json::to_string(spec).ok())
                    .unwrap_or_else(|| "{}".to_string()),
                now,
            ],
        )
        .map_err(|error| format!("STATE_VOCABULARY_UPDATE_FAILED: {error}"))?;
    let revision = bump_revision(&transaction, repository_id)?;
    transaction
        .commit()
        .map_err(|error| format!("STATE_VOCABULARY_COMMIT_FAILED: {error}"))?;
    invalidate_registry_cache(repository_id);
    log::info!("feature=state_vocabulary_crud stage=update_complete operation_id={operation_id} revision={revision}");
    let mut output = custom_definition(field_id.to_string(), &input);
    output.enabled = existing.enabled;
    Ok(output)
}

pub fn set_custom_state_field_enabled(
    connection: &Connection,
    repository_id: &str,
    field_id: &str,
    enabled: bool,
) -> Result<StateFieldDefinition, String> {
    let operation_id = Uuid::new_v4().to_string();
    log::info!("feature=state_vocabulary_crud stage=enable_start operation_id={operation_id} enabled={enabled}");
    let registry = load_state_vocabulary(connection, repository_id)?;
    let existing = registry
        .field(field_id)
        .filter(|field| field.origin == VocabularyOrigin::Custom)
        .ok_or_else(|| "STATE_VOCABULARY_FIELD_NOT_FOUND".to_string())?;
    if enabled && !existing.enabled && registry.enabled_custom_count() >= 64 {
        return Err("STATE_VOCABULARY_CUSTOM_LIMIT_REACHED".to_string());
    }
    if enabled {
        validate_alias_conflicts(
            &registry,
            &CustomStateFieldInput {
                kind: existing.kind,
                label: existing.label.clone(),
                description: existing.description.clone(),
                aliases: existing.aliases.clone(),
                examples: existing.examples.clone(),
                parameter_spec: existing.parameter_spec.clone(),
            },
            Some(field_id),
        )?;
    }
    let transaction = connection
        .unchecked_transaction()
        .map_err(|error| format!("STATE_VOCABULARY_TRANSACTION_FAILED: {error}"))?;
    transaction
        .execute(
            "UPDATE qa_state_vocabulary_fields SET enabled=?3,updated_at=?4
             WHERE repository_id=?1 AND field_id=?2",
            params![repository_id, field_id, enabled, super::now_string()],
        )
        .map_err(|error| format!("STATE_VOCABULARY_ENABLE_FAILED: {error}"))?;
    let revision = bump_revision(&transaction, repository_id)?;
    transaction
        .commit()
        .map_err(|error| format!("STATE_VOCABULARY_COMMIT_FAILED: {error}"))?;
    invalidate_registry_cache(repository_id);
    let mut output = existing.clone();
    output.enabled = enabled;
    log::info!("feature=state_vocabulary_crud stage=enable_complete operation_id={operation_id} revision={revision} enabled={enabled}");
    Ok(output)
}

pub fn delete_custom_state_field(
    connection: &Connection,
    repository_id: &str,
    field_id: &str,
) -> Result<(), String> {
    let operation_id = Uuid::new_v4().to_string();
    log::info!("feature=state_vocabulary_crud stage=delete_start operation_id={operation_id}");
    let transaction = connection
        .unchecked_transaction()
        .map_err(|error| format!("STATE_VOCABULARY_TRANSACTION_FAILED: {error}"))?;
    if field_is_referenced(&transaction, repository_id, field_id)? {
        log::error!("feature=state_vocabulary_crud stage=delete_failed operation_id={operation_id} error_code=STATE_VOCABULARY_FIELD_REFERENCED");
        return Err("STATE_VOCABULARY_FIELD_REFERENCED".to_string());
    }
    let affected = transaction
        .execute(
            "DELETE FROM qa_state_vocabulary_fields WHERE repository_id=?1 AND field_id=?2",
            params![repository_id, field_id],
        )
        .map_err(|error| format!("STATE_VOCABULARY_DELETE_FAILED: {error}"))?;
    if affected == 0 {
        return Err("STATE_VOCABULARY_FIELD_NOT_FOUND".to_string());
    }
    let revision = bump_revision(&transaction, repository_id)?;
    transaction
        .commit()
        .map_err(|error| format!("STATE_VOCABULARY_COMMIT_FAILED: {error}"))?;
    invalidate_registry_cache(repository_id);
    log::info!("feature=state_vocabulary_crud stage=delete_complete operation_id={operation_id} revision={revision}");
    Ok(())
}

fn validate_parameter_value(
    value: &ParameterValue,
    spec: &ParameterSpec,
) -> Result<(), &'static str> {
    let type_matches = matches!(
        (spec.value_kind, value),
        (ParameterValueKind::Integer, ParameterValue::Integer(_))
            | (ParameterValueKind::Float, ParameterValue::Integer(_))
            | (ParameterValueKind::Float, ParameterValue::Float(_))
            | (ParameterValueKind::Boolean, ParameterValue::Boolean(_))
            | (ParameterValueKind::Text, ParameterValue::Text(_))
            | (ParameterValueKind::Enum, ParameterValue::Text(_))
    );
    if !type_matches {
        return Err("STATE_VOCABULARY_VALUE_TYPE_MISMATCH");
    }
    let numeric = match value {
        ParameterValue::Integer(value) => Some(*value as f64),
        ParameterValue::Float(value) => Some(*value),
        _ => None,
    };
    if numeric.is_some_and(|value| {
        spec.minimum.is_some_and(|minimum| value < minimum)
            || spec.maximum.is_some_and(|maximum| value > maximum)
    }) {
        return Err("STATE_VOCABULARY_VALUE_OUT_OF_RANGE");
    }
    if spec.value_kind == ParameterValueKind::Enum {
        let ParameterValue::Text(value) = value else {
            return Err("STATE_VOCABULARY_VALUE_TYPE_MISMATCH");
        };
        if !spec.enum_values.iter().any(|allowed| allowed == value) {
            return Err("STATE_VOCABULARY_VALUE_OUT_OF_RANGE");
        }
    }
    Ok(())
}

pub fn validate_patch_against_vocabulary(
    patch: &ResearchStatePatch,
    registry: &StateVocabularyRegistry,
) -> Result<VocabularyValidationStats, String> {
    let mut stats = VocabularyValidationStats::default();
    for operation in &patch.operations {
        let expected_kind = VocabularyKind::from_state_field(operation.field);
        let ids = match &operation.value {
            Some(StateValue::Text { value }) => vec![value.as_str()],
            Some(StateValue::TextList { values }) => values.iter().map(String::as_str).collect(),
            Some(StateValue::Parameter { parameter }) => vec![parameter.key.as_str()],
            None if operation.action == StateAction::Clear => continue,
            None => Vec::new(),
        };
        for id in ids {
            // Legacy ephemeral parameter keys remain readable and writable for
            // backward compatibility, but are never exposed to the model.
            if expected_kind == VocabularyKind::Parameter
                && id.starts_with("custom:")
                && !id.starts_with("custom:parameter:")
            {
                continue;
            }
            let Some(definition) = registry.field(id) else {
                stats.unknown_id_count += 1;
                return vocabulary_validation_failure(stats, "STATE_VOCABULARY_UNKNOWN_ID");
            };
            if definition.kind != expected_kind {
                stats.kind_mismatch_count += 1;
                return vocabulary_validation_failure(stats, "STATE_VOCABULARY_KIND_MISMATCH");
            }
            let removal = matches!(operation.action, StateAction::Remove | StateAction::Clear);
            if !definition.enabled && !removal {
                stats.disabled_field_count += 1;
                return vocabulary_validation_failure(stats, "STATE_VOCABULARY_FIELD_DISABLED");
            }
            if let Some(StateValue::Parameter { parameter }) = &operation.value {
                let Some(spec) = &definition.parameter_spec else {
                    stats.kind_mismatch_count += 1;
                    return vocabulary_validation_failure(stats, "STATE_VOCABULARY_KIND_MISMATCH");
                };
                if let Some(required_unit) = &spec.unit {
                    if parameter
                        .unit
                        .as_deref()
                        .is_some_and(|unit| unit != required_unit)
                    {
                        stats.value_type_mismatch_count += 1;
                        return vocabulary_validation_failure(
                            stats,
                            "STATE_VOCABULARY_VALUE_TYPE_MISMATCH",
                        );
                    }
                }
                if let Err(error) = validate_parameter_value(&parameter.value, spec) {
                    if error.ends_with("OUT_OF_RANGE") {
                        stats.value_out_of_range_count += 1;
                    } else {
                        stats.value_type_mismatch_count += 1;
                    }
                    return vocabulary_validation_failure(stats, error);
                }
            }
        }
    }
    Ok(stats)
}

fn vocabulary_validation_failure(
    stats: VocabularyValidationStats,
    error_code: &str,
) -> Result<VocabularyValidationStats, String> {
    log::error!(
        "feature=state_vocabulary_validation stage=failed operation_id=validation unknown_id_count={} kind_mismatch_count={} disabled_field_count={} value_type_mismatch_count={} value_out_of_range_count={} error_code={error_code}",
        stats.unknown_id_count,
        stats.kind_mismatch_count,
        stats.disabled_field_count,
        stats.value_type_mismatch_count,
        stats.value_out_of_range_count,
    );
    Err(error_code.to_string())
}

pub fn active_ids_from_state(
    objectives: &[String],
    constraints: &[String],
    assumptions: &[String],
    methods: &[String],
    parameters: &BTreeMap<String, super::state_mutation::ResearchParameter>,
) -> Vec<String> {
    let mut values = objectives
        .iter()
        .chain(constraints)
        .chain(assumptions)
        .chain(methods)
        .cloned()
        .collect::<Vec<_>>();
    values.extend(parameters.keys().cloned());
    values.sort();
    values.dedup();
    values
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::qa::state_mutation::{
        PatchConfidence, ResearchParameter, ResearchStateOperation, STATE_PATCH_VERSION,
    };

    fn parameter_patch(key: &str, value: ParameterValue) -> ResearchStatePatch {
        ResearchStatePatch {
            schema_version: STATE_PATCH_VERSION.to_string(),
            patch_id: "fixture".to_string(),
            operations: vec![ResearchStateOperation {
                action: StateAction::Set,
                field: StateField::Parameter,
                value: Some(StateValue::Parameter {
                    parameter: ResearchParameter {
                        key: key.to_string(),
                        value,
                        unit: None,
                        source_message_id: None,
                        updated_at_turn: 0,
                    },
                }),
                previous_value: None,
                confidence: PatchConfidence::High,
            }],
            confidence: PatchConfidence::High,
            source_message_id: None,
            parameter_implicit_reference_resolved_count: 0,
            parameter_implicit_reference_rejected_count: 0,
            parameter_unknown_name_count: 0,
            parameter_state_corruption_count: 0,
        }
    }

    fn custom_constraint(label: &str, alias: &str) -> CustomStateFieldInput {
        CustomStateFieldInput {
            kind: VocabularyKind::Constraint,
            label: label.to_string(),
            description: "用于仓库级自定义约束测试的稳定描述。".to_string(),
            aliases: vec![alias.to_string()],
            examples: vec![format!("需要考虑{alias}")],
            parameter_spec: None,
        }
    }

    fn storage() -> Connection {
        let connection = Connection::open_in_memory().unwrap();
        crate::qa::db_schema(&connection).unwrap();
        connection
    }

    #[test]
    fn builtin_contract_distinguishes_coordination_count_and_capacity() {
        let registry = StateVocabularyRegistry::default();
        assert_eq!(
            registry
                .exact_matches("需要多套移动供能设备共同调度", VocabularyKind::Constraint)
                .iter()
                .map(|field| field.id.as_str())
                .collect::<Vec<_>>(),
            ["multi_vehicle_coordination"]
        );
        assert_eq!(
            registry
                .exact_matches("充电器容量=50", VocabularyKind::Parameter)
                .iter()
                .map(|field| field.id.as_str())
                .collect::<Vec<_>>(),
            ["charger_capacity"]
        );
        assert!(registry
            .exact_matches("充电器容量=50", VocabularyKind::Parameter)
            .iter()
            .all(|field| field.id != "mobile_charger_count"));
    }

    #[test]
    fn validator_rejects_hallucinated_wrong_kind_and_out_of_range_fields() {
        let registry = StateVocabularyRegistry::default();
        assert_eq!(
            validate_patch_against_vocabulary(
                &parameter_patch("invented_parameter", ParameterValue::Integer(1)),
                &registry
            ),
            Err("STATE_VOCABULARY_UNKNOWN_ID".to_string())
        );

        let mut wrong_kind =
            parameter_patch("multi_vehicle_coordination", ParameterValue::Integer(2));
        assert_eq!(
            validate_patch_against_vocabulary(&wrong_kind, &registry),
            Err("STATE_VOCABULARY_KIND_MISMATCH".to_string())
        );

        wrong_kind = parameter_patch("mobile_charger_count", ParameterValue::Integer(0));
        assert_eq!(
            validate_patch_against_vocabulary(&wrong_kind, &registry),
            Err("STATE_VOCABULARY_VALUE_OUT_OF_RANGE".to_string())
        );
    }

    #[test]
    fn custom_crud_bumps_revision_preserves_id_and_is_repository_scoped() {
        let connection = storage();
        let created = create_custom_state_field(
            &connection,
            "repo-a",
            custom_constraint("高温环境约束", "高温环境"),
        )
        .unwrap();
        assert!(created.id.starts_with("custom:constraint:"));
        assert_eq!(
            load_state_vocabulary(&connection, "repo-a")
                .unwrap()
                .revision,
            1
        );
        assert_eq!(
            load_state_vocabulary(&connection, "repo-b")
                .unwrap()
                .revision,
            0
        );
        assert!(load_state_vocabulary(&connection, "repo-b")
            .unwrap()
            .field(&created.id)
            .is_none());

        let updated = update_custom_state_field(
            &connection,
            "repo-a",
            &created.id,
            custom_constraint("极端高温约束", "温度过高"),
        )
        .unwrap();
        assert_eq!(updated.id, created.id);
        assert_eq!(updated.label, "极端高温约束");
        assert_eq!(
            load_state_vocabulary(&connection, "repo-a")
                .unwrap()
                .revision,
            2
        );

        let disabled =
            set_custom_state_field_enabled(&connection, "repo-a", &created.id, false).unwrap();
        assert!(!disabled.enabled);
        let enabled =
            set_custom_state_field_enabled(&connection, "repo-a", &created.id, true).unwrap();
        assert!(enabled.enabled);
        delete_custom_state_field(&connection, "repo-a", &created.id).unwrap();
        assert!(load_state_vocabulary(&connection, "repo-a")
            .unwrap()
            .field(&created.id)
            .is_none());
    }

    #[test]
    fn same_kind_alias_collision_and_referenced_delete_are_blocked() {
        let connection = storage();
        let created = create_custom_state_field(
            &connection,
            "repo-c",
            custom_constraint("高温环境约束", "高温环境"),
        )
        .unwrap();
        assert_eq!(
            create_custom_state_field(
                &connection,
                "repo-c",
                custom_constraint("温控安全约束", "高温环境"),
            ),
            Err("STATE_VOCABULARY_ALIAS_CONFLICT".to_string())
        );
        connection
            .execute(
                "INSERT INTO qa_session_research_state(
                   session_id,repository_id,state_schema_version,vocabulary_revision,state_json,
                   last_source_message_id,updated_at
                 ) VALUES('s1','repo-c','v',1,?1,'m1','1')",
                params![format!("{{\"constraints\":[\"{}\"]}}", created.id)],
            )
            .unwrap();
        assert_eq!(
            delete_custom_state_field(&connection, "repo-c", &created.id),
            Err("STATE_VOCABULARY_FIELD_REFERENCED".to_string())
        );
    }
}
