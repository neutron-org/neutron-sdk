BIN=neutrond

CONTRACT=../artifacts/neutron_interchain_queries.wasm

CHAIN_ID_1=test-1
CHAIN_ID_2=test-2

NEUTRON_DIR=${NEUTRON_DIR:-../../neutron}
HOME_1=${NEUTRON_DIR}/data/test-1/
HOME_2=${NEUTRON_DIR}/data/test-2/

ADDRESS_1=neutron1m9l358xunhhwds0568za49mzhvuxx9ux8xafx2
ADDRESS_2=cosmos10h9stc5v6ntgeygf5xf945njqq5h32r53uquvw
ADMIN=neutron1m9l358xunhhwds0568za49mzhvuxx9ux8xafx2

VAL2=cosmos1qnk2n4nlkpw9xfqntladh74w6ujtulwn7j8za9

# Upload the queries contract
echo "Upload the queries contract"
RES=$(${BIN} tx wasm store ${CONTRACT} --from ${ADDRESS_1} --gas 50000000  --chain-id ${CHAIN_ID_1} --broadcast-mode=block --gas-prices 0.0025stake  -y --output json  --keyring-backend test --home ${HOME_1} --node tcp://127.0.0.1:16657)
QUERIES_CONTRACT_CODE_ID=$(echo $RES | jq -r '.logs[0].events[1].attributes[0].value')
echo $RES
echo $QUERIES_CONTRACT_CODE_ID

# Instantiate the queries contract
echo "Instantiate the queries contract"
INIT_QUERIES_CONTRACT='{}'

RES=$(${BIN} tx wasm instantiate $QUERIES_CONTRACT_CODE_ID "$INIT_QUERIES_CONTRACT" --from ${ADDRESS_1} --admin ${ADMIN} -y --chain-id ${CHAIN_ID_1} --output json --broadcast-mode=block --label "init"  --keyring-backend test --gas-prices 0.0025stake --home ${HOME_1} --node tcp://127.0.0.1:16657)
echo $RES
QUERIES_CONTRACT_ADDRESS=$(echo $RES | jq -r '.logs[0].events[0].attributes[0].value')
echo $QUERIES_CONTRACT_ADDRESS

# Send coins from USERNAME_1 to QUERIES_CONTRACT_ADDRESS to perform register_interchain_query message
echo "Send coins from ${ADDRESS_1} to ${QUERIES_CONTRACT_ADDRESS} to perform register_interchain_query message"
echo $(${BIN} tx bank send ${ADDRESS_1} ${QUERIES_CONTRACT_ADDRESS} 10000000stake -y --chain-id ${CHAIN_ID_1} --output json --broadcast-mode=block --gas-prices 0.0025stake --gas 300000 --keyring-backend test --home ${HOME_1} --node tcp://127.0.0.1:16657)

# Register a query for a total supply of stake token
echo "Register a query for total supply"
RES=$(${BIN} tx wasm execute $QUERIES_CONTRACT_ADDRESS "{\"register_bank_total_supply_query\": {\"connection_id\": \"connection-0\", \"denoms\": [\"stake\"], \"update_period\": 5}}" --from ${ADDRESS_1}  -y --chain-id ${CHAIN_ID_1} --output json --broadcast-mode=block --gas-prices 0.0025stake --gas 1000000 --keyring-backend test --home ${HOME_1} --node tcp://127.0.0.1:16657)
echo $RES | jq

sleep 5

echo "KV query response:"
${BIN} query wasm contract-state smart ${QUERIES_CONTRACT_ADDRESS} '{"bank_total_supply": {"query_id": 1}}' --node tcp://127.0.0.1:16657