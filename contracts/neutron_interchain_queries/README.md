# Neutron Interchain Queries Example Contract

The example contract shows how to use and interact with [Interchain Queries Module](https://docs.neutron.org/neutron/modules/interchain-queries/overview).

## How to test

1. run `make build` in the root folder of `neutron-sdk/`
2. set up [Localnet](https://docs.neutron.org/neutron/build-and-run/localnet)
3. cd `scripts/`
4. `./test_kv_query.sh` (or `NEUTRON_DIR=/path/to/somedir/ ./test_kv_query.sh` if the neutron dir is not `../../neutron/`)
5. `./test_tx_query.sh` (or `NEUTRON_DIR=/path/to/somedir/ ./test_tx_query.sh` if the neutron dir is not `../../neutron/`)
