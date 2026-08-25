use super::understanding::ExecutionMode;
use serde::{Deserialize, Serialize};

pub const ROUTING_POLICY_VERSION: &str = "adaptive-routing-v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RoutingPolicy {
    pub version: String,
    pub mode: ExecutionMode,
    pub planner_enabled: bool,
    pub max_retrieval_rounds: usize,
    pub max_queries: usize,
    pub max_candidates: usize,
    pub llm_call_budget: usize,
    pub token_cost_ceiling: u32,
}

pub fn policy(mode: &str) -> RoutingPolicy {
    let mode = match mode {
        "exploratory" => ExecutionMode::Exploratory,
        "research" => ExecutionMode::Research,
        _ => ExecutionMode::Direct,
    };
    let (planner_enabled, rounds, queries, candidates, calls, tokens) = match mode {
        ExecutionMode::Direct => (false, 2, 4, 40, 1, 8_000),
        ExecutionMode::Research => (true, 2, 12, 80, 3, 18_000),
        ExecutionMode::Exploratory => (true, 3, 20, 120, 5, 32_000),
    };
    RoutingPolicy {
        version: ROUTING_POLICY_VERSION.to_string(),
        mode,
        planner_enabled,
        max_retrieval_rounds: rounds,
        max_queries: queries,
        max_candidates: candidates,
        llm_call_budget: calls,
        token_cost_ceiling: tokens,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct_research_and_exploratory_have_monotonic_cost_budgets() {
        let direct = policy("direct");
        let research = policy("research");
        let exploratory = policy("exploratory");
        assert!(!direct.planner_enabled);
        assert!(research.planner_enabled && exploratory.planner_enabled);
        assert!(direct.max_retrieval_rounds <= research.max_retrieval_rounds);
        assert!(research.max_retrieval_rounds < exploratory.max_retrieval_rounds);
        assert!(direct.llm_call_budget < research.llm_call_budget);
        assert!(research.llm_call_budget < exploratory.llm_call_budget);
        assert!(direct.token_cost_ceiling < research.token_cost_ceiling);
        assert!(research.token_cost_ceiling < exploratory.token_cost_ceiling);
    }
}
