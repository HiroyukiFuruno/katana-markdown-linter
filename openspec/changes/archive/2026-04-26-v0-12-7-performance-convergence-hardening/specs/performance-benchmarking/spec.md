## ADDED Requirements

### Requirement: post-migration performance SHALL be explained before stable scoring

parser / context migration 後の performance は、安定版スコアリング前に説明されなければならない（SHALL）。

#### Scenario: post-migration performance を確認する

- **WHEN** developer が `v0.12.7` の performance check を実行する
- **THEN** system は parser index construction、API lint、API fix、CLI check、CLI fix、CLI fmt の代表 case を比較する
- **THEN** system は unexplained regression を release evidence として記録する
- **THEN** system は benchmark 数値だけを目的に rule semantics を弱めない

### Requirement: baseline refresh SHALL follow correctness gates

baseline refresh は正しさの gate 後にだけ行わなければならない（SHALL）。

#### Scenario: baseline を更新する

- **WHEN** developer が performance baseline を更新する
- **THEN** system は precision fixture、fix/fmt convergence、dogfood、release-check が成功していることを確認する
- **THEN** system は refresh 対象 case、before / after、理由を tasks に記録する
