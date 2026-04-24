## Design

## Golden Corpus

Golden corpus は `tests/fixtures/upstream-golden-corpus` を source とし、local `kml` と upstream markdownlint oracle の deterministic output を比較する。

各 active rule は以下の状態のどれかを持つ。

- `baseline`: upstream と local の期待値が固定されている
- `known_delta`: upstream と local の差分が理由付きで許可されている
- `pending`: corpus または expected output が未整備

`pending` はこの change で原則解消する。

## Edge Coverage

Edge case は rule group ごとに最小限の代表を置く。

- empty file / no trailing newline
- front matter
- fenced code / indented code
- inline code spans
- HTML
- list nesting
- table boundaries
- Unicode / multibyte
- Windows CRLF

## Fixture Generation

公式 document の fenced examples はそのままコピーしない。
必要な最小 case に分解し、local fixture として意図がわかる名前を付ける。

## Known Delta Policy

known delta は次を必須にする。

- rule id
- upstream behavior summary
- local behavior summary
- reason
- planned resolution: accept / align later / impossible

理由がない delta は test failure とする。

## Non-Goals

- upstream markdownlint の全 test suite を vendor すること
- official examples を無加工で大量コピーすること
