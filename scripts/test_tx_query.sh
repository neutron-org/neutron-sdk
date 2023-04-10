#!/usr/bin/env bash

# http://redsymbol.net/articles/unofficial-bash-strict-mode/
set -euo pipefail
IFS=$'\n\t'

BIN="neutrond"
GAIA_BIN="gaiad"
CONTRACT_PATH="../artifacts/neutron_interchain_queries.wasm"
CHAIN_ID_1="test-1"
CHAIN_ID_2="test-2"
NEUTRON_DIR="${NEUTRON_DIR:-../../neutron}"
HOME_1="${NEUTRON_DIR}/data/test-1/"
HOME_2="${NEUTRON_DIR}/data/test-2/"
ADDRESS_1="neutron1m9l358xunhhwds0568za49mzhvuxx9ux8xafx2"
ADDRESS_2="cosmos10h9stc5v6ntgeygf5xf945njqq5h32r53uquvw"
ADMIN="neutron1m9l358xunhhwds0568za49mzhvuxx9ux8xafx2"
VALIDATOR="cosmos1qnk2n4nlkpw9xfqntladh74w6ujtulwn7j8za9"
NEUTRON_NODE="tcp://127.0.0.1:26657"
GAIA_NODE="tcp://127.0.0.1:16657"

code_id="$("$BIN" tx wasm store "$CONTRACT_PATH"                \
    --from "$ADDRESS_1" --gas 50000000 --chain-id "$CHAIN_ID_1" \
    --broadcast-mode=block --gas-prices 0.0025untrn -y          \
    --output json  --keyring-backend=test --home "$HOME_1"      \
    --node "$NEUTRON_NODE"                                      \
    | jq -r '.logs[0].events[] | select(.type == "store_code").attributes[] | select(.key == "code_id").value')"
echo "Code ID: $code_id"

contract_address="$("$BIN" tx wasm instantiate "$code_id" '{}'                 \
    --from "$ADDRESS_1" --admin "$ADMIN" -y --chain-id "$CHAIN_ID_1"           \
    --output json --broadcast-mode=block --label "init" --keyring-backend=test \
    --gas-prices 0.0025untrn --home "$HOME_1" --node "$NEUTRON_NODE"           \
    | jq -r '.logs[0].events[] | select(.type == "instantiate").attributes[] | select(.key == "_contract_address").value')"
echo "Contract address: $contract_address"

tx_result="$("$BIN" tx bank send "$ADDRESS_1" "$contract_address" 10000000untrn \
    -y --chain-id "$CHAIN_ID_1" --output json --broadcast-mode=block            \
    --gas-prices 0.0025untrn --gas 300000 --keyring-backend=test                \
    --home "$HOME_1" --node "$NEUTRON_NODE")"
code="$(echo "$tx_result" | jq '.code')"
if [[ "$code" -ne 0 ]]; then
  echo "Failed to send money to contract: $(echo "$tx_result" | jq '.raw_log')" && exit 1
fi
echo "Sent money to contract to pay for deposit"

msg="$(printf '{"register_transfers_query": {
  "connection_id": "connection-0",
  "recipient": "%s",
  "update_period": 5,
  "min_height": 1
}}' "$VALIDATOR")"
tx_result="$("$BIN" tx wasm execute "$contract_address" "$msg"    \
    --from "$ADDRESS_1" -y --chain-id "$CHAIN_ID_1" --output json \
    --broadcast-mode=block --gas-prices 0.0025untrn --gas 1000000 \
    --keyring-backend=test --home "$HOME_1" --node "$NEUTRON_NODE")"
code="$(echo "$tx_result" | jq '.code')"
if [[ "$code" -ne 0 ]]; then
  echo "Failed to register ICQ: $(echo "$tx_result" | jq '.raw_log')" && exit 1
fi
echo "Registered transfers ICQ"

tx_result="$("$GAIA_BIN" tx bank send "$ADDRESS_2" "$VALIDATOR" 1000uatom \
    --gas 50000000 --gas-adjustment 1.4 --output json -y                  \
    --gas-prices 0.5uatom --broadcast-mode=block --chain-id "$CHAIN_ID_2" \
    --keyring-backend=test --home "$HOME_2" --node "$GAIA_NODE")"
code="$(echo "$tx_result" | jq '.code')"
if [[ "$code" -ne 0 ]]; then
  echo "Failed to transfer funds to trigger TX ICQ: $(echo "$tx_result" | jq '.raw_log')" && exit 1
fi
echo "Triggered TX ICQ via transferring funds to watched address"

echo "Waiting 10 seconds for ICQ result to arrive…"
# shellcheck disable=SC2034
for i in $(seq 10); do
  sleep 1
  echo -n .
done
echo " done"

echo
echo "TX query response:"
query="$(printf '{"get_recipient_txs": {"recipient": "%s"}}' "$VALIDATOR")"
"$BIN" query wasm contract-state smart "$contract_address" "$query" --node "$NEUTRON_NODE" --output json | jq
