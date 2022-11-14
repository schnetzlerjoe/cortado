use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Call {
    pub creator: Addr,
    pub owner: Addr,
    pub collateral: Vec<Coin>,
    pub counter_offer: Vec<Coin>,
    pub expires: u64,
}

impl Call {
    pub fn new (creator: Addr, owner: Addr, collateral: Vec<Coin>, counter_offer: Vec<Coin>, expires: u64, option_type: String,) -> Self {
        Call {
            creator,
            owner,
            collateral,
            counter_offer,
            expires,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Put {
    pub creator: Addr,
    pub owner: Addr,
    pub collateral: Vec<Coin>,
    pub counter_offer: Vec<Coin>,
    pub expires: u64,
}

impl Put {
    pub fn new (creator: Addr, owner: Addr, collateral: Vec<Coin>, counter_offer: Vec<Coin>, expires: u64, option_type: String,) -> Self {
        Put {
            creator,
            owner,
            collateral,
            counter_offer,
            expires,
        }
    }
}
