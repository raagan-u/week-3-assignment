use super::history::{DepthHistory, EarningsHistory, Pool, RunePoolHistory, SwapHistory};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum AllowedModel {
    DepthHistory(DepthHistory),
    SwapHistory(SwapHistory),
    EarnHistory(EarningsHistory),
    RunePoolHistory(RunePoolHistory),
    Pool(Pool),
}
