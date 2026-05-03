# ============================================================
# Zed Extension
# ============================================================

zed-extension-compile:
    cd editors/zed && cargo check --target wasm32-wasip1 --locked

zed-extension-verify:
    python3 scripts/release/verify-zed-extension.py editors/zed

zed-extension-check: zed-extension-compile zed-extension-verify
