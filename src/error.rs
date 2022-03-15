use cosmwasm_std::{Binary, StdError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Contract execution is not supported")]
    ExecuteNotSupported,
}

#[derive(Error, Debug)]
pub enum QueryError {
    #[error("{0}")]
    System(String),

    #[error("{0}")]
    Contract(String),
}

pub type QueryResult = core::result::Result<Binary, QueryError>;

impl QueryError {
    pub fn std_at_index(self, i: usize) -> StdError {
        StdError::generic_err(format!("Error at index {}, {}", i, self))
    }

    pub fn std(self) -> StdError {
        StdError::generic_err(self)
    }
}

impl Into<String> for QueryError {
    fn into(self) -> String {
        match self {
            QueryError::System(i) => format!("Querier system error: {}", i),
            QueryError::Contract(i) => format!("Querier system error: {}", i),
        }
    }
}

impl From<QueryError> for StdError {
    fn from(source: QueryError) -> Self {
        source.std()
    }
}
