BIN=neutrond

CONTRACT=./artifacts/neutron_interchain_queries.wasm

CHAIN_ID_1=test-1
CHAIN_ID_2=test-2

NEUTRON_DIR=${NEUTRON_DIR:-../neutron}
HOME_1=${NEUTRON_DIR}/data/test-1/
HOME_2=${NEUTRON_DIR}/data/test-2/

USERNAME_1=demowallet1
USERNAME_2=demowallet2
KEY_2=$(neutrond keys show ${USERNAME_2} -a --keyring-backend test --home ${HOME_2})
ADMIN=$(neutrond keys show ${USERNAME_1} -a --keyring-backend test --home ${HOME_1})

TARGET_ADDRESS=neutron1mjk79fjjgpplak5wq838w0yd982gzkyf8fxu8u
VAL2=neutronvaloper1qnk2n4nlkpw9xfqntladh74w6ujtulwnqshepx
TEST_WALLET=test_wallet

yes | ${BIN} keys add ${TEST_WALLET} --home ${HOME_1} --keyring-backend=test
TEST_ADDR=$(${BIN} keys show ${TEST_WALLET} --keyring-backend test -a --home ${HOME_1})

${BIN} tx bank send ${USERNAME_1} ${TEST_ADDR} 100000000stake --chain-id ${CHAIN_ID_1} --home ${HOME_1} --node tcp://localhost:16657 --keyring-backend test -y --gas-prices 0.0025stake --broadcast-mode=block


# Upload the queries contract
echo "Upload the queries contract"
RES=$(${BIN} tx wasm store ${CONTRACT} --from ${USERNAME_1} --gas 50000000  --chain-id ${CHAIN_ID_1} --broadcast-mode=block --gas-prices 0.0025stake  -y --output json  --keyring-backend test --home ${HOME_1} --node tcp://127.0.0.1:16657)
QUERIES_CONTRACT_CODE_ID=$(echo $RES | jq -r '.logs[0].events[1].attributes[0].value')
echo $RES
echo $QUERIES_CONTRACT_CODE_ID

# Instantiate the queries contract
echo "Instantiate the queries contract"
INIT_QUERIES_CONTRACT='{}'

RES=$(${BIN} tx wasm instantiate $QUERIES_CONTRACT_CODE_ID "$INIT_QUERIES_CONTRACT" --from ${USERNAME_1} --admin ${ADMIN} -y --chain-id ${CHAIN_ID_1} --output json --broadcast-mode=block --label "init"  --keyring-backend test --gas-prices 0.0025stake --home ${HOME_1} --node tcp://127.0.0.1:16657)
echo $RES
QUERIES_CONTRACT_ADDRESS=$(echo $RES | jq -r '.logs[0].events[0].attributes[0].value')
echo $QUERIES_CONTRACT_ADDRESS

# Send coins from USERNAME_1 to QUERIES_CONTRACT_ADDRESS to perform register_interchain_query message
echo "Send coins from ${USERNAME_1} to ${QUERIES_CONTRACT_ADDRESS} to perform register_interchain_query message"
echo $(${BIN} tx bank send ${USERNAME_1} ${QUERIES_CONTRACT_ADDRESS} 10000000stake -y --chain-id ${CHAIN_ID_1} --output json --broadcast-mode=block --gas-prices 0.0025stake --gas 300000 --keyring-backend test --home ${HOME_1} --node tcp://127.0.0.1:16657)


# Register a query for KEY_2 balance
echo "Register a query for ${KEY_2} balance"
RES=$(${BIN} tx wasm execute $QUERIES_CONTRACT_ADDRESS "{\"register_balance_query\": {\"connection_id\": \"connection-0\", \"denom\": \"stake\", \"addr\": \"${KEY_2}\", \"update_period\": 5}}" --from ${TEST_WALLET}  -y --chain-id ${CHAIN_ID_1} --output json --broadcast-mode=block --gas-prices 0.0025stake --gas 1000000 --keyring-backend test --home ${HOME_1} --node tcp://127.0.0.1:16657)
echo $RES | jq

# Register a query for KEY_2 balance
echo "Register a query for total supply"
RES=$(${BIN} tx wasm execute $QUERIES_CONTRACT_ADDRESS "{\"register_bank_total_supply_query\": {\"connection_id\": \"connection-0\", \"denoms\": [\"stake\"], \"update_period\": 5}}" --from ${TEST_WALLET}  -y --chain-id ${CHAIN_ID_1} --output json --broadcast-mode=block --gas-prices 0.0025stake --gas 1000000 --keyring-backend test --home ${HOME_1} --node tcp://127.0.0.1:16657)
echo $RES | jq
