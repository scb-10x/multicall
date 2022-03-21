use cosmwasm_std::{Addr, Binary};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    ContractVersion {},
    Aggregate {
        queries: Vec<Call>,
    },
    TryAggregate {
        require_success: Option<bool>,
        include_cause: Option<bool>,
        queries: Vec<Call>,
    },
    TryAggregateOptional {
        include_cause: Option<bool>,
        queries: Vec<CallOptional>,
    },
    BlockAggregate {
        queries: Vec<Call>,
    },
    BlockTryAggregate {
        require_success: Option<bool>,
        include_cause: Option<bool>,
        queries: Vec<Call>,
    },
    BlockTryAggregateOptional {
        include_cause: Option<bool>,
        queries: Vec<CallOptional>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Call {
    pub address: Addr,
    pub data: Binary,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CallOptional {
    pub require_success: bool,
    pub address: Addr,
    pub data: Binary,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
pub struct CallResult {
    pub success: bool,
    pub data: Binary,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AggregateResult {
    pub return_data: Vec<CallResult>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BlockAggregateResult {
    pub block: u64,
    pub return_data: Vec<CallResult>,
}

impl AggregateResult {
    pub fn from_return_data(return_data: Vec<CallResult>) -> AggregateResult {
        AggregateResult { return_data }
    }
}

impl BlockAggregateResult {
    pub fn from_return_data(block: u64, return_data: Vec<CallResult>) -> BlockAggregateResult {
        BlockAggregateResult { block, return_data }
    }
}
