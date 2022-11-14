use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin, Timestamp};
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Option {
    pub creator: Addr,
    pub owner: Addr,
    pub collateral: Coin,
    pub strike: Coin,
    pub expires: Timestamp,
    pub option_type: String,
}

pub trait Call {
    fn new(creator: Addr, owner: Addr, collateral: Coin, strike: Coin, expires: Timestamp,) -> Option;
}

impl Call for Option {
    fn new (creator: Addr, owner: Addr, collateral: Coin, strike: Coin, expires: Timestamp,) -> Option {
        Option {
            creator,
            owner,
            collateral,
            strike,
            expires,
            option_type: String::from("call"),
        }
    }
}

pub trait Put {
    fn new(creator: Addr, owner: Addr, collateral: Coin, strike: Coin, expires: Timestamp,) -> Option;
}

impl Put for Option {
    fn new (creator: Addr, owner: Addr, collateral: Coin, strike: Coin, expires: Timestamp,) -> Option {
        Option {
            creator,
            owner,
            collateral,
            strike,
            expires,
            option_type: String::from("put"),
        }
    }
}

pub const CONFIG_KEY: &str = "config";
pub const CONFIG: Item<Option> = Item::new(CONFIG_KEY);
