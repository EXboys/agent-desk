.PHONY: fmt fmt-check clippy-cli clippy-desktop test-cli build-cli frontend cli desktop check all

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

clippy-cli:
	cargo clippy -p agent-desk-core -p agent-desk --all-targets -- -D warnings

clippy-desktop:
	cargo clippy -p agent-desk-desktop --all-targets -- -D warnings

test-cli:
	cargo test -p agent-desk-core -p agent-desk

build-cli:
	cargo build --release -p agent-desk

frontend:
	cd desktop && npm ci && npm run build

cli: fmt-check clippy-cli test-cli build-cli

desktop: clippy-desktop

check: cli frontend

all:
	./scripts/check.sh all
