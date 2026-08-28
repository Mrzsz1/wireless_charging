use super::understanding::ExecutionMode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
    pub token_cost_in_flight: u32,
    pub token_cost_reserved: u32,
    pub token_cost_reserved_total: u32,
    pub rejections: Vec<String>,
    pub stages: Vec<String>,
}

#[derive(Debug)]
struct ActiveReservation {
    stage: String,
    token_ceiling: u32,
}

#[derive(Debug)]
struct LlmBudgetState {
    policy: RoutingPolicy,
    usage: LlmBudgetUsage,
    next_reservation_id: u64,
    active_reservations: HashMap<u64, ActiveReservation>,
}

#[derive(Debug, Clone)]
pub struct LlmBudgetGuard {
    state: Arc<Mutex<LlmBudgetState>>,
}

#[derive(Debug)]
#[must_use = "an LLM reservation must be settled or explicitly released"]
pub struct LlmReservation {
    id: u64,
    guard: LlmBudgetGuard,
    closed: bool,
}

impl LlmReservation {
    pub fn settle(mut self, actual_token_cost: u32) -> Result<(), String> {
        self.guard
            .close_reservation(self.id, Some(actual_token_cost))?;
        self.closed = true;
        Ok(())
    }

    pub fn release(mut self) -> Result<(), String> {
        self.guard.close_reservation(self.id, None)?;
        self.closed = true;
        Ok(())
    }
}

impl Drop for LlmReservation {
    fn drop(&mut self) {
        if !self.closed {
            let _ = self.guard.close_reservation(self.id, None);
            self.closed = true;
        }
    }
}

impl LlmBudgetGuard {
    pub fn new(policy: RoutingPolicy) -> Self {
        Self {
            state: Arc::new(Mutex::new(LlmBudgetState {
                policy,
                usage: LlmBudgetUsage::default(),
                next_reservation_id: 1,
                active_reservations: HashMap::new(),
            })),
        }
    }

    pub fn reconfigure(&self, policy: RoutingPolicy) {
        if let Ok(mut state) = self.state.lock() {
            state.policy = policy;
        }
    }

