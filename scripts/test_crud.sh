CONTRACT=./artifacts/neutron_interchain_queries.wasm
CHAINID=test-1
CHAINID2=test-2
NEUTRON_DIR=${NEUTRON_DIR:-../neutron}
HOME=${NEUTRON_DIR}/data/test-1/
HOME2=${NEUTRON_DIR}/data/test-2/
KEY=demowallet1
FAUCET=demowallet3
ADMIN=neutron1m9l358xunhhwds0568za49mzhvuxx9ux8xafx2
RLY2=cosmos17dtl0mjt3t77kpuhg2edqzjpszulwhgzuj9ljs

NEUTROND_BIN=neutrond
GAIAD_BIN=gaiad

TEST_WALLET=test_wallet

yes | ${NEUTROND_BIN} keys add ${TEST_WALLET} --home ${HOME} --keyring-backend=test
TEST_ADDR=$(${NEUTROND_BIN} keys show ${TEST_WALLET} --keyring-backend test -a --home ${HOME})
${NEUTROND_BIN} tx bank send ${FAUCET} ${TEST_ADDR} 100000000stake --chain-id ${CHAINID} --home ${HOME} --node tcp://localhost:16657 --keyring-backend test -y --gas-prices 0.0025stake --broadcast-mode=block


RES=$(${NEUTROND_BIN} tx wasm store ${CONTRACT} --from ${TEST_ADDR} --gas 50000000  --chain-id ${CHAINID} --broadcast-mode=block --gas-prices 0.0025stake  -y --output json  --keyring-backend test --home ${HOME} --node tcp://127.0.0.1:16657)
ICQ_CONTRACT_CODE_ID=$(echo $RES | jq -r '.logs[0].events[1].attributes[0].value')
echo $RES
echo $ICQ_CONTRACT_CODE_ID

INIT_ICQ_CONTRACT='{}'

RES=$(${NEUTROND_BIN} tx wasm instantiate $ICQ_CONTRACT_CODE_ID "$INIT_ICQ_CONTRACT" --from ${TEST_ADDR} --admin ${ADMIN} -y --chain-id ${CHAINID} --output json --broadcast-mode=block --label "init"  --keyring-backend test --gas-prices 0.0025stake --home ${HOME} --node tcp://127.0.0.1:16657)
echo $RES
ICQ_CONTRACT_ADDRESS=$(echo $RES | jq -r '.logs[0].events[0].attributes[0].value')
echo $ICQ_CONTRACT_ADDRESS

${NEUTROND_BIN} tx bank send ${TEST_WALLET} ${ICQ_CONTRACT_ADDRESS} 20000000stake --chain-id ${CHAINID} --home ${HOME} --node tcp://localhost:16657 --keyring-backend test -y --gas-prices 0.0025stake --broadcast-mode=block


RES=$(${NEUTROND_BIN} tx wasm execute $ICQ_CONTRACT_ADDRESS '{"register_balance_query":{"connection_id":"connection-0","addr":"cosmos17dtl0mjt3t77kpuhg2edqzjpszulwhgzuj9ljs","denom":"stake","update_period":10}}' --from ${TEST_ADDR}  -y --chain-id ${CHAINID} --output json --broadcast-mode=block --gas-prices 0.0025stake --gas 1000000 --keyring-backend test --home ${HOME} --node tcp://127.0.0.1:16657)
echo $RES

${NEUTROND_BIN} q interchainqueries registered-queries  --chain-id ${CHAINID}  --home ${HOME} --node tcp://127.0.0.1:16657

RES=$(${NEUTROND_BIN} tx wasm execute $ICQ_CONTRACT_ADDRESS '{"update_interchain_query":{"query_id":1,"new_keys":[{"path":"staking","key":"AhTzV/fuS4r96weXQrLQCkGAufddAnN0YWtl"}]}}' --from ${TEST_ADDR}  -y --chain-id ${CHAINID} --output json --broadcast-mode=block --gas-prices 0.0025stake --gas 1000000 --keyring-backend test --home ${HOME} --node tcp://127.0.0.1:16657)
echo $RES

${NEUTROND_BIN} q interchainqueries registered-queries  --chain-id ${CHAINID}  --home ${HOME} --node tcp://127.0.0.1:16657

RES=$(${NEUTROND_BIN} tx wasm execute $ICQ_CONTRACT_ADDRESS '{"remove_interchain_query":{"query_id":1}}' --from ${TEST_ADDR}  -y --chain-id ${CHAINID} --output json --broadcast-mode=block --gas-prices 0.0025stake --gas 1000000 --keyring-backend test --home ${HOME} --node tcp://127.0.0.1:16657)
echo $RES

RES=$(${NEUTROND_BIN} tx wasm execute $ICQ_CONTRACT_ADDRESS '{"register_balance_query":{"connection_id":"connection-0","addr":"cosmos17dtl0mjt3t77kpuhg2edqzjpszulwhgzuj9ljs","denom":"stake","update_period":10}}' --from ${TEST_ADDR}  -y --chain-id ${CHAINID} --output json --broadcast-mode=block --gas-prices 0.0025stake --gas 1000000 --keyring-backend test --home ${HOME} --node tcp://127.0.0.1:16657)
echo $RES

${NEUTROND_BIN} q interchainqueries registered-queries  --chain-id ${CHAINID}  --home ${HOME} --node tcp://127.0.0.1:16657