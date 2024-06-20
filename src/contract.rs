#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, SudoMsg};
use crate::state::{Config, CONFIG};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-cron-example";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    CONFIG.save(deps.storage, &Config { b: 0, e: 0 })?;
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
        ExecuteMsg::IncrementBegin {} => {
            increment_begin(deps)?;
            Ok(Response::new())
        },
        ExecuteMsg::IncrementEnd {} => {
            increment_end(deps)?;
            Ok(Response::new())
        },        
    }
}

fn increment_begin(deps: DepsMut) -> Result<(), ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    config.b += 1;
    CONFIG.save(deps.storage, &config)?;
    Ok(())
}

fn increment_end(deps: DepsMut) -> Result<(), ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    config.e += 1;
    CONFIG.save(deps.storage, &config)?;
    Ok(())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_json_binary(&query_config(deps)?),
    }
}

fn query_config(deps: Deps) -> StdResult<Config> {
    let config = CONFIG.load(deps.storage)?;
    Ok(Config { b: config.b, e: config.e })
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, _env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::CronBeginBlock {} => {
            increment_begin(deps)?;
            Ok(Response::new())
        },
        SudoMsg::CronEndBlock {} => {
            increment_end(deps)?;
            Ok(Response::new())
        }
    }
}

#[cfg(test)]
mod tests {}
