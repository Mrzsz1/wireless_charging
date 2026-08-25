use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub const PROBLEM_UNDERSTANDING_VERSION: &str = "problem-understanding-v1";
pub const METHOD_MATCHER_VERSION: &str = "method-matcher-v2";

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProblemRepresentation {
    pub domain: String,
    pub objectives: Vec<String>,
    pub constraints: Vec<String>,
    pub assumptions: Vec<String>,
    pub decision_variables: Vec<String>,
    pub related_problem_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MethodMatch {
    pub method: String,
    pub rationale: String,
    pub required_conditions: Vec<String>,
    pub source: String,
    pub corroborated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProblemUnderstandingResult {
    pub parser_version: String,
    pub matcher_version: String,
    pub status: String,
    pub representation: ProblemRepresentation,
    pub candidate_methods: Vec<MethodMatch>,
    pub search_terms: Vec<String>,
}

pub trait ProblemParser {
    fn parse(&self, question: &str) -> Result<ProblemRepresentation, String>;
}

pub trait MethodMatcher {
    fn match_methods(&self, problem: &ProblemRepresentation) -> Vec<MethodMatch>;
}

#[derive(Debug, Default)]
pub struct DeterministicProblemParser;

#[derive(Debug, Default)]
pub struct DeterministicMethodMatcher;

impl ProblemParser for DeterministicProblemParser {
    fn parse(&self, question: &str) -> Result<ProblemRepresentation, String> {
        let value = question.to_lowercase();
        let mut problem = ProblemRepresentation {
            domain: classify_domain(&value).to_string(),
            ..ProblemRepresentation::default()
        };
        collect_rules(&value, OBJECTIVE_RULES, &mut problem.objectives);
        collect_rules(&value, CONSTRAINT_RULES, &mut problem.constraints);
        collect_rules(&value, ASSUMPTION_RULES, &mut problem.assumptions);
        collect_rules(&value, VARIABLE_RULES, &mut problem.decision_variables);
        collect_rules(
            &value,
            RELATED_PROBLEM_RULES,
            &mut problem.related_problem_types,
        );
        if problem.domain == "wireless_sensor_network"
            && contains_any(&value, &["移动充电", "mobile charger", "充电车"])
        {
            push_unique(&mut problem.related_problem_types, "mobile_charger_routing");
            push_unique(
                &mut problem.related_problem_types,
                "team_orienteering_problem",
            );
        }
        if contains_any(&value, &["多个", "多辆", "multiple", "multi-charger"])
            && contains_any(&value, &["充电车", "移动充电", "charger"])
        {
            push_unique(&mut problem.related_problem_types, "multi_depot_vrp");
            push_unique(&mut problem.related_problem_types, "multiple_tsp");
            push_unique(&mut problem.constraints, "multi_vehicle_coordination");
        }
        Ok(problem)
    }
}

impl MethodMatcher for DeterministicMethodMatcher {
    fn match_methods(&self, problem: &ProblemRepresentation) -> Vec<MethodMatch> {
        let mut methods = Vec::new();
        let related = problem.related_problem_types.join(" ");
        if related.contains("vrptw")
            || problem
                .constraints
                .iter()
                .any(|value| value == "time_windows")
        {
            add_method(
                &mut methods,
                "adaptive_large_neighborhood_search",
                "适合带时间窗的路径与调度联合优化",
                &["feasible_repair_operator"],
            );
            add_method(
                &mut methods,
                "column_generation",
                "可按路径变量分解大规模 VRPTW",
                &["pricing_subproblem"],
            );
        }
        if related.contains("facility_location") || related.contains("set_cover") {
            add_method(
                &mut methods,
                "mixed_integer_linear_programming",
                "可表达站点选择、覆盖与容量约束",
                &["linearizable_constraints"],
            );
            add_method(
                &mut methods,
                "greedy_approximation",
                "适合覆盖型目标并可提供可解释基线",
                &["submodular_or_cover_structure"],
            );
        }
        if related.contains("tsp")
            || related.contains("routing")
            || related.contains("orienteering")
        {
            add_method(
                &mut methods,
                "branch_and_cut",
                "适合中小规模精确路径优化与最优性基线",
                &["integer_route_model"],
            );
            add_method(
                &mut methods,
                "particle_swarm_optimization",
                "适合非线性移动充电调度的启发式搜索",
                &["solution_encoding", "constraint_repair"],
            );
        }
        if related.contains("multi_depot") || related.contains("multiple_tsp") {
            add_method(
                &mut methods,
                "decomposition_matheuristic",
                "分解车辆分配与单车路径可控制组合规模",
                &["decomposable_assignment"],
            );
            add_method(
                &mut methods,
                "multi_agent_reinforcement_learning",
                "适合动态多充电车协同策略",
                &["simulator", "stationary_training_distribution"],
            );
        }
        if problem
            .objectives
            .iter()
            .any(|value| value == "maximize_network_lifetime")
        {
            add_method(
                &mut methods,
                "lyapunov_optimization",
                "适合在线能量队列稳定与长期寿命目标",
                &["online_state_observation"],
            );
        }
        if methods.is_empty() && problem.domain != "unknown" {
            add_method(
                &mut methods,
                "mixed_integer_linear_programming",
                "先建立可审计精确基线并识别结构",
                &["finite_decision_space"],
            );
            add_method(
                &mut methods,
                "genetic_algorithm",
                "为大规模组合问题提供稳健启发式基线",
                &["solution_encoding"],
            );
        }
        methods.truncate(8);
        methods
    }
}

pub fn understand(question: &str) -> ProblemUnderstandingResult {
    let parser = DeterministicProblemParser;
    let matcher = DeterministicMethodMatcher;
    match parser.parse(question) {
        Ok(representation) => {
            let candidate_methods = matcher.match_methods(&representation);
            let mut search_terms = representation.related_problem_types.clone();
            search_terms.extend(representation.objectives.iter().cloned());
            deduplicate(&mut search_terms);
            ProblemUnderstandingResult {
                parser_version: PROBLEM_UNDERSTANDING_VERSION.to_string(),
                matcher_version: METHOD_MATCHER_VERSION.to_string(),
                status: "succeeded".to_string(),
                representation,
                candidate_methods,
                search_terms,
            }
        }
        Err(_) => ProblemUnderstandingResult {
            parser_version: PROBLEM_UNDERSTANDING_VERSION.to_string(),
            matcher_version: METHOD_MATCHER_VERSION.to_string(),
            status: "failed_fallback_empty".to_string(),
            ..ProblemUnderstandingResult::default()
        },
    }
}

type Rules = &'static [(&'static [&'static str], &'static str)];

const OBJECTIVE_RULES: Rules = &[
    (
        &["网络寿命", "network lifetime"],
        "maximize_network_lifetime",
    ),
    (&["死亡节点", "dead node"], "minimize_dead_nodes"),
    (
        &["总路程", "行驶距离", "travel distance"],
        "minimize_travel_distance",
    ),
    (
        &["能耗", "energy consumption"],
        "minimize_energy_consumption",
    ),
    (&["覆盖", "coverage"], "maximize_coverage"),
    (&["吞吐", "throughput"], "maximize_throughput"),
    (&["延迟", "latency", "时延"], "minimize_latency"),
];

const CONSTRAINT_RULES: Rules = &[
    (&["时间窗", "time window"], "time_windows"),
    (&["电池容量", "battery capacity"], "battery_capacity"),
    (&["充电容量", "charger capacity"], "charger_capacity"),
    (&["障碍", "obstacle"], "obstacle_avoidance"),
    (&["截止时间", "deadline"], "deadlines"),
    (&["能量守恒", "energy balance"], "energy_balance"),
    (&["服务时间", "service time"], "service_duration"),
    (&["优先级", "priority"], "service_priority"),
];

const ASSUMPTION_RULES: Rules = &[
    (
        &["静态节点", "节点静态", "stationary node"],
        "stationary_sensor_nodes",
    ),
    (&["能量已知", "known energy"], "observable_energy_state"),
    (&["确定性", "deterministic"], "deterministic_demand"),
    (&["周期性", "periodic"], "periodic_operation"),
];

const VARIABLE_RULES: Rules = &[
    (&["路径", "route", "trajectory"], "charger_route"),
    (&["充电时间", "charging time"], "charging_duration"),
    (&["访问顺序", "visit order"], "visit_order"),
    (&["站点", "station", "location"], "facility_selection"),
    (&["功率", "power"], "charging_power"),
    (&["车辆分配", "vehicle assignment"], "vehicle_assignment"),
];

const RELATED_PROBLEM_RULES: Rules = &[
    (&["时间窗", "time window", "vrptw"], "vrptw"),
    (&["旅行商", "tsp", "巡回"], "traveling_salesperson_problem"),
    (
        &["选址", "facility", "站点部署", "充电站"],
        "facility_location",
    ),
    (&["覆盖", "set cover"], "set_cover"),
    (&["背包", "knapsack", "预算选择"], "knapsack_problem"),
    (
        &["轨迹", "路径", "route", "trajectory"],
        "vehicle_routing_problem",
    ),
    (
        &["排程", "调度", "scheduling"],
        "resource_constrained_scheduling",
    ),
    (&["博弈", "game"], "algorithmic_game"),
];

fn classify_domain(value: &str) -> &'static str {
    if contains_any(
        value,
        &["无线传感器", "wrsn", "sensor network", "移动充电", "充电车"],
    ) {
        "wireless_sensor_network"
    } else if contains_any(value, &["无人机", "uav", "drone"]) {
        "uav_routing"
    } else if contains_any(value, &["电动车", "ev charging", "充电站"]) {
        "electric_vehicle_charging"
    } else if contains_any(value, &["物流", "配送", "vehicle routing", "vrp"]) {
        "vehicle_routing"
    } else if contains_any(value, &["边缘计算", "edge computing", "任务卸载"]) {
        "edge_computing"
    } else {
        "unknown"
    }
}

