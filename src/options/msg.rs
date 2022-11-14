use cosmwasm_std::{Coin, Addr, Timestamp};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub creator: Addr,
    pub owner: Addr,
    pub collateral: Coin,
    pub strike: Coin,
    pub premium: Coin,
    pub expires: Timestamp,
    pub option_type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Owner can transfer to a new owner
    Transfer { recipient: String },
    /// Owner can post counter_offer on unexpired option to execute and get the collateral
    Execute {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
}
