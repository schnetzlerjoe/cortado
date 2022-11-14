use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Uint64};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Create a new options market
    CreateMarket(CreateMarketMsg),
    /// Submit a new buy limit order
    CreateBuyOrder(CreateBuyOrderMsg),
    /// Submit a new sell limit order
    CreateSellOrder(CreateSellOrderMsg),
    /// Cancel a buy limit order
    CancelBuyOrder(CancelBuyOrderMsg),
    /// Cancel a sell limit order
    CancelSellOrder(CancelSellOrderMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CreateMarketMsg {
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CreateBuyOrderMsg {
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CreateSellOrderMsg {
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CancelBuyOrderMsg {
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CancelSellOrderMsg {
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Show the option market for this contract
    Market { id: String },
    /// Show the buy order depth for this contract
    Buys {},
    /// Show a buy order by its id
    Buy { id: Uint64 },
    /// Show a sell order by its id
    Sell { id: Uint64 },
    /// Show the sell order depth for this contract
    Sells {},
    /// Show the current price of the option for this market. The highest bid + lowest ask / 2.
    CurrentPrice {},
}
