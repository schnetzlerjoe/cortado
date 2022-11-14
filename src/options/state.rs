use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin, Timestamp};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum Option {
    Call,
    Put,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Call {
    pub creator: Addr,
    pub owner: Addr,
    pub collateral: Coin,
    pub strike: Coin,
    pub premium: Coin,
    pub expires: Timestamp,
}

impl Call {
    pub fn new (creator: Addr, owner: Addr, collateral: Coin, strike: Coin, premium: Coin, expires: Timestamp,) -> Self {
        Call {
            creator,
            owner,
            collateral,
            strike,
            premium,
            expires,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Put {
    pub creator: Addr,
    pub owner: Addr,
    pub collateral: Coin,
    pub strike: Coin,
    pub premium: Coin,
    pub expires: Timestamp,
}

impl Put {
    pub fn new (creator: Addr, owner: Addr, collateral: Coin, strike: Coin, premium: Coin, expires: Timestamp,) -> Self {
        Put {
            creator,
            owner,
            collateral,
            strike,
            premium,
            expires,
        }
    }
}
