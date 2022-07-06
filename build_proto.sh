docker build thirdparty/protoc-image  -t protoc_builder

docker run --rm \
-v $PWD:/opt \
protoc_builder sh -c "protoc \
-I/opt/proto/ \
/opt/proto/gogoproto/gogo.proto \
/opt/proto/cosmos/distribution/v1beta1/*.proto \
/opt/proto/cosmos/base/v1beta1/*.proto \
/opt/proto/cosmos/staking/v1beta1/*.proto \
/opt/proto/tendermint/abci/abcitypes.proto \
/opt/proto/tendermint/types/*.proto  \
/opt/proto/tendermint/crypto/*.proto \
/opt/proto/tendermint/version/*.proto \
--rust_out /opt/packages/stargate/src/interchain/ /opt/proto/interchainqueries/*.proto /opt/proto/interchainaccounts/*.proto; "