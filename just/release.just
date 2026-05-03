# ============================================================
# Release / Packaging
# ============================================================

# Smoke test the repository GitHub Action through shared action scripts
action-smoke:
    mkdir -p "{{ACTION_SMOKE_DIR}}"
    printf '# Action Smoke\n\nText\n' > "{{ACTION_SMOKE_DIR}}/README.md"
    printf '{\n  "default": true,\n  "MD013": false\n}\n' > "{{ACTION_SMOKE_DIR}}/.markdownlint.json"
    KML_ACTION_INSTALL_SOURCE=path KML_ACTION_PATH=. KML_ACTION_INSTALL_ROOT="{{ACTION_SMOKE_DIR}}/install" bash scripts/action/install-kml.sh
    PATH="{{REPO_ROOT}}/{{ACTION_SMOKE_DIR}}/install/bin:$PATH" KML_ACTION_COMMAND=check KML_ACTION_PATHS="{{ACTION_SMOKE_DIR}}/README.md" KML_ACTION_CONFIG="{{ACTION_SMOKE_DIR}}/.markdownlint.json" KML_ACTION_LOCALE=en KML_ACTION_OUTPUT=text bash scripts/action/run-kml.sh

# Build standalone kml binary archive and checksum for VERSION
binary-package:
    scripts/release/verify-version.sh "{{VERSION}}"
    target_arg=""; \
    if [ -n "{{BINARY_TARGET}}" ]; then target_arg="--target {{BINARY_TARGET}}"; fi; \
    python3 scripts/release/binary_artifacts.py package --version "{{VERSION}}" --dist-dir "{{BINARY_DIST_DIR}}" $target_arg

# Exercise the standalone binary archive after extraction
binary-smoke: binary-package
    target_arg=""; \
    if [ -n "{{BINARY_TARGET}}" ]; then target_arg="--target {{BINARY_TARGET}}"; fi; \
    python3 scripts/release/binary_artifacts.py smoke --version "{{VERSION}}" --dist-dir "{{BINARY_DIST_DIR}}" $target_arg

# Render Homebrew formula from binary archive checksums
homebrew-formula: binary-package
    python3 scripts/release/homebrew_formula.py generate --version "{{VERSION}}" --dist-dir "{{BINARY_DIST_DIR}}" --formula-name kml --output "{{HOMEBREW_FORMULA}}"
    python3 scripts/release/homebrew_formula.py generate --version "{{VERSION}}" --dist-dir "{{BINARY_DIST_DIR}}" --formula-name "kml@{{VERSION_BARE}}" --output "{{HOMEBREW_VERSIONED_FORMULA}}"

# Validate generated Homebrew formula output
homebrew-formula-check: homebrew-formula
    python3 scripts/release/homebrew_formula.py check --version "{{VERSION}}" --formula-name kml --output "{{HOMEBREW_FORMULA}}"
    python3 scripts/release/homebrew_formula.py check --version "{{VERSION}}" --formula-name "kml@{{VERSION_BARE}}" --output "{{HOMEBREW_VERSIONED_FORMULA}}"

# Exercise npm and Python wrapper launchers against local binary archive
wrapper-smoke: binary-package
    scripts/release/smoke-wrappers.sh "{{VERSION}}" "{{BINARY_DIST_DIR}}"

# Verify npm wrapper package README, metadata, and tarball contents
npm-package-check:
    node scripts/release/verify-npm-package.js wrappers/npm

# Verify npm wrapper version can be published before npm publish
npm-publish-target-check:
    node scripts/release/verify-npm-publish-target.js wrappers/npm

# Verify PyPI wrapper README, metadata, and distributions
pypi-package-check:
    python3 scripts/release/verify-pypi-package.py wrappers/python

# Verify wrapper publish flags stay deferred outside trusted publishing
wrapper-publish-gate:
    scripts/release/wrapper-publish-gate.sh

# Run release-equivalent tests with all optional features
release-test:
    cargo test --all-features --locked

# Verify VERSION follows the published release line
release-target-check:
    scripts/release/verify-version.sh "{{VERSION}}"
    python3 scripts/release/verify-release-target.py --target-version "{{VERSION}}" --repo "{{RELEASE_REPO}}"

# Plan non-destructive recovery for an accidental release (BAD_VERSION=vX.Y.Z)
release-recovery-plan: bad-version-required
    replacement_arg=""; \
    if [ -n "{{REPLACEMENT_VERSION}}" ]; then replacement_arg="--replacement-version {{REPLACEMENT_VERSION}}"; fi; \
    python3 scripts/release/recover-accidental-release.py --bad-version "{{BAD_VERSION}}" $replacement_arg

