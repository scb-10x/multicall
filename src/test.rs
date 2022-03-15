#[cfg(test)]
mod tests {
    use crate::{
        contract::query,
        mock_querier::{mock_dependencies, AnotherStructResponse, MockQueryMsg},
        msg::QueryMsg,
    };
    use cosmwasm_std::{from_binary, testing::mock_env, to_binary, Addr, Binary, StdError};
    use test_case::test_case;

    #[test_case(""; "empty")]
    #[test_case("1"; "number")]
    #[test_case("11231123"; "numbers")]
    #[test_case("112311233910930150"; "very long numbers")]
    #[test_case("x"; "character")]
    #[test_case("hello"; "word")]
    #[test_case("hello world"; "words")]
    fn aggregate(x: &str) {
        let deps = mock_dependencies(&[]);
        let env = mock_env();

        let q: Vec<Binary> = from_binary(
            &query(
                deps.as_ref(),
                env.clone(),
                QueryMsg::Aggregate {
                    queries: vec![(
                        Addr::unchecked(""),
                        to_binary(&MockQueryMsg::Str(x.to_string())).unwrap(),
                    )],
                },
            )
            .unwrap(),
        )
        .unwrap();

        assert_eq!(base64::encode(x), q.first().unwrap().to_base64());

        let q: Vec<Binary> = from_binary(
            &query(
                deps.as_ref(),
                env.clone(),
                QueryMsg::Aggregate {
                    queries: vec![
                        (
                            Addr::unchecked(""),
                            to_binary(&MockQueryMsg::Str(x.to_string())).unwrap(),
                        ),
                        (
                            Addr::unchecked(""),
                            to_binary(&MockQueryMsg::Str(x.to_string())).unwrap(),
                        ),
                    ],
                },
            )
            .unwrap(),
        )
        .unwrap();

        assert_eq!(q.len(), 2);
        assert_eq!(base64::encode(x), q.first().unwrap().to_base64());
        assert_eq!(base64::encode(x), q.last().unwrap().to_base64());
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

        let q: Vec<Binary> = from_binary(
            &query(
                deps.as_ref(),
                env.clone(),
                QueryMsg::Aggregate {
                    queries: vec![
                        (
                            Addr::unchecked(""),
                            to_binary(&MockQueryMsg::StructStr(x.to_string())).unwrap(),
                        ),
                        (
                            Addr::unchecked(""),
                            to_binary(&MockQueryMsg::StructStr(x.to_string())).unwrap(),
                        ),
                    ],
                },
            )
            .unwrap(),
        )
        .unwrap();

        assert_eq!(q.len(), 2);

        let first: AnotherStructResponse = from_binary(q.first().unwrap()).unwrap();
        let last: AnotherStructResponse = from_binary(q.last().unwrap()).unwrap();

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
            .map(|i| {
                (
                    Addr::unchecked(""),
                    to_binary(&match i {
                        _ if error_at.contains(&i) => MockQueryMsg::FailSystem,
                        _ => MockQueryMsg::One,
                    })
                    .unwrap(),
                )
            })
            .collect::<Vec<_>>();

        let q: Vec<Binary> = from_binary(
            &query(
                deps.as_ref(),
                env.clone(),
                QueryMsg::TryAggregate {
                    require_success: Some(false),
                    queries: body.clone(),
                },
            )
            .unwrap(),
        )
        .unwrap();

        for (i, bin) in q.iter().enumerate() {
            assert_eq!(
                match i {
                    _ if error_at.contains(&i) => String::new(),
                    _ => base64::encode(b"1"),
                },
                bin.to_base64()
            );
        }

        let q = query(
            deps.as_ref(),
            env.clone(),
            QueryMsg::TryAggregate {
                require_success: Some(true),
                queries: body.clone(),
            },
        );

        match error_at[..] {
            [] => assert!(
                matches!(from_binary::<Vec<Binary>>(&q.unwrap()).unwrap(), x if x.len() == total)
            ),
            _ => assert!(matches!(q.unwrap_err(), StdError::GenericErr { msg: _ })),
        }
    }

    #[test]
    fn try_aggregate_optional() {
        let deps = mock_dependencies(&[]);
        let env = mock_env();

        let q: Vec<Binary> = from_binary(
            &query(
                deps.as_ref(),
                env.clone(),
                QueryMsg::TryAggregateOptional {
                    queries: vec![
                        (
                            true,
                            Addr::unchecked(""),
                            to_binary(&MockQueryMsg::One).unwrap(),
                        ),
                        (
                            true,
                            Addr::unchecked(""),
                            to_binary(&MockQueryMsg::Str("2".to_string())).unwrap(),
                        ),
                        (
                            true,
                            Addr::unchecked(""),
                            to_binary(&MockQueryMsg::Str("3".to_string())).unwrap(),
                        ),
                        (
                            true,
                            Addr::unchecked(""),
                            to_binary(&MockQueryMsg::Str("4".to_string())).unwrap(),
                        ),
                        (
                            false,
                            Addr::unchecked(""),
                            to_binary(&MockQueryMsg::FailSystem).unwrap(),
                        ),
                    ],
                },
            )
            .unwrap(),
        )
        .unwrap();

        let mut it = q.iter();
        assert_eq!(base64::encode(b"1"), it.next().unwrap().to_base64());
        assert_eq!(base64::encode(b"2"), it.next().unwrap().to_base64());
        assert_eq!(base64::encode(b"3"), it.next().unwrap().to_base64());
        assert_eq!(base64::encode(b"4"), it.next().unwrap().to_base64());
        assert_eq!(base64::encode(b""), it.next().unwrap().to_base64());
        assert_eq!(None, it.next());

        let err = query(
            deps.as_ref(),
            env.clone(),
            QueryMsg::TryAggregateOptional {
                queries: vec![
                    (
                        false,
                        Addr::unchecked(""),
                        to_binary(&MockQueryMsg::FailContract).unwrap(),
                    ),
                    (
                        true,
                        Addr::unchecked(""),
                        to_binary(&MockQueryMsg::FailSystem).unwrap(),
                    ),
                ],
            },
        )
        .unwrap_err();

        assert!(matches!(err, StdError::GenericErr { msg: _ }));
    }
}
