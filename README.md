# Multicall

On-chain query aggregator/batcher in Terra.

---

Mainnet Code Id: `3758`

Mainnet Address: [`terra1y60jx2jqh5qpmcnvgz3n0zg2p6ky4mr6ax2qa5`](https://finder.extraterrestrial.money/mainnet/contract/terra1y60jx2jqh5qpmcnvgz3n0zg2p6ky4mr6ax2qa5)

Testnet Code Id: `53261`

Testnet Address: [`terra1z9p02s5fkasx5qxdaes6mfyf2gt3kxuhcsd4va`]
(https://finder.extraterrestrial.money/testnet/account/terra1z9p02s5fkasx5qxdaes6mfyf2gt3kxuhcsd4va)

## Example Usage

### Aggregate

#### Aggregate

Example Query: [Link](https://bombay-fcd.terra.dev/wasm/contracts/terra1z9p02s5fkasx5qxdaes6mfyf2gt3kxuhcsd4va/store?query_msg=%7B%22aggregate%22:%7B%22queries%22:%5B%7B%22address%22:%22terra15dwd5mj8v59wpj0wvt233mf5efdff808c5tkal%22,%22data%22:%22eyJjb25maWciOnt9fQ==%22%7D,%7B%22address%22:%22terra15dwd5mj8v59wpj0wvt233mf5efdff808c5tkal%22,%22data%22:%22eyJlcG9jaF9zdGF0ZSI6e319%22%7D%5D%7D%7D)

```ts
const multicallRes: any = await terra.wasm.contractQuery(multicall, {
  aggregate: {
    queries: [
      {
        address: "terra15dwd5mj8v59wpj0wvt23mf5efdff808c5tkal",
        data: toBase64({ config: {} }),
      },
      {
        address: "terra15dwd5mj8v59wpj0wvt233mf5efdff808c5tkal",
        data: toBase64({ epoch_state: {} }),
      },
    ],
  },
})

console.log(multicallRes)

// ---
{
  datas: [
    {
      success: true,
      data: "eyJvd25lcl9hZGRyIjoidGVycmExNmNrZXV1N2M2Z2d1NTJhOHNlMD....",
    },
    {
      success: true,
      data: "eyJleGNoYWnZV9yYXRlIjoiMS4yMzE0NTYyNzU4MjA1MDYwMDQiLC.....",
    },
  ]
}
// ---

const decoded = multicallRes.return_data.map((e) => {
  return JSON.parse(Buffer.from(e.data, "base64").toString())
})

console.log(decoded)[
  // ---
  ({
    owner_addr: "terra16ckeuu7c6ggu52a8se005mg5c0kd2kmuun63cu",
    aterra_contract: "terra1ajt556dpzvjwl0kl5tzku3fc3p3knkg9mkv8jl",
    interest_model: "terra1m25aqupscdw2kw4tnq5ql6hexgr34mr76azh5x",
    distribution_model: "terra1u64cezah94sq3ye8y0ung28x3pxc37tv8fth7h",
    overseer_contract: "terra1qljxd0y3j3gk97025qvl3lgq8ygup4gsksvaxv",
    collector_contract: "terra1hlctcrrhcl2azxzcsns467le876cfuzam6jty4",
    distributor_contract: "terra1z7nxemcnm8kp7fs33cs7ge4wfuld307v80gypj",
    stable_denom: "uusd",
    max_borrow_factor: "0.95",
  },
  {
    exchange_rate: "1.231456275820506004",
    aterra_supply: "146558727243845",
  })
]
```

#### Try Aggregate

Aggregate with error suppression variant. If `include_cause` is `true`, `data` of the query will be error message in `String` if that query is return error. Else will return **empty string**.

Example Query: [Link](https://bombay-fcd.terra.dev/wasm/contracts/terra1z9p02s5fkasx5qxdaes6mfyf2gt3kxuhcsd4va/store?query_msg=%7B%22try_aggregate%22:%7B%22require_success%22:false,%22include_cause%22:true,%22queries%22:%5B%7B%22address%22:%22terra15dwd5mj8v59wpj0wvt233mf5efdff808c5tkal%22,%22data%22:%22eyJjb25maWciOnt9fQ==%22%7D,%7B%22address%22:%22terra15dwd5mj8v59wpj0wvt233mf5efdff808c5tkal%22,%22data%22:%22eyJlcG9jaF9zdGF0ZSI6e319%22%7D%5D%7D%7D)

```ts
const multicallRes: any = await terra.wasm.contractQuery(multicall, {
  try_aggregate: {
    require_success: false, // defualt to false
    include_cause: true, // default to false
    queries: [
      {
        address: "terra15dwd5mj8v59wpj0wvt233mf5efdff808c5tkal",
        data: toBase64({ config: {} }),
      },
      {
        address: "terra15dwd5mj8v59wpj0wvt233mf5efdff808c5tkal",
        data: toBase64({ epoch_state: {} }),
      },
    ],
  },
})

const decoded = multicallRes.return_data.map((e) => {
  return e.length == 0
    ? null
    : JSON.parse(Buffer.from(e.data, "base64").toString())
})
```

#### Try Aggregate With Optional Require Success

Aggregate with specific error suppression variant. Same as `try_aggregate` but with element-specific error handling.

Example Query: [Link](https://bombay-fcd.terra.dev/wasm/contracts/terra1z9p02s5fkasx5qxdaes6mfyf2gt3kxuhcsd4va/store?query_msg=%7B%22try_aggregate_optional%22:%7B%22include_cause%22:true,%22queries%22:%5B%7B%22require_success%22:true,%22address%22:%22terra15dwd5mj8v59wpj0wvt233mf5efdff808c5tkal%22,%22data%22:%22eyJjb25maWciOnt9fQ==%22%7D,%7B%22require_success%22:false,%22address%22:%22terra15dwd5mj8v59wpj0wvt233mf5efdff808c5tkal%22,%22data%22:%22eyJlcG9jaF9zdGF0ZSI6e319%22%7D%5D%7D%7D)

```ts
const multicallRes: any = await terra.wasm.contractQuery(multicall, {
  try_aggregate_optional: {
    include_cause: true, // default to false
    queries: []]]]
      {
        require_success: true,
        address: "terra15dwd5mj8v59wpj0wvt233mf5efdff808c5tkal",
        data: toBase64({ config: {} })
      },
      {
        require_success: false,
        address: "terra15dwd5mj8v59wpj0wvt233mf5efdff808c5tkal",
        data: toBase64({ epoch_state: {} })
      },
    ]
  }
})

const decoded = multicallRes.return_data.map((e) => {
  return e.length == 0 ? null : JSON.parse(Buffer.from(e.data, 'base64').toString())
})
```

### Aggregate With Block

Include `block_` as prefix for query message to include block height as a result.

Example Query: [Link](https://bombay-fcd.terra.dev/wasm/contracts/terra1z9p02s5fkasx5qxdaes6mfyf2gt3kxuhcsd4va/store?query_msg=%7B%22block_aggregate%22:%7B%22queries%22:%5B%7B%22address%22:%22terra15dwd5mj8v59wpj0wvt233mf5efdff808c5tkal%22,%22data%22:%22eyJjb25maWciOnt9fQ==%22%7D,%7B%22address%22:%22terra15dwd5mj8v59wpj0wvt233mf5efdff808c5tkal%22,%22data%22:%22eyJlcG9jaF9zdGF0ZSI6e319%22%7D%5D%7D%7D)

```ts
const multicallRes: any = await terra.wasm.contractQuery(multicall, {
  block_aggregate: {
    queries: [
      {
        address: "terra15dwd5mj8v59wpj0wvt233mf5efdff808c5tkal",
        data: toBase64({ config: {} })
      },
      {
        address: "terra15dwd5mj8v59wpj0wvt233mf5efdff808c5tkal",
        data: toBase64({ epoch_state: {} })
      },
    ]
  }
})

console.log(multicallRes)

// --

{
  block: 8259453,
  return_data: [
    {
      success: true,
      data: 'eyJvd25lcl9hZGRyIjoidGVycmExNmNrZXV1N2M2Z2d1NTJhOHNlMDA1bWc1YzBrZDJrbXV1bjYzY3UiLCJhdGVycmFfY29udHJhY3QiOiJ0ZXJyYTFhanQ1NTZkcHp2andsMGtsNXR6a3UzZmMzcDNrbmtnOW1rdjhqbCIsImludGVyZXN0X21vZGVsIjoidGVycmExbTI1YXF1cHNjZHcya3c0dG5xNXFsNmhleGdyMzRtcjc2YXpoNXgiLCJkaXN0cmlidXRpb25fbW9kZWwiOiJ0ZXJyYTF1NjRjZXphaDk0c3EzeWU4eTB1bmcyOHgzcHhjMzd0djhmdGg3aCIsIm92ZXJzZWVyX2NvbnRyYWN0IjoidGVycmExcWxqeGQweTNqM2drOTcwMjVxdmwzbGdxOHlndXA0Z3Nrc3ZheHYiLCJjb2xsZWN0b3JfY29udHJhY3QiOiJ0ZXJyYTFobGN0Y3JyaGNsMmF6eHpjc25zNDY3bGU4NzZjZnV6YW02anR5NCIsImRpc3RyaWJ1dG9yX2NvbnRyYWN0IjoidGVycmExejdueGVtY25tOGtwN2ZzMzNjczdnZTR3ZnVsZDMwN3Y4MGd5cGoiLCJzdGFibGVfZGVub20iOiJ1dXNkIiwibWF4X2JvcnJvd19mYWN0b3IiOiIwLjk1In0='
    },
    {
      success: true,
      data: 'eyJleGNoYW5nZV9yYXRlIjoiMS4yMzIxNzc1ODQ0NTQzOTY2OTQiLCJhdGVycmFfc3VwcGx5IjoiMTQxNjE3NTE5MTk2NTY2In0='
    }
  ]
}
```
