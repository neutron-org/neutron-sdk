# Migrating

This guide explains what is needed to upgrade contracts when migrating over releases of`neutron-sdk`. Note that you can
also view the
[complete CHANGELOG](./CHANGELOG.md) to understand the differences.

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