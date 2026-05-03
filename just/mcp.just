# ============================================================
# MCP
# ============================================================

# Build optional experimental MCP server
mcp-build:
    cargo build --bin kml-mcp --features mcp --locked

# Build optional remote MCP Streamable HTTP server
mcp-remote-build:
    cargo build --bin kml-mcp-remote --features mcp-remote --locked

# Build optimized optional MCP server for packaging
mcp-release-build:
    cargo build --release --bin kml-mcp --features mcp --locked

# Run optional experimental MCP server tests
mcp-test:
    cargo test --features mcp --bin kml-mcp --locked

# Install optional MCP server binary into a local smoke-test root
mcp-install-smoke:
    PATH="$(pwd)/{{MCP_INSTALL_SMOKE_DIR}}/bin:$PATH" cargo install --path . --locked --features mcp --bin kml-mcp --root "{{MCP_INSTALL_SMOKE_DIR}}" --force
    test -x "{{MCP_INSTALL_SMOKE_DIR}}/bin/kml-mcp"

# Exercise kml-mcp through MCP stdio JSON-RPC
mcp-stdio-smoke: mcp-install-smoke
    python3 scripts/ci/mcp-stdio-smoke.py --bin "{{MCP_INSTALL_SMOKE_DIR}}/bin/kml-mcp"

# Install optional remote MCP server binary into a local smoke-test root
mcp-remote-install-smoke:
    PATH="$(pwd)/{{MCP_REMOTE_INSTALL_SMOKE_DIR}}/bin:$PATH" cargo install --path . --locked --features mcp-remote --bin kml-mcp-remote --root "{{MCP_REMOTE_INSTALL_SMOKE_DIR}}" --force
    test -x "{{MCP_REMOTE_INSTALL_SMOKE_DIR}}/bin/kml-mcp-remote"

# Exercise kml-mcp-remote through Streamable HTTP JSON-RPC
mcp-remote-smoke: mcp-remote-install-smoke
    python3 scripts/ci/mcp-remote-smoke.py --bin "{{MCP_REMOTE_INSTALL_SMOKE_DIR}}/bin/kml-mcp-remote"

# Build .mcpb bundle and sha256 checksum for VERSION
mcpb-package: mcp-release-build
    scripts/release/package-mcpb.sh "{{VERSION}}"

# Exercise the bundled kml-mcp binary from the .mcpb artifact
mcpb-smoke: mcpb-package
    python3 scripts/ci/mcpb-smoke.py --mcpb "{{MCPB_PACKAGE}}" --version "{{VERSION}}"

# Render release-ready MCP Registry metadata
mcp-server-json: mcpb-package
    python3 scripts/release/render-mcp-server-json.py --version "{{VERSION}}" --mcpb-package "{{MCPB_PACKAGE}}" --output "{{MCP_SERVER_JSON}}"

# Validate rendered MCP Registry metadata
server-json-validate: mcp-server-json
    python3 scripts/ci/server-json-validate.py --server-json "{{MCP_SERVER_JSON}}" --version "{{VERSION}}"
