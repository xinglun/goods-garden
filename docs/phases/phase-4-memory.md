# Phase 4 — Memory

## Goal

Goods が何が起き、誰が助け、どの action を行い、結果が何だったかを記憶する。不変の Phase 1-3 State/Need/Care model
の上に、retention policy を創作せず Relationship Memory を作る。

## Implemented boundary

Phase 4 に含めるのは次だけである。

- MemoryRecord: Need を促した State と、それに応答した Care Action（それ自体が既に Caregiver と Human
  Feedback を持つ）を結びつける。Memory Record は Care Action が効果を発揮したかを決して判断しない。後の
  State をこの記録と比較することは Phase 5 の Verification と Learning の仕事である。
- GoodsMemory: 1つの good に対する Memory Record の append-only な in-process collection。それ自体の
  永続化は持たず、呼び出し元（本 phase では CLI）が所有し、繰り返しの observation をまたいで受け渡す値である。
- retention や eviction policy は定義しない：何も削除・expire されない。これは見落としではなく意図的な選択
  である——Phase 4 自身の goal が明示的に「retention policy を創作しない」と述べており、`MemoryStore` port
  は空プレースホルダーのままで、database や file-backed adapter は実装しない。

bounded rule は、Care Action が記録された時に限りその Care episode を記憶するというもので、Need が無い場合
（したがって Care Request も Care Action も無い場合）は何も記憶されない。

## Local demo と data boundary

repository root から demo を実行する。

```bash
cargo run -p goods-garden-cli -- demo
```

path は Phase 1-3 を拡張する。

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
Memory（新規：この実行内で記憶された episode 数）
    ↓
CLI output
```

同梱 fixture は引き続き Need（したがって Care Action も記憶も）が発生しない値に調整し、Phase 1 の落ち着いた
reference 体験を保つ。Memory の経路——繰り返しの episode が既存記録を eviction せず蓄積することを含む——は
自動テストが crafted な observation で検証するのみである。

## Exit criteria

- local user が demo を実行し、不変の Phase 1-3 出力に加え、この実行内で記憶された Care episode の件数を
  確認できる。
- episode無し・単一episode・繰り返しepisode（append-onlyでeviction無し）の case を test する。
- Learning、Verification、Outcome、永続化、自律 action は実装しない。
