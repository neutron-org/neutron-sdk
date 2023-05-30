# Neutron IBC Transfer Example Contract

The example contract shows how to use and interact with [IBC Transfer Module](https://docs.neutron.org/neutron/modules/transfer/overview).

## IBC transfer contract

Interacting with counterpart chain via ibc transfer is two phases process.
1. Send ibc transfer message
2. Accept and process ibc acknowledgement(sudo_response call)

## How to test

1. run `make build` in the root folder of `neutron-sdk/`
2. set up [Localnet](https://docs.neutron.org/neutron/build-and-run/localnet)
3. cd `scripts/`
4. `./test_ibc_transfer.sh` (or `NEUTRON_DIR=/path/to/somedir/ ./test_ibc_transfer.sh` if the neutron dir is not `../../neutron/`)

Checkout logs from Neutron chain: `grep -E '(ibc-transfer|WASMDEBUG)' ../../neutron/data/test-1/test-1.log`.
You will see there debug messages from contract and neutron's ibc-transfer module itself.

### Tracing ibc transfer ack(sudo)
long story short, we catch packet_sequence id in the reply handler and passthrough any payload to sudo handler using the seq_id

1) ExecuteHandler. We save the payload we want to pass to sudo handler with a "unique-enought" id in the storage
2) ExecuteHandler. Force submsg to replyOn::success with the msd.id we picked above
3) ReplyHandler. In the reply handler we parse ibc packet_sequence id and map the payload to the seq_id in the storage
4) SudoHandler. In the sudo handler we read the payload from the storage with a provided seq_id(in sudo ack packet)
