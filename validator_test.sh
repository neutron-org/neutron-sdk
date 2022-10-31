args=("$@")
## Set up envs
echo "What network you going to interact:"
echo "1. CosmosHub"
echo "2. Juno"

read -N 1 i
echo $i

DENOMS=([1]="atom" [2]="junox")
TARGET_DENOMS=([1]="uatom" [2]="ujunox")
ICA_LENGTH=([1]="65" [2]="63")
REMOTE_EXPLORER=([1]="https://explorer.theta-testnet.polypore.xyz" [2]="https://testnet.juno.explorers.guru")

if [[ -z ${DENOMS[$i]} ]]
then
    echo "No such network"
fi

BIN=neutrond
CONTRACT=${args[0]}
CONNECTION_ID=${args[1]}

NEUTRON_KEY_NAME=validator_test
INTERCHAIN_ACCOUNT_ID=version1
GAS_PRICES=0.01untrn
TARGET_VALIDATORS=([1]="cosmosvaloper1mngvkkhm6g7nqxh4hcv8hjxvgax4m8xujzt964" [2]="junovaloper18wgy6hy6yv3fvevl5pyfn7cvzx3t5use2vssnf")
TARGET_VALIDATOR=${TARGET_VALIDATORS[$i]}
TARGET_DENOM=${TARGET_DENOMS[$i]}
EXPLORER_URL=http://23.109.159.28:3333/
FAUCET_URL=http://23.109.159.28/

if [ $# != "2" ]
then
    echo "Usage: ./validator_test.sh [path_to_wasm_artifact] [connection-id]"
    exit
fi

if [[ ! -f $CONTRACT ]]
then
    echo "Artifact file doesn't exists"
    exit
fi

if ! command -v $BIN &> /dev/null
then
    echo "$BIN could not be found.
You can add symlink from your neutron binary to /bin this way: ln -s PATH_TO_NEUTRON_BIN /bin/neutron"
    exit
fi

NEUTRON_CHAIN_ID=$(neutrond status | jq -r '.NodeInfo.network')

if [ -z "$NEUTRON_CHAIN_ID" ]
then
    echo "Cannot get chain id"
    exit;
fi
echo "Chain id: $NEUTRON_CHAIN_ID"

## Check if ibc connection does exist
RES=$(neutrond query ibc connection end $CONNECTION_ID 2>/dev/null)

if [ -z "$RES" ]
then
    echo "No such open connection for provided connection-id"
    exit;
fi
echo "Connection id: $CONNECTION_ID"
echo ""

## Add a new key
RES=$($BIN keys add $NEUTRON_KEY_NAME --output json)
NEUTRON_ADDRESS=$(echo $RES | jq -r .address)
MNEMONIC=$(echo $RES | jq -r .mnemonic)
if [ $NEUTRON_ADDRESS = "null" ]
then
    echo "Can't get address from key"
    exit
fi

echo "Local address in neutron: $NEUTRON_ADDRESS"
echo "Key mnemonic: $MNEMONIC"
echo "Key name: $NEUTRON_KEY_NAME"
echo ""
echo "Please go to $FAUCET_URL and get tokens for $NEUTRON_ADDRESS"
echo "Make sure tx is passed by going to $EXPLORER_URL/accounts/$NEUTRON_ADDRESS"
echo "Hit enter when ready"
read

## Upload contract
echo "Upload the queries contract"
RES=$(${BIN} tx wasm store ${CONTRACT} --from ${NEUTRON_KEY_NAME} --gas 50000000 --chain-id ${NEUTRON_CHAIN_ID} --broadcast-mode=block --gas-prices ${GAS_PRICES}  -y --output json)
CONTRACT_CODE_ID=$(echo $RES | jq -r '.logs[0].events[1].attributes[0].value')
if [ $CONTRACT_CODE_ID = "null" ]
then
    echo "Can't get code id"
    exit
fi
echo "Contract code id: $CONTRACT_CODE_ID"
echo ""

## Instantiate contract
echo "Instantiate the contract"
INIT_CONTRACT='{}'
RES=$(${BIN} tx wasm instantiate $CONTRACT_CODE_ID "$INIT_CONTRACT" --from $NEUTRON_KEY_NAME --admin ${NEUTRON_ADDRESS} -y --chain-id ${NEUTRON_CHAIN_ID} --output json --broadcast-mode=block --label "init"  --gas-prices ${GAS_PRICES} --gas auto --gas-adjustment 1.4)
CONTRACT_ADDRESS=$(echo $RES | jq -r '.logs[0].events[0].attributes[0].value')
echo "Contract address: $CONTRACT_ADDRESS"
if [ $CONTRACT_ADDRESS = "null" ]
then
    echo "Can't get contract address"
    exit
fi
echo ""


## Register interchain account
echo "Register interchain account"
RES=$(${BIN} tx wasm execute ${CONTRACT_ADDRESS} "{\"register\": {\"connection_id\": \"${CONNECTION_ID}\", \"interchain_account_id\": \"${INTERCHAIN_ACCOUNT_ID}\"}}" --from $NEUTRON_KEY_NAME  -y --chain-id ${NEUTRON_CHAIN_ID} --output json --broadcast-mode=block --gas-prices ${GAS_PRICES} --gas 1000000)
echo "Waiting for registering account..."

## Wait until ICA appears on the target chain
j=40
while [[ $j -gt 0 ]]
do
    ((j--))
    RES=$(neutrond query wasm contract-state smart ${CONTRACT_ADDRESS} "{\"interchain_account_address_from_contract\":{\"interchain_account_id\":\"${INTERCHAIN_ACCOUNT_ID}\"}}" --chain-id ${NEUTRON_CHAIN_ID} --output json 2>/dev/null)
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
RES=$(${BIN} tx wasm execute ${CONTRACT_ADDRESS} "{\"delegate\": {\"interchain_account_id\": \"${INTERCHAIN_ACCOUNT_ID}\", \"validator\": \"${TARGET_VALIDATOR}\", \"denom\":\"${TARGET_DENOM}\", \"amount\":\"9000\"}}" --from ${NEUTRON_KEY_NAME}  -y --chain-id ${NEUTRON_CHAIN_ID} --output json --broadcast-mode=block --gas-prices ${GAS_PRICES} --gas 1000000)
CODE=$(echo $RES | jq -r '.code')
if [ $CODE != "0" ]
then
    echo "Delegation failed"
fi
echo "Waiting for delegation..."

## Wait until ackowledgement appears on the source chain
j=40
while [[ $j -gt 0 ]]
do
    ((j--))
    RES=$(${BIN} query wasm contract-state smart ${CONTRACT_ADDRESS} "{\"acknowledgement_result\":{\"interchain_account_id\":\"${INTERCHAIN_ACCOUNT_ID}\", \"sequence_id\": 1}}" --chain-id ${NEUTRON_CHAIN_ID} --output json 2>/dev/null)
    if [ "$RES" = "{\"data\":{\"success\":[\"/cosmos.staking.v1beta1.MsgDelegate\"]}}" ]
    then
	echo "Acknowledgement has  been received"
	echo ""
	echo "Now you can check your delegation here ${REMOTE_EXPLORER[$i]}/account/$ICA_ADDRESS"
	echo "Hit return to exit"
	read
	exit
    fi
    sleep 5
done
echo "Error: Acknowledgement has not been received"

