.PHONY: schema test clippy proto-gen build fmt compile check_contracts

schema:
	@find contracts/* -maxdepth 0 -type d \( ! -name . \) -exec bash -c "cd '{}' && cargo schema" \;
	@find packages/neutron-sdk -maxdepth 0 -type d \( ! -name . \) -exec bash -c "cd '{}' && cargo schema" \;

test:
	@cargo test

clippy:
	@cargo clippy --all --all-targets -- -D warnings

fmt:
	@cargo fmt -- --check

build_proto:
	@./build_proto.sh

compile:
	@./build_release.sh

check_contracts:
	@cargo install cosmwasm-check
	@cosmwasm-check --available-capabilities iterator,staking,stargate,neutron,cosmwasm_1_1 artifacts/*.wasm

build: build_proto schema clippy test fmt compile check_contracts
