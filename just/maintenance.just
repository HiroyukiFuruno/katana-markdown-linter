# ============================================================
# Maintenance
# ============================================================

# Sweep old build artifacts locally (older than 7 days)
sweep:
    @{{RTK_CMD}}cargo sweep --time 7 || true

# Remove build artifacts
clean: sweep
    cargo clean

# Update dependency crates safely (respects Cargo.toml SemVer)
update-safe:
    {{RTK_CMD}}cargo update

# Upgrade ALL dependencies to absolute latest versions (including breaking changes)
update:
    {{RTK_CMD}}cargo upgrade -i
    {{RTK_CMD}}cargo update

# List outdated dependencies (requires cargo-outdated)
outdated:
    @cp Cargo.toml Cargo.toml.bak
    @sed -e '/^\[patch\.crates-io\]/,$d' Cargo.toml.bak > Cargo.toml
    @{{RTK_CMD}}cargo outdated --workspace || (mv Cargo.toml.bak Cargo.toml && exit 1)
    @mv Cargo.toml.bak Cargo.toml
