.DEFAULT_GOAL := help
JOBS ?= 2
VERSION ?= $(shell awk -F '"' '/^version = / { print $$2; exit }' Cargo.toml)
VERSION_BARE := $(patsubst v%,%,$(VERSION))
TAG := v$(VERSION_BARE)
BAD_VERSION ?=
REPLACEMENT_VERSION ?=
RELEASE_REPO ?= HiroyukiFuruno/katana-markdown-linter
RELEASE_TAGGER_NAME ?= HiroyukiFuruno
RELEASE_TAGGER_EMAIL ?= hfuruno0114@gmail.com
KML ?= cargo run --quiet --bin kml --
DOGFOOD_TARGETS ?= README.md docs openspec
DOGFOOD_CONFIG ?= .markdownlint-dogfood.json
DOGFOOD_LOCALE ?= --locale en
DOGFOOD_EXCLUDES ?= --exclude "openspec/changes/archive/**" --exclude "target/**"
DOGFOOD_BASELINE ?= tests/fixtures/dogfood-baseline.json
DOGFOOD_REPORT ?= target/dogfood-report.json
PUBLIC_CONFIDENCE_CONFIG ?= tests/fixtures/public-confidence/.markdownlint.json
PUBLIC_CONFIDENCE_CORPUS ?= tests/fixtures/public-confidence/corpus
PUBLIC_CONFIDENCE_REPORT ?= target/public-confidence-report.json
DOCUMENT_ANSWER_FIX_CONFIG ?= tests/fixtures/document-answer-fix/.markdownlint.json
DOCUMENT_ANSWER_FIX_MANIFEST ?= tests/fixtures/document-answer-fix/manifest.json
DOCUMENT_ANSWER_FIX_REPORT ?= target/document-answer-fix-report.json
DOCUMENT_ANSWER_FIX_KML ?= target/debug/kml
KATANA_CHECKOUT ?=
PERF_BASELINE ?= tests/fixtures/perf-baseline.json
PERF_REPORT ?= target/perf-report.json
PERF_ITERATIONS ?= 20
PERF_SAMPLES ?= 5
PERF_WARMUP ?= 1
CROSS_TOOL_BIN ?= target/release/kml
CROSS_TOOL_REPORT ?= target/cross-tool-benchmark.json
CROSS_TOOL_SUMMARY ?= target/cross-tool-benchmark.md
CROSS_TOOL_RUNS ?= 5
CROSS_TOOL_WARMUP ?= 1
CROSS_TOOL_ARGS ?=
ACTION_SMOKE_DIR ?= target/action-smoke
MCP_INSTALL_SMOKE_DIR ?= target/mcp-install-smoke
MCP_REMOTE_INSTALL_SMOKE_DIR ?= target/mcp-remote-install-smoke
MCPB_DIST_DIR ?= target/mcpb
MCPB_PACKAGE ?= $(MCPB_DIST_DIR)/katana-markdown-linter-$(VERSION_BARE).mcpb
MCP_SERVER_JSON ?= $(MCPB_DIST_DIR)/server.json
BINARY_DIST_DIR ?= target/binary
BINARY_TARGET ?=
HOMEBREW_FORMULA ?= target/homebrew/kml.rb
HOMEBREW_VERSIONED_FORMULA ?= target/homebrew/kml@$(VERSION_BARE).rb
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
	cargo clippy -j $(JOBS) --workspace --all-targets --all-features --locked -- -D warnings

.PHONY: lint-fix
lint-fix: ## Run Clippy and apply automatic fixes
	cargo clippy --workspace --all-targets --all-features --fix --allow-dirty --allow-staged -- -D warnings

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
	cargo run --quiet --example rule_coverage_dashboard --locked -- tests/fixtures/rule-fixture-matrix.json tests/fixtures/upstream-golden-baseline.json tests/fixtures/upstream-golden-known-deltas.json docs/rule-coverage-dashboard.md

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
	python3 scripts/ci/dogfood-baseline.py --baseline $(DOGFOOD_BASELINE) --report $(DOGFOOD_REPORT) -- $(KML) check $(DOGFOOD_TARGETS) --config $(DOGFOOD_CONFIG) $(DOGFOOD_LOCALE) --force-exclude $(DOGFOOD_EXCLUDES) --output json

