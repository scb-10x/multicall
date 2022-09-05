use std::collections::HashSet;

use crate::{
    contract::query,
    mock_querier::{mock_dependencies, AnotherStructResponse, MockQueryMsg},
    msg::{AggregateResult, BlockAggregateResult, Call, CallOptional, QueryMsg},
};
use cosmwasm_std::{from_binary, testing::mock_env, to_binary, Addr, BlockInfo, Env, StdError};
use test_case::test_case;

fn env_with_height(height: u64) -> Env {
    let mock = mock_env();

    Env {
        block: BlockInfo {
            height,
            ..mock.block
        },
        ..mock
    }
}

#[test_case(10; "block ten")]
#[test_case(100; "block hundred")]
#[test_case(1289189451295; "block random")]
fn block_aggregate(x: u64) {
    let deps = mock_dependencies(&[]);
    let env = env_with_height(x);

    let err = query(
        deps.as_ref(),
        env.clone(),
        QueryMsg::BlockAggregate {
            queries: vec![Call {
                address: Addr::unchecked(""),
                data: to_binary(&MockQueryMsg::FailSystem).unwrap(),
            }],
        },
    )
    .unwrap_err();

    assert!(matches!(err, StdError::GenericErr { msg: _ }));

    let q: BlockAggregateResult = from_binary(
        &query(
            deps.as_ref(),
            env.clone(),
            QueryMsg::BlockAggregate {
                queries: vec![Call {
                    address: Addr::unchecked(""),
                    data: to_binary(&MockQueryMsg::One).unwrap(),
                }],
            },
        )
        .unwrap(),
    )
    .unwrap();

    assert_eq!(q.block, x);
    assert_eq!(
        base64::encode(b"1"),
        q.return_data.first().unwrap().data.to_base64()
    );
    assert!(q.return_data.first().unwrap().success);

    let q: BlockAggregateResult = from_binary(
        &query(
            deps.as_ref(),
            env.clone(),
            QueryMsg::BlockTryAggregate {
                require_success: None,
                include_cause: None,
                queries: vec![Call {
                    address: Addr::unchecked(""),
                    data: to_binary(&MockQueryMsg::One).unwrap(),
                }],
            },
        )
        .unwrap(),
    )
    .unwrap();

    assert_eq!(q.block, x);
    assert_eq!(
        base64::encode(b"1"),
        q.return_data.first().unwrap().data.to_base64()
    );
    assert!(q.return_data.first().unwrap().success);

    let q: BlockAggregateResult = from_binary(
        &query(
            deps.as_ref(),
            env,
            QueryMsg::BlockTryAggregateOptional {
                include_cause: None,
                queries: vec![CallOptional {
                    require_success: false,
                    address: Addr::unchecked(""),
                    data: to_binary(&MockQueryMsg::One).unwrap(),
                }],
            },
        )
        .unwrap(),
    )
    .unwrap();

    assert_eq!(q.block, x);
    assert_eq!(
        base64::encode(b"1"),
        q.return_data.first().unwrap().data.to_base64()
    );
    assert!(q.return_data.first().unwrap().success);
}

#[test_case("1"; "number")]
#[test_case("11231123"; "numbers")]
#[test_case("112311233910930150"; "very long numbers")]
#[test_case("x"; "character")]
#[test_case("hello"; "word")]
#[test_case("hello world"; "words")]
fn aggregate(x: &str) {
    let deps = mock_dependencies(&[]);
    let env = mock_env();

    let err = query(
        deps.as_ref(),
        env.clone(),
        QueryMsg::Aggregate {
            queries: vec![Call {
                address: Addr::unchecked(""),
                data: to_binary(&MockQueryMsg::FailSystem).unwrap(),
            }],
        },
    )
    .unwrap_err();

    assert!(matches!(err, StdError::GenericErr { msg: _ }));

    let q: AggregateResult = from_binary(
        &query(
            deps.as_ref(),
            env.clone(),
            QueryMsg::Aggregate {
                queries: vec![Call {
                    address: Addr::unchecked(""),
                    data: to_binary(&MockQueryMsg::Str(x.to_string())).unwrap(),
                }],
            },
        )
        .unwrap(),
    )
    .unwrap();

    assert_eq!(
        base64::encode(x),
        q.return_data.first().unwrap().data.to_base64()
    );
    assert!(q.return_data.first().unwrap().success);

    let q: AggregateResult = from_binary(
        &query(
            deps.as_ref(),
            env,
            QueryMsg::Aggregate {
                queries: vec![
                    Call {
                        address: Addr::unchecked(""),
                        data: to_binary(&MockQueryMsg::Str(x.to_string())).unwrap(),
                    },
                    Call {
                        address: Addr::unchecked(""),
                        data: to_binary(&MockQueryMsg::Str(x.to_string())).unwrap(),
                    },
                ],
            },
        )
        .unwrap(),
    )
    .unwrap();

    assert_eq!(q.return_data.len(), 2);
    assert_eq!(
        base64::encode(x),
        q.return_data.first().unwrap().data.to_base64()
    );
    assert!(q.return_data.first().unwrap().success);
    assert_eq!(
        base64::encode(x),
        q.return_data.last().unwrap().data.to_base64()
    );
    assert!(q.return_data.last().unwrap().success);
}

