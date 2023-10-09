#!/usr/bin/env bash

# http://redsymbol.net/articles/unofficial-bash-strict-mode/
set -euo pipefail
IFS=$'\n\t'

CONTRACT_PATH="../artifacts/neutron_interchain_queries.wasm"
CHAIN_ID_1="test-1"
NEUTRON_DIR="${NEUTRON_DIR:-../../neutron}"
HOME_1="${NEUTRON_DIR}/data/test-1/"
ADDRESS_1="neutron1m9l358xunhhwds0568za49mzhvuxx9ux8xafx2"
ADMIN="neutron1m9l358xunhhwds0568za49mzhvuxx9ux8xafx2"
NODE="tcp://127.0.0.1:26657"

wait_tx() {
  local txhash
  local attempts
  txhash="$(jq -r '.txhash' </dev/stdin)"
  ((attempts=50))
  while ! neutrond query tx --type=hash "$txhash" --output json --node "$NODE" 2>/dev/null; do
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
    --node "$NODE"                                              \
    | wait_tx | jq -r '.logs[0].events[] | select(.type == "store_code").attributes[] | select(.key == "code_id").value')"
echo "Code ID: $code_id"

contract_address="$(neutrond tx wasm instantiate "$code_id" '{}'      \
    --from "$ADDRESS_1" --admin "$ADMIN" -y --chain-id "$CHAIN_ID_1"  \
    --output json --broadcast-mode=sync --label "init" --node "$NODE" \
    --keyring-backend=test --gas-prices 0.0025untrn --home "$HOME_1"  \
    | wait_tx | jq -r '.logs[0].events[] | select(.type == "instantiate").attributes[] | select(.key == "_contract_address").value')"
echo "Contract address: $contract_address"

tx_result="$(neutrond tx bank send "$ADDRESS_1" "$contract_address" 10000000untrn \
    -y --chain-id "$CHAIN_ID_1" --output json --broadcast-mode=sync               \
    --gas-prices 0.0025untrn --gas 300000 --keyring-backend=test                  \
    --home "$HOME_1" --node "$NODE" | wait_tx)"
code="$(echo "$tx_result" | jq '.code')"
if [[ "$code" -ne 0 ]]; then
  echo "Failed to send money to contract: $(echo "$tx_result" | jq '.raw_log')" && exit 1
fi
echo "Sent money to contract to pay for deposit"

msg='{"register_bank_total_supply_query":{
  "connection_id": "connection-0",
  "denoms": ["uatom"],
  "update_period": 5
}}'
tx_result="$(neutrond tx wasm execute "$contract_address" "$msg"  \
    --from "$ADDRESS_1" -y --chain-id "$CHAIN_ID_1" --output json \
    --broadcast-mode=sync --gas-prices 0.0025untrn --gas 1000000  \
    --keyring-backend=test --home "$HOME_1" --node "$NODE" | wait_tx)"
code="$(echo "$tx_result" | jq '.code')"
if [[ "$code" -ne 0 ]]; then
  echo "Failed to register ICQ: $(echo "$tx_result" | jq '.raw_log')" && exit 1
fi
query_id="$(echo "$tx_result" | jq -r '.logs[0].events[] | select(.type == "neutron").attributes[] | select(.key == "query_id").value')"
echo "Registered total supply ICQ with query ID: $query_id"

echo "Waiting 10 seconds for ICQ result to arrive…"
# shellcheck disable=SC2034
for i in $(seq 10); do
  sleep 1
  echo -n .
done
echo " done"

echo
echo "KV query total supply response:"
query="$(printf '{"bank_total_supply": {"query_id": %s}}' "$query_id")"
neutrond query wasm contract-state smart "$contract_address" "$query" --node "$NODE" --output json | jq
