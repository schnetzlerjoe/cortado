use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Coin, Timestamp};

use crate::options::{Call, Put};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct SellOrder {
    pub option: Option<Call, Put>,
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

    pub fn current_price(&self) -> Coin {
        return Coin::new(100, "uosmo")
    }
}

#[cfg(test)]
mod tests {
    use super::Orderbook;
    use super::Option;

    #[test]
    fn orderbook_price_test() {
        let option = Option::new(creator, owner, collateral, counter_offer, expires, option_type);

        Orderbook {
            market: String::from("uosmo"),
            sells: Some(Addr::unchecked("recip")),
            buys: Addr::unchecked("source"),
        };
        assert_eq!(actual, 2);
    }
}
