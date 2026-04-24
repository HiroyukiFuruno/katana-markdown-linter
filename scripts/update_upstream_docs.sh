#!/usr/bin/env bash
set -e
mkdir -p upstream_docs
cd upstream_docs
echo "Fetching upstream rules from DavidAnson/markdownlint..."
# We can just clone locally temporarily to get the doc files
tmp_dir=$(mktemp -d)
git clone --depth 1 https://github.com/DavidAnson/markdownlint.git "$tmp_dir"
rm -f *.md
cp "$tmp_dir"/doc/*.md .
rm -rf "$tmp_dir"
echo "Done! Downloaded $(ls -1 *.md | wc -l) rule files."
