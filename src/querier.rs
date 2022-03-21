use cosmwasm_std::{
    to_binary, to_vec, Addr, Binary, ContractResult, Deps, Empty, Env, QuerierResult, QueryRequest,
    StdResult, SystemResult, WasmQuery,
};

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

pub fn block_aggregrate(
    deps: Deps,
    env: Env,
    queries: Vec<Call>,
) -> StdResult<BlockAggregateResult> {
    let block = env.block.height;
    let result = aggregrate(deps, queries)?;

    Ok(BlockAggregateResult::from_return_data(
        block,
        result.return_data,
    ))
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

    Ok(BlockAggregateResult::from_return_data(
        block,
        result.return_data,
    ))
}

pub fn block_try_aggregate_optional(
    deps: Deps,
    env: Env,
    include_cause: Option<bool>,
    queries: Vec<CallOptional>,
) -> StdResult<BlockAggregateResult> {
    let block = env.block.height;
    let result = try_aggregate_optional(deps, include_cause, queries)?;

    Ok(BlockAggregateResult::from_return_data(
        block,
        result.return_data,
    ))
}

pub fn aggregrate(deps: Deps, queries: Vec<Call>) -> StdResult<AggregateResult> {
    let n = queries.len();
    let mut result: Vec<CallResult> = vec![CallResult::default(); n];

    for i in 0..n {
        let query = queries[i].clone();
        let wasm = &process_wasm_query(query.address, query.data)?;
        let res = deps.querier.raw_query(wasm);
        let data = match process_query_result(res) {
            Ok(res) => res,
            Err(err) => return Err(err.std_at_index(i)),
        };
        result[i] = CallResult {
            success: true,
            data,
        };
    }

    Ok(AggregateResult::from_return_data(result))
}

pub fn try_aggregate(
    deps: Deps,
    require_success: Option<bool>,
    include_cause: Option<bool>,
    queries: Vec<Call>,
) -> StdResult<AggregateResult> {
    let n = queries.len();
    let mut result: Vec<CallResult> = vec![CallResult::default(); n];

    for i in 0..n {
        let query = queries[i].clone();
        let wasm = &process_wasm_query(query.address, query.data)?;
        let res = deps.querier.raw_query(wasm);
        result[i] = match process_query_result(res) {
            Ok(res) => CallResult {
                success: true,
                data: res,
            },
            Err(err) => match require_success.unwrap_or(false) {
                true => return Err(err.std_at_index(i)),
                false => match include_cause.unwrap_or(false) {
                    true => CallResult {
                        success: false,
                        data: to_binary(&err.to_string())?,
                    },
                    false => CallResult {
                        success: false,
                        data: Binary::default(),
                    },
                },
            },
        };
    }

    Ok(AggregateResult::from_return_data(result))
}

pub fn try_aggregate_optional(
    deps: Deps,
    include_cause: Option<bool>,
    queries: Vec<CallOptional>,
) -> StdResult<AggregateResult> {
    let n = queries.len();
    let mut result: Vec<CallResult> = vec![CallResult::default(); n];

    for i in 0..n {
        let query = queries[i].clone();
        let wasm = &process_wasm_query(query.address, query.data)?;
        let res = deps.querier.raw_query(wasm);
        result[i] = match process_query_result(res) {
            Ok(res) => CallResult {
                success: true,
                data: res,
            },
            Err(err) => match query.require_success {
                true => return Err(err.std_at_index(i)),
                false => match include_cause.unwrap_or(false) {
                    true => CallResult {
                        success: false,
                        data: to_binary(&err.to_string())?,
                    },
                    false => CallResult {
                        success: false,
                        data: Binary::default(),
                    },
                },
            },
        };
    }

    Ok(AggregateResult::from_return_data(result))
}
