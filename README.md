# Neutron Cosmwasm Contracts

This monorepository contains the source code of smart contracts and packages for interacting with [Neutron blockchain](https://github.com/neutron-org/neutron)

## Overview

### Packages

The following packages are maintained here:

| Package            | Reference                                                                              | Description                                                                        |
|--------------------|----------------------------------------------------------------------------------------|------------------------------------------------------------------------------------|
| Interchain Queries | https://github.com/neutron-org/neutron-contracts/tree/main/packages/interchain_queries | Queries And Messages for interacting with Neutron Interchain Queries Contract      |
| Stargate           | https://github.com/neutron-org/neutron-contracts/tree/main/packages/stargate           | Structures and Helper methods for interacting with Neutron blockchain via Stargate |

### Contracts

The following contracts are maintained here:

| Contract                            | Reference                                                                                       | Description                                                                                                                                                                   |
|-------------------------------------|-------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Neutron Interchain Queries Contract | https://github.com/neutron-org/neutron-contracts/tree/main/contracts/neutron-interchain-queries | The contract provides a convenient interface for interacting with [Interchain Queries Module](https://github.com/neutron-org/neutron/tree/master/x/interchainqueries). |

## Development

### Environment Setup

- Rust v1.60.0+
- `wasm32-unknown-unknown` target
- Docker

1. Install `rustup` via https://rustup.rs/

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

### Production

For production builds, run the following:

```sh
make build
```

This performs several optimizations which can significantly reduce the final size of the contract binaries, which will be available inside the `artifacts/` directory.

## Documentation

Check you the documentation at <LINK_TO_DOCS_PAGE>.

## Codegen

To process .proto files(`make proto-gen`)

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
