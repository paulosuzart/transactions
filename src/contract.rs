use cosmwasm_std::to_binary;
use crate::state::ChargeId;
use crate::state::Transaction;
use crate::state::TRANS;

use cosmwasm_std::{entry_point, Deps, Binary, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// Note, you can use StdResult in some functions where you do not
// make use of the custom errors
#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

// And declare a custom Error variant for the ones where you will want to make use of it
#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::RecordCharge { id, amount } => record_charge(deps, info, id, amount),
        ExecuteMsg::Settle { id } => settle(deps, info, id),
    }
}

pub fn settle(deps: DepsMut, _info: MessageInfo, id: ChargeId) -> Result<Response, ContractError> {
    let mut state: Transaction = TRANS.load(deps.storage, id.as_k().as_bytes())?;
    if state.settled {
        return Err(ContractError::AlreadySettled {});
    }

    state.settled = true;
    TRANS.save(deps.storage, id.as_k().as_bytes(), &state)?;
    let mut result = Response::new();
    result.add_attribute("action", "settle");
    Ok(result)
}

pub fn record_charge(
    deps: DepsMut,
    info: MessageInfo,
    id: ChargeId,
    amount: i32,
) -> Result<Response, ContractError> {

    let data = Transaction {
        id: id.clone(),
        amount: amount,
        owner: info.sender,
        settled: false,
    };

    TRANS.save(deps.storage, id.as_k().as_bytes(), &data)?;
    let mut result = Response::new();
    result.add_attribute("action", "record");
    Ok(result)
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCharge { id } => to_binary(&get_charge(deps, id)?),
    }
}

pub fn get_charge(deps: Deps, id: ChargeId) -> StdResult<Transaction> {
    let state = TRANS.load(deps.storage, id.as_k().as_bytes())?;

    Ok(state)
}