#[test_case(""; "empty")]
#[test_case("1"; "number")]
#[test_case("11231123"; "numbers")]
#[test_case("112311233910930150"; "very long numbers")]
#[test_case("x"; "character")]
#[test_case("hello"; "word")]
#[test_case("hello world"; "words")]
fn aggregate_struct(x: &str) {
    let deps = mock_dependencies(&[]);
    let env = mock_env();

    let q: AggregateResult = from_binary(
        &query(
            deps.as_ref(),
            env,
            QueryMsg::Aggregate {
                queries: vec![
                    Call {
                        address: Addr::unchecked(""),
                        data: to_binary(&MockQueryMsg::StructStr(x.to_string())).unwrap(),
                    },
                    Call {
                        address: Addr::unchecked(""),
                        data: to_binary(&MockQueryMsg::StructStr(x.to_string())).unwrap(),
                    },
                ],
            },
        )
        .unwrap(),
    )
    .unwrap();

    assert_eq!(q.return_data.len(), 2);

    let first: AnotherStructResponse = from_binary(&q.return_data.first().unwrap().data).unwrap();
    let last: AnotherStructResponse = from_binary(&q.return_data.last().unwrap().data).unwrap();

    assert_eq!(first.result, x);
    assert_eq!(first.another_result, x.to_uppercase());
    assert_eq!(last.result, x);
    assert_eq!(last.another_result, x.to_uppercase());
}

#[test_case(10, vec![]; "no error")]
#[test_case(10, vec![0]; "error at start")]
#[test_case(10, vec![9]; "error at end")]
#[test_case(10, vec![0, 3, 4, 5, 8]; "multiple error")]
#[test_case(5, vec![0, 1, 2, 3, 4]; "all error")]
fn try_aggregate(total: usize, error_at: Vec<usize>) {
    let deps = mock_dependencies(&[]);
    let env = mock_env();

    let body = (0..total)
        .map(|i| Call {
            address: Addr::unchecked(""),
            data: to_binary(&match i {
                _ if error_at.contains(&i) => MockQueryMsg::FailSystem,
                _ => MockQueryMsg::One,
            })
            .unwrap(),
        })
        .collect::<Vec<_>>();

    let q: AggregateResult = from_binary(
        &query(
            deps.as_ref(),
            env.clone(),
            QueryMsg::TryAggregate {
                require_success: Some(false),
                queries: body.clone(),
                include_cause: Some(false),
            },
        )
        .unwrap(),
    )
    .unwrap();

    for (i, bin) in q.return_data.iter().enumerate() {
        assert_eq!(
            match i {
                _ if error_at.contains(&i) => String::new(),
                _ => base64::encode(b"1"),
            },
            bin.data.to_base64()
        );
    }

    let q = query(
        deps.as_ref(),
        env,
        QueryMsg::TryAggregate {
            require_success: Some(true),
            queries: body,
            include_cause: Some(false),
        },
    );

    match error_at[..] {
        [] => assert!(
            matches!(from_binary::<AggregateResult>(&q.unwrap()).unwrap(), x if x.return_data.len() == total)
        ),
        _ => assert!(matches!(q.unwrap_err(), StdError::GenericErr { msg: _ })),
    }
}

#[test_case(10, vec![], vec![]; "no error")]
#[test_case(10, vec![0, 3, 4, 5, 8], vec![]; "multiple error")]
#[test_case(10, vec![0, 3, 4, 5, 8], vec![1, 2, 6]; "multiple error and none require success")]
#[test_case(10, vec![0, 3, 4, 5, 8], vec![3]; "multiple error and some require success")]
#[test_case(10, vec![0, 3, 4, 5, 8], vec![0, 3, 4, 5, 8]; "multiple error and all require success")]
fn try_aggregate_optional(total: usize, error_at: Vec<usize>, required: Vec<usize>) {
    let deps = mock_dependencies(&[]);
    let env = mock_env();

    let body = (0..total)
        .map(|i| CallOptional {
            require_success: false,
            address: Addr::unchecked(""),
            data: to_binary(&match i {
                _ if error_at.contains(&i) => MockQueryMsg::FailSystem,
                _ => MockQueryMsg::One,
            })
            .unwrap(),
        })
        .collect::<Vec<_>>();

    let q: AggregateResult = from_binary(
        &query(
            deps.as_ref(),
            env.clone(),
            QueryMsg::TryAggregateOptional {
                include_cause: Some(false),
                queries: body,
            },
        )
        .unwrap(),
    )
    .unwrap();

    for (i, bin) in q.return_data.iter().enumerate() {
        assert_eq!(
            match i {
                _ if error_at.contains(&i) => String::new(),
                _ => base64::encode(b"1"),
            },
            bin.data.to_base64()
        );
    }

    let body = (0..total)
        .map(|i| CallOptional {
            require_success: matches!(i, _ if required.contains(&i)),
            address: Addr::unchecked(""),
            data: to_binary(&match i {
                _ if error_at.contains(&i) => MockQueryMsg::FailSystem,
                _ => MockQueryMsg::One,
            })
            .unwrap(),
        })
        .collect::<Vec<_>>();

    let q = query(
        deps.as_ref(),
        env,
        QueryMsg::TryAggregateOptional {
            include_cause: Some(false),
            queries: body,
        },
    );

    let err_hs = error_at.iter().collect::<HashSet<_>>();
    let rq_hs = required.iter().collect::<HashSet<_>>();

    match err_hs.intersection(&rq_hs).into_iter().next() {
        Some(_) => assert!(matches!(q.unwrap_err(), StdError::GenericErr { msg: _ })),
        None => assert!(
            matches!(from_binary::<AggregateResult>(&q.unwrap()).unwrap(), x if x.return_data.len() == total)
        ),
    }
}
