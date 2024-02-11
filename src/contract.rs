#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cosmwasm_std::{Coin};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{CountResponse, CollectionResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

use crate::state::{CONFIG, Config};
use crate::state::{COLLECTION, Collection};

use std::str::FromStr;
// version info for migration info
const CONTRACT_NAME: &str = "MStaking Contract";
const CONTRACT_VERSION: &str = "0.0.1";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Set Config Settings for Master & Owner
    let config_state: Config = Config {
        master: msg.master,
        owner: msg.owner,
    };
    CONFIG.save(deps.storage, &config_state)?;

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // Set Collection Data Init
    let init_collection: Collection = Collection {
        col_admin: String::from_str("").unwrap(),
        col_type: 2,
        col_state: true,
        col_auto_restart: false,
        col_lock_dur: 86400,
        col_dur: 172800,
        col_reward: Coin::new(0, String::from_str("inj").unwrap()),
        col_reward_by_rank: false,
        col_airdrop: Coin::new(0, String::from_str("inj").unwrap()),
        col_nairdrop: Coin::new(0, String::from_str("inj").unwrap()),
        col_tx_fee: Coin::new(0, String::from_str("inj").unwrap()),
        col_unstaking_fee: Coin::new(0, String::from_str("inj").unwrap()),
        col_unstaking_fee_share: 20,
        col_fee_receiver: String::from_str("").unwrap(),
    };

    COLLECTION.save(deps.storage, &init_collection)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Increment {} => try_increment(deps),
        ExecuteMsg::Reset { count } => try_reset(deps, info, count),
        ExecuteMsg::SetCollectionInfo {
            col_admin,
            col_type,
            col_state,
            col_auto_restart,
            col_lock_dur,
            col_dur,
            col_reward,
            col_reward_by_rank,
            col_airdrop,
            col_nairdrop,
            col_tx_fee,
            col_unstaking_fee,
            col_unstaking_fee_share,
            col_fee_receiver,
        } => set_collection_info(
            deps,
            info,
            col_admin,
            col_type,
            col_state,
            col_auto_restart,
            col_lock_dur,
            col_dur,
            col_reward,
            col_reward_by_rank,
            col_airdrop,
            col_nairdrop,
            col_tx_fee,
            col_unstaking_fee,
            col_unstaking_fee_share,
            col_fee_receiver,
        )
    }
}

pub fn set_collection_info(deps: DepsMut, info: MessageInfo, 
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
    col_fee_receiver: String
    ) -> Result<Response, ContractError> {
        COLLECTION.update(deps.storage, |mut state| -> Result<_, ContractError> {
            state.col_admin = col_admin.clone();
            state.col_type = col_type;
            state.col_state = col_state;
            state.col_auto_restart = col_auto_restart;
            state.col_lock_dur = col_lock_dur;
            state.col_dur = col_dur;
            state.col_reward = col_reward.clone();
            state.col_reward_by_rank = col_reward_by_rank;
            state.col_airdrop = col_airdrop.clone();
            state.col_nairdrop = col_nairdrop.clone();
            state.col_tx_fee = col_tx_fee.clone();
            state.col_unstaking_fee = col_unstaking_fee.clone();
            state.col_fee_receiver = col_fee_receiver;
            Ok(state)
        })?;
        Ok(Response::new()
            .add_attribute("method", "set Collection Info")
            .add_attribute("admin", col_admin.clone())
        )
    }

pub fn try_increment(deps: DepsMut) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.count += 1;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("method", "try_increment"))
}

pub fn try_reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        state.count = count;
        Ok(state)
    })?;
    Ok(Response::new().add_attribute("method", "reset"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
        QueryMsg::GetCollectionInfo {} => to_binary(&get_collection_info(deps)?),
    }
}

fn get_collection_info(deps: Deps) -> StdResult<CollectionResponse> {
    let state = COLLECTION.load(deps.storage)?;
    Ok(CollectionResponse {
        col_admin: state.col_admin,
        col_type: state.col_type, //1: staked NFTs locked, free unstake after X days, 2: Lock NFT, pay fee for early unstaking
        col_state: state.col_state, //true: allow to stake, false: disallowed to stake
        col_auto_restart: state.col_auto_restart, // true: auto restart false: locked after duration
        col_lock_dur: state.col_lock_dur, // seconds
        col_dur: state.col_dur, //seconds
        col_reward: state.col_reward.clone(),
        col_reward_by_rank: state.col_reward_by_rank,
        col_airdrop: state.col_airdrop.clone(),
        col_nairdrop: state.col_nairdrop.clone(),
        col_tx_fee: state.col_tx_fee.clone(),
        col_unstaking_fee: state.col_unstaking_fee.clone(),
        col_unstaking_fee_share: state.col_unstaking_fee_share,
        col_fee_receiver: state.col_fee_receiver,
    })
}

fn query_count(deps: Deps) -> StdResult<CountResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(CountResponse { count: state.count })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(17, value.count);
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Increment {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(18, value.count);
    }

    #[test]
    fn reset() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(5, value.count);
    }
}
