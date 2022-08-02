# Neutron demo contracts
## IBC transfer contract
Interacting with counterpart chain via ibc transfer is two phases process.
1. Send ibc transfer message
2. Accept and process ibc acknowlegement(sudo_response call)

to run the contract you need to init two chain network connected with hermes relayer. We use a neutron nodes as both ends.

1) Install hermes `cargo install --version 0.14.1 ibc-relayer-cli --bin hermes --locked`
2) Clone the neutron repo next to the demo contracts - `git clone git@github.com:neutron-org/neutron.git` (at this moment we have to use https://github.com/neutron-org/neutron/tree/feat/ibc-transfer-ack branch)
3) init nodes and hermes in the neutron dir, create a transfer channel then run hermes  `make init && hermes -c ./network/hermes/config.toml create channel --port-a transfer --port-b transfer test-1 connection-0 && make start-rly`

You are ready to execute the contract

In the contracts dir execute `bash test_ibc_transfer.sh` (or `NEUTRON_DIR=../somedir/ bash test_ibc_transfer.sh` if the neutron dir is not - `../neutron`)

In the console you will see transactions responses

Checkout logs from test-1 chain
`tail -f ./data/test-1.log | grep -E '(ibc-transfer|WASMDEBUG)'` (in neutron dir). There are debud messages from contrat and neutron's ibc-transfer module intself.

### Tracing ibc transfer ack(sudo)
long story short, we catch packet_sequence id in the reply handler and passthrough any payload to sudo handler using the seq_id

1) ExecuteHandler. We save the payload we want to pass to sudo handler with a "unique-enought" id in the storage
2) ExecuteHandler. Force submsg to replyOn::success with the msd.id we picked above
3) ReplyHandler. In the reply handler we parse ibc packet_sequence id and map the payload to the seq_id in the storage
4) SudoHandler. In the sudo handler we read the payload from the storage with a provided seq_id(in sudo ack packet)