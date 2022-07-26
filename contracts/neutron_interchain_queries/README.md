# Neutron Interchain Queries Contract

The contract provides a convenient interface for interacting with [Interchain Queries Module](https://github.com/neutron-org/neutron/tree/master/x/interchainqueries).

The contract responsible for registering an interchain query, querying and decoding raw KV-storage results to clear and convenient structures.

# Testing the `SudoMessageCheckTxQueryResult` handler

Install hermes:
```
cargo install --version 0.14.1 ibc-relayer-cli --bin hermes --locked
```

Clone the Neutron repo next to the demo contracts:
```
git clone git@github.com:neutron-org/neutron.git
```

Init the nodes and hermes in the Neutron dir, create a transfer channel then run hermes:
```
make build && make init && make start-rly
```

Clone the relayer repo next to the demo contracts: 
```
git@github.com:neutron-org/cosmos-query-relayer.git
```

and run it:
```
docker run --env-file .env.example -v $PWD/neutron/data:/data -p 9999:9999 neutron-org/cosmos-query-relayer
```

Inside the `neutron-contracts` directory execute `bash test_check_tx_query_result.sh` (or `NEUTRON_DIR=../somedir/ bash test_ibc_transfer.sh` if the neutron dir is not - `../neutron`).

Then check for the handler logs:
```
cat $PWD/neutron/data/test-1.log | grep WASMDEBUG
```

You are looking for lines like:
```
WASMDEBUG: sudo_check_tx_query_result found a matching transaction
```