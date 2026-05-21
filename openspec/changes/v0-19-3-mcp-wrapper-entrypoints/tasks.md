# Tasks

## 0. Definition of Ready

- [ ] 0.1 `v0.19.3` が npm / PyPI / crates.io / GitHub Release に未公開であることを確認する
- [ ] 0.2 公式に載せる `npx` / `bunx` / `uvx` の exact command を実機で検証し、通らない形式を docs に載せない方針を確認する
- [ ] 0.3 `kml` 既存 wrapper の `npx --yes katana-markdown-linter@<version> --version` と `uvx --from katana-markdown-linter kml --version` の互換性を維持する方針を確認する
- [ ] 0.4 追加の registry package 名、secret、workflow input が必要に見えた場合は、自己判断で増やさず設計差分として報告する

## 1. Binary archive packaging

- [ ] 1.1 `scripts/release/binary_artifacts.py` を executable parameter 対応にし、既存 `kml-vX.Y.Z-<target>` archive 名と smoke 結果が変わらないことをテストで固定する
- [ ] 1.2 `kml-mcp-vX.Y.Z-<target>` archive を `mcp` feature 付き `kml-mcp` binary から作成する path を追加する
- [ ] 1.3 `kml-mcp-remote-vX.Y.Z-<target>` archive を `mcp-remote` feature 付き `kml-mcp-remote` binary から作成する path を追加する
- [ ] 1.4 各 MCP archive に executable、`LICENSE`、install note、`.sha256` が含まれることを検証する
- [ ] 1.5 `kml-mcp` archive smoke が MCP stdio の `initialize` / `tools/list` / text tool call を実行するようにする
- [ ] 1.6 `kml-mcp-remote` archive smoke が bearer token 付き Streamable HTTP の `initialize` / `tools/list` を実行し、workspace file tools が出ないことを確認する

## 2. npm wrapper entrypoints

- [ ] 2.1 `wrappers/npm` の installer を binary role 対応にし、`kml` / `kml-mcp` / `kml-mcp-remote` ごとに archive prefix と executable 名を解決する
- [ ] 2.2 npm cache path を version / target / executable ごとに分離し、古い `kml` cache が MCP 実行に使われないことをテストする
- [ ] 2.3 `bin.kml` は既存 CLI launcher として維持する
- [ ] 2.4 `bin.kml-mcp` を stdio MCP launcher として追加し、stdout に wrapper log を出さないことをテストする
- [ ] 2.5 `bin.kml-mcp-remote` を remote MCP launcher として追加する
- [ ] 2.6 `bin.katana-markdown-linter` dispatcher を追加し、`npx --yes katana-markdown-linter@<version> kml-mcp --workspace-root <path>` が MCP server を起動することを検証する
- [ ] 2.7 `bunx katana-markdown-linter@<version> kml-mcp --workspace-root <path>` または実機で通った exact equivalent を検証する
- [ ] 2.8 `scripts/release/verify-npm-package.js` を更新し、README、bin entries、package tarball file list を検証する

## 3. PyPI wrapper entrypoints

- [ ] 3.1 `wrappers/python` の installer を binary role 対応にし、`kml` / `kml-mcp` / `kml-mcp-remote` ごとに archive prefix と executable 名を解決する
- [ ] 3.2 Python cache path を version / target / executable ごとに分離し、古い `kml` cache が MCP 実行に使われないことをテストする
- [ ] 3.3 `kml` console script は既存 CLI launcher として維持する
- [ ] 3.4 `kml-mcp` console script を stdio MCP launcher として追加し、stdout に wrapper log を出さないことをテストする
- [ ] 3.5 `kml-mcp-remote` console script を remote MCP launcher として追加する
- [ ] 3.6 `uvx --from katana-markdown-linter==<version> kml-mcp --workspace-root <path>` が MCP server を起動することを検証する
- [ ] 3.7 `scripts/release/verify-pypi-package.py` を更新し、README、console scripts、sdist / wheel metadata を検証する

## 4. Wrapper smoke tests

