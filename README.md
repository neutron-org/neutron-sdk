# Neutron Cosmwasm SDK

This repository contains the source code of Neutron Cosmwasm SDK for interacting with [Neutron blockchain](https://github.com/neutron-org/neutron)

## Overview

### Neutron SDK

The Neutron SDK is contained inside `packages` folder and consists of the following subpackages:

| Package                         | Reference                                                                                        | Description                                                                                      |
|---------------------------------|--------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------|
| Neutron Interchain Queries      | <https://github.com/neutron-org/neutron-sdk/tree/main/packages/neutron-sdk/src/interchain_queries> | Queries, messages and helper methods for interacting with Neutron Interchain Queries Module      |
| Neutron Interchain Transactions | <https://github.com/neutron-org/neutron-sdk/tree/main/packages/neutron-sdk/src/interchain_txs>     | Queries, messages and helper methods for interacting with Neutron Interchain Transactions Module |
| Neutron Bindings                | <https://github.com/neutron-org/neutron-sdk/tree/main/packages/neutron-sdk/src/bindings>           | Structures and helper methods for interacting with Neutron blockchain                            |
| Neutron Sudo                    | <https://github.com/neutron-org/neutron-sdk/tree/main/packages/neutron-sdk/src/sudo>               | Structures for Sudo Contract callbacks from Neutron blockchain                                   |
| Neutron Errors                  | <https://github.com/neutron-org/neutron-sdk/tree/main/packages/neutron-sdk/src/errors>             | Structures and helpers for Neutron specific error and result types                               |
| Neutron Stargate                | <https://github.com/neutron-org/neutron-sdk/tree/main/packages/neutron-sdk/src/stargate>           | Structures and helpers for interacting with Neutron via Stargate                                 |

### Example Contracts

We provide sample contracts that either implement or consume these specifications to both provide examples, and provide a basis for code you can extend for more custom contacts, without worrying about reinventing the wheel each time:

| Contract                                         | Reference                                                                                 | Description                                                                                                                                                                                                                                                                                                                            |
|--------------------------------------------------|-------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Neutron Interchain Queries Example Contract      | <https://github.com/neutron-org/neutron-sdk/tree/main/contracts/neutron_interchain_queries> | The contract shows how to properly work with [Interchain Queries Module](https://github.com/neutron-org/neutron/tree/master/x/interchainqueries) using [Interchain Queries SDK package](https://github.com/neutron-org/neutron-contracts/tree/main/packages/neutron-sdk/src/interchain_queries) via CosmWasm smart-contract.           |
| Neutron Interchain Transactions Example Contract | <https://github.com/neutron-org/neutron-sdk/tree/main/contracts/neutron_interchain_txs>     | The contract shows how to properly work with [Neutron Interchain Transactions Module](https://github.com/neutron-org/neutron/tree/master/x/interchaintxs) using [Interchain Transactions SDK package](https://github.com/neutron-org/neutron-contracts/tree/main/packages/neutron-sdk/src/interchain_txs) via CosmWasm smart-contract. |
| Neutron IBC Transfer Example Contract            | <https://github.com/neutron-org/neutron-sdk/tree/main/contracts/ibc_transfer>               | The contract shows how to properly work with [Neutron Sudo Package](https://github.com/neutron-org/neutron-contracts/tree/main/packages/neutron_sudo) to handle a callback from IBC transfer.                                                                                                                                          |

## Development

### Environment Setup

- Rust v1.78.0+
- `wasm32-unknown-unknown` target
- Docker

1. Install `rustup` via <https://rustup.rs/>

2. Run the following:

```sh
rustup default stable
rustup target add wasm32-unknown-unknown
```

3. Make sure [Docker](https://www.docker.com/) is installed

### Unit Tests

Each contract contains Rust unit tests embedded within the contract source directories. You can run:

```sh
make test
```

### Generating schema

```sh
make schema
```

### Generating proto files

Neutron proto files represented as generated Rust code is a part of the Neutron SDK. In case Neutron
proto files have changed there's a command for Rust generated code rebuild. To rebuild the files,
run the following command:

```sh
make build-proto
```

### Production

For production builds, run the following:

```sh
make build
```

This performs several optimizations which can significantly reduce the final size of the contract binaries, which will be available inside the `artifacts/` directory.

## Documentation

Check out our documentation at <https://docs.neutron.org>.

## License

Copyright 2022 Neutron

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
