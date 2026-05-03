# ============================================================
# katana-markdown-linter - Development Justfile
# ============================================================
# Stable local, CI/CD, and release task entrypoint.
# Usage:
#   just
#   just <recipe>
#   just VERSION=vX.Y.Z release-check
# ============================================================

set shell := ["bash", "-uc"]

REPO_ROOT := justfile_directory()
RTK := env_var_or_default("RTK", `command -v rtk 2> /dev/null || true`)
RTK_CMD := if RTK == "" { "" } else { RTK + " " }
JOBS := env_var_or_default("JOBS", "2")
VERSION := env_var_or_default("VERSION", `awk -F '"' '/^version = / { print $2; exit }' Cargo.toml`)
VERSION_BARE := replace(VERSION, "v", "")
TAG := "v" + VERSION_BARE
BAD_VERSION := env_var_or_default("BAD_VERSION", "")
REPLACEMENT_VERSION := env_var_or_default("REPLACEMENT_VERSION", "")
RELEASE_REPO := env_var_or_default("RELEASE_REPO", "HiroyukiFuruno/katana-markdown-linter")
RELEASE_TAGGER_NAME := env_var_or_default("RELEASE_TAGGER_NAME", "HiroyukiFuruno")
RELEASE_TAGGER_EMAIL := env_var_or_default("RELEASE_TAGGER_EMAIL", "hfuruno0114@gmail.com")

KML := env_var_or_default("KML", "cargo run --quiet --bin kml --")
DOGFOOD_TARGETS := env_var_or_default("DOGFOOD_TARGETS", "README.md docs openspec")
DOGFOOD_CONFIG := env_var_or_default("DOGFOOD_CONFIG", ".markdownlint-dogfood.json")
DOGFOOD_LOCALE := env_var_or_default("DOGFOOD_LOCALE", "--locale en")
DOGFOOD_EXCLUDES := env_var_or_default("DOGFOOD_EXCLUDES", "--exclude 'openspec/changes/archive/**' --exclude 'target/**'")
DOGFOOD_BASELINE := env_var_or_default("DOGFOOD_BASELINE", "tests/fixtures/dogfood-baseline.json")
DOGFOOD_REPORT := env_var_or_default("DOGFOOD_REPORT", "target/dogfood-report.json")
PUBLIC_CONFIDENCE_CONFIG := env_var_or_default("PUBLIC_CONFIDENCE_CONFIG", "tests/fixtures/public-confidence/.markdownlint.json")
PUBLIC_CONFIDENCE_CORPUS := env_var_or_default("PUBLIC_CONFIDENCE_CORPUS", "tests/fixtures/public-confidence/corpus")
PUBLIC_CONFIDENCE_REPORT := env_var_or_default("PUBLIC_CONFIDENCE_REPORT", "target/public-confidence-report.json")
DOCUMENT_ANSWER_FIX_CONFIG := env_var_or_default("DOCUMENT_ANSWER_FIX_CONFIG", "tests/fixtures/document-answer-fix/.markdownlint.json")
DOCUMENT_ANSWER_FIX_MANIFEST := env_var_or_default("DOCUMENT_ANSWER_FIX_MANIFEST", "tests/fixtures/document-answer-fix/manifest.json")
DOCUMENT_ANSWER_FIX_REPORT := env_var_or_default("DOCUMENT_ANSWER_FIX_REPORT", "target/document-answer-fix-report.json")
DOCUMENT_ANSWER_FIX_KML := env_var_or_default("DOCUMENT_ANSWER_FIX_KML", "target/debug/kml")
KATANA_CHECKOUT := env_var_or_default("KATANA_CHECKOUT", "")

PERF_BASELINE := env_var_or_default("PERF_BASELINE", "tests/fixtures/perf-baseline.json")
PERF_REPORT := env_var_or_default("PERF_REPORT", "target/perf-report.json")
PERF_ITERATIONS := env_var_or_default("PERF_ITERATIONS", "20")
PERF_SAMPLES := env_var_or_default("PERF_SAMPLES", "5")
PERF_WARMUP := env_var_or_default("PERF_WARMUP", "1")
CROSS_TOOL_BIN := env_var_or_default("CROSS_TOOL_BIN", "target/release/kml")
CROSS_TOOL_REPORT := env_var_or_default("CROSS_TOOL_REPORT", "target/cross-tool-benchmark.json")
CROSS_TOOL_SUMMARY := env_var_or_default("CROSS_TOOL_SUMMARY", "target/cross-tool-benchmark.md")
CROSS_TOOL_RUNS := env_var_or_default("CROSS_TOOL_RUNS", "5")
CROSS_TOOL_WARMUP := env_var_or_default("CROSS_TOOL_WARMUP", "1")
CROSS_TOOL_ARGS := env_var_or_default("CROSS_TOOL_ARGS", "")

ACTION_SMOKE_DIR := env_var_or_default("ACTION_SMOKE_DIR", "target/action-smoke")
MCP_INSTALL_SMOKE_DIR := env_var_or_default("MCP_INSTALL_SMOKE_DIR", "target/mcp-install-smoke")
MCP_REMOTE_INSTALL_SMOKE_DIR := env_var_or_default("MCP_REMOTE_INSTALL_SMOKE_DIR", "target/mcp-remote-install-smoke")
MCPB_DIST_DIR := env_var_or_default("MCPB_DIST_DIR", "target/mcpb")
MCPB_PACKAGE := env_var_or_default("MCPB_PACKAGE", MCPB_DIST_DIR + "/katana-markdown-linter-" + VERSION_BARE + ".mcpb")
MCP_SERVER_JSON := env_var_or_default("MCP_SERVER_JSON", MCPB_DIST_DIR + "/server.json")
BINARY_DIST_DIR := env_var_or_default("BINARY_DIST_DIR", "target/binary")
BINARY_TARGET := env_var_or_default("BINARY_TARGET", "")
HOMEBREW_FORMULA := env_var_or_default("HOMEBREW_FORMULA", "target/homebrew/kml.rb")
HOMEBREW_VERSIONED_FORMULA := env_var_or_default("HOMEBREW_VERSIONED_FORMULA", "target/homebrew/kml@" + VERSION_BARE + ".rb")

export RUSTFLAGS := env_var_or_default("RUSTFLAGS", "-D warnings")

[private]
default: help

# Show this help
help:
    @just --list --unsorted

[private]
bad-version-required:
    @test -n "{{BAD_VERSION}}" || { echo "BAD_VERSION is required. Usage: just BAD_VERSION=vX.Y.Z <recipe>"; exit 2; }

import 'just/setup.just'
import 'just/quality.just'
import 'just/dogfood.just'
import 'just/performance.just'
import 'just/mcp.just'
import 'just/vscode.just'
import 'just/zed.just'
import 'just/release.just'
import 'just/maintenance.just'