.PHONY: dogfood-fix
dogfood-fix: ## Apply safe kml fixes to this repository's non-archived Markdown docs
	$(KML) check --fix $(DOGFOOD_TARGETS) --config $(DOGFOOD_CONFIG) $(DOGFOOD_LOCALE) --force-exclude $(DOGFOOD_EXCLUDES) --statistics

.PHONY: dogfood-json
dogfood-json: ## Emit dogfood diagnostics as JSON
	$(KML) check $(DOGFOOD_TARGETS) --config $(DOGFOOD_CONFIG) $(DOGFOOD_LOCALE) --force-exclude $(DOGFOOD_EXCLUDES) --output json

.PHONY: dogfood-refresh-baseline
dogfood-refresh-baseline: ## Refresh dogfood baseline after intentional Markdown cleanup
	python3 scripts/ci/dogfood-baseline.py --update --baseline $(DOGFOOD_BASELINE) --report $(DOGFOOD_REPORT) -- $(KML) check $(DOGFOOD_TARGETS) --config $(DOGFOOD_CONFIG) $(DOGFOOD_LOCALE) --force-exclude $(DOGFOOD_EXCLUDES) --output json

.PHONY: dogfood-archive
dogfood-archive: ## Explicitly check archived OpenSpec Markdown
	$(KML) check openspec/changes/archive --statistics

.PHONY: public-confidence
public-confidence: ## Run public confidence check/fix/fmt convergence evidence on curated Markdown
	python3 scripts/ci/public-confidence.py --report $(PUBLIC_CONFIDENCE_REPORT) --corpus $(PUBLIC_CONFIDENCE_CORPUS) --config $(PUBLIC_CONFIDENCE_CONFIG) -- $(KML)

.PHONY: document-answer-fix
document-answer-fix: ## Compare document-level fix output with reviewed answer fixtures
	cargo build --bin kml --locked
	python3 scripts/ci/document-answer-fix.py --manifest $(DOCUMENT_ANSWER_FIX_MANIFEST) --config $(DOCUMENT_ANSWER_FIX_CONFIG) --report $(DOCUMENT_ANSWER_FIX_REPORT) -- $(DOCUMENT_ANSWER_FIX_KML)

.PHONY: external-katana-dogfood
external-katana-dogfood: ## Run optional KatanA docs/assets Markdown confidence dogfood (KATANA_CHECKOUT=/path)
	@test -n "$(KATANA_CHECKOUT)" || (echo "KATANA_CHECKOUT is required" >&2; exit 2)
	python3 scripts/ci/public-confidence.py --report $(PUBLIC_CONFIDENCE_REPORT) --katana-checkout "$(KATANA_CHECKOUT)" --config $(PUBLIC_CONFIDENCE_CONFIG) -- $(KML)

.PHONY: bench
bench: ## Run repeatable performance benchmarks and write target/perf-report.json
	cargo run --release --example perf_benchmark --locked -- --output $(PERF_REPORT) --iterations $(PERF_ITERATIONS) --samples $(PERF_SAMPLES) --warmup $(PERF_WARMUP)

.PHONY: perf-check
perf-check: bench ## Compare performance report with the committed baseline
	python3 scripts/ci/perf-check.py --baseline $(PERF_BASELINE) --report $(PERF_REPORT)

.PHONY: perf-check-strict
perf-check-strict: bench ## Compare performance report and fail when strict ratio thresholds are exceeded
	python3 scripts/ci/perf-check.py --baseline $(PERF_BASELINE) --report $(PERF_REPORT) --strict --max-ratio 1.4

