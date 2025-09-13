
use cosmwasm_std::{entry_point, DepsMut, Deps, Env, MessageInfo, Response, StdResult};
use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg};
use crate::state::{Poll, POLLS};

pub mod msg;
pub mod state;

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Vote { poll_id: _, option_index: _ } => {
            Ok(Response::new().add_attribute("action", "vote"))
        }
    }
}

#[entry_point]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Response> {
    Ok(Response::default())
}
