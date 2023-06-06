#!/usr/bin/env bash

# http://redsymbol.net/articles/unofficial-bash-strict-mode/
set -euo pipefail
IFS=$'\n\t'

HERMES_CONFIG="./config.toml"
NEUTRON_CHAIN_ID="test-1"
JUNO_CHAIN_ID="uni-6"

NEUTRON_MNEMONIC="alley afraid soup fall idea toss can goose become valve initial strong forward bright dish figure check leopard decide warfare hub unusual join cart"
JUNO_MNEMONIC="ahead horn apart broom chapter pause culture defy original install critic common build act reunion frame shadow trick erode just average keen miss soda"

for chain in "$NEUTRON_CHAIN_ID" "$JUNO_CHAIN_ID"; do
  hermes --config "$HERMES_CONFIG" keys delete --chain "$chain" --all
done

hermes --config "$HERMES_CONFIG" keys add \
  --key-name testkey_1                    \
  --chain "$NEUTRON_CHAIN_ID"             \
  --mnemonic-file <(echo "$NEUTRON_MNEMONIC")

hermes --config "$HERMES_CONFIG" keys add \
  --key-name testkey_2                    \
  --chain "$JUNO_CHAIN_ID"                \
  --mnemonic-file <(echo "$JUNO_MNEMONIC")

hermes --config "$HERMES_CONFIG" create connection \
  --a-chain "$NEUTRON_CHAIN_ID"                    \
  --b-chain "$JUNO_CHAIN_ID"
