# Neutron Interchain Queries Example Contract

The example contract shows how to use and interact with [Interchain Queries Module](https://docs.neutron.org/neutron/interchain-queries/overview).

## How to test

1. `cargo install --version 1.0.0 ibc-relayer-cli --bin hermes --locked`
2. `git clone git@github.com:neutron-org/neutron.git`
3. `cd neutron && make init && make start-rly`
4. `bash test_kv_query.sh` (or `bash test_tx_query.sh`) (in the Neutron SDK `scripts` directory)