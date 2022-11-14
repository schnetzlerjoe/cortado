use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Coin, Timestamp, Addr, Uint64};

use crate::options::{Call, Option};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct SellOrder {
    pub option: Addr,
    pub price: Coin,
    pub submitted: Timestamp,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct BuyOrder {
    pub option: Addr,
    pub price: Coin,
    pub submitted: Timestamp,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Orderbook {
    /// the underlying token for this market
    pub market: String,
    pub sells: Vec<SellOrder>,
    pub buys: Vec<BuyOrder>,
    /// the max depth on each side of the orderbook. Default is 15, maximum is 50.
    pub depth: Uint64,
}

impl Orderbook {
    pub fn new(market: String, sells: Vec<SellOrder>, buys: Vec<BuyOrder>, depth: Uint64) -> Self {
        Orderbook {
            market,
            sells,
            buys,
            depth,
        }
    }
}