.PHONY: perf-refresh-baseline
perf-refresh-baseline: bench ## Refresh performance baseline after intentional optimization
	python3 scripts/ci/perf-check.py --update --baseline $(PERF_BASELINE) --report $(PERF_REPORT)

.PHONY: bench-cross-tools
bench-cross-tools: ## Benchmark kml against optional mado and rumdl CLIs
	cargo build --release --bin kml --locked
	python3 scripts/bench/cross-tool-cli-benchmark.py --kml $(CROSS_TOOL_BIN) --output $(CROSS_TOOL_REPORT) --summary $(CROSS_TOOL_SUMMARY) --runs $(CROSS_TOOL_RUNS) --warmup $(CROSS_TOOL_WARMUP) $(CROSS_TOOL_ARGS)

.PHONY: bench-cross-tools-default
bench-cross-tools-default: ## Benchmark default check behavior across available CLIs
	cargo build --release --bin kml --locked
	python3 scripts/bench/cross-tool-cli-benchmark.py --mode default --workflow check --kml $(CROSS_TOOL_BIN) --output $(CROSS_TOOL_REPORT) --summary $(CROSS_TOOL_SUMMARY) --runs $(CROSS_TOOL_RUNS) --warmup $(CROSS_TOOL_WARMUP) $(CROSS_TOOL_ARGS)

.PHONY: bench-cross-tools-common
bench-cross-tools-common: ## Benchmark common-subset check behavior across available CLIs
	cargo build --release --bin kml --locked
	python3 scripts/bench/cross-tool-cli-benchmark.py --mode common --workflow check --kml $(CROSS_TOOL_BIN) --output $(CROSS_TOOL_REPORT) --summary $(CROSS_TOOL_SUMMARY) --runs $(CROSS_TOOL_RUNS) --warmup $(CROSS_TOOL_WARMUP) $(CROSS_TOOL_ARGS)

.PHONY: bench-cross-tools-fix
bench-cross-tools-fix: ## Benchmark fix behavior across available CLIs
	cargo build --release --bin kml --locked
	python3 scripts/bench/cross-tool-cli-benchmark.py --workflow fix --kml $(CROSS_TOOL_BIN) --output $(CROSS_TOOL_REPORT) --summary $(CROSS_TOOL_SUMMARY) --runs $(CROSS_TOOL_RUNS) --warmup $(CROSS_TOOL_WARMUP) $(CROSS_TOOL_ARGS)

.PHONY: examples
examples: ## Compile public Rust embedding examples
	cargo build --examples --locked

.PHONY: action-smoke
action-smoke: ## Smoke test the repository GitHub Action through shared action scripts
	mkdir -p "$(ACTION_SMOKE_DIR)"
	printf '# Action Smoke\n\nText\n' > "$(ACTION_SMOKE_DIR)/README.md"
	printf '{\n  "default": true,\n  "MD013": false\n}\n' > "$(ACTION_SMOKE_DIR)/.markdownlint.json"
	KML_ACTION_INSTALL_SOURCE=path KML_ACTION_PATH=. KML_ACTION_INSTALL_ROOT="$(ACTION_SMOKE_DIR)/install" bash scripts/action/install-kml.sh
	PATH="$(CURDIR)/$(ACTION_SMOKE_DIR)/install/bin:$$PATH" KML_ACTION_COMMAND=check KML_ACTION_PATHS="$(ACTION_SMOKE_DIR)/README.md" KML_ACTION_CONFIG="$(ACTION_SMOKE_DIR)/.markdownlint.json" KML_ACTION_LOCALE=en KML_ACTION_OUTPUT=text bash scripts/action/run-kml.sh

