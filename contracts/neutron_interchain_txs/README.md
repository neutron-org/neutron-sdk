# Neutron Interchain Txs Example Contract

The example contract shows how to use and interact with [Interchain Txs Module](https://docs.neutron.org/neutron/interchain-txs/overview).

## How to test

1. `cargo install --git https://github.com/neutron-org/hermes --rev 7defaf0 ibc-relayer-cli --bin hermes --locked`
2. `git clone git@github.com:neutron-org/neutron.git`
3. `cd neutron && make init && make start-rly`
4. `bash test_interchain_txs.sh` (in the Neutron SDK `scripts` directory)
5. Checkout logs from test-1 chain
   `tail -f ./data/test-1.log | grep -E 'WASMDEBUG'` (in neutron dir).
