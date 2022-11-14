use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Coin, Timestamp, Uint64, Uint256};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct SellOrder {
    /// the string wasm address for the option being sold 
    pub option: String,
    /// the price you are willing to sell for
    pub price: Coin,
    /// time the order was submitted
    pub submitted: Timestamp,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct BuyOrder {
    /// amount of underlying shares you would like
    pub amount: Uint256,
    /// what option type you want (either call or put)
    pub option_type: String,
    /// price per underlying you are willing to pay
    pub price: Coin,
    /// the time the order was submitted
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
