## Why

この repository を「cargo で install して使える」公開対象にするには、内部実装だけでなく、publish 前提の package metadata、release 手順、CI 検証が必要である。
phase3 は、その公開準備を spec として固定する。

## What Changes

- crates.io に publish 可能な package metadata を整える
- `cargo install` で使える binary target の公開条件を満たす
- `cargo publish --dry-run` / `cargo package` を CI で検証する
- README / license / repository metadata を公開向けに揃える

## Impact

- public crate としての discoverability が上がる
- publish 前の破損を CI で止められる
- phase4 の CLI 実装が、公開手順と矛盾しない形で進められる
