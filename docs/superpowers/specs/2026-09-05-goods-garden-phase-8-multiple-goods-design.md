# Goods Garden Phase 8 — Multiple Goods Design Specification

**Status:** PROPOSED — requires human review before implementation

**Date:** 2026-09-05

**Scope:** `apps/goods-garden-cli` only. No `goods-domain`/`goods-application`/`goods-runtime`
changes.

## 背景と課題

`docs/phases/phase-8-multiple-goods.md` は「salmon rice ball、coffee、sandwich、bento を
追加する目的は数を増やすことではなく、Goods Intelligence が class of capability、concrete good が
object/instance であることを証明すること」を求め、商品を「数合わせのためだけに」追加することを
明示的に戒めている。Phase 1 は `the_same_runtime_supports_a_different_goods_profile` で2つ目の
`coffee` profile を使い、既にこれを狭い範囲で証明していた。Phase 8 はこの証明を、本当に異なる幅を
持つ profile 値を持つ4つの named product に広げるものであり、不変の Phase 1-7 model を使い、
新しい domain semantics を必要としない。

## 目標

1. salmon rice ball、coffee、sandwich、bento という4つの named species を、全く同じ
   `GoodsRuntime::request_care_and_remember` call で実行し、新しい `multiple-goods` CLI
   subcommand として公開する。
2. 4つの商品でほぼ同一の複製ではなく本当に異なる `GoodsProfile` 値（expected lifetime、
   minimum stock）を選び、`goods-domain`/`goods-application`/`goods-runtime` のどこにも商品固有の
   分岐が存在しないことを目視で確認する。
3. 少なくとも1つの商品（最も寿命の短い coffee）に Need/Care episode を発生させ、他は healthy の
   ままにし、統一された code path であることを確認する明示的な statement を表示する。
4. 既存の `demo`、`seven-day-life`、`multiple-individuals` subcommand の挙動と出力は完全に
   変更しない。

## 非目標

本提案は新しい domain type、商品固有の分岐、persistence、自律 action、frontend の変更のいずれも
実装・許可しない。

## 決定：4つの species は幅を持たせるために選び、水増しではない

選んだ値——coffee（4h/最低在庫3）、sandwich（6h/2）、salmon rice ball（8h/2）、bento（10h/1）——は
expected lifetime でおよそ2.5倍、minimum stock quantity でおよそ3倍の幅にまたがる。coffee は
意図的に expectation を1時間超えて observe され、正確に1つの商品だけが完全な Need/Care path を
実演する。残る3つは、それぞれ異なる意味のある profile 値で healthy path を実演する。これは
phase document の「数合わせのためだけに商品を追加しない」という警告に直接応えるものである。

## 決定：汎用性は新しい抽象化ではなく不在によって検証する

汎用性を証明するために「capability registry」のような機構を追加する代わりに、本 phase は既に
codebase に成り立っているシンプルで確認可能な事実に依拠する：`goods-domain`、
`goods-application`、`goods-runtime` を species/product 文字列に対する条件分岐で grep しても
何もヒットしない。demo の締めの statement はこの事実を読者に名指しするだけであり、design はそれを
証明するために新しい type を発明する必要はない。

## 検討した代替案

**「網羅性のために」さらに多くの商品を追加する**：不採用。phase document が明示的にこれを戒めており、
4つの商品で既に意味のある profile 値の幅をカバーしている。

**`ProductCatalog`/`Species` registry type を導入する**：demonstration phase としては時期尚早な
architecture として不採用。Phase 7 と同じ理由による；
`apps/goods-garden-cli/src/main.rs` のローカルな `ProductSpecies` struct は CLI のみの data
table であり、新しい domain concept ではない。

## Known、inference、unknown

- **KNOWN：** `goods-domain`、`goods-application`、`goods-runtime` には species/product 名に
  対する条件分岐が存在しない（本 spec 執筆前に grep で確認済み）。Phase 8 もこれを追加しない。
- **INFERRED：** 本当に幅のある profile 値を持つ4つの商品で、水増しなしに capability vs.
  instance の区別を示すのに十分である。
- **UNKNOWN/UNAVAILABLE：** real な商品ごとの保存期限と在庫データ；script の具体的な数値は
  `example`/`synthetic` のみである。

## Review gate と acceptance

この文書は `PROPOSED` である。承認では次を確認する。

1. 新しい domain type や商品固有の分岐を導入しないこと。
2. 選んだ4つの商品とその profile 値が、水増しの無い本 milestone の妥当な最小 script であること。
3. 既存の `demo`、`seven-day-life`、`multiple-individuals` subcommand が変更されないままであること。
