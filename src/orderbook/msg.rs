use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Api, Coin, StdResult};

use cw20::{Cw20Coin, Cw20ReceiveMsg};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Create a new options market
    CreateMarket(CreateMarketMsg),
    /// Submit a new market order
    CreateMarketOrder(CreateMarketOrderMsg),
    /// Submit a new limit order
    CreateLimitOrder(CreateLimitOrderMsg),
    /// Cancel a market order
    CancelMarketOrder(CancelMarketOrderMsg),
    /// Cancel a limit order
    CancelLimitOrder(CancelLimitOrderMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CreateMarketMsg {
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CreateMarketOrderMsg {
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CreateLimitOrderMsg {
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CancelMarketOrderMsg {
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CancelLimitOrderMsg {
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Show all options markets
    Markets {},
    /// Show an options market
    Market { id: String },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ListResponse {
    /// list all registered ids
    pub escrows: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct DetailsResponse {
    /// id of this escrow
    pub id: String,
    /// arbiter can decide to approve or refund the escrow
    pub arbiter: String,
    /// if approved, funds go to the recipient
    pub recipient: Option<String>,
    /// if refunded, funds go to the source
    pub source: String,
    /// Title of the escrow
    pub title: String,
    /// Longer description of the escrow, e.g. what conditions should be met
    pub description: String,
    /// When end height set and block height exceeds this value, the escrow is expired.
    /// Once an escrow is expired, it can be returned to the original funder (via "refund").
    pub end_height: Option<u64>,
    /// When end time (in seconds since epoch 00:00:00 UTC on 1 January 1970) is set and
    /// block time exceeds this value, the escrow is expired.
    /// Once an escrow is expired, it can be returned to the original funder (via "refund").
    pub end_time: Option<u64>,
    /// Balance in native tokens
    pub native_balance: Vec<Coin>,
    /// Balance in cw20 tokens
    pub cw20_balance: Vec<Cw20Coin>,
    /// Whitelisted cw20 tokens
    pub cw20_whitelist: Vec<String>,
}
