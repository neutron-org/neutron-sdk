# Neutron CosmWasm SDK

This repository contains the source code of Neutron CosmWasm SDK for interacting with [Neutron blockchain](https://github.com/neutron-org/neutron)

## Overview

### Neutron SDK

The Neutron SDK is contained inside `src` folder and consists of the following subpackages:

| Package                         | Reference                                                                                              | Description                                                                                      |
|---------------------------------|--------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------|
| Neutron Interchain Queries      | https://github.com/neutron-org/neutron-contracts/tree/main/packages/neutron-sdk/src/interchain_queries | Queries, messages and helper methods for interacting with Neutron Interchain Queries Module      |
| Neutron Interchain Transactions | https://github.com/neutron-org/neutron-contracts/tree/main/packages/neutron-sdk/src/interchain_txs     | Queries, messages and helper methods for interacting with Neutron Interchain Transactions Module |
| Neutron Bindings                | https://github.com/neutron-org/neutron-contracts/tree/main/packages/neutron-sdk/src/bindings           | Structures and helper methods for interacting with Neutron blockchain                            |
| Neutron Sudo                    | https://github.com/neutron-org/neutron-contracts/tree/main/packages/neutron-sdk/src/sudo               | Structures for Sudo Contract callbacks from Neutron blockchain                                   |
| Neutron Errors                  | https://github.com/neutron-org/neutron-contracts/tree/main/packages/neutron-sdk/src/errors             | Structures and helpers for Neutron specific error and result types                               |
| Neutron Proto Types             | https://github.com/neutron-org/neutron-contracts/tree/main/packages/neutron-sdk/src/proto_types        | Neutron specific protobuf types.                                                                 |

## Development

### Environment Setup

- Rust v1.63.0+
- `wasm32-unknown-unknown` target

1. Install `rustup` via https://rustup.rs/

2. Run the following:

```sh
rustup default stable
rustup target add wasm32-unknown-unknown
```

### Unit Tests

```sh
make test
```

## Documentation

Check out our documentation at https://docs.neutron.org

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
