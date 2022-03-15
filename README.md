# Multicall

On-chain query aggregator/batcher in Terra.

---

Testnet Code Id: `52971`
Testnet Address: `terra1z9p02s5fkasx5qxdaes6mfyf2gt3kxuhcsd4va`

## Example Usage

### Aggregate

```ts
const multicallRes: any = await terra.wasm.contractQuery(multicall, {
  try_aggregate: {
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

// ---
{
  datas: [
    {
      success: true,
      data: 'eyJvd25lcl9hZGRyIjoidGVycmExNmNrZXV1N2M2Z2d1NTJhOHNlMD....'
    },
    {
      success: true,
      data: 'eyJleGNoYWnZV9yYXRlIjoiMS4yMzE0NTYyNzU4MjA1MDYwMDQiLC.....'
    }
  ]
}
// ---

const decoded = multicallRes.datas.map((e) => {
  return e.length == 0 ? null : JSON.parse(Buffer.from(e.data, 'base64').toString())
})

console.log(decoded)

// ---
[
  {
    owner_addr: 'terra16ckeuu7c6ggu52a8se005mg5c0kd2kmuun63cu',
    aterra_contract: 'terra1ajt556dpzvjwl0kl5tzku3fc3p3knkg9mkv8jl',
    interest_model: 'terra1m25aqupscdw2kw4tnq5ql6hexgr34mr76azh5x',
    distribution_model: 'terra1u64cezah94sq3ye8y0ung28x3pxc37tv8fth7h',
    overseer_contract: 'terra1qljxd0y3j3gk97025qvl3lgq8ygup4gsksvaxv',
    collector_contract: 'terra1hlctcrrhcl2azxzcsns467le876cfuzam6jty4',
    distributor_contract: 'terra1z7nxemcnm8kp7fs33cs7ge4wfuld307v80gypj',
    stable_denom: 'uusd',
    max_borrow_factor: '0.95'
  },
  {
    exchange_rate: '1.231456275820506004',
    aterra_supply: '146558727243845'
  }
]
```

### Raw Aggregate

This directly parse input into WasmMsg without deconstructing the input struct first.

```ts
const multicallRes: string[] = await terra.wasm.contractQuery(multicall, {
  r_try_aggregate: {
    queries: [
      [
        "terra15dwd5mj8v59wpj0wvt233mf5efdff808c5tkal",
        toBase64({ config: {} })
      ],
      [
        "terra15dwd5mj8v59wpj0wvt233mf5efdff808c5tkal",
        toBase64({ epoch_state: {} })
      ]
    ]
  }
})

const decoded = multicallRes.map((e) =>
  e.length == 0 ? null : JSON.parse(Buffer.from(e, 'base64').toString())
)

console.log(decoded)

// ---
[
  {
    owner_addr: 'terra16ckeuu7c6ggu52a8se005mg5c0kd2kmuun63cu',
    aterra_contract: 'terra1ajt556dpzvjwl0kl5tzku3fc3p3knkg9mkv8jl',
    interest_model: 'terra1m25aqupscdw2kw4tnq5ql6hexgr34mr76azh5x',
    distribution_model: 'terra1u64cezah94sq3ye8y0ung28x3pxc37tv8fth7h',
    overseer_contract: 'terra1qljxd0y3j3gk97025qvl3lgq8ygup4gsksvaxv',
    collector_contract: 'terra1hlctcrrhcl2azxzcsns467le876cfuzam6jty4',
    distributor_contract: 'terra1z7nxemcnm8kp7fs33cs7ge4wfuld307v80gypj',
    stable_denom: 'uusd',
    max_borrow_factor: '0.95'
  },
  {
    exchange_rate: '1.231456136426067692',
    aterra_supply: '146557567760189'
  }
]
``` 5
