CURRENT_DIR = $(shell pwd)
CURRENT_DIR_RELATIVE = $(notdir $(shell pwd))

clippy:
	rustup component add clippy || true
	cargo clippy --all-targets --all-features --workspace -- -D warnings

test: clippy
	cargo unit-test
