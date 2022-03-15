#[cfg(test)]
mod tests {
    use crate::{
        contract::query,
        mock_querier::{mock_dependencies, AnotherStructResponse, MockQueryMsg},
        msg::{AggregateResult, Call, CallOptional, QueryMsg},
    };
    use cosmwasm_std::{from_binary, testing::mock_env, to_binary, Addr, StdError};
    use test_case::test_case;

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

        assert_eq!(base64::encode(x), q.datas.first().unwrap().data.to_base64());
        assert_eq!(true, q.datas.first().unwrap().success);

        let q: AggregateResult = from_binary(
            &query(
                deps.as_ref(),
                env.clone(),
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

        assert_eq!(q.datas.len(), 2);
        assert_eq!(base64::encode(x), q.datas.first().unwrap().data.to_base64());
        assert_eq!(true, q.datas.first().unwrap().success);
        assert_eq!(base64::encode(x), q.datas.last().unwrap().data.to_base64());
        assert_eq!(true, q.datas.last().unwrap().success);
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
                env.clone(),
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

        assert_eq!(q.datas.len(), 2);

        let first: AnotherStructResponse = from_binary(&q.datas.first().unwrap().data).unwrap();
        let last: AnotherStructResponse = from_binary(&q.datas.last().unwrap().data).unwrap();

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

        for (i, bin) in q.datas.iter().enumerate() {
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
            env.clone(),
            QueryMsg::TryAggregate {
                require_success: Some(true),
                queries: body.clone(),
                include_cause: Some(false),
            },
        );

        match error_at[..] {
            [] => assert!(
                matches!(from_binary::<AggregateResult>(&q.unwrap()).unwrap(), x if x.datas.len() == total)
            ),
            _ => assert!(matches!(q.unwrap_err(), StdError::GenericErr { msg: _ })),
        }
    }

    #[test]
    fn try_aggregate_optional() {
        let deps = mock_dependencies(&[]);
        let env = mock_env();

        let q: AggregateResult = from_binary(
            &query(
                deps.as_ref(),
                env.clone(),
                QueryMsg::TryAggregateOptional {
                    include_cause: Some(false),
                    queries: vec![
                        CallOptional {
                            require_success: true,
                            address: Addr::unchecked(""),
                            data: to_binary(&MockQueryMsg::One).unwrap(),
                        },
                        CallOptional {
                            require_success: true,
                            address: Addr::unchecked(""),
                            data: to_binary(&MockQueryMsg::Str("2".to_string())).unwrap(),
                        },
                        CallOptional {
                            require_success: true,
                            address: Addr::unchecked(""),
                            data: to_binary(&MockQueryMsg::Str("3".to_string())).unwrap(),
                        },
                        CallOptional {
                            require_success: true,
                            address: Addr::unchecked(""),
                            data: to_binary(&MockQueryMsg::Str("4".to_string())).unwrap(),
                        },
                        CallOptional {
                            require_success: false,
                            address: Addr::unchecked(""),
                            data: to_binary(&MockQueryMsg::FailSystem).unwrap(),
                        },
                    ],
                },
            )
            .unwrap(),
        )
        .unwrap();

        let mut it = q.datas.iter();
        assert_eq!(base64::encode(b"1"), it.next().unwrap().data.to_base64());
        assert_eq!(base64::encode(b"2"), it.next().unwrap().data.to_base64());
        assert_eq!(base64::encode(b"3"), it.next().unwrap().data.to_base64());
        assert_eq!(base64::encode(b"4"), it.next().unwrap().data.to_base64());
        assert_eq!(base64::encode(b""), it.next().unwrap().data.to_base64());
        assert_eq!(None, it.next());

        let err = query(
            deps.as_ref(),
            env.clone(),
            QueryMsg::TryAggregateOptional {
                include_cause: Some(false),
                queries: vec![
                    CallOptional {
                        require_success: false,
                        address: Addr::unchecked(""),
                        data: to_binary(&MockQueryMsg::FailContract).unwrap(),
                    },
                    CallOptional {
                        require_success: true,
                        address: Addr::unchecked(""),
                        data: to_binary(&MockQueryMsg::FailSystem).unwrap(),
                    },
                ],
            },
        )
        .unwrap_err();

        assert!(matches!(err, StdError::GenericErr { msg: _ }));
    }
}
