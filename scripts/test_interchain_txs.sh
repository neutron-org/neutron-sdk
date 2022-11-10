BIN=neutrond

CONTRACT=./artifacts/neutron_interchain_txs.wasm

CHAIN_ID_1=test-1
CHAIN_ID_2=test-2

NEUTRON_DIR=${NEUTRON_DIR:-../neutron}
HOME_1=${NEUTRON_DIR}/data/test-1/
HOME_2=${NEUTRON_DIR}/data/test-2/

ADDRESS_1=neutron1m9l358xunhhwds0568za49mzhvuxx9ux8xafx2
ADDRESS_2=neutron10h9stc5v6ntgeygf5xf945njqq5h32r54rf7kf
ADMIN=neutron1m9l358xunhhwds0568za49mzhvuxx9ux8xafx2

VAL2=neutronvaloper1qnk2n4nlkpw9xfqntladh74w6ujtulwnqshepx

# Upload the queries contract
RES=$(${BIN} tx wasm store ${CONTRACT} --from ${ADDRESS_1} --gas 50000000  --chain-id ${CHAIN_ID_1} --broadcast-mode=block --gas-prices 0.0025stake  -y --output json  --keyring-backend test --home ${HOME_1} --node tcp://127.0.0.1:16657)
CONTRACT_CODE_ID=$(echo $RES | jq -r '.logs[0].events[1].attributes[0].value')
echo $RES
echo $CONTRACT_CODE_ID

# Instantiate the contract
INIT_CONTRACT='{}'
echo "Instantiate"
RES=$(${BIN} tx wasm instantiate $CONTRACT_CODE_ID "$INIT_CONTRACT" --from ${ADDRESS_1} --admin ${ADMIN} -y --chain-id ${CHAIN_ID_1} --output json --broadcast-mode=block --label "init"  --keyring-backend test --gas-prices 0.0025stake --gas auto --gas-adjustment 1.4 --home ${HOME_1} --node tcp://127.0.0.1:16657)
CONTRACT_ADDRESS=$(echo $RES | jq -r '.logs[0].events[0].attributes[0].value')
echo $CONTRACT_ADDRESS

#Register interchain account
RES=$(${BIN} tx wasm execute $CONTRACT_ADDRESS "{\"register\": {\"connection_id\": \"connection-0\", \"interchain_account_id\": \"test\"}}" --from ${ADDRESS_1}  -y --chain-id ${CHAIN_ID_1} --output json --broadcast-mode=block --gas-prices 0.0025stake --gas 1000000 --keyring-backend test --home ${HOME_1} --node tcp://127.0.0.1:16657)
echo $RES
sleep 10

RES=$(curl http://127.0.0.1:1316/wasm/contract/$CONTRACT_ADDRESS/smart/eyJpbnRlcmNoYWluX2FjY291bnRfYWRkcmVzc19mcm9tX2NvbnRyYWN0Ijp7ImludGVyY2hhaW5fYWNjb3VudF9pZCI6InRlc3QifX0\=?encoding\=base64 | jq -r ".result.smart")
echo $RES
ICA_ADDRESS=$(echo $RES | base64 --decode | jq -r ".[0]")
echo $ICA_ADDRESS

#Send some money to ICA
RES=$(${BIN} tx bank send ${ADDRESS_2} ${ICA_ADDRESS} 10000stake --chain-id ${CHAIN_ID_2}  --broadcast-mode=block --gas-prices 0.0025stake -y --output json --keyring-backend test --home ${HOME_2} --node tcp://127.0.0.1:26657)
echo $RES

#Delegate
RES=$(${BIN} tx wasm execute $CONTRACT_ADDRESS "{\"delegate\": {\"interchain_account_id\": \"test\", \"validator\": \"${VAL2}\", \"amount\":\"5000\",\"denom\":\"stake\"}}" --from ${ADDRESS_1}  -y --chain-id ${CHAIN_ID_1} --output json --broadcast-mode=block --gas-prices 0.0025stake --gas 1000000 --keyring-backend test --home ${HOME_1} --node tcp://127.0.0.1:16657)
echo $RES

sleep 7;
curl http://127.0.0.1:1317/staking/delegators/$ICA_ADDRESS/delegations

echo "Try to delegate with sudo error"
echo "Wait for previous transactions to be processed and turn off sudo handler"
sleep 10
echo "Get failures list before test"
FAILURES_BEFORE_TEST=$(${BIN} q contractmanager failures $CONTRACT_ADDRESS \
    --output json \
    --node tcp://127.0.0.1:16657)

echo "Turn off sudo handler"
RES=$(${BIN} tx wasm execute $CONTRACT_ADDRESS \
    '{"integration_tests_set_sudo_failure_mock":{}}' \
    --from ${ADDRESS_1}  -y \
    --chain-id ${CHAIN_ID_1} \
    --output json \
    --broadcast-mode=block \
    --gas-prices 0.0025stake \
    --gas 1000000 \
    --keyring-backend test \
    --home ${HOME_1} \
    --node tcp://127.0.0.1:16657)
echo $RES | jq

echo "Delegate"
RES=$(${BIN} tx wasm execute $CONTRACT_ADDRESS \
    "{\"delegate\": {\"interchain_account_id\": \"test\", \"validator\": \"${VAL2}\", \"amount\":\"5000\",\"denom\":\"stake\"}}" \
    --from ${ADDRESS_1}  -y \
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
sleep 10
RES=$(${BIN} tx wasm execute $CONTRACT_ADDRESS \
    '{"integration_tests_unset_sudo_failure_mock":{}}' \
    --from ${ADDRESS_1}  -y \
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
${BIN} q contractmanager failures $CONTRACT_ADDRESS \
    --output json \
    --node tcp://127.0.0.1:16657 | jq ''
    