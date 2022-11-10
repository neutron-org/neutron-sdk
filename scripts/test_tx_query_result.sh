BIN=neutrond

CONTRACT=./artifacts/neutron_interchain_queries.wasm

CHAIN_ID_1=test-1
CHAIN_ID_2=test-2

NEUTRON_DIR=${NEUTRON_DIR:-../neutron}
HOME_1=${NEUTRON_DIR}/data/test-1/
HOME_2=${NEUTRON_DIR}/data/test-2/

USERNAME_1=demowallet1
USERNAME_2=demowallet2
KEY_2=$(neutrond keys show demowallet2 -a --keyring-backend test --home ${HOME_2})
ADMIN=$(neutrond keys show demowallet1 -a --keyring-backend test --home ${HOME_1})

TARGET_ADDRESS=neutron1mjk79fjjgpplak5wq838w0yd982gzkyf8fxu8u
VAL2=neutronvaloper1qnk2n4nlkpw9xfqntladh74w6ujtulwnqshepx

# Upload the queries contract
RES=$(${BIN} tx wasm store ${CONTRACT} --from ${USERNAME_1} --gas 50000000  --chain-id ${CHAIN_ID_1} --broadcast-mode=block --gas-prices 0.0025stake  -y --output json  --keyring-backend test --home ${HOME_1} --node tcp://127.0.0.1:16657)
QUERIES_CONTRACT_CODE_ID=$(echo $RES | jq -r '.logs[0].events[1].attributes[0].value')
echo $RES
echo $QUERIES_CONTRACT_CODE_ID

# Instantiate the queries contract
INIT_QUERIES_CONTRACT='{}'

RES=$(${BIN} tx wasm instantiate $QUERIES_CONTRACT_CODE_ID "$INIT_QUERIES_CONTRACT" --from ${USERNAME_1} --admin ${ADMIN} -y --chain-id ${CHAIN_ID_1} --output json --broadcast-mode=block --label "init"  --keyring-backend test --gas-prices 0.0025stake --home ${HOME_1} --node tcp://127.0.0.1:16657)
echo $RES
QUERIES_CONTRACT_ADDRESS=$(echo $RES | jq -r '.logs[0].events[0].attributes[0].value')
echo $QUERIES_CONTRACT_ADDRESS

# Send some money to contract for deposit
RES=$(${BIN} tx bank send ${USERNAME_1} ${QUERIES_CONTRACT_ADDRESS} 1000000stake --chain-id ${CHAIN_ID_1}  --broadcast-mode=block --gas-prices 0.0025stake -y --output json --keyring-backend test --home ${HOME_1} --node tcp://127.0.0.1:16657)
echo $RES

# Register a query for Send transactions
RES=$(${BIN} tx wasm execute $QUERIES_CONTRACT_ADDRESS "{\"register_transfers_query\": {\"connection_id\": \"connection-0\", \"recipient\": \"${TARGET_ADDRESS}\", \"update_period\": 5, \"min_height\": 1}}" --from ${USERNAME_1}  -y --chain-id ${CHAIN_ID_1} --output json --broadcast-mode=block --gas-prices 0.0025stake --gas 1000000 --keyring-backend test --home ${HOME_1} --node tcp://127.0.0.1:16657)
echo $RES

# Issue a Send transaction that we will be querying for
for i in `seq 0 100`; do
RES=$(${BIN} tx bank send ${KEY_2} ${TARGET_ADDRESS} 1000stake --sequence ${i} --from ${USERNAME_2} --gas 50000000 --gas-adjustment 1.4 --gas-prices 0.5stake --broadcast-mode sync --chain-id ${CHAIN_ID_2} --keyring-backend test --home ${HOME_2} --node tcp://127.0.0.1:26657 -y)
echo $RES
done