.PHONY: binary-package
binary-package: ## Build standalone kml binary archive and checksum for VERSION
	scripts/release/verify-version.sh "$(VERSION)"
	target_arg=""; \
	if [ -n "$(BINARY_TARGET)" ]; then target_arg="--target $(BINARY_TARGET)"; fi; \
	python3 scripts/release/binary_artifacts.py package --version "$(VERSION)" --dist-dir "$(BINARY_DIST_DIR)" $$target_arg

.PHONY: binary-smoke
binary-smoke: binary-package ## Exercise the standalone binary archive after extraction
	target_arg=""; \
	if [ -n "$(BINARY_TARGET)" ]; then target_arg="--target $(BINARY_TARGET)"; fi; \
	python3 scripts/release/binary_artifacts.py smoke --version "$(VERSION)" --dist-dir "$(BINARY_DIST_DIR)" $$target_arg

.PHONY: homebrew-formula
homebrew-formula: binary-package ## Render Homebrew formula from binary archive checksums
	python3 scripts/release/homebrew_formula.py generate --version "$(VERSION)" --dist-dir "$(BINARY_DIST_DIR)" --formula-name kml --output "$(HOMEBREW_FORMULA)"
	python3 scripts/release/homebrew_formula.py generate --version "$(VERSION)" --dist-dir "$(BINARY_DIST_DIR)" --formula-name "kml@$(VERSION_BARE)" --output "$(HOMEBREW_VERSIONED_FORMULA)"

.PHONY: homebrew-formula-check
homebrew-formula-check: homebrew-formula ## Validate generated Homebrew formula output
	python3 scripts/release/homebrew_formula.py check --version "$(VERSION)" --formula-name kml --output "$(HOMEBREW_FORMULA)"
	python3 scripts/release/homebrew_formula.py check --version "$(VERSION)" --formula-name "kml@$(VERSION_BARE)" --output "$(HOMEBREW_VERSIONED_FORMULA)"

.PHONY: wrapper-smoke
wrapper-smoke: binary-package ## Exercise npm and Python wrapper launchers against local binary archive
	scripts/release/smoke-wrappers.sh "$(VERSION)" "$(BINARY_DIST_DIR)"

.PHONY: npm-package-check
npm-package-check: ## Verify npm wrapper package README, metadata, and tarball contents
	node scripts/release/verify-npm-package.js wrappers/npm

.PHONY: pypi-package-check
pypi-package-check: ## Verify PyPI wrapper README, metadata, and distributions
	python3 scripts/release/verify-pypi-package.py wrappers/python

.PHONY: wrapper-publish-gate
wrapper-publish-gate: ## Verify wrapper publish flags stay deferred outside trusted publishing
	scripts/release/wrapper-publish-gate.sh

.PHONY: mcp-build
mcp-build: ## Build optional experimental MCP server
	cargo build --bin kml-mcp --features mcp --locked

.PHONY: mcp-remote-build
mcp-remote-build: ## Build optional remote MCP Streamable HTTP server
	cargo build --bin kml-mcp-remote --features mcp-remote --locked

.PHONY: mcp-release-build
mcp-release-build: ## Build optimized optional MCP server for packaging
	cargo build --release --bin kml-mcp --features mcp --locked

.PHONY: mcp-test
mcp-test: ## Run optional experimental MCP server tests
	cargo test --features mcp --bin kml-mcp --locked

.PHONY: mcp-install-smoke
mcp-install-smoke: ## Install optional MCP server binary into a local smoke-test root
	cargo install --path . --locked --features mcp --bin kml-mcp --root "$(MCP_INSTALL_SMOKE_DIR)" --force
	test -x "$(MCP_INSTALL_SMOKE_DIR)/bin/kml-mcp"

.PHONY: mcp-stdio-smoke
mcp-stdio-smoke: mcp-install-smoke ## Exercise kml-mcp through MCP stdio JSON-RPC
	python3 scripts/ci/mcp-stdio-smoke.py --bin "$(MCP_INSTALL_SMOKE_DIR)/bin/kml-mcp"

