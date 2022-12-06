#!/bin/bash

wait_for_tx () {
    local NEUTROND_BIN=$1
    local CONTRACT_ADDRESS=$2
    local NEUTRON_CHAIN_ID=$3
    local NODE_URL=$4
    local QUERY=$5
    local SELECTOR=$6

    echo "Waiting for transaction to be committed..."
    for RETRY_NUM in {0..60}
    do
        RES=$(${NEUTROND_BIN} query wasm contract-state smart \
            ${CONTRACT_ADDRESS} "${QUERY}" \
            --chain-id "$NEUTRON_CHAIN_ID" \
            --output json \
            --node ${NODE_URL} 2>&1)

        CONRACT_CALL_ERROR=$?

        if [ "$CONRACT_CALL_ERROR" != "0" ]
        then
            sleep 1
            continue
        fi
        
        FUNC_RETURN=$(echo "$RES" | jq -r "${SELECTOR}")
        return 
    done

    echo "Timeout waiting for tx"
    exit 1    
}

