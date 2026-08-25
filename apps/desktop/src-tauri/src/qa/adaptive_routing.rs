use super::understanding::ExecutionMode;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

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

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LlmBudgetUsage {
    pub calls_used: usize,
    pub token_cost_used: u32,
    pub token_cost_reserved: u32,
    pub rejections: Vec<String>,
    pub stages: Vec<String>,
}

#[derive(Debug)]
struct LlmBudgetState {
    policy: RoutingPolicy,
    usage: LlmBudgetUsage,
}

#[derive(Debug, Clone)]
pub struct LlmBudgetGuard {
    state: Arc<Mutex<LlmBudgetState>>,
}

impl LlmBudgetGuard {
    pub fn new(policy: RoutingPolicy) -> Self {
        Self {
            state: Arc::new(Mutex::new(LlmBudgetState {
                policy,
                usage: LlmBudgetUsage::default(),
            })),
        }
    }

    pub fn reconfigure(&self, policy: RoutingPolicy) {
        if let Ok(mut state) = self.state.lock() {
            state.policy = policy;
        }
    }

    pub fn reserve(&self, stage: &str, token_ceiling: u32) -> Result<(), String> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| "LLM_BUDGET_STATE_ERROR: budget_lock".to_string())?;
        let next_calls = state.usage.calls_used.saturating_add(1);
        let next_tokens = state
            .usage
            .token_cost_reserved
            .saturating_add(token_ceiling);
        let reason = if next_calls > state.policy.llm_call_budget {
            Some("call_budget")
        } else if next_tokens > state.policy.token_cost_ceiling {
            Some("token_budget")
        } else {
            None
        };
        if let Some(reason) = reason {
            state.usage.rejections.push(format!("{stage}:{reason}"));
            return Err(format!("LLM_BUDGET_EXCEEDED: {stage}:{reason}"));
        }
        state.usage.calls_used = next_calls;
        state.usage.token_cost_reserved = next_tokens;
        state.usage.token_cost_used = state.usage.token_cost_used.saturating_add(token_ceiling);
        state.usage.stages.push(stage.to_string());
        Ok(())
    }

    pub fn settle(&self, stage: &str, actual_token_cost: u32, reserved: u32) {
        if let Ok(mut state) = self.state.lock() {
            let released = reserved.saturating_sub(actual_token_cost.min(reserved));
            state.usage.token_cost_used = state.usage.token_cost_used.saturating_sub(released);
            state.usage.stages.push(format!("{stage}:settled"));
        }
    }

    pub fn usage(&self) -> LlmBudgetUsage {
        self.state
            .lock()
            .map(|state| state.usage.clone())
            .unwrap_or_else(|_| LlmBudgetUsage {
                rejections: vec!["budget_lock".to_string()],
                ..LlmBudgetUsage::default()
            })
    }
}

pub fn policy(mode: &str) -> RoutingPolicy {
    let mode = match mode {
        "exploratory" => ExecutionMode::Exploratory,
        "research" => ExecutionMode::Research,
        _ => ExecutionMode::Direct,
    };
    let (planner_enabled, rounds, queries, candidates, calls, tokens) = match mode {
        ExecutionMode::Direct => (false, 1, 4, 40, 2, 8_000),
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

    #[test]
    fn budget_guard_reserves_settles_and_rejects_over_budget_calls() {
        let guard = LlmBudgetGuard::new(policy("direct"));
        guard.reserve("understanding", 2_000).unwrap();
        guard.settle("understanding", 1_200, 2_000);
        guard.reserve("generator", 4_000).unwrap();
        let error = guard.reserve("verifier", 1_000).unwrap_err();
        assert_eq!(error, "LLM_BUDGET_EXCEEDED: verifier:call_budget");
        let usage = guard.usage();
        assert_eq!(usage.calls_used, 2);
        assert_eq!(usage.token_cost_reserved, 6_000);
        assert_eq!(usage.token_cost_used, 5_200);
        assert_eq!(usage.rejections, vec!["verifier:call_budget"]);
    }

    #[test]
    fn budget_guard_reconfiguration_preserves_usage() {
        let guard = LlmBudgetGuard::new(policy("direct"));
        guard.reserve("understanding", 1_000).unwrap();
        guard.reconfigure(policy("research"));
        guard.reserve("planner", 1_000).unwrap();
        guard.reserve("generator", 1_000).unwrap();
        assert_eq!(guard.usage().calls_used, 3);
    }
}
