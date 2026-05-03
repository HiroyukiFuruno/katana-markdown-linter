# ============================================================
# Zed Extension
# ============================================================

zed-extension-check:
    cd editors/zed && cargo check --target wasm32-wasip1 --locked
