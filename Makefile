SHELL := /usr/bin/env bash

build:
	@echo building atomic-rust-team
	cargo build --all

test: 
	@echo run tests for entire repo
	cargo test --all

check:
	@echo running lint and check on repo
	cargo check
	cargo fmt --all -- --check
	cargo clippy -- -D warnings

get-atomic:
	git clone https://github.com/redcanaryco/atomic-red-team.git atomic-red-team