.PHONY: mcp-remote-install-smoke
mcp-remote-install-smoke: ## Install optional remote MCP server binary into a local smoke-test root
	cargo install --path . --locked --features mcp-remote --bin kml-mcp-remote --root "$(MCP_REMOTE_INSTALL_SMOKE_DIR)" --force
	test -x "$(MCP_REMOTE_INSTALL_SMOKE_DIR)/bin/kml-mcp-remote"

.PHONY: mcp-remote-smoke
mcp-remote-smoke: mcp-remote-install-smoke ## Exercise kml-mcp-remote through Streamable HTTP JSON-RPC
	python3 scripts/ci/mcp-remote-smoke.py --bin "$(MCP_REMOTE_INSTALL_SMOKE_DIR)/bin/kml-mcp-remote"

.PHONY: mcpb-package
mcpb-package: mcp-release-build ## Build .mcpb bundle and sha256 checksum for VERSION
	scripts/release/package-mcpb.sh "$(VERSION)"

.PHONY: mcpb-smoke
mcpb-smoke: mcpb-package ## Exercise the bundled kml-mcp binary from the .mcpb artifact
	python3 scripts/ci/mcpb-smoke.py --mcpb "$(MCPB_PACKAGE)" --version "$(VERSION)"

.PHONY: mcp-server-json
mcp-server-json: mcpb-package ## Render release-ready MCP Registry metadata
	python3 scripts/release/render-mcp-server-json.py --version "$(VERSION)" --mcpb-package "$(MCPB_PACKAGE)" --output "$(MCP_SERVER_JSON)"

.PHONY: server-json-validate
server-json-validate: mcp-server-json ## Validate rendered MCP Registry metadata
	python3 scripts/ci/server-json-validate.py --server-json "$(MCP_SERVER_JSON)" --version "$(VERSION)"

.PHONY: internal-quality-check
internal-quality-check: ## Capture internal code quality evidence for src, tests, and build.rs
	python3 scripts/ci/internal-quality.py --report target/internal-quality-report.json --src src --extra-paths tests build.rs

.PHONY: release-test
release-test: ## Run release-equivalent tests with all optional features
	cargo test --all-features --locked

.PHONY: release-target-check
release-target-check: ## Verify VERSION follows the published release line
	scripts/release/verify-version.sh "$(VERSION)"
	python3 scripts/release/verify-release-target.py --target-version "$(VERSION)" --repo "$(RELEASE_REPO)"

.PHONY: release-recovery-plan
release-recovery-plan: ## Plan non-destructive recovery for an accidental release (BAD_VERSION=vX.Y.Z)
	@test -n "$(BAD_VERSION)" || (echo "BAD_VERSION is required" >&2; exit 2)
	replacement_arg=""; \
	if [ -n "$(REPLACEMENT_VERSION)" ]; then replacement_arg="--replacement-version $(REPLACEMENT_VERSION)"; fi; \
	python3 scripts/release/recover-accidental-release.py --bad-version "$(BAD_VERSION)" $$replacement_arg

.PHONY: release-recover
release-recover: ## Run non-destructive accidental release recovery after explicit confirmation
	@test -n "$(BAD_VERSION)" || (echo "BAD_VERSION is required" >&2; exit 2)
	replacement_arg=""; \
	if [ -n "$(REPLACEMENT_VERSION)" ]; then replacement_arg="--replacement-version $(REPLACEMENT_VERSION)"; fi; \
	python3 scripts/release/recover-accidental-release.py --bad-version "$(BAD_VERSION)" $$replacement_arg --execute

