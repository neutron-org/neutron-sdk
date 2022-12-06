NEUTROND_BIN=neutrond
GAIAD_BIN=gaiad

CONTRACT=./artifacts/neutron_interchain_queries.wasm

CHAIN_ID_1=test-1
CHAIN_ID_2=test-2

NEUTRON_DIR=${NEUTRON_DIR:-../neutron}
HOME_1=${NEUTRON_DIR}/data/test-1/
HOME_2=${NEUTRON_DIR}/data/test-2/

USERNAME_1=demowallet1
FAUCET=demowallet3
USERNAME_2=demowallet2
KEY_2=$(gaiad keys show ${USERNAME_2} -a --keyring-backend test --home ${HOME_2})
ADMIN=$(neutrond keys show ${USERNAME_1} -a --keyring-backend test --home ${HOME_1})

TARGET_ADDRESS=cosmos17dtl0mjt3t77kpuhg2edqzjpszulwhgzuj9ljs
VAL2=cosmosvaloper1qnk2n4nlkpw9xfqntladh74w6ujtulwnmxnh3k
TEST_WALLET=test_wallet

yes | ${NEUTROND_BIN} keys add ${TEST_WALLET} --home ${HOME_1} --keyring-backend=test
TEST_ADDR=$(${NEUTROND_BIN} keys show ${TEST_WALLET} --keyring-backend test -a --home ${HOME_1})
${NEUTROND_BIN} tx bank send ${FAUCET} ${TEST_ADDR} 100000000stake --chain-id ${CHAIN_ID_1} --home ${HOME_1} --node tcp://localhost:16657 --keyring-backend test -y --gas-prices 0.0025stake --broadcast-mode=block


# Upload the queries contract
echo "Upload the queries contract"
RES=$(${NEUTROND_BIN} tx wasm store ${CONTRACT} --from ${TEST_ADDR} --gas 50000000  --chain-id ${CHAIN_ID_1} --broadcast-mode=block --gas-prices 0.0025stake  -y --output json  --keyring-backend test --home ${HOME_1} --node tcp://127.0.0.1:16657)
QUERIES_CONTRACT_CODE_ID=$(echo $RES | jq -r '.logs[0].events[1].attributes[0].value')
echo $RES
echo $QUERIES_CONTRACT_CODE_ID

# Instantiate the queries contract
echo "Instantiate the queries contract"
INIT_QUERIES_CONTRACT='{}'

RES=$(${NEUTROND_BIN} tx wasm instantiate $QUERIES_CONTRACT_CODE_ID "$INIT_QUERIES_CONTRACT" --from ${TEST_ADDR} --admin ${ADMIN} -y --chain-id ${CHAIN_ID_1} --output json --broadcast-mode=block --label "init"  --keyring-backend test --gas-prices 0.0025stake --home ${HOME_1} --node tcp://127.0.0.1:16657)
echo $RES
QUERIES_CONTRACT_ADDRESS=$(echo $RES | jq -r '.logs[0].events[0].attributes[0].value')
echo $QUERIES_CONTRACT_ADDRESS

# Send coins from USERNAME_1 to QUERIES_CONTRACT_ADDRESS to perform register_interchain_query message
echo "Send coins from ${TEST_ADDR} to ${QUERIES_CONTRACT_ADDRESS} to perform register_interchain_query message"
echo $(${NEUTROND_BIN} tx bank send ${TEST_ADDR} ${QUERIES_CONTRACT_ADDRESS} 10000000stake -y --chain-id ${CHAIN_ID_1} --output json --broadcast-mode=block --gas-prices 0.0025stake --gas 300000 --keyring-backend test --home ${HOME_1} --node tcp://127.0.0.1:16657)


# Register a query for KEY_2 balance
echo "Register a query for ${KEY_2} balance"
RES=$(${NEUTROND_BIN} tx wasm execute $QUERIES_CONTRACT_ADDRESS "{\"register_balance_query\": {\"connection_id\": \"connection-0\", \"denom\": \"stake\", \"addr\": \"${KEY_2}\", \"update_period\": 5}}" --from ${TEST_WALLET}  -y --chain-id ${CHAIN_ID_1} --output json --broadcast-mode=block --gas-prices 0.0025stake --gas 1000000 --keyring-backend test --home ${HOME_1} --node tcp://127.0.0.1:16657)
echo $RES | jq

# Register a query for KEY_2 balance
echo "Register a query for total supply"
RES=$(${NEUTROND_BIN} tx wasm execute $QUERIES_CONTRACT_ADDRESS "{\"register_bank_total_supply_query\": {\"connection_id\": \"connection-0\", \"denoms\": [\"stake\"], \"update_period\": 5}}" --from ${TEST_WALLET}  -y --chain-id ${CHAIN_ID_1} --output json --broadcast-mode=block --gas-prices 0.0025stake --gas 1000000 --keyring-backend test --home ${HOME_1} --node tcp://127.0.0.1:16657)
echo $RES | jq
