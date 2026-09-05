# Goods Garden Phase 4 — Memory Design Specification

**Status:** PROPOSED — requires human review before implementation

**Date:** 2026-09-05

**Scope:** `goods-domain`, `goods-application`, `goods-runtime` and `apps/goods-garden-cli`
additions for Phase 4 (GoodsMemory, MemoryRecord).

## 背景と課題

Phase 3 は good に explainable な Care model（CareRequest、Caregiver、HumanFeedback、CareAction）を
与えた。`crates/goods-domain/src/memory/memory.rs` は空プレースホルダーのまま残されていた。
`docs/phases/phase-4-memory.md` は Relationship Memory を挙げ、retention policy を創作しないよう
明示的に警告している。

repository 自身の governance（`AGENTS.md`）は memory retention を human decision を要する protected
semantic area として扱う。この制約を守る最も安全な方法は、本 phase では retention policy を一切定義
しないことである：ここでの Memory は永続化・expiry・eviction のない一時的な in-process append-only log
であり、答えるべき retention の question 自体が残らない——単に対象外とする。

## 目標

1. Phase 1-3 の State/Need/Care model を変更せず、`goods-domain` に `GoodsMemory` と `MemoryRecord`
   を追加する。
2. Memory を永続化のない plain な append-only in-process 値にする：`GoodsMemory` は呼び出し元（本 phase
   では CLI）が所有し、繰り返しの observation をまたいで受け渡す。`GoodsRuntime` の内部や、依然として空の
   `MemoryStore` port の裏には保持しない。
3. `goods-application` に `RememberCare` を実装し、既存の `request_care` を呼び出した上で結果の Care
   Action を記憶する `GoodsRuntime::request_care_and_remember` を公開する。`observe_and_assess`、
   `observe_and_identify_needs`、`request_care` は変更しない。
4. CLI demo の同梱デフォルト実行では Need（したがって記憶される episode）が発生しないままにし、Phase 1
   の落ち着いた reference 体験を保つ。蓄積の挙動は crafted な observation を使う test で検証する。

## 非目標

本提案は persistence、database、file-backed adapter、`MemoryStore` の実装、retention や eviction
policy、Outcome/Verification/Learning（Phase 5）、自律 business action、frontend の変更（
`apps/goods-garden-web/` は無変更）を実装・許可しない。

## 決定：Memory は一時的で in-process、store ではない

`GoodsMemory` は `remember` と `records` method を持つ単純な `Vec<MemoryRecord>` wrapper であり、
trait を実装せず、adapter を持たず、disk に serialize されることもない。CLI が `GoodsMemory::new()` で
新規作成し、`GoodsRuntime::request_care_and_remember` に `&mut` reference で渡す。これにより、
未 review の default（例えば恣意的な record 上限や TTL）で答えるのではなく、retention の question 自体を
完全に回避する。Trust Model の「Unknown over fabrication」に一致する。

## 決定：Memory Record は Care Action を判断しない

`MemoryRecord { state, action }` は、Need を促した State と、それに応答した Care Action を結びつける
だけである。「resolved」/「unresolved」の判断も、後の State との比較も、スコアも加えない。昨日の Care が
実際に効果があったかという比較は明示的に Phase 5（`docs/phases/phase-5-learning.md` の
「Care Action → Outcome → Verification → Learning」）に予約されており、本提案はそれを先取りしない。

## 検討した代替案

**`GoodsMemory` に有限の容量を持たせる**（例：直近 N 件のみ保持）：不採用。特定の数値は根拠のない
retention policy の発明そのものであり、まさに Phase 4 自身の goal が禁じていることである。

**今 `MemoryStore` port を実装する**（in-memory または file-backed store）：時期尚早として不採用。
Phase 4 自身の phase document は Memory を永続化要件なしの「Relationship Memory」に限定しており、
今 store を追加すると、後続 phase が求める前に retention/lifecycle の question（保存された record は
いつ読み込まれ、いつ破棄されるか）を発明することになる。

**`MemoryRecord` に Need が解決したかを示す「result」field を記録する**：不採用。outcome の評価は
Phase 5 の仕事であり、Memory がそれを先取りして判断すると、phase doc が引く「記憶する」ことと
「verify/learn する」ことの境界を曖昧にする。

## Known、inference、unknown

- **KNOWN：** `GoodsMemory` は本 phase 用に予約された空プレースホルダーであり、Phase 1-3 の
  State/Need/Care type は本提案の影響を受けない。
- **KNOWN：** 複数の observation をまたいで `MemoryRecord` を蓄積する唯一の方法は、呼び出し元が同じ
  `GoodsMemory` 値を保持し `&mut` reference で渡すことである。現在の CLI demo は依然として1プロセスに
  つき1回しか実行しないため、1回の実行内では最大1件の記録になるが、繰り返し呼び出す test が eviction
  なしの蓄積を証明する。
- **INFERRED：** 一時的な in-process Memory で、未解決の retention question に答えることなく
  Relationship Memory の concept を示すには十分である。
- **UNKNOWN：** real な retention/expiry policy、real な persistence 要件、`docs/phases/phase-6-seven-day-life.md`
  が挙げる複数日にまたがる scenario で Memory がどう振る舞うべきか。これらは、後続の evidence を伴う
  Work Item が扱うまで `UNKNOWN`/対象外のままとする。
- **UNAVAILABLE：** この repository 内の real external Memory/persistence system。

## Review gate と acceptance

この文書は `PROPOSED` である。承認では次を確認する。

1. Phase 4 の Memory が一時的・in-process のみであり、いかなる persistence、retention、eviction
   policy も持たないこと。
2. `MemoryRecord` が記録する Care Action を判断・採点しないこと。
3. `state::goods_state`、Phase 2 Need model、Phase 3 Care model、frontend が無変更のままであること。