    pub fn reserve(&self, stage: &str, token_ceiling: u32) -> Result<LlmReservation, String> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| "LLM_BUDGET_STATE_ERROR: budget_lock".to_string())?;
        let next_calls = state.usage.calls_used.saturating_add(1);
        let next_tokens = state
            .usage
            .token_cost_used
            .saturating_add(state.usage.token_cost_in_flight)
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
        let reservation_id = state.next_reservation_id;
        state.next_reservation_id = state
            .next_reservation_id
            .checked_add(1)
            .ok_or_else(|| "LLM_BUDGET_STATE_ERROR: reservation_id_exhausted".to_string())?;
        state.usage.calls_used = next_calls;
        state.usage.token_cost_in_flight = state
            .usage
            .token_cost_in_flight
            .saturating_add(token_ceiling);
        state.usage.token_cost_reserved = state
            .usage
            .token_cost_reserved
            .saturating_add(token_ceiling);
        state.usage.token_cost_reserved_total = state
            .usage
            .token_cost_reserved_total
            .saturating_add(token_ceiling);
        state.usage.stages.push(stage.to_string());
        state.active_reservations.insert(
            reservation_id,
            ActiveReservation {
                stage: stage.to_string(),
                token_ceiling,
            },
        );
        Ok(LlmReservation {
            id: reservation_id,
            guard: self.clone(),
            closed: false,
        })
    }

    fn close_reservation(
        &self,
        reservation_id: u64,
        actual_token_cost: Option<u32>,
    ) -> Result<(), String> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| "LLM_BUDGET_STATE_ERROR: budget_lock".to_string())?;
        let reservation = state
            .active_reservations
            .remove(&reservation_id)
            .ok_or_else(|| "LLM_BUDGET_STATE_ERROR: reservation_closed".to_string())?;
        state.usage.token_cost_in_flight = state
            .usage
            .token_cost_in_flight
            .saturating_sub(reservation.token_ceiling);
        match actual_token_cost {
            Some(actual) => {
                state.usage.token_cost_used = state.usage.token_cost_used.saturating_add(actual);
                state
                    .usage
                    .stages
                    .push(format!("{}:settled", reservation.stage));
            }
            None => state
                .usage
                .stages
                .push(format!("{}:released", reservation.stage)),
        }
        Ok(())
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
        ExecutionMode::Direct => (false, 1, 4, 30, 2, 8_000),
        ExecutionMode::Research => (true, 2, 12, 50, 3, 18_000),
        ExecutionMode::Exploratory => (true, 3, 20, 60, 5, 32_000),
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

    fn test_policy(token_cost_ceiling: u32, llm_call_budget: usize) -> RoutingPolicy {
        RoutingPolicy {
            token_cost_ceiling,
            llm_call_budget,
            ..policy("direct")
        }
    }

    #[test]
    fn completed_stage_unused_reservation_is_reusable_by_generator() {
        let guard = LlmBudgetGuard::new(test_policy(8_000, 4));
        guard
            .reserve("understanding", 4_000)
            .unwrap()
            .settle(1_000)
            .unwrap();
        let generator = guard.reserve("generator", 6_000).unwrap();
        let usage = guard.usage();
        assert_eq!(usage.token_cost_used, 1_000);
        assert_eq!(usage.token_cost_in_flight, 6_000);
        assert_eq!(usage.token_cost_reserved, 10_000);
        assert_eq!(usage.token_cost_reserved_total, 10_000);
        generator.release().unwrap();
    }

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
        assert_eq!(
            (
                direct.token_cost_ceiling,
                research.token_cost_ceiling,
                exploratory.token_cost_ceiling,
            ),
            (8_000, 18_000, 32_000)
        );
    }

    #[test]
    fn budget_guard_reserves_settles_and_rejects_over_budget_calls() {
        let guard = LlmBudgetGuard::new(policy("direct"));
        guard
            .reserve("understanding", 2_000)
            .unwrap()
            .settle(1_200)
            .unwrap();
        let generator = guard.reserve("generator", 4_000).unwrap();
        let error = guard.reserve("verifier", 1_000).unwrap_err();
        assert_eq!(error, "LLM_BUDGET_EXCEEDED: verifier:call_budget");
        let usage = guard.usage();
        assert_eq!(usage.calls_used, 2);
        assert_eq!(usage.token_cost_reserved, 6_000);
        assert_eq!(usage.token_cost_reserved_total, 6_000);
        assert_eq!(usage.token_cost_used, 1_200);
        assert_eq!(usage.token_cost_in_flight, 4_000);
        assert_eq!(usage.rejections, vec!["verifier:call_budget"]);
        generator.release().unwrap();
    }

    #[test]
    fn budget_guard_reconfiguration_preserves_usage() {
        let guard = LlmBudgetGuard::new(policy("direct"));
        guard
            .reserve("understanding", 4_000)
            .unwrap()
            .settle(1_000)
            .unwrap();
        let planner = guard.reserve("planner", 6_000).unwrap();
        guard.reconfigure(policy("research"));
        let generator = guard.reserve("generator", 10_000).unwrap();
        let usage = guard.usage();
        assert_eq!(usage.calls_used, 3);
        assert_eq!(usage.token_cost_used, 1_000);
        assert_eq!(usage.token_cost_in_flight, 16_000);
        assert_eq!(usage.token_cost_reserved_total, 20_000);
        planner.release().unwrap();
        generator.release().unwrap();
    }

    #[test]
    fn actual_usage_and_concurrent_reservations_cannot_exceed_ceiling() {
        let guard = LlmBudgetGuard::new(test_policy(8_000, 8));
        guard
            .reserve("understanding", 5_000)
            .unwrap()
            .settle(5_000)
            .unwrap();
        let planner = guard.reserve("planner", 1_000).unwrap();
        let error = guard.reserve("generator", 3_000).unwrap_err();
        assert_eq!(error, "LLM_BUDGET_EXCEEDED: generator:token_budget");
        assert_eq!(guard.usage().token_cost_in_flight, 1_000);
        planner.release().unwrap();
    }

    #[test]
    fn concurrent_reservations_are_not_oversold_and_release_restores_capacity() {
        let guard = LlmBudgetGuard::new(test_policy(8_000, 8));
        let understanding = guard.reserve("understanding", 4_000).unwrap();
        let planner = guard.reserve("planner", 3_000).unwrap();
        assert_eq!(guard.usage().token_cost_in_flight, 7_000);
        assert_eq!(
            guard.reserve("generator", 1_001).unwrap_err(),
            "LLM_BUDGET_EXCEEDED: generator:token_budget"
        );
        understanding.release().unwrap();
        let generator = guard.reserve("generator", 4_000).unwrap();
        planner.release().unwrap();
        generator.release().unwrap();
    }

    #[test]
    fn settle_release_and_drop_never_refund_call_budget_or_leak_in_flight() {
        let guard = LlmBudgetGuard::new(test_policy(8_000, 8));
        guard
            .reserve("understanding", 2_000)
            .unwrap()
            .settle(500)
            .unwrap();
        guard.reserve("planner", 2_000).unwrap().release().unwrap();
        {
            let _provider_failure = guard.reserve("generator", 2_000).unwrap();
        }
        let usage = guard.usage();
        assert_eq!(usage.calls_used, 3);
        assert_eq!(usage.token_cost_used, 500);
        assert_eq!(usage.token_cost_in_flight, 0);
        assert_eq!(usage.token_cost_reserved_total, 6_000);
        assert!(usage.stages.contains(&"generator:released".to_string()));
    }

    #[test]
    fn provider_failure_before_settlement_releases_reservation_on_error_return() {
        fn failing_provider(guard: &LlmBudgetGuard) -> Result<(), String> {
            let _reservation = guard.reserve("generator", 6_000)?;
            Err("PROVIDER_FAILURE: fixture".to_string())
        }

        let guard = LlmBudgetGuard::new(test_policy(8_000, 4));
        assert_eq!(
            failing_provider(&guard).unwrap_err(),
            "PROVIDER_FAILURE: fixture"
        );
        let usage = guard.usage();
        assert_eq!(usage.calls_used, 1);
        assert_eq!(usage.token_cost_used, 0);
        assert_eq!(usage.token_cost_in_flight, 0);
        let generator = guard.reserve("generator", 8_000).unwrap();
        generator.release().unwrap();
    }

    #[test]
    fn task_panic_unwind_releases_active_reservation() {
        let guard = LlmBudgetGuard::new(test_policy(8_000, 4));
        let panic_guard = guard.clone();
        let result = std::panic::catch_unwind(move || {
            let _reservation = panic_guard.reserve("generator", 8_000).unwrap();
            panic!("synthetic provider task panic");
        });
        assert!(result.is_err());
        let usage = guard.usage();
        assert_eq!(usage.calls_used, 1);
        assert_eq!(usage.token_cost_used, 0);
        assert_eq!(usage.token_cost_in_flight, 0);
    }

    #[test]
    fn closed_reservation_rejects_duplicate_internal_close_without_side_effects() {
        let guard = LlmBudgetGuard::new(test_policy(8_000, 4));
        let reservation = guard.reserve("generator", 4_000).unwrap();
        let reservation_id = reservation.id;
        reservation.settle(1_000).unwrap();
        let before = guard.usage();
        assert_eq!(
            guard.close_reservation(reservation_id, Some(1_000)),
            Err("LLM_BUDGET_STATE_ERROR: reservation_closed".to_string())
        );
        assert_eq!(guard.usage(), before);
    }

    #[test]
    fn exploratory_development_stress_reaches_generator_and_verifier() {
        let guard = LlmBudgetGuard::new(policy("exploratory"));
        guard
            .reserve("understanding", 8_000)
            .unwrap()
            .settle(1_000)
            .unwrap();
        guard
            .reserve("planner", 10_000)
            .unwrap()
            .settle(2_000)
            .unwrap();
        guard
            .reserve("generator", 24_000)
            .unwrap()
            .settle(8_000)
            .unwrap();
        guard
            .reserve("semantic_verifier", 1_000)
            .unwrap()
            .settle(500)
            .unwrap();
        let usage = guard.usage();
        assert_eq!(usage.calls_used, 4);
        assert_eq!(usage.token_cost_used, 11_500);
        assert_eq!(usage.token_cost_in_flight, 0);
        assert_eq!(usage.token_cost_reserved_total, 43_000);
    }
}
