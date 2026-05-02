# ============================================================
# Dogfood / Corpus Gates
# ============================================================

# Run kml against repository Markdown and fail only on new diagnostics
dogfood:
    python3 scripts/ci/dogfood-baseline.py --baseline {{DOGFOOD_BASELINE}} --report {{DOGFOOD_REPORT}} -- {{KML}} check {{DOGFOOD_TARGETS}} --config {{DOGFOOD_CONFIG}} {{DOGFOOD_LOCALE}} --force-exclude {{DOGFOOD_EXCLUDES}} --output json

# Apply safe kml fixes to this repository's non-archived Markdown docs
dogfood-fix:
    {{KML}} check --fix {{DOGFOOD_TARGETS}} --config {{DOGFOOD_CONFIG}} {{DOGFOOD_LOCALE}} --force-exclude {{DOGFOOD_EXCLUDES}} --statistics

# Emit dogfood diagnostics as JSON
dogfood-json:
    {{KML}} check {{DOGFOOD_TARGETS}} --config {{DOGFOOD_CONFIG}} {{DOGFOOD_LOCALE}} --force-exclude {{DOGFOOD_EXCLUDES}} --output json

# Refresh dogfood baseline after intentional Markdown cleanup
dogfood-refresh-baseline:
    python3 scripts/ci/dogfood-baseline.py --update --baseline {{DOGFOOD_BASELINE}} --report {{DOGFOOD_REPORT}} -- {{KML}} check {{DOGFOOD_TARGETS}} --config {{DOGFOOD_CONFIG}} {{DOGFOOD_LOCALE}} --force-exclude {{DOGFOOD_EXCLUDES}} --output json

# Explicitly check archived OpenSpec Markdown
dogfood-archive:
    {{KML}} check openspec/changes/archive --statistics

# Run public confidence check/fix/fmt convergence evidence on curated Markdown
public-confidence:
    python3 scripts/ci/public-confidence.py --report {{PUBLIC_CONFIDENCE_REPORT}} --corpus {{PUBLIC_CONFIDENCE_CORPUS}} --config {{PUBLIC_CONFIDENCE_CONFIG}} -- {{KML}}

# Compare document-level fix output with reviewed answer fixtures
document-answer-fix:
    cargo build --bin kml --locked
    python3 scripts/ci/document-answer-fix.py --manifest {{DOCUMENT_ANSWER_FIX_MANIFEST}} --config {{DOCUMENT_ANSWER_FIX_CONFIG}} --report {{DOCUMENT_ANSWER_FIX_REPORT}} -- {{DOCUMENT_ANSWER_FIX_KML}}

# Run optional KatanA docs/assets Markdown confidence dogfood (KATANA_CHECKOUT=/path)
external-katana-dogfood:
    @test -n "{{KATANA_CHECKOUT}}" || { echo "KATANA_CHECKOUT is required" >&2; exit 2; }
    python3 scripts/ci/public-confidence.py --report {{PUBLIC_CONFIDENCE_REPORT}} --katana-checkout "{{KATANA_CHECKOUT}}" --config {{PUBLIC_CONFIDENCE_CONFIG}} -- {{KML}}
