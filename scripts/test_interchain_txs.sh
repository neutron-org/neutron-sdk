#!/usr/bin/env bash

# http://redsymbol.net/articles/unofficial-bash-strict-mode/
set -euo pipefail
IFS=$'\n\t'

BIN="neutrond"
GAIA_BIN="gaiad"
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

code_id="$("$BIN" tx wasm store "$CONTRACT_PATH"                \
    --from "$ADDRESS_1" --gas 50000000 --chain-id "$CHAIN_ID_1" \
    --broadcast-mode=block --gas-prices 0.0025untrn -y          \
    --output json --keyring-backend=test --home "$HOME_1"       \
    --node "$NEUTRON_NODE"                                      \
    | jq -r '.logs[0].events[] | select(.type == "store_code").attributes[] | select(.key == "code_id").value')"
echo "Code ID: $code_id"

contract_address=$("$BIN" tx wasm instantiate "$code_id" '{}'        \
    --from "$ADDRESS_1" --admin "$ADMIN" -y --chain-id "$CHAIN_ID_1" \
    --output json --broadcast-mode=block --label "init"              \
    --keyring-backend=test --gas-prices 0.0025untrn --gas auto       \
    --gas-adjustment 1.4 --home "$HOME_1"                            \
    --node "$NEUTRON_NODE" 2>/dev/null                               \
    | jq -r '.logs[0].events[] | select(.type == "instantiate").attributes[] | select(.key == "_contract_address").value')
echo "Contract address: $contract_address"

tx_result="$("$BIN" tx bank send demowallet1 "$contract_address" 100000untrn \
    --chain-id "$CHAIN_ID_1" --home "$HOME_1" --node "$NEUTRON_NODE"         \
    --keyring-backend=test -y --gas-prices 0.0025untrn                       \
    --broadcast-mode=block --output json)"
code="$(echo "$tx_result" | jq '.code')"
if [[ "$code" -ne 0 ]]; then
  echo "Failed to send money to contract: $(echo "$tx_result" | jq '.raw_log')" && exit 1
fi
echo "Sent money to contract to pay fees"

msg='{"register":{
  "connection_id": "connection-0",
  "interchain_account_id": "test"
}}'
tx_result="$("$BIN" tx wasm execute "$contract_address" "$msg"    \
    --from "$ADDRESS_1" -y --chain-id "$CHAIN_ID_1" --output json \
    --broadcast-mode=block --gas-prices 0.0025untrn --gas 1000000 \
    --keyring-backend=test --home "$HOME_1" --node "$NEUTRON_NODE")"
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

# FIXME: why do we even perform a query like this, can't we do `neutrond query wasm smart blah-blah`?
query='{"interchain_account_address_from_contract":{"interchain_account_id":"test"}}'
query_b64_urlenc="$(echo -n "$query" | base64 | tr -d '\n' | jq -sRr '@uri')"
url="http://127.0.0.1:1317/wasm/contract/$contract_address/smart/$query_b64_urlenc?encoding=base64"
ica_address=$(curl -s "$url" | jq -r '.result.smart' | base64 -d | jq -r '.[0]')
echo "ICA address: $ica_address"

tx_result=$("$GAIA_BIN" tx bank send "$ADDRESS_2" "$ica_address" 50000uatom  \
    --chain-id "$CHAIN_ID_2" --broadcast-mode=block --gas-prices 0.0025uatom \
    -y --output json --keyring-backend=test --home "$HOME_2" --node "$GAIA_NODE")
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
tx_result="$("$BIN" tx wasm execute "$contract_address" "$msg"    \
    --from "$ADDRESS_1" -y --chain-id "$CHAIN_ID_1" --output json \
    --broadcast-mode=block --gas-prices 0.0025untrn --gas 1000000 \
    --keyring-backend=test --home "$HOME_1" --node "$NEUTRON_NODE")"
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
