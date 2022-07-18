CONTRACT=./artifacts/ibc_transfer.wasm
CHAINID=test-1
NEUTRON_DIR=${NEUTRON_DIR:-../neutron}
HOME=${NEUTRON_DIR}/data/test-1/
HOME2=${NEUTRON_DIR}/data/test-2/
KEY=demowallet1
ADMIN=neutron1m9l358xunhhwds0568za49mzhvuxx9ux8xafx2
BIN=neutrond


RES=$(${BIN} tx wasm store ${CONTRACT} --from ${KEY} --gas 50000000  --chain-id ${CHAINID} --broadcast-mode=block --gas-prices 0.0025stake  -y --output json  --keyring-backend test --home ${HOME} --node tcp://127.0.0.1:16657)
HUB_CODE_ID=$(echo $RES | jq -r '.logs[0].events[1].attributes[0].value')
echo $RES
echo $HUB_CODE_ID

INIT_HUB='{}'

RES=$(${BIN} tx wasm instantiate $HUB_CODE_ID "$INIT_HUB" --from ${KEY} --admin ${ADMIN} -y --chain-id ${CHAINID} --output json --broadcast-mode=block --label "init"  --keyring-backend test --gas-prices 0.0025stake --home ${HOME} --node tcp://127.0.0.1:16657)
echo $RES
HUB_ADDRESS=$(echo $RES | jq -r '.logs[0].events[0].attributes[0].value')
echo $HUB_ADDRESS

${BIN} tx bank send demowallet1 ${HUB_ADDRESS} 10000stake --chain-id ${CHAINID} --home ${HOME} --node tcp://localhost:16657 --keyring-backend test -y --gas-prices 0.0025stake --broadcast-mode=block


RES=$(${BIN} tx wasm execute $HUB_ADDRESS '{"send":{"to":"neutron1m9l358xunhhwds0568za49mzhvuxx9ux8xafx2","amount":"1000"}}' --from ${KEY}  -y --chain-id ${CHAINID} --output json --broadcast-mode=block --gas-prices 0.0025stake --gas 1000000 --keyring-backend test --home ${HOME} --node tcp://127.0.0.1:16657)
echo $RES
