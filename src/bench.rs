#[cfg(test)]
mod benchs {
    use cosmwasm_std::{from_binary, testing::mock_env, to_binary, Addr};
    use test::Bencher;

    use crate::{
        contract::query,
        mock_querier::{mock_dependencies, MockQueryMsg},
        msg::{AggregateResult, Call, QueryMsg},
    };

    fn aggregate() {
        let deps = mock_dependencies(&[]);
        let env = mock_env();
        let queries = (0..100)
            .map(|i| Call {
                address: Addr::unchecked(""),
                data: to_binary(&MockQueryMsg::StructStr((i * 100).to_string())).unwrap(),
            })
            .collect::<Vec<_>>();

        from_binary::<AggregateResult>(
            &query(deps.as_ref(), env.clone(), QueryMsg::Aggregate { queries }).unwrap(),
        )
        .unwrap();
    }

    #[bench]
    fn bench_aggregate(b: &mut Bencher) {
        b.iter(|| aggregate());
    }
}
