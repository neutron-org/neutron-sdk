# Migrating

This guide explains what is needed to upgrade contracts when migrating over releases of`neutron-sdk`. Note that you can
also view the
[complete CHANGELOG](./CHANGELOG.md) to understand the differences.

## 0.11.0 -> 1.0.0

* Update`neutron-sdk`dependencies in Cargo.toml:

```
[dependencies]
neutron-sdk = "1.0.0-rc0"
# ...
```

* If you use Stargate queries and messages, now you need to move to a usage of gRPC queries and Any message correspondingly, using auto-generated types and helpers, e.g.:

```diff
-let msg = bank::v1beta1::QueryBalanceRequest { address, denom };
-let resp = make_stargate_query(
-    deps,
-    "/cosmos.bank.v1beta1.Query/Balance".to_string(),
-    msg.encode_to_vec(),
-)?;

+use neutron_sdk::proto_types::{cosmos::{auth, bank}};
+let bank_querier = bank::v1beta1::BankQuerier::new(&deps.querier);
+let resp = &bank_querier.balance(address, denom)?;
```

You can find more usages example of different queries [here](https://github.com/neutron-org/neutron-dev-contracts/blob/c819eff7696c2feb0501f02ba48d2b4aa5250419/contracts/grpc_querier/src/contract.rs#L43).

## 0.10.0 -> 0.11.0

* Update`neutron-sdk`dependencies in Cargo.toml:

```
[dependencies]
neutron-sdk = "0.11.0"
# ...
```

* Follow [CosmWasm MIGRATING.md instructions ](https://github.com/CosmWasm/cosmwasm/blob/main/MIGRATING.md#15x---20x) to update to v2.0 version of `cosmwasm-std`

## 0.9.0 -> 0.10.0

* Update`neutron-sdk`dependencies in Cargo.toml:

```
[dependencies]
neutron-sdk = "0.10.0"
# ...
```

## 0.8.0 -> 0.9.0

* Update`neutron-sdk`dependencies in Cargo.toml:

```
[dependencies]
neutron-sdk = "0.9.0"
# ...
```

* If you want to use ICQ helpers compatible with Cosmos SDK 0.47, you must use helpers from v047 package now (you don't
  need to change the code otherwise):

```diff
-use neutron_sdk::interchain_queries::v045::queries::{...}
+use neutron_sdk::interchain_queries::v047::queries::{...}

-use neutron_sdk::interchain_queries::v045::register_queries::{...}
+use neutron_sdk::interchain_queries::v047::register_queries::{...}

-use neutron_sdk::interchain_queries::v045::types::{...};
+use neutron_sdk::interchain_queries::v047::types::{...};
```

* Helper for Interchain transactions module `decode_acknowledgement_response` has been moved
  from `neutron_sdk::interchain_txs::helpers` package to respective packages for different Cosmos SDK version (`v045`
  and `v047` respectively):

```diff
-use neutron_sdk::interchain_txs::helpers::decode_acknowledgement_response;
+use neutron_sdk::interchain_txs::v047::helpers::decode_acknowledgement_response;
```
