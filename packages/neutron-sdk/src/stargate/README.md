# Neutron Stargate interface

This package contains list of helpers to interact with Neutron blockchain via Stargate.

### Dex module

For the `dex` module, there are helpers for all possible messages and queries in the package. The helpers have manually written adapted types for requests and responses instead of proto generated ones because proto-gen works poorly with rust code as the output.

- helpers to construct CosmosMsgs to the dex module are placed in the [msg_dex.rs](https://github.com/neutron-org/neutron-sdk/tree/main/packages/neutron-sdk/src/stargate/dex/msg.rs) file;
- helpers to retrieve data from the dex module are placed in the [query_dex.rs](https://github.com/neutron-org/neutron-sdk/tree/main/packages/neutron-sdk/src/stargate/dex/query.rs) file;
- different types (e.g. request/response types) are placed in the [types_dex.rs](https://github.com/neutron-org/neutron-sdk/tree/main/packages/neutron-sdk/src/stargate/dex/types.rs) file.