# Run non-destructive accidental release recovery after explicit confirmation
release-recover: bad-version-required
    replacement_arg=""; \
    if [ -n "{{REPLACEMENT_VERSION}}" ]; then replacement_arg="--replacement-version {{REPLACEMENT_VERSION}}"; fi; \
    python3 scripts/release/recover-accidental-release.py --bad-version "{{BAD_VERSION}}" $replacement_arg --execute

# Run local release preflight gates except upstream drift (VERSION=vX.Y.Z)
release-check: release-target-check fmt-check lint ast-lint schema-check release-test dogfood coverage-blocking examples mcp-build mcp-stdio-smoke mcp-remote-build mcp-remote-smoke mcpb-smoke server-json-validate action-smoke binary-smoke homebrew-formula-check wrapper-smoke npm-package-check npm-publish-target-check pypi-package-check wrapper-publish-gate document-answer-fix public-confidence vscode-extension-check
    cargo publish --dry-run --locked --allow-dirty
    cargo install --path . --locked --force --root "${TMPDIR:-/tmp}/kml-release-install-check" --bin kml
    "${TMPDIR:-/tmp}/kml-release-install-check/bin/kml" init-config --config "${TMPDIR:-/tmp}/kml-release-install-check/.markdownlint.json"

# Verify OpenSpec tasks.md has no open work and the quality score is 100/100
release-task-ledger-check:
    python3 scripts/release/verify-task-ledger.py --version "{{VERSION}}" --allow-open-containing "release-task-ledger-check"

# Build .crate package and sha256 checksum for VERSION
release-package:
    scripts/release/verify-version.sh "{{VERSION}}"
    scripts/release/package-crate.sh "{{VERSION_BARE}}"

# Dispatch GitHub Release workflow without crates.io publish
release-github: release-tag
    scripts/release/verify-version.sh "{{VERSION}}"
    gh workflow run release.yml --repo {{RELEASE_REPO}} --ref main -f version="{{TAG}}" -f publish_crate=false

# Dispatch GitHub Release workflow with crates.io publish
release-publish: release-tag
    scripts/release/verify-version.sh "{{VERSION}}"
    scripts/release/assert-crate-not-published.sh "{{VERSION_BARE}}"
    gh secret list --repo {{RELEASE_REPO}} | grep -q '^CARGO_REGISTRY_TOKEN[[:space:]]' || { echo "CARGO_REGISTRY_TOKEN secret is required" >&2; exit 1; }
    gh workflow run release.yml --repo {{RELEASE_REPO}} --ref main -f version="{{TAG}}" -f publish_crate=true -f publish_npm_wrapper=true -f publish_pypi_wrapper=true

# Dispatch the full release workflow (GitHub Release + crates.io, VERSION=vX.Y.Z)
release: release-publish

# Create and push a signed annotated tag for VERSION
release-tag:
    scripts/release/verify-version.sh "{{VERSION}}"
    python3 scripts/release/verify-release-target.py --target-version "{{VERSION}}" --repo "{{RELEASE_REPO}}"
    scripts/release/assert-tag-safe.sh "{{TAG}}"
    @if git rev-parse -q --verify "refs/tags/{{TAG}}" >/dev/null; then \
        if [ "$(git cat-file -t "{{TAG}}")" != "tag" ]; then \
            echo "{{TAG}} exists but is not an annotated signed tag" >&2; \
            exit 1; \
        fi; \
        git tag -v "{{TAG}}"; \
    else \
        GIT_COMMITTER_NAME="{{RELEASE_TAGGER_NAME}}" \
        GIT_COMMITTER_EMAIL="{{RELEASE_TAGGER_EMAIL}}" \
        git -c user.name="{{RELEASE_TAGGER_NAME}}" -c user.email="{{RELEASE_TAGGER_EMAIL}}" tag -s "{{TAG}}" -m "katana-markdown-linter {{TAG}}"; \
    fi
    git push origin "refs/tags/{{TAG}}"
    scripts/release/verify-tag-verified.sh "{{TAG}}" "{{RELEASE_REPO}}"

# Verify tag, GitHub Release, and crates.io state after publication
release-verify:
    scripts/release/verify-version.sh "{{VERSION}}"
    scripts/release/verify-tag-verified.sh "{{TAG}}" "{{RELEASE_REPO}}"
    scripts/release/verify-release-published.sh "{{VERSION_BARE}}" "{{RELEASE_REPO}}"

# Show recent Release workflow runs
release-status:
    gh run list --repo {{RELEASE_REPO}} --workflow Release --limit 5
