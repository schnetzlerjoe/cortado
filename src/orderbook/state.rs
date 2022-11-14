use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Coin, Timestamp};

use crate::options::{Call, Option};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct SellOrder {
    pub option: Option,
    pub price: Coin,
    pub submitted: Timestamp,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct BuyOrder {
    pub option: Option,
    pub price: Coin,
    pub submitted: Timestamp,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Orderbook {
    /// the underlying token for this market
    pub market: String,
    pub sells: Vec<SellOrder>,
    pub buys: Vec<BuyOrder>,
}

impl Orderbook {
    pub fn new(market: String, sells: Vec<SellOrder>, buys: Vec<BuyOrder>) -> Self {
        Orderbook {
            market,
            sells,
            buys,
        }
    }
}
