#!/usr/bin/env bash

# http://redsymbol.net/articles/unofficial-bash-strict-mode/
set -euo pipefail
IFS=$'\n\t'

CONTRACT_PATH="../artifacts/neutron_interchain_txs.wasm"
CHAIN_ID_1="test-1"
CHAIN_ID_2="test-2"
NEUTRON_DIR="${NEUTRON_DIR:-../../neutron}"
HOME_1="${NEUTRON_DIR}/data/test-1/"
HOME_2="${NEUTRON_DIR}/data/test-2/"
ADDRESS_1="neutron1m9l358xunhhwds0568za49mzhvuxx9ux8xafx2"
ADDRESS_2="cosmos10h9stc5v6ntgeygf5xf945njqq5h32r53uquvw"
ADMIN="neutron1m9l358xunhhwds0568za49mzhvuxx9ux8xafx2"
VALIDATOR="cosmosvaloper18hl5c9xn5dze2g50uaw0l2mr02ew57zk0auktn"
NEUTRON_NODE="tcp://127.0.0.1:26657"
GAIA_NODE="tcp://127.0.0.1:16657"

wait_tx() {
  local txhash
  local attempts
  txhash="$(jq -r '.txhash' </dev/stdin)"
  ((attempts=50))
  while ! neutrond query tx --type=hash "$txhash" --output json --node "$NEUTRON_NODE" 2>/dev/null; do
    ((attempts-=1)) || {
      echo "tx $txhash still not included in block" 1>&2
      exit 1
    }
    sleep 0.1
  done
}

wait_tx_gaia() {
  local txhash
    local attempts
    txhash="$(jq -r '.txhash' </dev/stdin)"
    ((attempts=50))
    while ! gaiad query tx --type=hash "$txhash" --output json --node "$GAIA_NODE" 2>/dev/null; do
      ((attempts-=1)) || {
        echo "tx $txhash still not included in block" 1>&2
        exit 1
      }
      sleep 0.1
    done
}

code_id="$(neutrond tx wasm store "$CONTRACT_PATH"              \
    --from "$ADDRESS_1" --gas 50000000 --chain-id "$CHAIN_ID_1" \
    --broadcast-mode=sync --gas-prices 0.0025untrn -y           \
    --output json --keyring-backend=test --home "$HOME_1"       \
    --node "$NEUTRON_NODE"                                      \
    | wait_tx | jq -r '.logs[0].events[] | select(.type == "store_code").attributes[] | select(.key == "code_id").value')"
echo "Code ID: $code_id"

contract_address=$(neutrond tx wasm instantiate "$code_id" '{}'      \
    --from "$ADDRESS_1" --admin "$ADMIN" -y --chain-id "$CHAIN_ID_1" \
    --output json --broadcast-mode=sync --label "init"               \
    --keyring-backend=test --gas-prices 0.0025untrn --gas auto       \
    --gas-adjustment 1.4 --home "$HOME_1"                            \
    --node "$NEUTRON_NODE" 2>/dev/null                               \
    | wait_tx | jq -r '.logs[0].events[] | select(.type == "instantiate").attributes[] | select(.key == "_contract_address").value')
echo "Contract address: $contract_address"

msg='{"register":{
  "connection_id": "connection-0",
  "interchain_account_id": "test",
  "register_fee": [{"denom":"untrn","amount":"1000000"}]
}}'
tx_result="$(neutrond tx wasm execute "$contract_address" "$msg" --amount 1100000untrn  \
    --from "$ADDRESS_1" -y --chain-id "$CHAIN_ID_1" --output json \
    --broadcast-mode=sync --gas-prices 0.0025untrn --gas 1000000  \
    --keyring-backend=test --home "$HOME_1" --node "$NEUTRON_NODE" | wait_tx)"
code="$(echo "$tx_result" | jq '.code')"
if [[ "$code" -ne 0 ]]; then
  echo "Failed to register interchain account: $(echo "$tx_result" | jq '.raw_log')" && exit 1
fi
echo "Waiting 60 seconds for interchain account (sometimes it takes a lot of time)…"
# shellcheck disable=SC2034
for i in $(seq 60); do
  sleep 1
  echo -n .
done
echo " done"

msg='{"interchain_account_address_from_contract":{"interchain_account_id":"test"}}'
ica_address="$(neutrond query wasm contract-state smart "$contract_address" "$msg" \
    --node "$NEUTRON_NODE" --output json | jq -r '.data[0]')"
echo "ICA address: $ica_address"

tx_result=$(gaiad tx bank send "$ADDRESS_2" "$ica_address" 50000uatom       \
    --chain-id "$CHAIN_ID_2" --broadcast-mode=sync --gas-prices 0.0025uatom \
    -y --output json --keyring-backend=test --home "$HOME_2" --node "$GAIA_NODE" | wait_tx_gaia)
code="$(echo "$tx_result" | jq '.code')"
if [[ "$code" -ne 0 ]]; then
  echo "Failed to send money to ICA: $(echo "$tx_result" | jq '.raw_log')" && exit 1
fi
echo "Sent money to ICA"

msg="$(printf '{"delegate":{
  "interchain_account_id": "test",
  "validator": "%s",
  "amount": "2000",
  "denom": "uatom"
}}' "$VALIDATOR")"
tx_result="$(neutrond tx wasm execute "$contract_address" "$msg"  \
    --from "$ADDRESS_1" -y --chain-id "$CHAIN_ID_1" --output json \
    --broadcast-mode=sync --gas-prices 0.0025untrn --gas 1000000  \
    --keyring-backend=test --home "$HOME_1" --node "$NEUTRON_NODE" | wait_tx)"
code="$(echo "$tx_result" | jq '.code')"
if [[ "$code" -ne 0 ]]; then
  echo "Failed to execute contract: $(echo "$tx_result" | jq '.raw_log')" && exit 1
fi

echo "Waiting 20 seconds for interchain transaction to complete…"
# shellcheck disable=SC2034
for i in $(seq 20); do
  sleep 1
  echo -n .
done
echo " done"

echo
echo "This delegation is performed using interchain tx module"
curl -s "http://127.0.0.1:1316/staking/delegators/$ica_address/delegations" | jq '.result'
