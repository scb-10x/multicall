#![feature(test)]

extern crate test;

pub mod contract;
mod error;
pub mod msg;
pub mod querier;

#[cfg(test)]
mod unit_test;

#[cfg(test)]
mod bench;

#[cfg(test)]
pub mod mock_querier;
