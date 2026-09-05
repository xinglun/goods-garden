# Phase 10 — Lifecycle

## Goal

Phase 0 から未実装のまま残っていた `LifecycleState` placeholder を実装し、
1つの Goods individual が現在 Goods Garden で監視されている（Active）か、
既に監視対象外になった（Retired：売り切れ・廃棄・取扱中止など）かを示す
最小の2値を表現する。これにより、`docs/architecture/domain-model.md` の
Future provisional concepts に残っていた最後の候補が実装済みになる。

## Implemented boundary

Phase 10 に含めるのは次だけである。

- `LifecycleState`：`Active`/`Retired` の2値 enum。
- `Goods` に `lifecycle: LifecycleState` field を追加する。`Goods::new` は
  引数を変えず、内部で `LifecycleState::Active` を既定値として設定する。
- `Goods::retire(&self) -> Self`：`identity`/`profile` を変えず `lifecycle`
  だけを `Retired` にした新しい値を返す非破壊的な method。これが唯一の
  遷移手段であり、`CareAction`/`HumanFeedback` の自由テキストから domain が
  遷移を推論することは一切ない。
- CLI（`demo`/`seven-day-life`/`multiple-individuals`/`multiple-goods` の
  4 subcommand）は `lifecycle: active`/`lifecycle: retired` を出力する。

`GoodsRuntime` の既存メソッド（`observe_and_assess`、
`observe_and_identify_needs`、`request_care`、
`request_care_and_remember`、`verify_and_learn`）は、Retired な Goods に
対しても現状どおり動作する。「Retired の good に対して何が起きるべきか」
という制約は本 phase では一切課さない——real な業務ルールの発明を避ける
ためである。

`goods-runtime` の `intelligence_loop`/`scheduler` placeholder は対象外の
ままである。これらは North Star の Intelligence Loop を自動的に駆動する
仕組み（自律実行）に踏み込む領域であり、AGENTS.md が禁じる autonomous
business action に触れるリスクが高いため、実装しない。

## Local demo と data boundary

repository root から4つの subcommand を実行する。

```bash
cargo run -p goods-garden-cli -- demo
cargo run -p goods-garden-cli -- seven-day-life
cargo run -p goods-garden-cli -- multiple-individuals
cargo run -p goods-garden-cli -- multiple-goods
```

既存の同梱 fixture はいずれも `Goods::retire()` を呼ばないため、4つとも
`lifecycle: active` を表示する。既存の scripted narrative（何日目にどの
Care Action が起きるか等）は一切変更しない。

## Exit criteria

- `Goods::new` が `LifecycleState::Active` を既定値とすること。
- `Goods::retire()` が `identity`/`profile` を変えず `lifecycle` だけを
  `Retired` にした新しい値を返し、元の値を変更しないこと。
- 4つの CLI subcommand の出力に `lifecycle: active` が表示されること。
- 新規 `phase_10_lifecycle.rs` が上記を直接検証し、パスすること。
- `GoodsRuntime` の既存メソッド、`goods-runtime` の `intelligence_loop`/
  `scheduler` placeholder、persistence、frontend の変更は実装・許可しない。