fn collect_rules(value: &str, rules: Rules, output: &mut Vec<String>) {
    for (needles, canonical) in rules {
        if contains_any(value, needles) {
            push_unique(output, canonical);
        }
    }
}

fn contains_any(value: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| value.contains(needle))
}

fn push_unique(output: &mut Vec<String>, value: &str) {
    if !output.iter().any(|item| item == value) {
        output.push(value.to_string());
    }
}

fn deduplicate(values: &mut Vec<String>) {
    let mut seen = HashSet::new();
    values.retain(|value| seen.insert(value.clone()));
}

fn add_method(output: &mut Vec<MethodMatch>, method: &str, rationale: &str, conditions: &[&str]) {
    if output.iter().any(|item| item.method == method) {
        return;
    }
    output.push(MethodMatch {
        method: method.to_string(),
        rationale: rationale.to_string(),
        required_conditions: conditions.iter().map(|value| value.to_string()).collect(),
        source: "hypothesis".to_string(),
        corroborated: false,
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Fixture {
        schema_version: String,
        cases: Vec<Case>,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Case {
        id: String,
        question: String,
        domain: String,
        objectives: Vec<String>,
        constraints: Vec<String>,
        related_problem_types: Vec<String>,
        candidate_methods: Vec<String>,
    }

    #[test]
    fn frozen_fifty_problem_descriptions_preserve_constraints_and_recall_methods() {
        let fixture: Fixture = serde_json::from_str(include_str!(
            "../../../../../evals/problem-understanding-cases.json"
        ))
        .expect("valid problem-understanding fixture");
        assert_eq!(fixture.schema_version, "problem-understanding-cases-v1");
        assert_eq!(fixture.cases.len(), 50);
        for case in fixture.cases {
            let result = understand(&case.question);
            assert_eq!(result.representation.domain, case.domain, "{}", case.id);
            for expected in case.objectives {
                assert!(
                    result.representation.objectives.contains(&expected),
                    "{} objective {expected}",
                    case.id
                );
            }
            for expected in case.constraints {
                assert!(
                    result.representation.constraints.contains(&expected),
                    "{} constraint {expected}",
                    case.id
                );
            }
            for expected in case.related_problem_types {
                assert!(
                    result
                        .representation
                        .related_problem_types
                        .contains(&expected),
                    "{} related {expected}",
                    case.id
                );
            }
            let methods = result
                .candidate_methods
                .iter()
                .map(|item| item.method.as_str())
                .collect::<HashSet<_>>();
            for expected in case.candidate_methods {
                assert!(
                    methods.contains(expected.as_str()),
                    "{} method {expected}",
                    case.id
                );
            }
        }
    }

    #[test]
    fn method_rules_are_hypotheses_and_do_not_seed_neutral_search_terms() {
        let result = understand("带时间窗的移动充电路径调度需要降低总路程");
        assert!(result
            .candidate_methods
            .iter()
            .any(|method| method.method == "adaptive_large_neighborhood_search"));
        assert!(result
            .candidate_methods
            .iter()
            .all(|method| method.source == "hypothesis" && !method.corroborated));
        assert!(!result
            .search_terms
            .iter()
            .any(|term| term.contains("adaptive_large_neighborhood_search")));
        assert!(result.search_terms.iter().any(|term| term == "vrptw"));
    }
}
