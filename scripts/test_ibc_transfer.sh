CONTRACT=../artifacts/ibc_transfer.wasm
CHAINID=test-1
NEUTRON_DIR=${NEUTRON_DIR:-../../neutron}
HOME=${NEUTRON_DIR}/data/test-1/
HOME2=${NEUTRON_DIR}/data/test-2/
KEY=demowallet1
ADMIN=neutron1m9l358xunhhwds0568za49mzhvuxx9ux8xafx2
BIN=neutrond

RES=$(${BIN} tx wasm store ${CONTRACT} --from ${KEY} --gas 50000000  --chain-id ${CHAINID} --broadcast-mode=block --gas-prices 0.0025stake  -y --output json  --keyring-backend test --home ${HOME} --node tcp://127.0.0.1:16657)
TRANSFER_CONTRACT_CODE_ID=$(echo $RES | jq -r '.logs[0].events[1].attributes[0].value')
echo $RES
echo $TRANSFER_CONTRACT_CODE_ID

INIT_TRANSFER_CONTRACT='{}'

RES=$(${BIN} tx wasm instantiate $TRANSFER_CONTRACT_CODE_ID "$INIT_TRANSFER_CONTRACT" --from ${KEY} --admin ${ADMIN} -y --chain-id ${CHAINID} --output json --broadcast-mode=block --label "init"  --keyring-backend test --gas-prices 0.0025stake --home ${HOME} --node tcp://127.0.0.1:16657)
echo $RES
TRANSFER_CONTRACT_ADDRESS=$(echo $RES | jq -r '.logs[0].events[0].attributes[0].value')
echo $TRANSFER_CONTRACT_ADDRESS

${BIN} tx bank send demowallet1 ${TRANSFER_CONTRACT_ADDRESS} 10000stake --chain-id ${CHAINID} --home ${HOME} --node tcp://localhost:16657 --keyring-backend test -y --gas-prices 0.0025stake --broadcast-mode=block


echo "Tranfer coins from test-1 to test-2"
RES=$(${BIN} tx wasm execute $TRANSFER_CONTRACT_ADDRESS \
    '{"send":{"to":"cosmos17dtl0mjt3t77kpuhg2edqzjpszulwhgzuj9ljs","amount":"1000", "denom": "stake", "channel": "channel-0"}}' \
    --from ${KEY}  -y \
    --chain-id ${CHAINID} \
    --output json \
    --broadcast-mode=block \
    --gas-prices 0.0025stake \
    --gas 1000000 \
    --keyring-backend test \
    --home ${HOME} \
    --node tcp://127.0.0.1:16657)
echo $RES | jq