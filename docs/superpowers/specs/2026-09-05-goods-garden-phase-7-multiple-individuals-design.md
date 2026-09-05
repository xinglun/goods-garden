# Goods Garden Phase 7 — Multiple Individuals Design Specification

**Status:** PROPOSED — requires human review before implementation

**Date:** 2026-09-05

**Scope:** `apps/goods-garden-cli` only. No `goods-domain`/`goods-application`/`goods-runtime`
changes.

## 背景と課題

`docs/phases/phase-7-multiple-individuals.md` は「`TunaMayo@StoreA` から複数 store の
individual へ拡張する」こと、「Goods Species と Individual Memory の分離を検証する」ことを求めている。
Phase 6 と同様、新しい domain semantics は不要である：`GoodsIdentity.species`/`GoodsProfile` は
既に既存の glossary の Species/Individual 定義が指す、共有される再利用可能な Species レベルの
データを表しており、`GoodsMemory` は既に species でキー付けされたグローバルな store ではなく、
それを構築した者が所有する値である。Phase 7 が求めているのは、この分離が実際に成立していることを
証明するデモであり、新しい機構ではない。

## 目標

1. 不変の Phase 1-6 model を、同一 species の2つの Goods individual に対して実行し、新しい
   `multiple-individuals` CLI subcommand として公開する。
2. 両 individual で1つの clone された `GoodsProfile` を共有して Species を表現しつつ、それぞれに
   異なる `GoodsIdentity.individual_id` と独立した `GoodsMemory` 値を与える。
3. 少なくとも1つの individual の observation が Need/Care episode を発生させ（自身の memory に
   記憶される）、もう一方は発生させないようにし、どちらの individual の memory にも相手の記録が
   含まれないことを確認する明示的な statement を表示する。
4. 既存の `demo` と `seven-day-life` subcommand の挙動と出力は完全に変更しない。

## 非目標

本提案は新しい domain type（特に専用の「Species」struct/registry）、persistence や `MemoryStore`
の実装、Multiple Goods（Phase 8）、frontend の変更のいずれも実装・許可しない。

## 決定：Species は共有・clone された GoodsProfile によって示される

`Species` type や registry を発明する代わりに、本 phase は1つの `GoodsProfile` 値を構築し、それを
各 individual の `Goods` に clone する。これはまさに glossary の Species 定義にある既存の
「再利用可能な goods profile の class」であり、共有を示すのに新しい抽象化は不要である：既に
`Clone` を derive している struct に対する Rust の `Clone` で十分である。

## 決定：Individual Memory の分離は抽象的な主張ではなく構築によって示す

各 individual は、それを構築する loop の中で自身の `GoodsMemory::new()` を得る。individual 間で
memory 値を受け渡すことは一切ない。demo は各 individual 自身の record 件数と、それらが交差しない
ことを述べる締めの一文を表示する。付随する test（`phase_7_multiple_individuals.rs`）は、抽象的な
主張ではなく具体的な件数（Store A で1件、Store B で0件）を assert するため、分離は実際に検証可能な
結果によって確認される。

## 検討した代替案

**individual と profile の registry を所有する `Species` domain type を導入する**：demonstration
phase としては時期尚早な architecture として不採用。「Goods Intelligence が capability class
である」というより本格的な扱いは次の phase の `docs/phases/phase-8-multiple-goods.md` の役割であり、
今 registry を発明することはその phase 自身の scope 決定を先取りしてしまう。

**両 individual に渡す共有の mutable `GoodsMemory` を使い、type レベルの分離ではなく identity ごとの
record 所有を示す**：不採用。これは Phase 7 が求めていること（Individual レベルの漏洩なしの
Species レベルの共有）の逆を示すことになり、Phase 4 の最小限の `GoodsMemory` には存在しない
record ごとの identity filtering を発明する必要が生じる。

## Known、inference、unknown

- **KNOWN：** `GoodsProfile` と `GoodsMemory` は既に、`goods-domain` へのコード変更なしにこの
  demonstration を可能にする振る舞いを持つ。本 phase は CLI level の orchestration だけを追加する。
- **INFERRED：** 2つの individual で Species vs. Individual Memory の分離を示すのに十分であり、
  3つ目を追加しても本質的に異なる性質は加わらない。
- **UNKNOWN/UNAVAILABLE：** real な複数店舗の運営データ；script 内の具体的な age/quantity は
  `example`/`synthetic` のみである。

## Review gate と acceptance

この文書は `PROPOSED` である。承認では次を確認する。

1. 新しい domain type を導入せず、Species の共有は clone された `GoodsProfile` によって、
   Individual Memory の分離は独立した `GoodsMemory` instance によって示されること。
2. 既存の `demo` と `seven-day-life` subcommand が変更されないままであること。
3. 2つの individual（Care episode のある Store A と無い Store B）が、本 milestone の妥当な
   最小 script であること。
