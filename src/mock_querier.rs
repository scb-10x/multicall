use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR};
use cosmwasm_std::{
    from_binary, from_slice, to_binary, Addr, Api, Binary, Coin, ContractResult, Decimal,
    OwnedDeps, Querier, QuerierResult, QueryRequest, SystemError, SystemResult, Uint128, WasmQuery,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use terra_cosmwasm::TerraQueryWrapper;
use terra_cosmwasm::{SwapResponse, TaxCapResponse, TaxRateResponse, TerraQuery, TerraRoute};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum MockQueryMsg {
    One,
    Str(String),
    FailSystem,
    FailContract,
    Struct,
    StructAmount(u64),
    StructStr(String),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SomeStructResponse {
    pub address: Addr,
    pub amount: Uint128,
    pub list: Vec<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AnotherStructResponse {
    pub result: String,
    pub another_result: String,
}

pub fn mock_dependencies(
    contract_balance: &[Coin],
) -> OwnedDeps<MockStorage, MockApi, WasmMockQuerier> {
    let contract_addr = MOCK_CONTRACT_ADDR.to_string();
    let custom_querier: WasmMockQuerier = WasmMockQuerier::new(
        MockQuerier::new(&[(&contract_addr, contract_balance)]),
        MockApi::default(),
    );

    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: custom_querier,
    }
}

pub struct WasmMockQuerier {
    base: MockQuerier<TerraQueryWrapper>,
    tax_querier: TaxQuerier,
}

#[derive(Clone, Default)]
pub struct TaxQuerier {
    rate: Decimal,
    // this lets us iterate over all pairs that match the first string
    caps: HashMap<String, Uint128>,
}

//impl TaxQuerier {
//fn new(rate: Decimal, caps: &[(&String, &Uint128)]) -> Self {
//TaxQuerier {
//rate,
//caps: caps_to_map(caps),
//}
//}
//}

//fn caps_to_map(caps: &[(&String, &Uint128)]) -> HashMap<String, Uint128> {
//let mut owner_map: HashMap<String, Uint128> = HashMap::new();
//for (denom, cap) in caps.iter() {
//owner_map.insert(denom.to_string(), **cap);
//}
//owner_map
//}

impl Querier for WasmMockQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        // MockQuerier doesn't support Custom, so we ignore it completely here
        let request: QueryRequest<TerraQueryWrapper> = match from_slice(bin_request) {
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
    pub fn handle_query(&self, request: &QueryRequest<TerraQueryWrapper>) -> QuerierResult {
        match &request {
            QueryRequest::Custom(TerraQueryWrapper { route, query_data }) => {
                if &TerraRoute::Treasury == route {
                    match query_data {
                        TerraQuery::TaxRate {} => {
                            let res = TaxRateResponse {
                                rate: self.tax_querier.rate,
                            };
                            SystemResult::Ok(ContractResult::Ok(to_binary(&res).unwrap()))
                        }
                        TerraQuery::TaxCap { denom } => {
                            let cap = self
                                .tax_querier
                                .caps
                                .get(denom)
                                .copied()
                                .unwrap_or_default();
                            let res = TaxCapResponse { cap };
                            SystemResult::Ok(ContractResult::Ok(to_binary(&res).unwrap()))
                        }
                        _ => panic!("DO NOT ENTER HERE"),
                    }
                } else if route == &TerraRoute::Market {
                    match query_data {
                        TerraQuery::Swap {
                            offer_coin,
                            ask_denom: _,
                        } => {
                            let res = SwapResponse {
                                receive: offer_coin.clone(),
                            };
                            SystemResult::Ok(ContractResult::Ok(to_binary(&res).unwrap()))
                        }
                        _ => panic!("DO NOT ENTER HERE"),
                    }
                } else {
                    panic!("DO NOT ENTER HERE")
                }
            }
            QueryRequest::Wasm(WasmQuery::Smart {
                contract_addr: _,
                msg,
            }) => match from_binary(&msg).unwrap() {
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
                        result: s.clone(),
                        another_result: s.clone().to_uppercase(),
                    })
                    .unwrap(),
                )),
            },
            _ => self.base.handle_query(request),
        }
    }
}

impl WasmMockQuerier {
    pub fn new<A: Api>(base: MockQuerier<TerraQueryWrapper>, _api: A) -> Self {
        WasmMockQuerier {
            base,
            tax_querier: TaxQuerier::default(),
        }
    }

    // configure the tax mock querier
    //pub fn with_tax(&mut self, rate: Decimal, caps: &[(&String, &Uint128)]) {
    //self.tax_querier = TaxQuerier::new(rate, caps);
    //}
}
