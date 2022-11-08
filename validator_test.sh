args=("$@")

if [ $# != "1" ]
then
    echo "Usage: ./validator_test.sh  [connection-id]"
    exit
fi

if [[ ! -f ./contract_address.tmp ]]
then
    echo "Run ./validator_test_upload_contract.sh first"
    exit
fi

NODE_URL="${NODE_URL:-tcp://localhost:26657}"
echo "Node url: $NODE_URL"

CONTRACT_ADDRESS=$(cat ./contract_address.tmp)
echo "Contract address: $CONTRACT_ADDRESS"


## Set up envs
echo "What network you going to interact:"
echo "1. CosmosHub"
echo "2. Juno"

read i

DENOMS=([1]="atom" [2]="junox")
TARGET_DENOMS=([1]="uatom" [2]="ujunox")
ICA_LENGTH=([1]="65" [2]="63")
REMOTE_EXPLORER=([1]="https://explorer.theta-testnet.polypore.xyz" [2]="https://testnet.juno.explorers.guru")

if [[ -z ${DENOMS[$i]} ]]
then
    echo "No such network"
fi

BIN=neutrond
CONNECTION_ID=${args[0]}

NEUTRON_KEY_NAME=validator_test
INTERCHAIN_ACCOUNT_ID=version1
GAS_PRICES=0.01untrn
TARGET_VALIDATORS=([1]="cosmosvaloper1mngvkkhm6g7nqxh4hcv8hjxvgax4m8xujzt964" [2]="junovaloper18wgy6hy6yv3fvevl5pyfn7cvzx3t5use2vssnf")
TARGET_VALIDATOR=${TARGET_VALIDATORS[$i]}
TARGET_DENOM=${TARGET_DENOMS[$i]}
EXPLORER_URL=http://23.109.159.28:3333/
FAUCET_URL=http://23.109.159.28/

if ! command -v $BIN &> /dev/null
then
    echo "$BIN could not be found.
You can add symlink from your neutron binary to /bin this way: ln -s PATH_TO_NEUTRON_BIN /bin/neutron"
    exit
fi

NEUTRON_CHAIN_ID=$(neutrond status --node ${NODE_URL}| jq -r '.NodeInfo.network')

if [ -z "$NEUTRON_CHAIN_ID" ]
then
    echo "Cannot get chain id"
    exit;
fi
echo "Chain id: $NEUTRON_CHAIN_ID"

## Check if ibc connection does exist
RES=$(neutrond query ibc connection end $CONNECTION_ID --node ${NODE_URL} 2>/dev/null)

if [ -z "$RES" ]
then
    echo "No such open connection for provided connection-id"
    exit;
fi
echo "Connection id: $CONNECTION_ID"
echo ""

## Set ibc fees
echo "Set IBC fees"
RES=$(${BIN} tx wasm execute ${CONTRACT_ADDRESS} "{\"set_fees\": {\"ack_fee\": \"2000\", \"recv_fee\": \"2000\",\"timeout_fee\": \"2000\", \"denom\": \"${INTERCHAIN_ACCOUNT_ID}\"}}" --from $NEUTRON_KEY_NAME  -y --chain-id ${NEUTRON_CHAIN_ID} --node ${NODE_URL} --output json --broadcast-mode=block --gas-prices ${GAS_PRICES} --gas 1000000)
CODE=$(echo $RES | jq -r '.code')
if [ $CODE != "0" ]
then
    echo "Set fees failed"
    echo "$RES"
    exit
fi


## Fund contract to be able to pay fees
echo "Fund the contract to pay for IBC fees"
RES=$(${BIN} tx bank send $NEUTRON_KEY_NAME ${CONTRACT_ADDRESS} 20000untrn --chain-id ${NEUTRON_CHAIN_ID} --node ${NODE_URL} --gas-prices 0${GAS_PRICES})
echo ""

## Register interchain account
echo "Register interchain account"
RES=$(${BIN} tx wasm execute ${CONTRACT_ADDRESS} "{\"register\": {\"connection_id\": \"${CONNECTION_ID}\", \"interchain_account_id\": \"${INTERCHAIN_ACCOUNT_ID}\"}}" --from $NEUTRON_KEY_NAME  -y --chain-id ${NEUTRON_CHAIN_ID} --node ${NODE_URL} --output json --broadcast-mode=block --gas-prices ${GAS_PRICES} --gas 2000000)
echo "$RES"
echo ""
echo "Waiting for registering account..."

## Wait until ICA appears on the target chain
j=40
while [[ $j -gt 0 ]]
do
    ((j--))
    RES=$(neutrond query wasm contract-state smart ${CONTRACT_ADDRESS} "{\"interchain_account_address_from_contract\":{\"interchain_account_id\":\"${INTERCHAIN_ACCOUNT_ID}\"}}" --chain-id ${NEUTRON_CHAIN_ID} --node ${NODE_URL} --output json 2>/dev/null)
    ICA_ADDRESS=$(echo $RES | jq -r '.data | .[0]')
    if [ ${#ICA_ADDRESS} = ${ICA_LENGTH[$i]} ]
    then
	break
    fi
    sleep 5
done

if [ ${#ICA_ADDRESS} != ${ICA_LENGTH[$i]} ]
then
    echo "Can't get ICA address"
    exit
fi
echo "ICA address: $ICA_ADDRESS"
echo ""
echo "Please send 0.02 ${DENOMS[$i]} to $ICA_ADDRESS"
echo "hit enter when you are ready"
read
echo ""

## Execute Interchain Delegate tx
echo "Execute Interchain Delegate tx"
RES=$(${BIN} tx wasm execute ${CONTRACT_ADDRESS} "{\"delegate\": {\"interchain_account_id\": \"${INTERCHAIN_ACCOUNT_ID}\", \"validator\": \"${TARGET_VALIDATOR}\", \"denom\":\"${TARGET_DENOM}\", \"amount\":\"9000\"}}" --from ${NEUTRON_KEY_NAME}  -y --chain-id ${NEUTRON_CHAIN_ID} --node ${NODE_URL} --output json --broadcast-mode=block --gas-prices ${GAS_PRICES} --gas 1000000)
CODE=$(echo $RES | jq -r '.code')
echo "$RES"
if [ $CODE != "0" ]
then
    echo "Delegation failed"
fi
echo "Waiting for delegation..."

## Wait until ackowledgement appears on the source chain
ACK=0
j=60
while [[ $j -gt 0 ]]
do
    ((j--))
    RES=$(${BIN} query wasm contract-state smart ${CONTRACT_ADDRESS} "{\"acknowledgement_result\":{\"interchain_account_id\":\"${INTERCHAIN_ACCOUNT_ID}\", \"sequence_id\": 1}}" --chain-id ${NEUTRON_CHAIN_ID} --node ${NODE_URL} --output json 2>/dev/null)
    if [ "$RES" = "{\"data\":{\"success\":[\"/cosmos.staking.v1beta1.MsgDelegate\"]}}" ]
    then
	ACK=1
	break
    fi
    sleep 5
done

if [ $ACK = "0" ] 
then
    echo "Error: Acknowledgement has not been received"
    exit
else
   echo "Acknowledgement has  been received"
   echo ""
   echo "Now you can check your delegation here ${REMOTE_EXPLORER[$i]}/account/$ICA_ADDRESS"
   echo "Hit return to continue"
   read
fi
echo ""

# Clear ACK results on contract before the next test
echo "Clear ACK results on contract before the next test"
RES=$(${BIN} tx wasm execute ${CONTRACT_ADDRESS} "{\"clean_ack_results\": {}}" --from ${NEUTRON_KEY_NAME}  -y --chain-id ${NEUTRON_CHAIN_ID} --node ${NODE_URL} --output json --broadcast-mode=block --gas-prices ${GAS_PRICES} --gas 1000000)
CODE=$(echo $RES | jq -r '.code')
if [ $CODE != "0" ]
then
    echo "Cleaning failed"
    echo "$RES"
fi


echo ""
# Execute Interchain Delegate tx (with host chain error)
echo "Execute Interchain Delegate tx (with host chain error)"
RES=$(${BIN} tx wasm execute ${CONTRACT_ADDRESS} "{\"delegate\": {\"interchain_account_id\": \"${INTERCHAIN_ACCOUNT_ID}\", \"validator\": \"fake_address\", \"denom\":\"${TARGET_DENOM}\", \"amount\":\"9000\"}}" --from ${NEUTRON_KEY_NAME}  -y --chain-id ${NEUTRON_CHAIN_ID} --node ${NODE_URL} --output json --broadcast-mode=block --gas-prices ${GAS_PRICES} --gas 1000000)
CODE=$(echo $RES | jq -r '.code')
echo "$RES"
if [ $CODE != "0" ]
then
    echo "Delegation failed"
fi
echo "Waiting for delegation..."

ACK=0
j=60
while [[ $j -gt 0 ]]
do
    ((j--))
    RES=$(${BIN} query wasm contract-state smart ${CONTRACT_ADDRESS} "{\"acknowledgement_result\":{\"interchain_account_id\":\"${INTERCHAIN_ACCOUNT_ID}\", \"sequence_id\": 2}}" --chain-id ${NEUTRON_CHAIN_ID} --node ${NODE_URL} --output json 2>/dev/null)
    if [ "$RES" = "{\"data\":{\"error\":[\"message\",\"ABCI code: 1: error handling packet on host chain: see events for details\"]}}" ]
    then
	ACK=1
	break
    fi
    sleep 5
done

if [ $ACK = "0" ] 
then
    echo "Error: Acknowledgement has not been received"
    exit
else
   echo "Acknowledgement has been received"
   echo "Hit return to continue"
   read
fi

echo ""
# Execute Interchain Delegate tx (with contract error)
echo "Execute Interchain Delegate tx (with contract error)"
RES=$(${BIN} tx wasm execute ${CONTRACT_ADDRESS} "{\"delegate\": {\"interchain_account_id\": \"${INTERCHAIN_ACCOUNT_ID}\", \"validator\": \"${TARGET_VALIDATOR}\", \"denom\":\"${TARGET_DENOM}\", \"amount\":\"6666\"}}" --from ${NEUTRON_KEY_NAME}  -y --chain-id ${NEUTRON_CHAIN_ID} --node ${NODE_URL} --output json --broadcast-mode=block --gas-prices ${GAS_PRICES} --gas 1000000)
echo "$RES"
CODE=$(echo $RES | jq -r '.code')
if [ $CODE != "0" ]
then
    echo "Delegation failed"
fi
echo "Waiting for delegation..."
# TODO: query neutron for contract failures


