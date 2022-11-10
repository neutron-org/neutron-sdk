CONTRACT=./artifacts/neutron_interchain_queries.wasm
CHAINID=test-1
CHAINID2=test-2
NEUTRON_DIR=${NEUTRON_DIR:-../neutron}
HOME=${NEUTRON_DIR}/data/test-1/
HOME2=${NEUTRON_DIR}/data/test-2/
KEY=demowallet1
ADMIN=neutron1m9l358xunhhwds0568za49mzhvuxx9ux8xafx2
RLY2=neutron17dtl0mjt3t77kpuhg2edqzjpszulwhgzcdvagh
BIN=neutrond

RES=$(${BIN} tx wasm store ${CONTRACT} --from ${KEY} --gas 50000000  --chain-id ${CHAINID} --broadcast-mode=block --gas-prices 0.0025stake  -y --output json  --keyring-backend test --home ${HOME} --node tcp://127.0.0.1:16657)
ICQ_CONTRACT_CODE_ID=$(echo $RES | jq -r '.logs[0].events[1].attributes[0].value')
echo $RES
echo $ICQ_CONTRACT_CODE_ID

INIT_ICQ_CONTRACT='{}'

RES=$(${BIN} tx wasm instantiate $ICQ_CONTRACT_CODE_ID "$INIT_ICQ_CONTRACT" --from ${KEY} --admin ${ADMIN} -y --chain-id ${CHAINID} --output json --broadcast-mode=block --label "init"  --keyring-backend test --gas-prices 0.0025stake --home ${HOME} --node tcp://127.0.0.1:16657)
echo $RES
ICQ_CONTRACT_ADDRESS=$(echo $RES | jq -r '.logs[0].events[0].attributes[0].value')
echo $ICQ_CONTRACT_ADDRESS

RES=$(${BIN} tx wasm execute $ICQ_CONTRACT_ADDRESS '{"register_balance_query":{"connection_id":"connection-0","addr":"neutron17dtl0mjt3t77kpuhg2edqzjpszulwhgzcdvagh","denom":"stake","update_period":10}}' --from ${KEY}  -y --chain-id ${CHAINID} --output json --broadcast-mode=block --gas-prices 0.0025stake --gas 1000000 --keyring-backend test --home ${HOME} --node tcp://127.0.0.1:16657)
echo $RES

${BIN} q interchainqueries registered-queries  --chain-id ${CHAINID}  --home ${HOME} --node tcp://127.0.0.1:16657

RES=$(${BIN} tx wasm execute $ICQ_CONTRACT_ADDRESS '{"update_interchain_query":{"query_id":1,"new_keys":[{"path":"staking","key":"AhTzV/fuS4r96weXQrLQCkGAufddAnN0YWtl"}]}}' --from ${KEY}  -y --chain-id ${CHAINID} --output json --broadcast-mode=block --gas-prices 0.0025stake --gas 1000000 --keyring-backend test --home ${HOME} --node tcp://127.0.0.1:16657)
echo $RES

${BIN} q interchainqueries registered-queries  --chain-id ${CHAINID}  --home ${HOME} --node tcp://127.0.0.1:16657

RES=$(${BIN} tx wasm execute $ICQ_CONTRACT_ADDRESS '{"remove_interchain_query":{"query_id":1}}' --from ${KEY}  -y --chain-id ${CHAINID} --output json --broadcast-mode=block --gas-prices 0.0025stake --gas 1000000 --keyring-backend test --home ${HOME} --node tcp://127.0.0.1:16657)
echo $RES

RES=$(${BIN} tx wasm execute $ICQ_CONTRACT_ADDRESS '{"register_balance_query":{"connection_id":"connection-0","addr":"neutron17dtl0mjt3t77kpuhg2edqzjpszulwhgzcdvagh","denom":"stake","update_period":10}}' --from ${KEY}  -y --chain-id ${CHAINID} --output json --broadcast-mode=block --gas-prices 0.0025stake --gas 1000000 --keyring-backend test --home ${HOME} --node tcp://127.0.0.1:16657)
echo $RES

${BIN} q interchainqueries registered-queries  --chain-id ${CHAINID}  --home ${HOME} --node tcp://127.0.0.1:16657