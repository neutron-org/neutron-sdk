.PHONY: test clippy fmt build_proto

test:
	@cargo test

clippy:
	@cargo clippy --all --all-targets -- -D warnings

fmt:
	@cargo fmt -- --check

build_proto:
	@./build_proto.sh
