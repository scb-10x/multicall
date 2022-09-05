use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR};
use cosmwasm_std::{
    from_binary, from_slice, to_binary, Addr, Api, Binary, Coin, ContractResult, Empty, OwnedDeps,
    Querier, QuerierResult, QueryRequest, SystemError, SystemResult, Uint128, WasmQuery,
};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum MockQueryMsg {
    One,
    Str(String),
    FailSystem,
    FailContract,
    Struct,
    StructAmount(u64),
    StructStr(String),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SomeStructResponse {
    pub address: Addr,
    pub amount: Uint128,
    pub list: Vec<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct AnotherStructResponse {
    pub result: String,
    pub another_result: String,
}

pub fn mock_dependencies(
    contract_balance: &[Coin],
) -> OwnedDeps<MockStorage, MockApi, WasmMockQuerier> {
    let contract_addr = MOCK_CONTRACT_ADDR.to_string();
    let custom_querier: WasmMockQuerier = WasmMockQuerier::new(
        MockQuerier::<Empty>::new(&[(&contract_addr, contract_balance)]),
        MockApi::default(),
    );

    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: custom_querier,
        custom_query_type: PhantomData,
    }
}

pub struct WasmMockQuerier {
    base: MockQuerier<Empty>,
}

impl Querier for WasmMockQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        // MockQuerier doesn't support Custom, so we ignore it completely here
        let request: QueryRequest<Empty> = match from_slice(bin_request) {
            Ok(v) => v,
            Err(e) => {
                return SystemResult::Err(SystemError::InvalidRequest {
                    error: format!("Parsing query request: {}", e),
                    request: bin_request.into(),
                })
            }
        };
        self.handle_query(&request)
    }
}

impl WasmMockQuerier {
    pub fn handle_query(&self, request: &QueryRequest<Empty>) -> QuerierResult {
        match &request {
            QueryRequest::Wasm(WasmQuery::Smart {
                contract_addr: _,
                msg,
            }) => match from_binary(msg).unwrap() {
                MockQueryMsg::One {} => SystemResult::Ok(ContractResult::Ok(
                    Binary::from_base64(base64::encode(b"1").as_str()).unwrap(),
                )),
                MockQueryMsg::Str(i) => SystemResult::Ok(ContractResult::Ok(
                    Binary::from_base64(base64::encode(i).as_str()).unwrap(),
                )),
                MockQueryMsg::FailSystem => SystemResult::Err(SystemError::Unknown {}),
                MockQueryMsg::FailContract => {
                    SystemResult::Ok(ContractResult::Err(String::from("error")))
                }
                MockQueryMsg::Struct => SystemResult::Ok(ContractResult::Ok(
                    to_binary(&SomeStructResponse {
                        address: Addr::unchecked("random"),
                        amount: Uint128::from(100_000_000u64),
                        list: vec![0, 1, 2, 3, 4, 5],
                    })
                    .unwrap(),
                )),
                MockQueryMsg::StructAmount(amt) => SystemResult::Ok(ContractResult::Ok(
                    to_binary(&SomeStructResponse {
                        address: Addr::unchecked("random_amount"),
                        amount: Uint128::from(amt * 3),
                        list: vec![amt, amt + 1, amt + 2],
                    })
                    .unwrap(),
                )),
                MockQueryMsg::StructStr(s) => SystemResult::Ok(ContractResult::Ok(
                    to_binary(&AnotherStructResponse {
                        another_result: s.to_uppercase(),
                        result: s,
                    })
                    .unwrap(),
                )),
            },
            _ => self.base.handle_query(request),
        }
    }
}

impl WasmMockQuerier {
    pub fn new<A: Api>(base: MockQuerier<Empty>, _api: A) -> Self {
        WasmMockQuerier { base }
    }
}
