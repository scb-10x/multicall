#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, to_binary, to_vec, Addr, Binary, ContractResult, Deps, DepsMut, Empty, Env,
    MessageInfo, QuerierResult, QueryRequest, Response, StdResult, SystemResult, WasmQuery,
};
use cw2::{get_contract_version, set_contract_version};
use std::ptr;

use crate::{
    error::{ContractError, QueryError, QueryResult},
    msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
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
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ContractVersion {} => to_binary(&get_contract_version(deps.storage)?),
        QueryMsg::Aggregate { queries } => to_binary(&aggregrate(deps, queries)?),
        QueryMsg::TryAggregate {
            require_success,
            queries,
        } => to_binary(&try_aggregate(deps, require_success, queries)?),
        QueryMsg::TryAggregateOptional { queries } => {
            to_binary(&try_aggregate_optional(deps, queries)?)
        }
    }
}

fn process_query_result(result: QuerierResult) -> QueryResult {
    match result {
        SystemResult::Err(system_err) => Err(QueryError::System(system_err.to_string())),
        SystemResult::Ok(ContractResult::Err(contract_err)) => {
            Err(QueryError::Contract(contract_err))
        }
        SystemResult::Ok(ContractResult::Ok(value)) => Ok(value),
    }
}

fn process_wasm_query(address: Addr, binary: Binary) -> StdResult<Vec<u8>> {
    to_vec(&QueryRequest::<Empty>::Wasm(WasmQuery::Smart {
        contract_addr: address.to_string(),
        msg: binary,
    }))
}

fn aggregrate(deps: Deps, mut queries: Vec<(Addr, Binary)>) -> StdResult<Vec<Binary>> {
    let mut result: Vec<Binary> = Vec::with_capacity(queries.len());
    while let Some(target) = queries.pop() {
        result.insert(
            0,
            process_query_result(
                deps.querier
                    .raw_query(&process_wasm_query(target.0, target.1)?),
            )?,
        );
    }

    Ok(result)
}

fn try_aggregate(
    deps: Deps,
    require_success: Option<bool>,
    mut queries: Vec<(Addr, Binary)>,
) -> StdResult<Vec<Binary>> {
    let mut i = queries.len();
    let mut result: Vec<Binary> = vec![Binary::default(); i];
    let req = require_success.unwrap_or(false);
    while let Some(target) = queries.pop() {
        let qr = match process_query_result(
            deps.querier
                .raw_query(&process_wasm_query(target.0, target.1)?),
        ) {
            Ok(res) => Some(res),
            Err(err) => match req {
                true => return Err(err.std_at_index(i)),
                false => None,
            },
        };

        unsafe {
            i -= 1;
            let p = result.as_mut_ptr();
            if let Some(res) = qr {
                ptr::write(p.offset(i as isize), res);
            }
        };
    }

    Ok(result)
}

fn try_aggregate_optional(
    deps: Deps,
    mut queries: Vec<(bool, Addr, Binary)>,
) -> StdResult<Vec<Binary>> {
    let mut i = queries.len();
    let mut result: Vec<Binary> = vec![Binary::default(); i];
    while let Some(target) = queries.pop() {
        let qr = match process_query_result(
            deps.querier
                .raw_query(&process_wasm_query(target.1, target.2)?),
        ) {
            Ok(res) => Some(res),
            Err(err) => match target.0 {
                true => return Err(err.std_at_index(i)),
                false => None,
            },
        };

        unsafe {
            i -= 1;
            let p = result.as_mut_ptr();
            if let Some(res) = qr {
                ptr::write(p.offset(i as isize), res);
            }
        };
    }

    Ok(result)
}