.PHONY: release-check
release-check: release-target-check fmt-check lint ast-lint release-test dogfood coverage-blocking examples mcp-build mcp-stdio-smoke mcp-remote-build mcp-remote-smoke mcpb-smoke server-json-validate action-smoke binary-smoke homebrew-formula-check wrapper-smoke npm-package-check pypi-package-check wrapper-publish-gate document-answer-fix ## Run local release preflight gates except upstream drift (VERSION=vX.Y.Z)
	$(MAKE) public-confidence
	cargo publish --dry-run --locked --allow-dirty
	cargo install --path . --locked --force --root "$${TMPDIR:-/tmp}/kml-release-install-check" --bin kml
	"$${TMPDIR:-/tmp}/kml-release-install-check/bin/kml" init-config --config "$${TMPDIR:-/tmp}/kml-release-install-check/.markdownlint.json"

.PHONY: release-task-ledger-check
release-task-ledger-check: ## Verify OpenSpec tasks.md has no open work and the quality score is 100/100
	python3 scripts/release/verify-task-ledger.py --version "$(VERSION)" --allow-open-containing "release-task-ledger-check"

.PHONY: release-package
release-package: ## Build .crate package and sha256 checksum for VERSION
	scripts/release/verify-version.sh "$(VERSION)"
	scripts/release/package-crate.sh "$(VERSION_BARE)"

.PHONY: release-github
release-github: release-tag ## Dispatch GitHub Release workflow without crates.io publish
	scripts/release/verify-version.sh "$(VERSION)"
	gh workflow run release.yml --repo $(RELEASE_REPO) --ref main -f version="$(TAG)" -f publish_crate=false

.PHONY: release-publish
release-publish: release-tag ## Dispatch GitHub Release workflow with crates.io publish
	scripts/release/verify-version.sh "$(VERSION)"
	scripts/release/assert-crate-not-published.sh "$(VERSION_BARE)"
	gh secret list --repo $(RELEASE_REPO) | grep -q '^CARGO_REGISTRY_TOKEN[[:space:]]' || (echo "CARGO_REGISTRY_TOKEN secret is required" >&2; exit 1)
	gh workflow run release.yml --repo $(RELEASE_REPO) --ref main \
		-f version="$(TAG)" \
		-f publish_crate=true \
		-f publish_npm_wrapper=true \
		-f publish_pypi_wrapper=true

.PHONY: release
release: release-publish ## Dispatch the full release workflow (GitHub Release + crates.io, VERSION=vX.Y.Z)

.PHONY: release-tag
release-tag: ## Create and push a signed annotated tag for VERSION
	scripts/release/verify-version.sh "$(VERSION)"
	python3 scripts/release/verify-release-target.py --target-version "$(VERSION)" --repo "$(RELEASE_REPO)"
	scripts/release/assert-tag-safe.sh "$(TAG)"
	@if git rev-parse -q --verify "refs/tags/$(TAG)" >/dev/null; then \
		if [ "$$(git cat-file -t "$(TAG)")" != "tag" ]; then \
			echo "$(TAG) exists but is not an annotated signed tag" >&2; \
			exit 1; \
		fi; \
		git tag -v "$(TAG)"; \
	else \
		GIT_COMMITTER_NAME="$(RELEASE_TAGGER_NAME)" \
		GIT_COMMITTER_EMAIL="$(RELEASE_TAGGER_EMAIL)" \
		git -c user.name="$(RELEASE_TAGGER_NAME)" -c user.email="$(RELEASE_TAGGER_EMAIL)" tag -s "$(TAG)" -m "katana-markdown-linter $(TAG)"; \
	fi
	git push origin "refs/tags/$(TAG)"
	scripts/release/verify-tag-verified.sh "$(TAG)" "$(RELEASE_REPO)"

.PHONY: release-verify
release-verify: ## Verify tag, GitHub Release, and crates.io state after publication
	scripts/release/verify-version.sh "$(VERSION)"
	scripts/release/verify-tag-verified.sh "$(TAG)" "$(RELEASE_REPO)"
	scripts/release/verify-release-published.sh "$(VERSION_BARE)" "$(RELEASE_REPO)"

.PHONY: release-status
release-status: ## Show recent Release workflow runs
	gh run list --repo $(RELEASE_REPO) --workflow Release --limit 5

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
