.PHONY: schema test clippy proto-gen build fmt

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

build: build_proto schema clippy test fmt
	@./build_release.sh
	@./check_contracts.sh
