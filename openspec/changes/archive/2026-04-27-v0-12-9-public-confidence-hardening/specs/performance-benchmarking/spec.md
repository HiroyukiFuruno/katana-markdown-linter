## ADDED Requirements

### Requirement: public confidence performance SHALL compare synthetic and real-document corpora

公開前 confidence の性能確認は、既存の synthetic benchmark と実文書寄り corpus の差分を分けて説明しなければならない（SHALL）。

#### Scenario: public confidence performance を確認する

- **WHEN** developer が `v0.12.9` の performance check を実行する
- **THEN** system は既存 `make perf-check` の benchmark result を記録する
- **THEN** system は KatanA corpus または curated public confidence fixture の check / fix / fmt timing を記録する
- **THEN** system は regression が synthetic corpus 由来か real-document corpus 由来かを分けて説明する
- **THEN** system は benchmark 数値だけを目的に rule semantics を弱めない

### Requirement: external corpus performance blockers SHALL stop distribution planning

外部 corpus で説明不能な重大性能劣化がある場合、配布計画へ進んではならない（SHALL NOT）。

#### Scenario: performance blocker を確認する

- **WHEN** external corpus timing に説明不能な重大 regression がある
- **THEN** system は finding を `perf-regression` として分類する
- **THEN** system は release blocker として tasks に記録する
- **THEN** system は `v0.13.0` の配布計画へ進まない
