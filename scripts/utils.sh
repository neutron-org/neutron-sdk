#!/bin/bash

wait_for_tx () {
    local QUERY=$1
    local SELECTOR=$2

    echo "Waiting for transaction to be committed..."
    for RETRY_NUM in {0..60}
    do
        RES=$(eval "curl -s ${QUERY}" | jq -r "${SELECTOR}")
        echo -n "."

        if [ "$RES" != "null" ]
        then
            echo ""
            FUNC_RETURN=$RES
            return 
        fi

        sleep 1
    done

echo ""
    echo "Timeout waiting for tx"
    exit 1    
}

