use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

pub fn deserialize_f64_from_string_or_number<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    match Value::deserialize(deserializer)? {
        Value::String(s) => s.parse::<f64>().map_err(serde::de::Error::custom),
        Value::Number(num) => num
            .as_f64()
            .ok_or_else(|| serde::de::Error::custom("invalid f64")),
        _ => Err(serde::de::Error::custom("expected a string or number")),
    }
}

pub fn deserialize_i64_from_string_or_number<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    match Value::deserialize(deserializer)? {
        Value::String(s) => s.parse::<i64>().map_err(serde::de::Error::custom),
        Value::Number(num) => num
            .as_i64()
            .ok_or_else(|| serde::de::Error::custom("invalid i64")),
        _ => Err(serde::de::Error::custom("expected a string or number")),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DepthHistory {
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub hist_id: i64,
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub start_time: i64, // Unix timestamp (Int64)
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub end_time: i64, // Unix timestamp (Int64)
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub asset_depth: i64, // Amount of Asset in the pool (Int64)
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub rune_depth: i64, // Amount of Rune in the pool (Int64)
    #[serde(deserialize_with = "deserialize_f64_from_string_or_number")]
    pub asset_price: f64, // Price of asset in Rune (Float)
    #[serde(deserialize_with = "deserialize_f64_from_string_or_number")]
    #[serde(rename = "assetPriceUSD")]
    pub asset_price_usd: f64, // Price of asset in USD (Float)
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub liquidity_units: i64, // Liquidity Units in the pool (Int64)
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub members_count: i64, // Number of liquidity members (Int64)
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub synth_units: i64, // Synth Units in the pool (Int64)
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub synth_supply: i64, // Synth Supply in the pool (Int64)
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub units: i64, // Total Units in the pool (Int64)
    #[serde(deserialize_with = "deserialize_f64_from_string_or_number")]
    pub luvi: f64, //original float
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SwapHistory {
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub hist_id: i64,
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub start_time: i64, // Unix timestamp (Int64)
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub end_time: i64, // Unix timestamp (Int64)
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub to_asset_count: i64, // Count of swaps from Rune to Asset (Int64)
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub to_rune_count: i64, // Count of swaps from Asset to Rune (Int64)
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub to_trade_count: i64, // Count of swaps from Rune to Trade Asset (Int64)
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub from_trade_count: i64, // Count of swaps from Trade Asset to Rune (Int64)
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub synth_mint_count: i64, // Count of swaps from Rune to Synthetic Asset (Int64)
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub synth_redeem_count: i64, // Count of swaps from Synthetic Asset to Rune (Int64)
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub total_count: i64, // Total count of swaps (Int64)

    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub to_asset_volume: i64, // Volume of swaps from Rune to Asset (Int64 e8)
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub to_rune_volume: i64, // Volume of swaps from Asset to Rune (Int64 e8)
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub to_trade_volume: i64, // Volume of swaps from Trade Asset to Rune (Int64 e8)
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub from_trade_volume: i64, // Volume of swaps from Rune to Trade Asset (Int64 e8)
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub synth_mint_volume: i64, // Volume of swaps from Rune to Synthetic Asset (Int64 e8)
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub synth_redeem_volume: i64, // Volume of swaps from Synthetic Asset to Rune (Int64 e8)
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub total_volume: i64, // Total volume of swaps (Int64 e8)

    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    #[serde(rename = "toAssetVolumeUSD")]
    pub to_asset_volume_usd: i64, // Volume of swaps from Rune to Asset in USD (Int64 e2)
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    #[serde(rename = "toRuneVolumeUSD")]
    pub to_rune_volume_usd: i64, // Volume of swaps from Asset to Rune in USD (Int64 e2)
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    #[serde(rename = "toTradeVolumeUSD")]
    pub to_trade_volume_usd: i64, // Volume of swaps from Rune to Trade Asset in USD (Int64 e2)
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    #[serde(rename = "fromTradeVolumeUSD")]
    pub from_trade_volume_usd: i64, // Volume of swaps from Trade Asset to Rune in USD (Int64 e2)
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    #[serde(rename = "synthMintVolumeUSD")]
    pub synth_mint_volume_usd: i64, // Volume of swaps from Rune to Synthetic Asset in USD (Int64 e2)
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    #[serde(rename = "synthRedeemVolumeUSD")]
    pub synth_redeem_volume_usd: i64, // Volume of swaps from Synthetic Asset to Rune in USD (Int64 e2)
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    #[serde(rename = "totalVolumeUSD")]
    pub total_volume_usd: i64, // Total volume in USD (Int64 e2)

    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub to_asset_fees: i64, // Fees from swaps Rune to Asset (Int64 e8)
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub to_rune_fees: i64, // Fees from swaps Asset to Rune (Int64 e8)
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub to_trade_fees: i64, // Fees from swaps Rune to Trade Asset (Int64 e8)
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub from_trade_fees: i64, // Fees from swaps Trade Asset to Rune (Int64 e8)
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub synth_mint_fees: i64, // Fees from Rune to Synthetic Asset (Int64 e8)
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub synth_redeem_fees: i64, // Fees from Synthetic Asset to Rune (Int64 e8)
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub total_fees: i64, // Total fees (Int64 e8)

    #[serde(deserialize_with = "deserialize_f64_from_string_or_number")]
    pub to_asset_average_slip: f64, // Average slip for Rune to Asset swaps (Float64, basis points)
    #[serde(deserialize_with = "deserialize_f64_from_string_or_number")]
    pub to_rune_average_slip: f64, // Average slip for Asset to Rune swaps (Float64, basis points)
    #[serde(deserialize_with = "deserialize_f64_from_string_or_number")]
    pub to_trade_average_slip: f64, // Average slip for Rune to Trade Asset swaps (Float64, basis points)
    #[serde(deserialize_with = "deserialize_f64_from_string_or_number")]
    pub from_trade_average_slip: f64, // Average slip for Trade Asset to Rune swaps (Float64, basis points)
    #[serde(deserialize_with = "deserialize_f64_from_string_or_number")]
    pub synth_mint_average_slip: f64, // Average slip for Rune to Synthetic Asset swaps (Float64, basis points)
    #[serde(deserialize_with = "deserialize_f64_from_string_or_number")]
    pub synth_redeem_average_slip: f64, // Average slip for Synthetic Asset to Rune swaps (Float64, basis points)
    #[serde(deserialize_with = "deserialize_f64_from_string_or_number")]
    pub average_slip: f64, // Weighted average slip (Float64, basis points)

    #[serde(deserialize_with = "deserialize_f64_from_string_or_number")]
    #[serde(rename = "runePriceUSD")]
    pub rune_price_usd: f64, // Price of Rune in USD (Float)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Pool {
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub pool_id: i64,
    pub pool: String,
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub asset_liquidity_fees: i64,
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub rune_liquidity_fees: i64,
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub total_liquidity_fees_rune: i64,
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub saver_earning: i64,
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub rewards: i64,
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub earnings: i64,
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub hist_id: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EarningsHistory {
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub hist_id: i64,
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub start_time: i64,
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub end_time: i64,
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub liquidity_fees: i64,
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub block_rewards: i64,
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub earnings: i64,
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub bonding_earnings: i64,
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub liquidity_earnings: i64,
    #[serde(deserialize_with = "deserialize_f64_from_string_or_number")]
    pub avg_node_count: f64,
    #[serde(deserialize_with = "deserialize_f64_from_string_or_number")]
    #[serde(rename = "runePriceUSD")]
    pub rune_price_usd: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EarningsWithPools {
    #[serde(flatten)]
    pub history: EarningsHistory,
    pub pools: Vec<Pool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RunePoolHistory {
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub hist_id: i64,
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub start_time: i64,
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub end_time: i64,
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub count: i64,
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub units: i64,
}

#[derive(Deserialize)]
pub struct HistoryQuery {
    pub date_range: Option<String>, // e.g., "2023-08-01,2023-09-01"
    pub sort_by: Option<String>,    // Sorting field, e.g., "timestamp"
    pub order: Option<String>,      // "asc" or "desc"
    pub page: Option<u32>,          // Page number for pagination
    pub limit: Option<u32>,         // Number of items per page
    pub from: Option<i64>,
    pub to: Option<i64>,
    pub count: Option<u32>,
    pub interval: Option<String>,
    pub cmp_field: Option<String>,
    pub cmp_units: Option<i64>,
    pub cmp_op: Option<String>,
}
