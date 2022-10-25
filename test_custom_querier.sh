CONTRACT=./artifacts/neutron_interchain_queries.wasm
CHAINID=test-1
CHAINID2=test-2
NEUTRON_DIR=${NEUTRON_DIR:-../neutron}
HOME=${NEUTRON_DIR}/data/test-1/
HOME2=${NEUTRON_DIR}/data/test-2/
KEY=demowallet1
ADMIN=neutron1m9l358xunhhwds0568za49mzhvuxx9ux8xafx2
RLY2=neutron17dtl0mjt3t77kpuhg2edqzjpszulwhgzcdvagh
TEST_WALLET=test_wallet
BIN=neutrond

yes | ${BIN} keys add ${TEST_WALLET} --home ${HOME} --keyring-backend=test
TEST_ADDR=$(${BIN} keys show ${TEST_WALLET} --keyring-backend test -a --home ${HOME})

${BIN} tx bank send ${KEY} ${TEST_ADDR} 100000000stake --chain-id ${CHAINID} --home ${HOME} --node tcp://localhost:16657 --keyring-backend test -y --gas-prices 0.0025stake --broadcast-mode=block


RES=$(${BIN} tx wasm store ${CONTRACT} --from ${KEY} --gas 50000000  --chain-id ${CHAINID} --broadcast-mode=block --gas-prices 0.0025stake  -y --output json  --keyring-backend test --home ${HOME} --node tcp://127.0.0.1:16657)
ICQ_CONTRACT_CODE_ID=$(echo $RES | jq -r '.logs[0].events[1].attributes[0].value')
echo $RES
echo $ICQ_CONTRACT_CODE_ID

INIT_ICQ_CONTRACT='{}'

RES=$(${BIN} tx wasm instantiate $ICQ_CONTRACT_CODE_ID "$INIT_ICQ_CONTRACT" --from ${KEY} --admin ${ADMIN} -y --chain-id ${CHAINID} --output json --broadcast-mode=block --label "init"  --keyring-backend test --gas-prices 0.0025stake --home ${HOME} --node tcp://127.0.0.1:16657)
echo $RES
ICQ_CONTRACT_ADDRESS=$(echo $RES | jq -r '.logs[0].events[0].attributes[0].value')
echo $ICQ_CONTRACT_ADDRESS

${BIN} tx bank send ${TEST_WALLET} ${ICQ_CONTRACT_ADDRESS} 20000000stake --chain-id ${CHAINID} --home ${HOME} --node tcp://localhost:16657 --keyring-backend test -y --gas-prices 0.0025stake --broadcast-mode=block

echo "register_balance_query"
RES=$(${BIN} tx wasm execute $ICQ_CONTRACT_ADDRESS '{"register_balance_query":{"connection_id":"connection-0","addr":"neutron17dtl0mjt3t77kpuhg2edqzjpszulwhgzcdvagh","denom":"stake","update_period":10}}' --from ${TEST_WALLET}  -y --chain-id ${CHAINID} --output json --broadcast-mode=block --gas-prices 0.0025stake --gas 1000000 --keyring-backend test --home ${HOME} --node tcp://127.0.0.1:16657)
echo $RES | jq

echo "register_delegator_delegations_query"
RES=$(${BIN} tx wasm execute $ICQ_CONTRACT_ADDRESS '{"register_delegator_delegations_query":{"connection_id":"connection-0","delegator":"neutron10h9stc5v6ntgeygf5xf945njqq5h32r54rf7kf", "validators": ["neutronvaloper1qnk2n4nlkpw9xfqntladh74w6ujtulwnqshepx"], "update_period":10}}' --from ${TEST_WALLET}  -y --chain-id ${CHAINID} --output json --broadcast-mode=block --gas-prices 0.0025stake --gas 1000000 --keyring-backend test --home ${HOME} --node tcp://127.0.0.1:16657)
echo $RES | jq

echo "register_bank_total_supply_query"
RES=$(${BIN} tx wasm execute $ICQ_CONTRACT_ADDRESS '{"register_bank_total_supply_query":{"connection_id":"connection-0","denoms":["stake","stake"], "update_period":10}}' --from ${TEST_WALLET}  -y --chain-id ${CHAINID} --output json --broadcast-mode=block --gas-prices 0.0025stake --gas 1000000 --keyring-backend test --home ${HOME} --node tcp://127.0.0.1:16657)
echo $RES | jq

echo "register_staking_validators_query"
RES=$(${BIN} tx wasm execute $ICQ_CONTRACT_ADDRESS '{"register_staking_validators_query":{"connection_id":"connection-0","validators":["neutronvaloper1qnk2n4nlkpw9xfqntladh74w6ujtulwnqshepx","neutronvaloper1qnk2n4nlkpw9xfqntladh74w6ujtulwnqshepx"], "update_period":10}}' --from ${TEST_WALLET}  -y --chain-id ${CHAINID} --output json --broadcast-mode=block --gas-prices 0.0025stake --gas 1000000 --keyring-backend test --home ${HOME} --node tcp://127.0.0.1:16657)
echo $RES | jq

echo "register_distribution_fee_pool_query"
RES=$(${BIN} tx wasm execute $ICQ_CONTRACT_ADDRESS '{"register_distribution_fee_pool_query":{"connection_id":"connection-0", "update_period":10}}' --from ${TEST_WALLET}  -y --chain-id ${CHAINID} --output json --broadcast-mode=block --gas-prices 0.0025stake --gas 1000000 --keyring-backend test --home ${HOME} --node tcp://127.0.0.1:16657)
echo $RES | jq

echo "Add two proposals"
${BIN} tx gov submit-proposal --type text --title "Test proposal" --description "Test proposal details" --deposit 10000000stake --chain-id ${CHAINID2} --from demowallet2 --home ${HOME2} --node tcp://localhost:26657 --keyring-backend test -y --gas-prices 0.0025stake --broadcast-mode=block
${BIN} tx gov submit-proposal --type text --title "Another test proposal" --description "Another test proposal details" --deposit 10000000stake --chain-id ${CHAINID2} --from demowallet2 --home ${HOME2} --node tcp://localhost:26657 --keyring-backend test -y --gas-prices 0.0025stake --broadcast-mode=block

echo "register_government_proposals_query"
RES=$(${BIN} tx wasm execute $ICQ_CONTRACT_ADDRESS '{"register_government_proposals_query":{"connection_id":"connection-0", "proposals_ids": [1, 2],"update_period":10}}' --from ${KEY}  -y --chain-id ${CHAINID} --output json --broadcast-mode=block --gas-prices 0.0025stake --gas 1000000 --keyring-backend test --home ${HOME} --node tcp://127.0.0.1:16657)
echo $RES | jq


${BIN} tx bank send demowallet2 neutron17dtl0mjt3t77kpuhg2edqzjpszulwhgzcdvagh 10000stake --chain-id ${CHAINID2} --home ${HOME2} --node tcp://localhost:26657 --keyring-backend test -y --gas-prices 0.0025stake --broadcast-mode=block

${BIN} tx staking delegate neutronvaloper1qnk2n4nlkpw9xfqntladh74w6ujtulwnqshepx 1000stake --chain-id ${CHAINID2} --home ${HOME2} --node tcp://localhost:26657 --keyring-backend test -y --gas-prices 0.0025stake --broadcast-mode=block  --from demowallet2

echo "${BIN} q wasm contract-state smart $ICQ_CONTRACT_ADDRESS '{\"get_registered_query\":{\"query_id\":1}}' --chain-id ${CHAINID} --output json  --home ${HOME} --node tcp://127.0.0.1:16657"
#note: query id is hardcodded into the query
RES=$(${BIN} q wasm contract-state smart $ICQ_CONTRACT_ADDRESS '{"get_registered_query":{"query_id":1}}' --chain-id ${CHAINID} --output json  --home ${HOME} --node tcp://127.0.0.1:16657)
echo $RES | jq

sleep 30

RES=$(${BIN} q wasm contract-state smart $ICQ_CONTRACT_ADDRESS '{"balance": {"query_id": 1}}' --chain-id ${CHAINID} --output json  --home ${HOME} --node tcp://127.0.0.1:16657)
echo $RES | jq

#demowallet2 - neutron10h9stc5v6ntgeygf5xf945njqq5h32r54rf7kf
RES=$(${BIN} q wasm contract-state smart $ICQ_CONTRACT_ADDRESS '{"get_delegations":{"query_id": 2}}' --chain-id ${CHAINID} --output json  --home ${HOME} --node tcp://127.0.0.1:16657)
echo $RES | jq

RES=$(${BIN} q wasm contract-state smart $ICQ_CONTRACT_ADDRESS '{"bank_total_supply":{"query_id": 3}}' --chain-id ${CHAINID} --output json  --home ${HOME} --node tcp://127.0.0.1:16657)
echo $RES | jq

RES=$(${BIN} q wasm contract-state smart $ICQ_CONTRACT_ADDRESS '{"staking_validators":{"query_id": 4}}' --chain-id ${CHAINID} --output json  --home ${HOME} --node tcp://127.0.0.1:16657)
echo $RES | jq

RES=$(${BIN} q wasm contract-state smart $ICQ_CONTRACT_ADDRESS '{"distribution_fee_pool":{"query_id": 5}}' --chain-id ${CHAINID} --output json  --home ${HOME} --node tcp://127.0.0.1:16657)
echo $RES | jq

RES=$(${BIN} q wasm contract-state smart $ICQ_CONTRACT_ADDRESS '{"government_proposals":{"query_id": 6}}' --chain-id ${CHAINID} --output json  --home ${HOME} --node tcp://127.0.0.1:16657)
echo $RES | jq

