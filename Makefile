.DEFAULT_GOAL := help
JOBS ?= 2
export RUSTFLAGS=-D warnings

.PHONY: help
help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-16s\033[0m %s\n", $$1, $$2}'

.PHONY: init
init: ## Bootstrap the development environment (install lefthook and cargo-llvm-cov)
	@echo "Installing cargo-llvm-cov..."
	cargo install cargo-llvm-cov
	@echo "Installing lefthook..."
	@command -v brew >/dev/null 2>&1 && brew install lefthook || echo "Please install lefthook manually if not on macOS: https://github.com/evilmartians/lefthook"
	lefthook install

.PHONY: fmt
fmt: ## Apply code formatting (rustfmt)
	cargo fmt --all

.PHONY: fmt-check
fmt-check: ## Check format differences (for CI)
	cargo fmt --all -- --check

.PHONY: lint
lint: ## Run Clippy (forces zero warnings)
	cargo clippy -j $(JOBS) --workspace -- -D warnings

.PHONY: lint-fix
lint-fix: ## Run Clippy and apply automatic fixes
	cargo clippy --workspace --fix --allow-dirty --allow-staged -- -D warnings

.PHONY: ast-lint
ast-lint: ## Run AST-based custom linters
	cargo test -j $(JOBS) --test ast_linter -- --nocapture

.PHONY: test
test: ## Run unit tests
	cargo test --workspace

.PHONY: coverage
coverage: ## Run tests and verify 100% test coverage (requires cargo-llvm-cov)
	JOBS=$(JOBS) scripts/ci/coverage.sh

.PHONY: check
check: fmt-check lint ast-lint test coverage ## Fast impacted verification (local default)
	@echo "✅ All checks passed"

.PHONY: sweep
sweep:
	cargo sweep --time 7 || true
