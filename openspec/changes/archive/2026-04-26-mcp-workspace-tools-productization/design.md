## Design

## Adapter Boundary

`kml-mcp` は adapter であり、core crate は MCP に依存しない。

~~~text
MCP client -> kml-mcp -> public kml library API
~~~

MCP-specific request validation、workspace path policy、tool schema は `src/bin/kml-mcp.rs` 側に置く。

## Tool Set

Initial productized tool set:

- `check_text`
- `fix_text`
- `config_validate`
- `rule_list`
- `rule_get`
- `check_file`
- `check_directory`
- `fix_file_preview`
- `fix_file_apply`

`fix_directory_apply` はこの change では実装しない。複数 file mutation は事故範囲が大きいため、preview と per-file apply を先に固定する。

## Workspace Policy

- root は server 起動時の current directory、または explicit `--workspace-root`
- tool input path は root からの relative path
- `..` による root 外参照は禁止
- symbolic path は default で追跡しない
- `.git`, `target`, ignored files は CLI と同等に除外

## Fix Policy

- preview は content diff と diagnostics を返す
- apply は explicit flag を必須にする
- apply 後は再 check して unresolved diagnostics を返す
- binary / non-UTF-8 file は skip/error として扱う

## Documentation

docs は Node/Python server を前提にしない。
`kml-mcp` binary を直接 stdio server として起動する設定例を載せる。

## Non-Goals

- network transport
- multi-root workspace
- directory-wide automatic mutation
- KatanA 固有 adapter
