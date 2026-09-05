# Phase 3 — Care

## Goal

Goods が自分だけでは Need を解決できず、誰が助けられるかを知る。不変の Phase 1/2 State/Need model の上に
CareRequest、Caregiver、Human Feedback、CareAction を追加する。初めて `Goods ↔ Human` の双方向 interaction が生まれる。

## Implemented boundary

Phase 3 に含めるのは次だけである。

- CareRequest: Need Assessment が1つ以上の Need を識別した場合に必ず発生する。識別された Need 群、Need
  Conflict の有無、要求する Caregiver role（単一の固定 role `"store staff"`。role-routing や permission
  model は実装しない）、plain-language な explanation を持つ。CareRequest は決定も行動も行わず、ただ求めるだけである。
- Caregiver: 誰が助けられるかを示す最小の `role` と `display_name`。
- Human Feedback: Caregiver の `decision` と `provided_at`。これは常に `HumanFeedbackSource` port
  経由の外部入力であり、domain と application layer はこれを発明・推論・合成しない。Trust Model の
  「evidence over fluency」「human authority over inferred authority」原則に一致する。
- CareAction: CareRequest とそれを解決した Human Feedback を結びつける traceable な記録。explanation は
  Caregiver が実際に言ったことを言い換えるだけで、その決定が正しいかどうかの独自の判断は加えない。

bounded rule は、識別された全ての Need が正確に1つの CareRequest を発生させるというもので、Need が無い場合は
CareRequest も Human Feedback の要求も発生しない。

## Local demo と data boundary

repository root から demo を実行する。

```bash
cargo run -p goods-garden-cli -- demo
```

path は Phase 1/2 を拡張する。

```text
examples/tuna-mayo/observation.example.txt
examples/tuna-mayo/human_feedback.example.txt
    ↓
DemoObservationSource / DemoHumanFeedbackSource
    ↓
ObservationSource / HumanFeedbackSource ports
    ↓
GoodsRuntime::request_care
    ↓
Goods State + Health Assessment（不変）
Need Assessment（不変）
Care Request + Care Action（新規）
    ↓
CLI output
```

両 fixture は引き続き `synthetic-example` と明記し、同梱 fixture は Need（したがって Care Request・Care
Action も）が発生しない値に調整する。Care の経路は自動テストが crafted な observation で検証する。real な
対話式 prompt や provider-backed Caregiver channel は実装しない。

## Exit criteria

- local user が demo を実行し、不変の Phase 1/2 出力に加え、Care Request の有無と、Human Feedback が
  利用可能な場合の Care Action を確認できる。
- need無し（Care Request無し）・単一Need・Need Conflict の case を test する。
- Memory、Learning、自律 action、real external Caregiver system は実装しない。
