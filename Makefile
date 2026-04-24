.DEFAULT_GOAL := help
JOBS ?= 2
VERSION ?= $(shell awk -F '"' '/^version = / { print $$2; exit }' Cargo.toml)
VERSION_BARE := $(patsubst v%,%,$(VERSION))
TAG := v$(VERSION_BARE)
KML ?= cargo run --quiet --bin kml --
DOGFOOD_TARGETS ?= README.md docs openspec
DOGFOOD_CONFIG ?= .markdownlint-dogfood.json
DOGFOOD_EXCLUDES ?= --exclude "openspec/changes/archive/**" --exclude "target/**"
DOGFOOD_BASELINE ?= tests/fixtures/dogfood-baseline.json
DOGFOOD_REPORT ?= target/dogfood-report.json
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

.PHONY: upstream-drift
upstream-drift: ## Run upstream markdownlint default-branch drift gate (requires KML_UPSTREAM_MARKDOWNLINT_DOC_DIR)
	cargo test upstream_default_branch_drift_has_no_unknown_items --all-features --locked -- --ignored

.PHONY: upstream-golden
upstream-golden: ## Run deterministic upstream markdownlint golden comparison
	cargo test --test upstream_golden_comparison --locked

.PHONY: upstream-golden-live
upstream-golden-live: ## Run live upstream markdownlint oracle against golden corpus
	sh scripts/upstream/markdownlint-oracle.sh tests/fixtures/upstream-golden-corpus

.PHONY: rule-dashboard
rule-dashboard: ## Regenerate docs/rule-coverage-dashboard.md from fixture metadata
	cargo run --quiet --example rule_coverage_dashboard --locked -- tests/fixtures/rule-fixture-matrix.json tests/fixtures/upstream-golden-known-deltas.json docs/rule-coverage-dashboard.md

.PHONY: test
test: ## Run unit tests
	cargo test --workspace

.PHONY: coverage
coverage: ## Report uncovered lines without failing the build (requires cargo-llvm-cov)
	JOBS=$(JOBS) scripts/ci/coverage.sh

.PHONY: coverage-blocking
coverage-blocking: ## Fail when uncovered lines exceed scripts/ci/coverage-baseline.txt
	COVERAGE_MODE=blocking JOBS=$(JOBS) scripts/ci/coverage.sh

.PHONY: check
check: fmt-check lint ast-lint test dogfood ## Fast impacted verification (local default)
	@echo "✅ All checks passed"

.PHONY: dogfood
dogfood: ## Run kml against repository Markdown and fail only on new diagnostics
	python3 scripts/ci/dogfood-baseline.py --baseline $(DOGFOOD_BASELINE) --report $(DOGFOOD_REPORT) -- $(KML) check $(DOGFOOD_TARGETS) --config $(DOGFOOD_CONFIG) --force-exclude $(DOGFOOD_EXCLUDES) --output json

.PHONY: dogfood-fix
dogfood-fix: ## Apply safe kml fixes to this repository's non-archived Markdown docs
	$(KML) check --fix $(DOGFOOD_TARGETS) --config $(DOGFOOD_CONFIG) --force-exclude $(DOGFOOD_EXCLUDES) --statistics

.PHONY: dogfood-json
dogfood-json: ## Emit dogfood diagnostics as JSON
	$(KML) check $(DOGFOOD_TARGETS) --config $(DOGFOOD_CONFIG) --force-exclude $(DOGFOOD_EXCLUDES) --output json

.PHONY: dogfood-refresh-baseline
dogfood-refresh-baseline: ## Refresh dogfood baseline after intentional Markdown cleanup
	python3 scripts/ci/dogfood-baseline.py --update --baseline $(DOGFOOD_BASELINE) --report $(DOGFOOD_REPORT) -- $(KML) check $(DOGFOOD_TARGETS) --config $(DOGFOOD_CONFIG) --force-exclude $(DOGFOOD_EXCLUDES) --output json

.PHONY: dogfood-archive
dogfood-archive: ## Explicitly check archived OpenSpec Markdown
	$(KML) check openspec/changes/archive --statistics

.PHONY: examples
examples: ## Compile public Rust embedding examples
	cargo build --examples --locked

.PHONY: mcp-build
mcp-build: ## Build optional experimental MCP server
	cargo build --bin kml-mcp --features mcp --locked

.PHONY: mcp-test
mcp-test: ## Run optional experimental MCP server tests
	cargo test --features mcp --bin kml-mcp --locked

.PHONY: release-check
release-check: fmt-check lint ast-lint test dogfood coverage-blocking ## Run local release preflight gates except upstream drift (VERSION=vX.Y.Z)
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
