use cosmwasm_std::{Coin, StdError, Timestamp};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("expired option (expired {expired:?})")]
    OptionExpired { expired: Timestamp },

    #[error("not expired option (expires {expires:?})")]
    OptionNotExpired { expires: u64 },

    #[error("unauthorized")]
    Unauthorized {},

    #[error("must send exact counter offer (offer {offer:?}, counter_offer: {counter_offer:?})")]
    CounterOfferMismatch {
        offer: Vec<Coin>,
        counter_offer: Vec<Coin>,
    },

    #[error("do not send funds with burn")]
    FundsSentWithBurn {},

    #[error("option type is invalid (must be call or put)")]
    InvalidOptionType { option_type: String },
}
