#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::{get_contract_version, set_contract_version};

use crate::{
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
    querier::{
        aggregrate, block_aggregrate, block_try_aggregate_optional, block_try_aggregrate,
        try_aggregate, try_aggregate_optional,
    },
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:multicall";
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
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    Err(ContractError::ExecuteNotSupported)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ContractVersion {} => to_binary(&get_contract_version(deps.storage)?),
        QueryMsg::Aggregate { queries } => to_binary(&aggregrate(deps, queries)?),
        QueryMsg::TryAggregate {
            require_success,
            include_cause,
            queries,
        } => to_binary(&try_aggregate(
            deps,
            require_success,
            include_cause,
            queries,
        )?),
        QueryMsg::TryAggregateOptional {
            include_cause,
            queries,
        } => to_binary(&try_aggregate_optional(deps, include_cause, queries)?),
        QueryMsg::BlockAggregate { queries } => to_binary(&block_aggregrate(deps, env, queries)?),
        QueryMsg::BlockTryAggregate {
            require_success,
            include_cause,
            queries,
        } => to_binary(&block_try_aggregrate(
            deps,
            env,
            require_success,
            include_cause,
            queries,
        )?),
        QueryMsg::BlockTryAggregateOptional {
            include_cause,
            queries,
        } => to_binary(&block_try_aggregate_optional(
            deps,
            env,
            include_cause,
            queries,
        )?),
    }
}
