#!/usr/bin/env bash

# http://redsymbol.net/articles/unofficial-bash-strict-mode/
set -euo pipefail
IFS=$'\n\t'

BIN="neutrond"
CONTRACT_PATH="../../artifacts/neutron_interchain_queries.wasm"
CHAIN_ID_1="test-1"
NEUTRON_DIR="${NEUTRON_DIR:-../../../neutron}"
HOME_1="${NEUTRON_DIR}/data/test-1/"
ADDRESS_1="neutron1m9l358xunhhwds0568za49mzhvuxx9ux8xafx2"
ADMIN="neutron1m9l358xunhhwds0568za49mzhvuxx9ux8xafx2"
NODE="tcp://127.0.0.1:26657"
CONNECTION_ID="connection-0"

code_id="$("$BIN" tx wasm store "$CONTRACT_PATH"                \
    --from "$ADDRESS_1" --gas 50000000 --chain-id "$CHAIN_ID_1" \
    --broadcast-mode=block --gas-prices 0.0025untrn -y          \
    --output json --keyring-backend=test --home "$HOME_1"       \
    --node "$NODE"                                              \
    | jq -r '.logs[0].events[] | select(.type == "store_code").attributes[] | select(.key == "code_id").value')"
echo "Code ID: $code_id"

contract_address="$("$BIN" tx wasm instantiate "$code_id" '{}'         \
    --from "$ADDRESS_1" --admin "$ADMIN" -y --chain-id "$CHAIN_ID_1"   \
    --output json --broadcast-mode=block --label "init" --node "$NODE" \
    --keyring-backend=test --gas-prices 0.0025untrn --home "$HOME_1"   \
    | jq -r '.logs[0].events[] | select(.type == "instantiate").attributes[] | select(.key == "_contract_address").value')"
echo "Contract address: $contract_address"

tx_result="$("$BIN" tx bank send "$ADDRESS_1" "$contract_address" 10000000untrn \
    -y --chain-id "$CHAIN_ID_1" --output json --broadcast-mode=block            \
    --gas-prices 0.0025untrn --gas 300000 --keyring-backend=test                \
    --home "$HOME_1" --node "$NODE")"
code="$(echo "$tx_result" | jq '.code')"
if [[ "$code" -ne 0 ]]; then
  echo "Failed to send money to contract: $(echo "$tx_result" | jq '.raw_log')" && exit 1
fi
echo "Sent money to contract to pay for deposit"

msg="$(printf '{"register_cw20_balance_query":{
  "connection_id": "%s",
  "update_period": 5,
  "cw20_contract_address": "juno1jw04ukttvcqrxwy6xsufwrehwhxl8d68d6z7gex4gxwkgta4cwcs8yufe0",
  "account_address": "juno1kk5eceqhcd0u65fqlzcvv52e5rjcshz704kere"
}}' "$CONNECTION_ID")"
tx_result="$("$BIN" tx wasm execute "$contract_address" "$msg"    \
    --from "$ADDRESS_1" -y --chain-id "$CHAIN_ID_1" --output json \
    --broadcast-mode=block --gas-prices 0.0025untrn --gas 1000000 \
    --keyring-backend=test --home "$HOME_1" --node "$NODE")"
code="$(echo "$tx_result" | jq '.code')"
if [[ "$code" -ne 0 ]]; then
  echo "Failed to register ICQ: $(echo "$tx_result" | jq '.raw_log')" && exit 1
fi
query_id="$(echo "$tx_result" | jq -r '.logs[0].events[] | select(.type == "neutron").attributes[] | select(.key == "query_id").value')"
echo "Registered cw20 balance ICQ with query ID: $query_id"

echo "Waiting 10 seconds for ICQ result to arrive…"
# shellcheck disable=SC2034
for i in $(seq 10); do
  sleep 1
  echo -n .
done
echo " done"

echo
echo "KV query cw20 balance response:"
query="$(printf '{"cw20_balance": {"query_id": %s}}' "$query_id")"
"$BIN" query wasm contract-state smart "$contract_address" "$query" --node "$NODE" --output json | jq
