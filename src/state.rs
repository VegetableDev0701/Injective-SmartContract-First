use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::vec::Vec;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use cosmwasm_std::{Coin,coin, Timestamp};
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
}

pub const STATE: Item<State> = Item::new("state");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Collection {
    pub col_admin: String,
    pub col_type: i8, //1: staked NFTs locked, free unstake after X days, 2: Lock NFT, pay fee for early unstaking
    pub col_state: bool, //true: allow to stake, false: disallowed to stake
    pub col_auto_restart: bool, // true: auto restart false: locked after duration
    pub col_lock_dur: i32, // seconds
    pub col_dur:i32, //seconds
    pub col_reward: Coin,
    pub col_reward_by_rank: bool,
    pub col_airdrop: Coin,
    pub col_nairdrop: Coin,
    pub col_tx_fee: Coin,
    pub col_unstaking_fee: Coin,
    pub col_unstaking_fee_share: i8,
    pub col_fee_receiver: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Staking {
    pub token_owner: String,
    pub token_address: String,
    pub token_id: String,
    pub token_start: Timestamp,
    pub token_end: Timestamp,
    pub token_claimed: Timestamp,
}
impl Staking {
    pub fn default() -> Self {
        Staking {
            token_owner: String::from_str("").unwrap(),
            token_address: String::from_str("").unwrap(),
            token_id: String::from_str("").unwrap(),
            token_start: Timestamp::from_seconds(0),
            token_end: Timestamp::from_seconds(0),
            token_claimed: Timestamp::from_seconds(0),
        }
    }
    pub fn new(token_address: String, token_owner: String, token_id: String, token_start: Timestamp) -> Self {
        Staking {
            token_owner,
            token_address,
            token_id,
            token_start,
            token_end: Timestamp::from_seconds(0),
            token_claimed: Timestamp::from_seconds(0),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub master: String,
    pub owner: String,
}

pub const CONFIG: Item<Config> = Item::new("Config");
pub const COLLECTION: Item<Collection> = Item::new("Collection");
pub const STAKINGS: Vec<Staking> = Vec::new();