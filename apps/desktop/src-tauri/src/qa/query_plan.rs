pub use super::retrieval_contract::{
    parse_retrieval_contract as parse_query_plan, retrieval_contract_prompt as query_plan_prompt,
    retrieval_contract_provider_schema as query_plan_provider_schema,
    RetrievalContract as QueryPlan, RetrievalFacet as QueryFacet,
    RetrievalPlanningCandidate as QueryPlanningCandidate,
    RetrievalPlanningInput as QueryPlanningInput, RETRIEVAL_CONTRACT_VERSION as QUERY_PLAN_VERSION,
};

pub fn query_plan_schema() -> serde_json::Value {
    super::retrieval_contract::retrieval_contract_schema()
}
#[cfg(test)]
pub use super::retrieval_contract::{RetrievalBudget as QueryBudget, RetrievalScope as QueryScope};
