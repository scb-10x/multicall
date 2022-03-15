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
    RAggregate {
        queries: Vec<(Addr, Binary)>,
    },
    RTryAggregate {
        require_success: Option<bool>,
        include_cause: Option<bool>,
        queries: Vec<(Addr, Binary)>,
    },
    RTryAggregateOptional {
        include_cause: Option<bool>,
        queries: Vec<(bool, Addr, Binary)>,
    },
    RBlockAggregate {
        queries: Vec<(Addr, Binary)>,
    },
    RBlockTryAggregate {
        require_success: Option<bool>,
        include_cause: Option<bool>,
        queries: Vec<(Addr, Binary)>,
    },
    RBlockTryAggregateOptional {
        include_cause: Option<bool>,
        queries: Vec<(bool, Addr, Binary)>,
    },
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CallResult {
    pub success: bool,
    pub data: Binary,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AggregateResult {
    pub datas: Vec<CallResult>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BlockAggregateResult {
    pub block: u64,
    pub datas: Vec<CallResult>,
}

impl AggregateResult {
    pub fn from_datas(datas: Vec<CallResult>) -> AggregateResult {
        AggregateResult { datas }
    }
}

impl BlockAggregateResult {
    pub fn from_datas(block: u64, datas: Vec<CallResult>) -> BlockAggregateResult {
        BlockAggregateResult { block, datas }
    }
}

impl Default for CallResult {
    fn default() -> Self {
        Self {
            success: false,
            data: Default::default(),
        }
    }
}
