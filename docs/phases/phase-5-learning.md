# Phase 5 — Verification & Learning

## Goal

Goods が昨日の care に効果があったか判断する。不変の Phase 1-4 State/Need/Care/Memory model の上に
`Care Action → Outcome → Verification → Learning` を、traceable evidence と explicit authority
とともに作る：Learning は reviewable な観測を記録するだけで、自らルールを調整することはない。

## Implemented boundary

Phase 5 に含めるのは次だけである。

- Outcome: CareAction が対象とした NeedKind が follow-up の Need Assessment にまだ存在するかを比較する。
  `OutcomeStatus` は `Resolved`（対象とした Need kind が1つも残っていない）または `Unresolved`（少なくとも
  1つ残っている）。これは事実の比較であり、Caregiver の決定が良かった・十分だったかの判断ではない。
- Learning: Outcome から導かれる、reviewable で plain-language な statement。Learning は `Urgency`
  閾値、`GoodsProfile` field、その他のルールを自ら調整することは決してない。それに基づいて行動するかは
  別の human decision であり、本 phase の対象外である。
- 汎用の `Evidence` placeholder は引き続き provisional・未実装のままとする：Outcome 自身の field
  （CareAction、follow-up の State と Need Assessment）が既に本 phase に必要な traceability を持つ。

bounded rule は、verification は常に他の箇所と同じ `ObservationSource` port から供給される follow-up
observation と比較するというもので、Learning は一度に正確に1つの Outcome に対して生成され、複数 episode
をまたぐ集約は行わない。

## Local demo と data boundary

repository root から demo を実行する。

```bash
cargo run -p goods-garden-cli -- demo
```

path は Phase 1-4 を拡張する。

```text
examples/tuna-mayo/observation.example.txt
examples/tuna-mayo/human_feedback.example.txt
    ↓
DemoObservationSource / DemoHumanFeedbackSource
    ↓
ObservationSource / HumanFeedbackSource ports
    ↓
GoodsRuntime::request_care_and_remember
    ↓
Goods State + Health Assessment（不変）
Need Assessment（不変）
Care Request + Care Action（不変）
Memory（不変）
    ↓
GoodsRuntime::verify_and_learn（新規、Care Action がある場合のみ）
    ↓
Outcome + Learning
    ↓
CLI output
```

同梱 fixture は引き続き Need（したがって Care Action も verify 対象も）が発生しない値に調整し、Phase 1 の
落ち着いた reference 体験を保つ。resolved と unresolved の verification 経路は、自動テストが2つの crafted
observation（初回と follow-up）で検証するのみである。

## Exit criteria

- local user が demo を実行し、不変の Phase 1-4 出力に加え、verify 待ちの Care episode の有無と、あれば
  Outcome status と Learning statement を確認できる。
- resolved と unresolved の両方の verification case を test する。
- Learning は閾値、profile field、その他のルールを一切変更せず、自律 business action は実装しない。