- [ ] 4.1 `scripts/release/smoke-wrappers.sh` を拡張し、local archive directory を使って `kml` / `kml-mcp` / `kml-mcp-remote` を検証する
- [ ] 4.2 npm wrapper smoke で `kml --version` と `kml check` の既存検証を維持する
- [ ] 4.3 npm wrapper smoke で `kml-mcp` の MCP stdio JSON-RPC を検証する
- [ ] 4.4 npm wrapper smoke で `kml-mcp-remote` の Streamable HTTP JSON-RPC を検証する
- [ ] 4.5 PyPI wrapper smoke で `kml --version` と `kml check` の既存検証を維持する
- [ ] 4.6 PyPI wrapper smoke で `kml-mcp` の MCP stdio JSON-RPC を検証する
- [ ] 4.7 PyPI wrapper smoke で `kml-mcp-remote` の Streamable HTTP JSON-RPC を検証する
- [ ] 4.8 `npx` / `bunx` / `uvx` の exact command smoke を release gate に含める

## 5. Release workflow and verification

- [ ] 5.1 `just/mcp.just` / `just/release.just` に MCP binary archive build / smoke target を追加し、`release-check` へ組み込む
- [ ] 5.2 `.github/workflows/release.yml` で supported target ごとの `kml-mcp` / `kml-mcp-remote` archive と checksum を upload する
- [ ] 5.3 `scripts/release/verify-release-published.sh` が MCP archive と checksum の存在を検証するようにする
- [ ] 5.4 release verification が current platform の `kml-mcp` archive を MCP stdio で smoke するようにする
- [ ] 5.5 release verification が current platform の `kml-mcp-remote` archive を Streamable HTTP で smoke するようにする
- [ ] 5.6 AST lint または release invariant test で MCP wrapper bins / console scripts / docs / release gate の対応漏れを検出する

## 6. Documentation

- [ ] 6.1 `README.md` の MCP Server セクションに `npx` / `bunx` / `uvx` での `kml-mcp` 起動例を追加する
- [ ] 6.2 `docs/mcp-server.md` に Codex / Claude Code / Antigravity 向けの command / args 例を追加する
- [ ] 6.3 `docs/remote-mcp-transport.md` に wrapper 経由の `kml-mcp-remote` 起動例を追加し、public hosted service ではないことを維持する
- [ ] 6.4 `docs/distribution.md` に MCP binary archive と wrapper entrypoint の公式配布状態を追加する
- [ ] 6.5 `docs/release-runbook.md` に MCP wrapper asset、registry wrapper entrypoint、失敗時の確認手順を追加する
- [ ] 6.6 `wrappers/npm/README.md` と `wrappers/python/README.md` に MCP 起動例と thin wrapper contract を追加する

## 7. Validation

- [ ] 7.1 `just mcp-stdio-smoke`
- [ ] 7.2 `just mcp-remote-smoke`
- [ ] 7.3 `just VERSION=v0.19.3 binary-smoke`
- [ ] 7.4 `just VERSION=v0.19.3 wrapper-smoke`
- [ ] 7.5 `just npm-package-check`
- [ ] 7.6 `just pypi-package-check`
- [ ] 7.7 `just ast-lint`
- [ ] 7.8 `just VERSION=v0.19.3 release-check`
- [ ] 7.9 `scripts/openspec validate v0-19-3-mcp-wrapper-entrypoints --strict`

## Definition of Done

- [ ] D1 `kml` 既存 wrapper の起動互換性が保たれている
- [ ] D2 `kml-mcp` が `npx` / `bunx` / `uvx` 経由で MCP stdio tool call まで成功する
- [ ] D3 `kml-mcp-remote` が wrapper 経由で Streamable HTTP tool call まで成功する
- [ ] D4 GitHub Release に `kml-mcp` / `kml-mcp-remote` target 別 archive と checksum が存在する
- [ ] D5 npm / PyPI package metadata と README が MCP entrypoint を説明している
- [ ] D6 MCP workspace safety boundary と remote text-only boundary が wrapper 経由でも変わらない
