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

# PRINT contract old info (should have admin set)

# Register new proposal
# json formatted proposal is a nightmare, so we use keys for now
RES=$(${BIN} tx gov submit-proposal clear-contract-admin ${QUERIES_CONTRACT_ADDRESS} --title="clear contract admin" \
  --description="ssss" \
  --type="Text" \
  --deposit="100000000stake" \
  --from ${USERNAME_1} \
  --gas 500000 \
  --fees 5000stake \
  -y \
  --chain-id ${CHAIN_ID_1} \
  --broadcast-mode=block \
  --home ${HOME_1} \
  --keyring-backend test \
  --node tcp://127.0.0.1:16657)
echo "--- tx gov submit-proposal result:"
echo $RES
echo

# print proposal in console, voting period should be active
RES=$(${BIN} q gov proposals --chain-id ${CHAIN_ID_1} --home ${HOME_1} --node tcp://127.0.0.1:16657)
echo "--- q gov proposals:"
echo $RES
echo

# vote yes (w dominance of votes)
RES=$(${BIN} tx gov vote 1 yes --from val1 --fees 5000stake --chain-id ${CHAIN_ID_1} -y --broadcast-mode=block --home ${HOME_1}  --keyring-backend test --node tcp://127.0.0.1:16657)
echo "--- tx gov vote:"
echo $RES
echo
# wait voting period to end
sleep 60

# Is proposal passed?
RES=$(${BIN} query gov proposal 1  --chain-id ${CHAIN_ID_1} --home ${HOME_1} --node tcp://127.0.0.1:16657)
echo "--- query gov proposal 1:"
echo $RES
echo

# Print new contract info (should have admin empty)
RES=$(${BIN} query wasm contract ${QUERIES_CONTRACT_ADDRESS} --chain-id ${CHAIN_ID_1} --home ${HOME_1} --node tcp://127.0.0.1:16657)
echo "--- query wasm contract:"
echo $RES