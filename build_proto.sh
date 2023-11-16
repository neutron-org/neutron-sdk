docker build thirdparty/protoc-image  -t protoc_builder

docker run --rm \
-v $PWD:/opt \
protoc_builder sh -c "protoc \
-I/opt/proto \
/opt/proto/gogoproto/*.proto \
/opt/proto/cosmos_proto/*.proto \
/opt/proto/cosmos/bank/v1beta1/bank.proto \
/opt/proto/transfer/v1/*.proto \
/opt/proto/neutron/dex/*.proto \
--rust_out /opt/packages/neutron-sdk/src/proto_types/"
