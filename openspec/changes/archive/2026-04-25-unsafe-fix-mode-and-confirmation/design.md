## Design

### Fix Safety Model

fix safety は完全な自動判定ではなく、kml が定義する policy として扱う。

- `safe`: default mode で適用してよい fix candidate
- `unsafe`: content の意味・構造・表示が変わる可能性があり、明示 opt-in が必要な fix candidate
- `manual_required`: 自動修正しない。理由と次の解消条件を fixture matrix に残す

`Fix` または fix candidate metadata は safety を持つ。
既存の safe fix は migration 時に `safe` として扱う。

### CLI Policy

default behavior:

- `kml fix`
- `kml fmt`
- `kml check --fix`

これらは safe fix のみを適用する。

unsafe behavior:

- `--unsafe` を指定した場合だけ unsafe fix candidate を含める
- TTY では unsafe fix summary を表示し、`[Y/n]` confirmation を求める
- `n` または EOF は適用せず exit code を明確にする
- non-interactive では `--unsafe` 単独を fail させる
- automation 用の明示 opt-in は `--yes` など別 option として扱う

### Output Policy

Text output:

- unsafe fix candidate がある場合、rule id、path、件数、危険性の要約を表示する
- confirmation 前に実際の file write は行わない

JSON output:

- existing top-level structure を維持する
- fix metadata に `safety` を追加する
- unsafe fix candidate が適用されなかった理由を structured field で表現する

### Library Boundary

library は safety metadata を返すが、confirmation は CLI の責務とする。
consumer application は safety metadata を見て独自の confirmation UX を構築できる。

### Non-Goals

- `v0.8.0` linter precision / safe fix expansion への混入
- unsafe fix を default で有効化すること
- user content の意味推測を完全自動化すること
- consumer application 固有の confirmation UI
