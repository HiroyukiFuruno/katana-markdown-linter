.DEFAULT_GOAL := help
JOBS ?= 2
VERSION ?= $(shell awk -F '"' '/^version = / { print $$2; exit }' Cargo.toml)
VERSION_BARE := $(patsubst v%,%,$(VERSION))
TAG := v$(VERSION_BARE)
export RUSTFLAGS=-D warnings

# AI context-aware CLI proxy (mandatory for agents)
RTK := $(shell command -v rtk 2> /dev/null || echo "")


###################################
# Help
###################################

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
coverage: ## Run tests and report uncovered lines (requires cargo-llvm-cov)
	JOBS=$(JOBS) scripts/ci/coverage.sh

.PHONY: check
check: fmt-check lint ast-lint test ## Fast impacted verification (local default)
	@echo "✅ All checks passed"

.PHONY: release-check
release-check: fmt-check lint ast-lint test ## Run local release preflight gates except upstream drift (VERSION=vX.Y.Z)
	scripts/release/verify-version.sh "$(VERSION)"
	cargo publish --dry-run --locked --allow-dirty
	cargo install --path . --locked --force --root "$${TMPDIR:-/tmp}/kml-release-install-check" --bin kml
	"$${TMPDIR:-/tmp}/kml-release-install-check/bin/kml" init-config --config "$${TMPDIR:-/tmp}/kml-release-install-check/.markdownlint.json"

.PHONY: release-package
release-package: ## Build .crate package and sha256 checksum for VERSION
	scripts/release/verify-version.sh "$(VERSION)"
	scripts/release/package-crate.sh "$(VERSION_BARE)"

.PHONY: release-github
release-github: release-tag ## Dispatch GitHub Release workflow without crates.io publish
	scripts/release/verify-version.sh "$(VERSION)"
	gh workflow run release.yml --repo HiroyukiFuruno/katana-markdown-linter --ref main -f version="$(TAG)" -f publish_crate=false

.PHONY: release-publish
release-publish: release-tag ## Dispatch GitHub Release workflow with crates.io publish
	scripts/release/verify-version.sh "$(VERSION)"
	gh secret list --repo HiroyukiFuruno/katana-markdown-linter | grep -q '^CARGO_REGISTRY_TOKEN[[:space:]]' || (echo "CARGO_REGISTRY_TOKEN secret is required" >&2; exit 1)
	gh workflow run release.yml --repo HiroyukiFuruno/katana-markdown-linter --ref main -f version="$(TAG)" -f publish_crate=true

.PHONY: release
release: release-publish ## Dispatch the full release workflow (GitHub Release + crates.io, VERSION=vX.Y.Z)

.PHONY: release-tag
release-tag: ## Create and push a signed annotated tag for VERSION
	scripts/release/verify-version.sh "$(VERSION)"
	@if git rev-parse -q --verify "refs/tags/$(TAG)" >/dev/null; then \
		if [ "$$(git cat-file -t "$(TAG)")" != "tag" ]; then \
			echo "$(TAG) exists but is not an annotated signed tag" >&2; \
			exit 1; \
		fi; \
		git tag -v "$(TAG)"; \
	else \
		git tag -s "$(TAG)" -m "katana-markdown-linter $(TAG)"; \
	fi
	git push origin "refs/tags/$(TAG)"

.PHONY: release-status
release-status: ## Show recent Release workflow runs
	gh run list --repo HiroyukiFuruno/katana-markdown-linter --workflow Release --limit 5

###################################
# Maintenance
###################################

.PHONY: sweep
sweep: ## Sweep old build artifacts locally (older than 7 days)
	@$(RTK) cargo sweep --time 7 || true

.PHONY: clean
clean: sweep ## Remove build artifacts
	cargo clean

.PHONY: update-safe
update-safe: ## Update dependency crates safely (respects Cargo.toml SemVer)
	$(RTK) cargo update

.PHONY: update
update: ## Upgrade ALL dependencies to absolute latest versions (including breaking changes)
	$(RTK) cargo upgrade -i
	$(RTK) cargo update

.PHONY: outdated
outdated: ## List outdated dependencies (requires cargo-outdated)
	@cp Cargo.toml Cargo.toml.bak
	@sed -e '/^\[patch\.crates-io\]/,$$d' Cargo.toml.bak > Cargo.toml
	@$(RTK) cargo outdated --workspace || (mv Cargo.toml.bak Cargo.toml && exit 1)
	@mv Cargo.toml.bak Cargo.toml
