use super::history::{
    DepthHistory, EarningsHistory, EarningsWithPools, Pool, RunePoolHistory, SwapHistory,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum AllowedModel {
    DepthHistory(DepthHistory),
    SwapHistory(SwapHistory),
    EarnHistory(EarningsHistory),
    RunePoolHistory(RunePoolHistory),
    EarningsWithPools(EarningsWithPools),
    Pool(Pool),
}

impl AllowedModel {
    pub fn hist_id(&self) -> Option<&i64> {
        match self {
            AllowedModel::EarnHistory(history) => Some(&history.hist_id),
            _ => None, // Return None if `hist_id` is not present in the variant
        }
    }
}
