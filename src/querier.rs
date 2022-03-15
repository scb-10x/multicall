use cosmwasm_std::{
    to_binary, to_vec, Addr, Binary, ContractResult, Deps, Empty, Env, QuerierResult, QueryRequest,
    StdResult, SystemResult, WasmQuery,
};

use std::ptr;

use crate::{
    error::{QueryError, QueryResult},
    msg::{AggregateResult, BlockAggregateResult, Call, CallOptional, CallResult},
};

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

pub fn r_block_aggregrate(
    deps: Deps,
    env: Env,
    queries: Vec<(Addr, Binary)>,
) -> StdResult<(u64, Vec<Binary>)> {
    let block = env.block.height;
    let result = r_aggregrate(deps, queries)?;

    Ok((block, result))
}

pub fn r_block_try_aggregrate(
    deps: Deps,
    env: Env,
    require_success: Option<bool>,
    include_cause: Option<bool>,
    queries: Vec<(Addr, Binary)>,
) -> StdResult<(u64, Vec<Binary>)> {
    let block = env.block.height;
    let result = r_try_aggregate(deps, require_success, include_cause, queries)?;

    Ok((block, result))
}

pub fn r_block_try_aggregate_optional(
    deps: Deps,
    env: Env,
    include_cause: Option<bool>,
    queries: Vec<(bool, Addr, Binary)>,
) -> StdResult<(u64, Vec<Binary>)> {
    let block = env.block.height;
    let result = r_try_aggregate_optional(deps, include_cause, queries)?;

    Ok((block, result))
}

