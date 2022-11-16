args=("$@")

BIN=neutrond
CONTRACT=${args[0]}

NEUTRON_KEY_NAME=validator_test
GAS_PRICES=0.01untrn
EXPLORER_URL=http://explorer.quark.ntrn.info/
FAUCET_URL=https://t.me/+Y_BQropm0_VlZjVi
NODE_URL="${NODE_URL:-tcp://localhost:26657}"

echo "Node url: $NODE_URL"


if [ $# != "1" ]
then
    echo "Usage: ./validator_test_upload_contract.sh [path_to_wasm_artifact]"
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

NEUTRON_CHAIN_ID=$(neutrond status --node ${NODE_URL}| jq -r '.NodeInfo.network')

if [ -z "$NEUTRON_CHAIN_ID" ]
then
    echo "Cannot get chain id"
    exit;
fi
echo "Chain id: $NEUTRON_CHAIN_ID"

RES=$($BIN keys add $NEUTRON_KEY_NAME --output json)
NEUTRON_ADDRESS=$(echo $RES | jq -r .address)
MNEMONIC=$(echo $RES | jq -r .mnemonic)
if [ "$NEUTRON_ADDRESS" = "" ]
then
    echo "Can't get address from key. Probably key already exists and you should agree to override the existing name"
    exit
fi

echo "Local address in neutron: $NEUTRON_ADDRESS"
echo ""
echo " #####                                                                                              ### ### ### "
echo "#     #   ##   #    # ######    #    # #####     #    # #    # ###### #    #  ####  #    # #  ####  ### ### ### "
echo "#        #  #  #    # #         #    # #    #    ##  ## ##   # #      ##  ## #    # ##   # # #    # ### ### ### "
echo " #####  #    # #    # #####     #    # #    #    # ## # # #  # #####  # ## # #    # # #  # # #       #   #   #  "
echo "      # ###### #    # #         #    # #####     #    # #  # # #      #    # #    # #  # # # #                  "
echo "#     # #    #  #  #  #         #    # #   #     #    # #   ## #      #    # #    # #   ## # #    # ### ### ### "
echo " #####  #    #   ##   ######     ####  #    #    #    # #    # ###### #    #  ####  #    # #  ####  ### ### ### "
echo ""
echo "Key mnemonic: $MNEMONIC"
echo "Key name: $NEUTRON_KEY_NAME"
echo ""
echo "Please go to $FAUCET_URL and get tokens for $NEUTRON_ADDRESS"
echo "Make sure tx is passed by going to $EXPLORER_URL/accounts/$NEUTRON_ADDRESS"
echo "Hit enter when ready"
read
echo "Upload the queries contract"
RES=$(${BIN} tx wasm store ${CONTRACT} --from ${NEUTRON_KEY_NAME} --gas 50000000  --node ${NODE_URL} --chain-id ${NEUTRON_CHAIN_ID} --broadcast-mode=block --gas-prices ${GAS_PRICES}  -y --output json)
CONTRACT_CODE_ID=$(echo $RES | jq -r '.logs[0].events[1].attributes[0].value')
if [[ $CONTRACT_CODE_ID == "null" ]]
then
    echo "Can't get code id"
    echo "$RES"
    exit
fi

echo "Contract code id: $CONTRACT_CODE_ID"
echo ""
echo "Instantiate the contract"
INIT_CONTRACT='{}'
RES=$(${BIN} tx wasm instantiate $CONTRACT_CODE_ID "$INIT_CONTRACT" --from $NEUTRON_KEY_NAME --admin ${NEUTRON_ADDRESS} -y  --node ${NODE_URL} --chain-id ${NEUTRON_CHAIN_ID} --output json --broadcast-mode=block --label "init"  --gas-prices ${GAS_PRICES} --gas auto --gas-adjustment 1.4)
CONTRACT_ADDRESS=$(echo $RES | jq -r '.logs[0].events[0].attributes[0].value')
echo "Contract address: $CONTRACT_ADDRESS"

if [[ $CONTRACT_ADDRESS == "null" ]]
then
    echo "Can't get contract address"
    echo "$RES"
    exit
fi

echo "$CONTRACT_ADDRESS" > ./contract_address.tmp