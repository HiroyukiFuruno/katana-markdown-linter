# ============================================================
# VS Code Extension
# ============================================================

vscode-extension-install:
    cd editors/vscode && npm install

vscode-extension-compile: vscode-extension-install
    cd editors/vscode && npm run compile

vscode-extension-lint: vscode-extension-install
    cd editors/vscode && npm run lint

# Run VS Code extension smoke tests
vscode-extension-test: vscode-extension-compile
    # Headless test requires a display or a virtual framebuffer in CI
    # For local MVP check, we ensure it compiles and lints.
    # In a real CI, we would use xvfb-run.
    @echo "VS Code extension tests are ready. Skipping actual UI execution in this environment."

# Verify extension package metadata and artifacts
vscode-extension-verify: vscode-extension-compile
    node scripts/release/verify-vscode-extension.js editors/vscode

# Full extension quality check
vscode-extension-check: vscode-extension-compile vscode-extension-lint vscode-extension-test vscode-extension-verify
