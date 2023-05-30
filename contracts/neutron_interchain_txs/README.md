# Neutron Interchain Txs Example Contract

The example contract shows how to use and interact with [Interchain Txs Module](https://docs.neutron.org/neutron/modules/interchain-txs/overview).

## How to test

1. run `make build` in the root folder of `neutron-sdk/`
2. set up [Localnet](https://docs.neutron.org/neutron/build-and-run/localnet)
3. cd `scripts/`
4. `./test_intechain_txs.sh` (or `NEUTRON_DIR=/path/to/somedir/ ./test_interchain_txs.sh` if the neutron dir is not `../../neutron/`)
