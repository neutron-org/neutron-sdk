# Changelog

## 0.12.0

### Improvements
* Rust 1.82;

## 0.11.0

### Improvements
* Rust 1.78;
* Cosmwasm optimizer 0.16;
* [CosmWasm STD is upgraded to v2.0](https://github.com/neutron-org/neutron-sdk/pull/147);
* ICQ balances query is [improved](https://github.com/neutron-org/neutron-sdk/pull/130) to support a list of denoms to query balances of;

### Added
* [Proto types generation](https://github.com/neutron-org/neutron-sdk/pull/125); for Stargate queries and messages;
* [ICQ for proposal voters](https://github.com/neutron-org/neutron-sdk/pull/129);
* `MsgRegisterInterchainAccountResponse` binding response [is added](https://github.com/neutron-org/neutron-sdk/pull/138), so now contracts are able to catch `channel_id` and `port_id` directly in a reply handler of `MsgRegisterInterchainAccount`;
* [Bindings](https://github.com/neutron-org/neutron-sdk/pull/141) for Slinky Oracle and MarketMap;
* `limit_sell_price` [is added](https://github.com/neutron-org/neutron-sdk/pull/143) to `PlaceLimitOrder` DEX message;

## 0.10.0

Bindings for [Neutron Dex module](https://docs.neutron.org/neutron/modules/dex/overview/) is added.

### Added

* feat: cw dex bindings by @swelf19 in https://github.com/neutron-org/neutron-sdk/pull/120

## 0.9.0

Now Neutron-SDK supports ICQ and ICTX helpers for different version of Cosmos-SDK and specifically 0.9.0 release
introduces ICQ and ICTX helpers for Cosmos SDK 0.47.

So if your contract requires interaction with remote chain that uses Cosmos SDK 0.47 you should use helpers from `v047`
packages.

### Added

* ICQ helpers for Cosmos SDK 0.47 by @pr0n00gler in https://github.com/neutron-org/neutron-sdk/pull/133
* Feat: missing tokenfactory bindings by @pr0n00gler in https://github.com/neutron-org/neutron-sdk/pull/128
* Add grpc option `IncludePoolData` to `QueryUserDeposits` by @sotnikov-s
  in https://github.com/neutron-org/neutron-sdk/pull/127
* Add query for validators signing infos and unbonding delegations query by @albertandrejev
  in https://github.com/neutron-org/neutron-sdk/pull/122

### Fixed

* NTRN-201 fix potential overflow during delegations reconstruct by @quasisamurai
  in https://github.com/neutron-org/neutron-sdk/pull/132

### Changed

* Remove usage of deprecated `data` field in Neutron SDK ICTX helper for SDK 0.47 chains & update SDK 0.45 helper to
  backw compat NTRN-223 by @quasisamurai in https://github.com/neutron-org/neutron-sdk/pull/134
