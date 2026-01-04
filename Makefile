# Default to running the help target
.DEFAULT_GOAL := help

.PHONY: help build test fmt lint check clean bench

help: ## Show this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

build: ## Build the entire workspace
	cargo build --workspace

test: ## Run all tests in the workspace
	cargo test --workspace

fmt: ## Format all files (using rustfmt)
	cargo fmt --all

lint: ## Run clippy on all targets (strict mode: fails on warnings)
	cargo clippy --workspace --all-targets --all-features -- -D warnings

check: fmt lint test ## Run formatting, linting, and tests (Great for pre-commit!)

bench: ## Run all benchmarks (e.g., Sieve)
	cargo bench --workspace

clean: ## Clean the target artifacts
	cargo clean
