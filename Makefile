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

doc:
	@cargo doc

compile:
	@docker run --rm -v "$(CURDIR)":/code \
		--mount type=volume,source="$(notdir $(CURDIR))_cache",target=/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		--platform linux/amd64 \
		cosmwasm/workspace-optimizer:0.15.0

check_contracts:
	@cargo install cosmwasm-check
	@cosmwasm-check --available-capabilities iterator,staking,stargate,neutron artifacts/*.wasm

build: schema clippy test fmt doc compile check_contracts

build-proto:
	@cargo run --bin proto-build $(revision)