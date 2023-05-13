#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary, CosmosMsg, SubMsg,
};
use cw2::set_contract_version;

use crate::{msg::{InstantiateMsg, ExecuteMsg, QueryMsg}, error::ContractError, state::MSG_QUEUE};

const CONTRACT_NAME: &str = "crates.io:cw-heartbeat";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Beat {} => process_head_msg(deps),
        ExecuteMsg::Schedule { msg } => schedule_cosmos_msg(msg, deps),
    }
}

fn process_head_msg(deps: DepsMut) -> Result<Response, ContractError> {
    // retrieve the oldest message and attempt to execute it
    let head_msg = match MSG_QUEUE.pop_front(deps.storage)? {
        Some(msg) => msg,
        // should not happen
        None => return Err(ContractError::QueuePopError {  }),
    };

    Ok(Response::default()
        .add_attribute("method", "process_head_msg")
        .add_submessage(SubMsg {
            id: 1, // todo
            msg: head_msg,
            gas_limit: None,
            reply_on: cosmwasm_std::ReplyOn::Always,
        }
    ))
}

fn schedule_cosmos_msg(msg: CosmosMsg, deps: DepsMut) -> Result<Response, ContractError> {
    // validate queue length
    if MSG_QUEUE.len(deps.storage)?.eq(&(u32::MAX - 1)) {
        return Err(ContractError::QueueFullError {})
    }

    // schedule the message
    MSG_QUEUE.push_back(deps.storage, &msg)?;

    // attempt to process the oldest msg and overwrite the method property
    Ok(process_head_msg(deps)?
        .add_attribute("method", "schedule_cosmos_msg")
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    to_binary(&true)
}

