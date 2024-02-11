use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Coin};
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub count: i32,
    pub master: String,
    pub owner: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Increment {},
    Reset { count: i32 },
    SetCollectionInfo { 
        col_admin: String,
        col_type: i8, 
        col_state: bool,
        col_auto_restart: bool,
        col_lock_dur: i32,
        col_dur:i32,
        col_reward: Coin,
        col_reward_by_rank: bool,
        col_airdrop: Coin,
        col_nairdrop: Coin,
        col_tx_fee: Coin,
        col_unstaking_fee: Coin,
        col_unstaking_fee_share: i8,
        col_fee_receiver: String,
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetCount {},
    GetCollectionInfo {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollectionResponse {
    pub col_admin: String,
    pub col_type: i8, 
    pub col_state: bool,
    pub col_auto_restart: bool,
    pub col_lock_dur: i32,
    pub col_dur:i32,
    pub col_reward: Coin,
    pub col_reward_by_rank: bool,
    pub col_airdrop: Coin,
    pub col_nairdrop: Coin,
    pub col_tx_fee: Coin,
    pub col_unstaking_fee: Coin,
    pub col_unstaking_fee_share: i8,
    pub col_fee_receiver: String,
}