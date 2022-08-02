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

${BIN} tx bank send demowallet1 ${ICQ_CONTRACT_ADDRESS} 10000stake --chain-id ${CHAINID} --home ${HOME} --node tcp://localhost:16657 --keyring-backend test -y --gas-prices 0.0025stake --broadcast-mode=block

RES=$(${BIN} tx wasm execute $ICQ_CONTRACT_ADDRESS '{"register_balance_query":{"zone_id":"test-2","connection_id":"connection-0","addr":"neutron17dtl0mjt3t77kpuhg2edqzjpszulwhgzcdvagh","denom":"stake","update_period":10}}' --from ${KEY}  -y --chain-id ${CHAINID} --output json --broadcast-mode=block --gas-prices 0.0025stake --gas 1000000 --keyring-backend test --home ${HOME} --node tcp://127.0.0.1:16657)
echo $RES

RES=$(${BIN} tx wasm execute $ICQ_CONTRACT_ADDRESS '{"register_delegator_delegations_query":{"zone_id":"test-2","connection_id":"connection-0","delegator":"neutron10h9stc5v6ntgeygf5xf945njqq5h32r54rf7kf","update_period":10}}' --from ${KEY}  -y --chain-id ${CHAINID} --output json --broadcast-mode=block --gas-prices 0.0025stake --gas 1000000 --keyring-backend test --home ${HOME} --node tcp://127.0.0.1:16657)
echo $RES

${BIN} tx bank send demowallet2 neutron17dtl0mjt3t77kpuhg2edqzjpszulwhgzcdvagh 10000stake --chain-id ${CHAINID2} --home ${HOME2} --node tcp://localhost:26657 --keyring-backend test -y --gas-prices 0.0025stake --broadcast-mode=block

${BIN} tx staking delegate neutronvaloper1qnk2n4nlkpw9xfqntladh74w6ujtulwnqshepx 1000stake --chain-id ${CHAINID2} --home ${HOME2} --node tcp://localhost:26657 --keyring-backend test -y --gas-prices 0.0025stake --broadcast-mode=block  --from demowallet2


#note: query id is hardcodded into the query
RES=$(${BIN} q wasm contract-state smart $ICQ_CONTRACT_ADDRESS '{"get_registered_query":{"query_id":1}}' --chain-id ${CHAINID} --output json  --home ${HOME} --node tcp://127.0.0.1:16657)
echo $RES

sleep 15

RES=$(${BIN} q wasm contract-state smart $ICQ_CONTRACT_ADDRESS '{"balance":{"zone_id":"test-2","addr":"neutron17dtl0mjt3t77kpuhg2edqzjpszulwhgzcdvagh","denom":"stake"}}' --chain-id ${CHAINID} --output json  --home ${HOME} --node tcp://127.0.0.1:16657)
echo $RES

#demowallet2 - neutron10h9stc5v6ntgeygf5xf945njqq5h32r54rf7kf
RES=$(${BIN} q wasm contract-state smart $ICQ_CONTRACT_ADDRESS '{"get_delegations":{"zone_id":"test-2","delegator":"neutron10h9stc5v6ntgeygf5xf945njqq5h32r54rf7kf"}}' --chain-id ${CHAINID} --output json  --home ${HOME} --node tcp://127.0.0.1:16657)
echo $RES
