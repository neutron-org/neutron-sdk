#!/bin/bash

SCRIPT_DIR=$(dirname $0)

source "${SCRIPT_DIR}/utils.sh"

NEUTROND_BIN=neutrond
GAIAD_BIN=gaiad

CONTRACT=./artifacts/neutron_interchain_txs.wasm

CHAIN_ID_1=test-1
CHAIN_ID_2=test-2

NEUTRON_DIR=${NEUTRON_DIR:-../neutron}
HOME_1=${NEUTRON_DIR}/data/test-1/
HOME_2=${NEUTRON_DIR}/data/test-2/

FAUCET=demowallet3
ADDRESS_2=cosmos10h9stc5v6ntgeygf5xf945njqq5h32r53uquvw
ADMIN=neutron1m9l358xunhhwds0568za49mzhvuxx9ux8xafx2

VAL2=cosmosvaloper1qnk2n4nlkpw9xfqntladh74w6ujtulwnmxnh3k
TEST_WALLET=test_wallet

yes | ${NEUTROND_BIN} keys add ${TEST_WALLET} --home ${HOME_1} --keyring-backend=test
TEST_ADDR=$(${NEUTROND_BIN} keys show ${TEST_WALLET} --keyring-backend test -a --home ${HOME_1})
${NEUTROND_BIN} tx bank send ${FAUCET} ${TEST_ADDR} 100000000stake --chain-id ${CHAIN_ID_1} --home ${HOME_1} --node tcp://localhost:16657 --keyring-backend test -y --gas-prices 0.0025stake --broadcast-mode=block


