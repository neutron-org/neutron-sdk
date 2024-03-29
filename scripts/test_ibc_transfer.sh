#!/usr/bin/env bash

# http://redsymbol.net/articles/unofficial-bash-strict-mode/
set -euo pipefail
IFS=$'\n\t'

CONTRACT_PATH="../artifacts/ibc_transfer.wasm"
CHAIN_ID="test-1"
NEUTRON_DIR="${NEUTRON_DIR:-../../neutron}"
HOME="$NEUTRON_DIR/data/test-1/"
KEY="demowallet1"
ADMIN="neutron1m9l358xunhhwds0568za49mzhvuxx9ux8xafx2"
NODE="tcp://127.0.0.1:26657"
GAIA_NODE="tcp://127.0.0.1:16657"

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

code_id="$(neutrond tx wasm store "$CONTRACT_PATH" \
    --from "$KEY" -y --chain-id "$CHAIN_ID"        \
    --gas 50000000 --gas-prices 0.0025untrn        \
    --broadcast-mode=sync --keyring-backend=test   \
    --output json --home "$HOME" --node "$NODE"    \
    | wait_tx | jq -r '.logs[0].events[] | select(.type == "store_code").attributes[] | select(.key == "code_id").value')"
echo "Code ID: $code_id"

contract_address="$(neutrond tx wasm instantiate "$code_id" '{}' \
    --from ${KEY} --admin ${ADMIN} -y --chain-id "$CHAIN_ID"     \
    --output json --broadcast-mode=sync --label "init"           \
    --keyring-backend=test --gas-prices 0.0025untrn              \
    --home "$HOME" --node "$NODE"                                \
    | wait_tx | jq -r '.logs[0].events[] | select(.type == "instantiate").attributes[] | select(.key == "_contract_address").value')"
echo "Contract address: $contract_address"

tx_result="$(neutrond tx bank send demowallet1 "$contract_address" 20000untrn \
    -y --chain-id "$CHAIN_ID" --home "$HOME" --node "$NODE"                   \
    --keyring-backend=test --gas-prices 0.0025untrn --output json             \
    --broadcast-mode=sync | wait_tx)"
code="$(echo "$tx_result" | jq '.code')"
if [[ "$code" -ne 0 ]]; then
  echo "Failed to send money to contract: $(echo "$tx_result" | jq '.raw_log')" && exit 1
fi
echo "Sent money to contract to pay fees"

msg='{"send":{
  "to": "cosmos17dtl0mjt3t77kpuhg2edqzjpszulwhgzuj9ljs",
  "amount": "1000",
  "denom": "untrn",
  "channel": "channel-0"
}}'
tx_result="$(neutrond tx wasm execute "$contract_address" "$msg" \
    --from ${KEY} -y --chain-id ${CHAIN_ID} --output json        \
    --broadcast-mode=sync --gas-prices 0.0025untrn --gas 1000000 \
    --keyring-backend test --home "$HOME" --node "$NODE" | wait_tx)"
code="$(echo "$tx_result" | jq '.code')"
if [[ "$code" -ne 0 ]]; then
  echo "Failed to execute contract: $(echo "$tx_result" | jq '.raw_log')" && exit 1
fi

echo "Waiting 20 seconds for IBC transfer to complete…"
# shellcheck disable=SC2034
for i in $(seq 20); do
  sleep 1
  echo -n .
done
echo " done"

echo
echo "cosmos17dtl0mjt3t77kpuhg2edqzjpszulwhgzuj9ljs should have 3000untrn now:"
gaiad query bank balances "cosmos17dtl0mjt3t77kpuhg2edqzjpszulwhgzuj9ljs" \
    --node "$GAIA_NODE" --output json | jq '.balances'

echo
echo "If you see more than 3000untrn, you have already run this test several times before"
