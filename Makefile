.DEFAULT_GOAL := help
JOBS ?= 2
VERSION ?= $(shell awk -F '"' '/^version = / { print $$2; exit }' Cargo.toml)
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

.PHONY: release-check
release-check: fmt-check lint ast-lint test ## Run local release preflight gates except upstream drift
	scripts/release/verify-version.sh "$(VERSION)"
	cargo publish --dry-run --locked --allow-dirty
	cargo install --path . --locked --force --root "$${TMPDIR:-/tmp}/kml-release-install-check" --bin kml
	"$${TMPDIR:-/tmp}/kml-release-install-check/bin/kml" init-config --config "$${TMPDIR:-/tmp}/kml-release-install-check/.markdownlint.json"

.PHONY: release-package
release-package: ## Build .crate package and sha256 checksum for VERSION
	scripts/release/verify-version.sh "$(VERSION)"
	scripts/release/package-crate.sh "$(VERSION)"

.PHONY: release-github
release-github: ## Dispatch GitHub Release workflow without crates.io publish
	scripts/release/verify-version.sh "$(VERSION)"
	gh workflow run release.yml --repo HiroyukiFuruno/katana-markdown-linter --ref main -f version="$(VERSION)" -f publish_crate=false

.PHONY: release-publish
release-publish: ## Dispatch GitHub Release workflow with crates.io publish
	scripts/release/verify-version.sh "$(VERSION)"
	gh secret list --repo HiroyukiFuruno/katana-markdown-linter | grep -q '^CARGO_REGISTRY_TOKEN[[:space:]]' || (echo "CARGO_REGISTRY_TOKEN secret is required" >&2; exit 1)
	gh workflow run release.yml --repo HiroyukiFuruno/katana-markdown-linter --ref main -f version="$(VERSION)" -f publish_crate=true

.PHONY: release-status
release-status: ## Show recent Release workflow runs
	gh run list --repo HiroyukiFuruno/katana-markdown-linter --workflow Release --limit 5

.PHONY: sweep
sweep:
	cargo sweep --time 7 || true
