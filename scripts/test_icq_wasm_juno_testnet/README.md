# Test ICQ wasm contract state using Juno testnet

This test serves as a tutorial on how to query remote cw20 balance.
This tutorial could be used as a general guide on how to query any
remote cw20 state. Sadly, storage keys are always contract dependent,
and cw20 balance is not an exception. This tutorial crafts storage keys for
[cw20-base](https://github.com/CosmWasm/cw-plus/tree/main/contracts/cw20-base)
contract. In general, it is always better to deploy your own contract with
a stable well-defined state known by you, so you can easily figure out
the storage keys, and they will never change this way. The contract you
could deploy manually is expected to perform wasm query and save query
result into its state.

This test uses
[neutron_interchain_queries](https://github.com/neutron-org/neutron-sdk/tree/main/contracts/neutron_interchain_queries)
contract, sending `ExecuteMsg::RegisterCw20BalanceQuery` and quering
`QueryMsg::Cw20Balance`. You can study this contract's code to learn
more on how to craft storage keys.

## 1. Install dependencies

Read [instructions](https://docs.neutron.org/neutron/build-and-run/localnet)
and install:
- neutron
- hermes
- neutron query relayer

## 2. Launch neutron

Go to `neutron/` directory and run `make start`.
This will deploy Neutron localnet.
Please wait 20 to 30 seconds for chain to initialize before
proceeding to step 3.

## 3. Connect to Juno testnet

Open `neutron-sdk/scripts/test_icq_wasm_juno_testnet/create_juno_connection.sh` in your text editor of choice.
Navigate to `JUNO_MNEMONIC=""` and insert there your own testnet mnemonic.
Please make sure to have at least 0.01 JUNOX on uni-6 testnet, these funds
are needed to create a connection.

Change current directory to `neutron-sdk/scripts/test_icq_wasm_juno_testnet/`
and run `./create_juno_connection.sh`. After it finishes, `connection-0` should
appear on Neutron localnet. You can use this snippet to query a list of
connections on Neutron's localnet:

```bash
neutrond query ibc connection connections --node tcp://0.0.0.0:26657 --output json | jq '.connections[] | {id, client_id, state, counterparty}'
```

If this is the first time you are running this script, you should only
see `connection-0`. Don't worry if you see `connection-1`, `connection-2`
and so on, simply use the last one you have created.

## 4. Deploy ICQ relayer

Open `neutron-sdk/scripts/test_icq_wasm_juno_testnet/icq.env` in your text editor of choice.
Navigate to `RELAYER_NEUTRON_CHAIN_CONNECTION_ID=` and insert there
connection ID you have just created.

```bash
cd neutron-sdk/scripts/test_icq_wasm_juno_testnet/
rm -rf storage/; export $(xargs < icq.env) && neutron_query_relayer start
```

## 5. Prepare to run test

First, navigate to root directory of neutron-sdk repo and execute
`rm -rf target/; cargo update && make build`.

Next, open `neutron-sdk/scripts/test_icq_wasm_juno_testnet/test_wasm_query.sh`
in your text editor of choice. Navigate to `CONNECTION_ID=""` and insert there
connection ID you have just created.

## 6. Run test

Change directory to `neutron-sdk/scripts/test_icq_wasm_juno_testnet` and
execute `./test_wasm_query.sh`.