# Upload the queries contract
RES=$(${NEUTROND_BIN} tx wasm store ${CONTRACT} --from ${TEST_ADDR} --gas 50000000  --chain-id ${CHAIN_ID_1} --broadcast-mode=block --gas-prices 0.0025stake  -y --output json  --keyring-backend test --home ${HOME_1} --node tcp://127.0.0.1:16657)
CONTRACT_CODE_ID=$(echo $RES | jq -r '.logs[0].events[1].attributes[0].value')
echo $RES
echo $CONTRACT_CODE_ID

# Instantiate the contract
INIT_CONTRACT='{}'
echo "Instantiate"
RES=$(${NEUTROND_BIN} tx wasm instantiate $CONTRACT_CODE_ID "$INIT_CONTRACT" --from ${TEST_ADDR} --admin ${ADMIN} -y --chain-id ${CHAIN_ID_1} --output json --broadcast-mode=block --label "init"  --keyring-backend test --gas-prices 0.0025stake --gas auto --gas-adjustment 1.4 --home ${HOME_1} --node tcp://127.0.0.1:16657)
CONTRACT_ADDRESS=$(echo $RES | jq -r '.logs[0].events[0].attributes[0].value')
echo $CONTRACT_ADDRESS

${NEUTROND_BIN} tx bank send ${TEST_WALLET} ${CONTRACT_ADDRESS} 20000000stake --chain-id ${CHAIN_ID_1} --home ${HOME_1} --node tcp://localhost:16657 --keyring-backend test -y --gas-prices 0.0025stake --broadcast-mode=block

#Register interchain account
RES=$(${NEUTROND_BIN} tx wasm execute $CONTRACT_ADDRESS "{\"register\": {\"connection_id\": \"connection-0\", \"interchain_account_id\": \"test\"}}" --from ${TEST_ADDR}  -y --chain-id ${CHAIN_ID_1} --output json --broadcast-mode=block --gas-prices 0.0025stake --gas 1000000 --keyring-backend test --home ${HOME_1} --node tcp://127.0.0.1:16657)
echo $RES

wait_for_tx ${NEUTROND_BIN} \
    ${CONTRACT_ADDRESS} \
    ${CHAIN_ID_1} \
    tcp://127.0.0.1:16657 \
    '{"interchain_account_address_from_contract":{"interchain_account_id":"test"}}' \
    ".data"
RES=$FUNC_RETURN

echo $RES
ICA_ADDRESS=$(echo $RES | jq -r ".[0]")
echo $ICA_ADDRESS

#Send some money to ICA
RES=$(${GAIAD_BIN} tx bank send ${ADDRESS_2} ${ICA_ADDRESS} 10000stake --chain-id ${CHAIN_ID_2}  --broadcast-mode=block --gas-prices 0.0025stake -y --output json --keyring-backend test --home ${HOME_2} --node tcp://127.0.0.1:26657)
echo $RES

echo "Set IBC fees"
RES=$(${NEUTROND_BIN} tx wasm execute $CONTRACT_ADDRESS "{\"set_fees\": {\"ack_fee\": \"2000\", \"recv_fee\": \"0\",\"timeout_fee\": \"2000\", \"denom\": \"stake\"}}" --from $TEST_ADDR  -y --chain-id ${CHAIN_ID_1} --node tcp://127.0.0.1:16657 --output json --broadcast-mode=block --gas-prices 0.0025stake --gas 1000000 --keyring-backend test --home ${HOME_1})


#Delegate
RES=$(${NEUTROND_BIN} tx wasm execute $CONTRACT_ADDRESS "{\"delegate\": {\"interchain_account_id\": \"test\", \"validator\": \"${VAL2}\", \"amount\":\"5000\",\"denom\":\"stake\"}}" --from ${TEST_ADDR}  -y --chain-id ${CHAIN_ID_1} --output json --broadcast-mode=block --gas-prices 0.0025stake --gas 1000000 --keyring-backend test --home ${HOME_1} --node tcp://127.0.0.1:16657)
echo $RES

sleep 7;
curl http://127.0.0.1:1317/staking/delegators/$ICA_ADDRESS/delegations

echo "Try to delegate with sudo error"
echo "Wait for previous transactions to be processed and turn off sudo handler"
sleep 10
echo "Get failures list before test"
FAILURES_BEFORE_TEST=$(${NEUTROND_BIN} q contractmanager failures $CONTRACT_ADDRESS \
    --output json \
    --node tcp://127.0.0.1:16657)

echo "Turn off sudo handler"
RES=$(${NEUTROND_BIN} tx wasm execute $CONTRACT_ADDRESS \
    '{"integration_tests_set_sudo_failure_mock":{}}' \
    --from ${TEST_ADDR}  -y \
    --chain-id ${CHAIN_ID_1} \
    --output json \
    --broadcast-mode=block \
    --gas-prices 0.0025stake \
    --gas 1000000 \
    --keyring-backend test \
    --home ${HOME_1} \
    --node tcp://127.0.0.1:16657)
echo $RES | jq
sleep 10

echo "Delegate"
RES=$(${NEUTROND_BIN} tx wasm execute $CONTRACT_ADDRESS \
    "{\"delegate\": {\"interchain_account_id\": \"test\", \"validator\": \"${VAL2}\", \"amount\":\"5000\",\"denom\":\"stake\"}}" \
    --from ${TEST_ADDR}  -y \
    --chain-id ${CHAIN_ID_1} \
    --output json \
    --broadcast-mode=block \
    --gas-prices 0.0025stake \
    --gas 1000000 \
    --keyring-backend test \
    --home ${HOME_1} \
    --node tcp://127.0.0.1:16657)
echo $RES | jq

echo "Wait for message to be processed and turn on sudo handler"
sleep 20
RES=$(${NEUTROND_BIN} tx wasm execute $CONTRACT_ADDRESS \
    '{"integration_tests_unset_sudo_failure_mock":{}}' \
    --from ${TEST_ADDR}  -y \
    --chain-id ${CHAIN_ID_1} \
    --output json \
    --broadcast-mode=block \
    --gas-prices 0.0025stake \
    --gas 1000000 \
    --keyring-backend test \
    --home ${HOME_1} \
    --node tcp://127.0.0.1:16657)
echo $RES | jq

echo "Failures before test. Should be empty"
echo "${FAILURES_BEFORE_TEST}" | jq ''

echo "Show failures after test"
${NEUTROND_BIN} q contractmanager failures $CONTRACT_ADDRESS \
    --output json \
    --node tcp://127.0.0.1:16657 | jq ''
    