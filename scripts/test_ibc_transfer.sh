CONTRACT=./artifacts/ibc_transfer.wasm
CHAINID=test-1
NEUTRON_DIR=${NEUTRON_DIR:-../neutron}
HOME=${NEUTRON_DIR}/data/test-1/
ADMIN=neutron1m9l358xunhhwds0568za49mzhvuxx9ux8xafx2

NEUTROND_BIN=neutrond
GAIAD_BIN=gaiad

FAUCET=demowallet3
TEST_WALLET=test_wallet

yes | ${NEUTROND_BIN} keys add ${TEST_WALLET} --home ${HOME} --keyring-backend=test
TEST_ADDR=$(${NEUTROND_BIN} keys show ${TEST_WALLET} --keyring-backend test -a --home ${HOME})
${NEUTROND_BIN} tx bank send ${FAUCET} ${TEST_ADDR} 100000000stake --chain-id ${CHAINID} --home ${HOME} --node tcp://localhost:16657 --keyring-backend test -y --gas-prices 0.0025stake --broadcast-mode=block


RES=$(${NEUTROND_BIN} tx wasm store ${CONTRACT} --from ${TEST_ADDR} --gas 50000000  --chain-id ${CHAINID} --broadcast-mode=block --gas-prices 0.0025stake  -y --output json  --keyring-backend test --home ${HOME} --node tcp://127.0.0.1:16657)
TRANSFER_CONTRACT_CODE_ID=$(echo $RES | jq -r '.logs[0].events[1].attributes[0].value')
echo $RES
echo $TRANSFER_CONTRACT_CODE_ID

INIT_TRANSFER_CONTRACT='{}'

RES=$(${NEUTROND_BIN} tx wasm instantiate $TRANSFER_CONTRACT_CODE_ID "$INIT_TRANSFER_CONTRACT" --from ${TEST_ADDR} --admin ${ADMIN} -y --chain-id ${CHAINID} --output json --broadcast-mode=block --label "init"  --keyring-backend test --gas-prices 0.0025stake --home ${HOME} --node tcp://127.0.0.1:16657)
echo $RES
TRANSFER_CONTRACT_ADDRESS=$(echo $RES | jq -r '.logs[0].events[0].attributes[0].value')
echo $TRANSFER_CONTRACT_ADDRESS

${NEUTROND_BIN} tx bank send ${FAUCET} ${TRANSFER_CONTRACT_ADDRESS} 10000stake --chain-id ${CHAINID} --home ${HOME} --node tcp://localhost:16657 --keyring-backend test -y --gas-prices 0.0025stake --broadcast-mode=block


echo "Tranfer coins from test-1 to test-2"
RES=$(${NEUTROND_BIN} tx wasm execute $TRANSFER_CONTRACT_ADDRESS \
    '{"send":{"to":"cosmos10h9stc5v6ntgeygf5xf945njqq5h32r53uquvw","amount":"1000", "denom": "stake", "channel": "channel-0"}}' \
    --from ${TEST_ADDR}  -y \
    --chain-id ${CHAINID} \
    --output json \
    --broadcast-mode=block \
    --gas-prices 0.0025stake \
    --gas 1000000 \
    --keyring-backend test \
    --home ${HOME} \
    --node tcp://127.0.0.1:16657)
echo $RES | jq

echo "Try to transfer coins from test-1 to test-2 again with failing sudo handler"
echo "Wait for previous transactions to be processed and turn off sudo handler"
sleep 10
echo "Get failures list before test"
FAILURES_BEFORE_TEST=$(${NEUTROND_BIN} q contractmanager failures $TRANSFER_CONTRACT_ADDRESS \
    --output json \
    --node tcp://127.0.0.1:16657)


echo "Turn off sudo handler"
RES=$(${NEUTROND_BIN} tx wasm execute $TRANSFER_CONTRACT_ADDRESS \
    '{"integration_tests_set_sudo_failure_mock":{}}' \
    --from ${TEST_ADDR}  -y \
    --chain-id ${CHAINID} \
    --output json \
    --broadcast-mode=block \
    --gas-prices 0.0025stake \
    --gas 1000000 \
    --keyring-backend test \
    --home ${HOME} \
    --node tcp://127.0.0.1:16657)
echo $RES | jq
sleep 20

echo "Send coins from test-2 to test-1"
RES=$(${NEUTROND_BIN} tx wasm execute $TRANSFER_CONTRACT_ADDRESS \
    '{"send":{"to":"cosmos10h9stc5v6ntgeygf5xf945njqq5h32r53uquvw","amount":"1000", "denom": "stake", "channel": "channel-0"}}' \
    --from ${TEST_ADDR}  -y \
    --chain-id ${CHAINID} \
    --output json \
    --broadcast-mode=block \
    --gas-prices 0.0025stake \
    --gas 1000000 \
    --keyring-backend test \
    --home ${HOME} \
    --node tcp://127.0.0.1:16657)
echo $RES | jq

echo "Wait for message to be processed and turn on sudo handler"
sleep 20
RES=$(${NEUTROND_BIN} tx wasm execute $TRANSFER_CONTRACT_ADDRESS \
    '{"integration_tests_unset_sudo_failure_mock":{}}' \
    --from ${TEST_ADDR}  -y \
    --chain-id ${CHAINID} \
    --output json \
    --broadcast-mode=block \
    --gas-prices 0.0025stake \
    --gas 1000000 \
    --keyring-backend test \
    --home ${HOME} \
    --node tcp://127.0.0.1:16657)
echo $RES | jq
sleep 40

echo "Failures before test. Should be empty"
echo "${FAILURES_BEFORE_TEST}" | jq ''

echo "Show failures after test"
${NEUTROND_BIN} q contractmanager failures $TRANSFER_CONTRACT_ADDRESS \
    --output json \
    --node tcp://127.0.0.1:16657 | jq ''

