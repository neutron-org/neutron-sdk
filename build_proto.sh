docker build thirdparty/protoc-image  -t protoc_builder

docker run --rm \
-v $PWD:/opt \
protoc_builder sh -c "protoc \
-I/opt/proto \
/opt/proto/transfer/v1/*.proto \
--rust_out /opt/packages/neutron-sdk/src/proto_types/"