pub fn r_aggregrate(deps: Deps, mut queries: Vec<(Addr, Binary)>) -> StdResult<Vec<Binary>> {
    let mut i = queries.len();
    let mut result: Vec<Binary> = vec![Binary::default(); i];

    while let Some(target) = queries.pop() {
        let qr = match process_query_result(
            deps.querier
                .raw_query(&process_wasm_query(target.0, target.1)?),
        ) {
            Ok(res) => Some(res),
            Err(err) => return Err(err.std_at_index(i)),
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

pub fn r_try_aggregate(
    deps: Deps,
    require_success: Option<bool>,
    include_cause: Option<bool>,
    mut queries: Vec<(Addr, Binary)>,
) -> StdResult<Vec<Binary>> {
    let mut i = queries.len();
    let mut result: Vec<Binary> = vec![Binary::default(); i];
    let req = require_success.unwrap_or(false);
    let incl = include_cause.unwrap_or(false);

    while let Some(target) = queries.pop() {
        let qr = match process_query_result(
            deps.querier
                .raw_query(&process_wasm_query(target.0, target.1)?),
        ) {
            Ok(res) => Some(res),
            Err(err) => match req {
                true => return Err(err.std_at_index(i)),
                false => match incl {
                    true => Some(to_binary(&err.to_string())?),
                    false => None,
                },
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

pub fn r_try_aggregate_optional(
    deps: Deps,
    include_cause: Option<bool>,
    mut queries: Vec<(bool, Addr, Binary)>,
) -> StdResult<Vec<Binary>> {
    let mut i = queries.len();
    let mut result: Vec<Binary> = vec![Binary::default(); i];
    let incl = include_cause.unwrap_or(false);

    while let Some(target) = queries.pop() {
        let qr = match process_query_result(
            deps.querier
                .raw_query(&process_wasm_query(target.1, target.2)?),
        ) {
            Ok(res) => Some(res),
            Err(err) => match target.0 {
                true => return Err(err.std_at_index(i)),
                false => match incl {
                    true => Some(to_binary(&err.to_string())?),
                    false => None,
                },
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

pub fn block_aggregrate(
    deps: Deps,
    env: Env,
    queries: Vec<Call>,
) -> StdResult<BlockAggregateResult> {
    let block = env.block.height;
    let result = aggregrate(deps, queries)?;

    Ok(BlockAggregateResult::from_datas(block, result.datas))
}

pub fn block_try_aggregrate(
    deps: Deps,
    env: Env,
    require_success: Option<bool>,
    include_cause: Option<bool>,
    queries: Vec<Call>,
) -> StdResult<BlockAggregateResult> {
    let block = env.block.height;
    let result = try_aggregate(deps, require_success, include_cause, queries)?;

    Ok(BlockAggregateResult::from_datas(block, result.datas))
}

pub fn block_try_aggregate_optional(
    deps: Deps,
    env: Env,
    include_cause: Option<bool>,
    queries: Vec<CallOptional>,
) -> StdResult<BlockAggregateResult> {
    let block = env.block.height;
    let result = try_aggregate_optional(deps, include_cause, queries)?;

    Ok(BlockAggregateResult::from_datas(block, result.datas))
}

pub fn aggregrate(deps: Deps, mut queries: Vec<Call>) -> StdResult<AggregateResult> {
    let mut i = queries.len();
    let mut result: Vec<CallResult> = vec![CallResult::default(); i];

    while let Some(target) = queries.pop() {
        let qr = match process_query_result(
            deps.querier
                .raw_query(&process_wasm_query(target.address, target.data)?),
        ) {
            Ok(res) => Some(res),
            Err(err) => return Err(err.std_at_index(i)),
        };

        unsafe {
            i -= 1;
            let p = result.as_mut_ptr();
            if let Some(res) = qr {
                ptr::write(
                    p.offset(i as isize),
                    CallResult {
                        success: true,
                        data: res,
                    },
                );
            }
        };
    }

    Ok(AggregateResult::from_datas(result))
}

pub fn try_aggregate(
    deps: Deps,
    require_success: Option<bool>,
    include_cause: Option<bool>,
    mut queries: Vec<Call>,
) -> StdResult<AggregateResult> {
    let mut i = queries.len();
    let mut result: Vec<CallResult> = vec![CallResult::default(); i];
    let req = require_success.unwrap_or(false);
    let incl = include_cause.unwrap_or(false);

    while let Some(target) = queries.pop() {
        let (suc, qr) = match process_query_result(
            deps.querier
                .raw_query(&process_wasm_query(target.address, target.data)?),
        ) {
            Ok(res) => (true, Some(res)),
            Err(err) => match req {
                true => return Err(err.std_at_index(i)),
                false => match incl {
                    true => (false, Some(to_binary(&err.to_string())?)),
                    false => (false, None),
                },
            },
        };

        unsafe {
            i -= 1;
            let p = result.as_mut_ptr();
            if let Some(res) = qr {
                ptr::write(
                    p.offset(i as isize),
                    CallResult {
                        success: suc,
                        data: res,
                    },
                );
            }
        };
    }

    Ok(AggregateResult::from_datas(result))
}

pub fn try_aggregate_optional(
    deps: Deps,
    include_cause: Option<bool>,
    mut queries: Vec<CallOptional>,
) -> StdResult<AggregateResult> {
    let mut i = queries.len();
    let mut result: Vec<CallResult> = vec![CallResult::default(); i];
    let incl = include_cause.unwrap_or(false);

    while let Some(target) = queries.pop() {
        let (suc, qr) = match process_query_result(
            deps.querier
                .raw_query(&process_wasm_query(target.address, target.data)?),
        ) {
            Ok(res) => (true, Some(res)),
            Err(err) => match target.require_success {
                true => return Err(err.std_at_index(i)),
                false => match incl {
                    true => (false, Some(to_binary(&err.to_string())?)),
                    false => (false, None),
                },
            },
        };

        unsafe {
            i -= 1;
            let p = result.as_mut_ptr();
            if let Some(res) = qr {
                ptr::write(
                    p.offset(i as isize),
                    CallResult {
                        success: suc,
                        data: res,
                    },
                );
            }
        };
    }

    Ok(AggregateResult::from_datas(result))
}
