## Definition of Ready

- [ ] `safe-fix-coverage-continuous-expansion` が完了済み、または default safe fix policy が十分に安定していること
- [ ] rule fixture matrix が safe / unsafe candidate / manual_required を区別できること
- [ ] CLI option 名 `--unsafe` が user decision として確定していること
- [ ] automation opt-in option 名が確定していること
- [ ] existing `kml fix` / `kml check --fix` behavior を safe-only として維持する方針が確認されていること

## 1. Safety Metadata

- [ ] 1.1 fix candidate に `safe` / `unsafe` safety metadata を追加する
- [ ] 1.2 existing fix は migration 時に `safe` として扱う
- [ ] 1.3 unsafe candidate と manual-required の違いを fixture matrix schema で表現する
- [ ] 1.4 library API consumer が safety metadata を読めることを example で示す

## 2. CLI Unsafe Mode

- [ ] 2.1 `--unsafe` option を parser に追加する
- [ ] 2.2 default `kml fix` / `kml fmt` / `kml check --fix` は safe fix のみ適用する
- [ ] 2.3 `--unsafe` 指定時は unsafe fix summary を file write 前に表示する
- [ ] 2.4 TTY では `[Y/n]` confirmation を求める
- [ ] 2.5 `n` または EOF では unsafe fix を適用しない
- [ ] 2.6 non-interactive `--unsafe` 単独は fail する
- [ ] 2.7 automation opt-in option がある場合のみ non-interactive unsafe fix を許可する

## 3. Output And Reporting

- [ ] 3.1 text output に unsafe fix candidate count と confirmation result を表示する
- [ ] 3.2 JSON output の fix metadata に `safety` を追加する
- [ ] 3.3 unsafe fix が未適用の理由を JSON で表現する
- [ ] 3.4 rule coverage dashboard を check / safe fix / unsafe fix / manual-required で分ける

## 4. Tests

- [ ] 4.1 default mode が unsafe fix を適用しないことを fixture で固定する
- [ ] 4.2 interactive `--unsafe` で `Y` を入力した場合だけ unsafe fix を適用する
- [ ] 4.3 interactive `--unsafe` で `n` を入力した場合は file write しない
- [ ] 4.4 non-interactive `--unsafe` 単独が fail することを test で固定する
- [ ] 4.5 automation opt-in option が explicit な場合だけ non-interactive unsafe fix を許可する

## Verification

- [ ] `cargo fmt --all -- --check` が成功する
- [ ] `cargo test --workspace --locked` が成功する
- [ ] `make check` が成功する
- [ ] unsafe confirmation の TTY scenario が scripted test で成功する
- [ ] non-interactive unsafe guard が scripted test で成功する
- [ ] `git diff --check` が成功する

## Definition of Done

- [ ] default fix mode が safe-only であること
- [ ] `--unsafe` は explicit opt-in であること
- [ ] CLI interactive usage は `[Y/n]` confirmation なしに unsafe fix を書き込まないこと
- [ ] non-interactive usage は explicit automation opt-in なしに unsafe fix を書き込まないこと
- [ ] JSON / dashboard / fixture matrix が fix safety を可視化していること
- [ ] library は safety metadata を公開し、confirmation policy は CLI / consumer に閉じていること
