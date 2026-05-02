# ============================================================
# Setup
# ============================================================

# Bootstrap the development environment (install lefthook and cargo-llvm-cov)
init:
    @echo "Installing cargo-llvm-cov..."
    cargo install cargo-llvm-cov
    @echo "Installing lefthook..."
    @command -v brew >/dev/null 2>&1 && brew install lefthook || echo "Please install lefthook manually if not on macOS: https://github.com/evilmartians/lefthook"
    lefthook install
