# Neutron Interchain Queries Contract

The contract provides a convenient interface for interacting with [Interchain Queries Module](https://github.com/neutron-org/neutron/tree/master/x/interchainqueries).

The contract responsible for registering an interchain query, querying and decoding raw KV-storage results to clear and convenient structures.

# Testing the `SudoMessageTxQueryResult` handler

Install hermes:
```
cargo install --version 0.14.1 ibc-relayer-cli --bin hermes --locked
```

Clone the Neutron repo next to the demo contracts:
```
git clone git@github.com:neutron-org/neutron.git
```

Init the nodes and hermes in the Neutron dir and run hermes:
```
make build && make init && make start-rly
```

Clone the relayer repo next to the demo contracts: 
```
git@github.com:neutron-org/cosmos-query-relayer.git
```

and run it (note that you have to use the absolute path to `neutron/data`):
```
docker run --env-file .env.example -v /absolute/path/to/neutron/data:/data -p 9999:9999 neutron-org/cosmos-query-relayer
```

Inside the `neutron-contracts` directory execute `bash test_tx_query_result.sh` (or `NEUTRON_DIR=../somedir/ bash test_tx_query_result.sh` if the neutron dir is not - `../neutron`).

Then check for the handler logs:
```
cat $PWD/neutron/data/test-1.log | grep WASMDEBUG
```

You are looking for lines like:
```
WASMDEBUG: sudo_check_tx_query_result found a matching transaction
```

Also, as now duplicate transactions are ignored, you should see only successful results in relayer logs:

```
{"level":"info","ts":1658927524.0552058,"caller":"relay/relayer.go:78","msg":"proof for query_id submitted successfully","query_id":1}
```

# Testing the `SudoMessageKVQueryResult` handler

Install hermes:
```
cargo install --version 0.14.1 ibc-relayer-cli --bin hermes --locked
```

Clone the Neutron repo next to the demo contracts:
```
git clone git@github.com:neutron-org/neutron.git
```

Init the nodes and hermes in the Neutron dir and run hermes:
```
make build && make init && make start-rly
```

Clone the relayer repo next to the demo contracts:
```
git@github.com:neutron-org/cosmos-query-relayer.git
```

and run it (note that you have to use the absolute path to `neutron/data`):
```
docker run --env-file .env.example -v /absolute/path/to/neutron/data:/data -p 9999:9999 neutron-org/cosmos-query-relayer
```

Inside the `neutron-contracts` directory execute `bash test_kv_query_result.sh` (or `NEUTRON_DIR=../somedir/ bash test_kv_query_result.sh` if the neutron dir is not - `../neutron`).

Then check for the handler logs:
```
cat ../neutron/data/test-1.log | grep WASMDEBUG
```

You are looking for lines like:
```
WASMDEBUG: sudo_kv_query_result found a matching transaction
```

You should see only successful results in relayer logs:

```
{"level":"info","ts":1658927524.0552058,"caller":"relay/relayer.go:78","msg":"proof for query_id submitted successfully","query_id":1}
```