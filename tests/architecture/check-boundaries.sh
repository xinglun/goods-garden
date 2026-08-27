#!/usr/bin/env bash
set -euo pipefail

repo_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_dir"

test -f Cargo.toml
test -f crates/goods-domain/src/lib.rs
test -f crates/goods-application/src/lib.rs
test -f crates/goods-infrastructure/src/lib.rs
test -f crates/goods-runtime/src/lib.rs
test -f apps/goods-garden-cli/src/main.rs

grep -q 'goods-domain = { path = "../goods-domain" }' crates/goods-application/Cargo.toml
grep -q 'goods-application = { path = "../goods-application" }' crates/goods-infrastructure/Cargo.toml
grep -q 'goods-application = { path = "../goods-application" }' crates/goods-runtime/Cargo.toml
grep -q 'goods-infrastructure = { path = "../goods-infrastructure" }' crates/goods-runtime/Cargo.toml
grep -q 'goods-runtime = { path = "../../crates/goods-runtime" }' apps/goods-garden-cli/Cargo.toml

if grep -R -nE 'goods-(application|infrastructure|runtime)|goods-garden-cli' crates/goods-domain/Cargo.toml; then
  echo "goods-domain has a forbidden outward dependency" >&2
  exit 1
fi

if grep -R -nE 'goods-(infrastructure|runtime)|goods-garden-cli' crates/goods-application/Cargo.toml; then
  echo "goods-application has a forbidden outward dependency" >&2
  exit 1
fi

if grep -R -nE 'goods-(runtime)|goods-garden-cli' crates/goods-infrastructure/Cargo.toml; then
  echo "goods-infrastructure has a forbidden outward dependency" >&2
  exit 1
fi

if grep -R -n 'goods-garden-cli' crates/goods-runtime/Cargo.toml; then
  echo "goods-runtime has a forbidden outward dependency" >&2
  exit 1
fi

if grep -R -nE 'postgres|sqlite|aws-sdk|openai|anthropic|reqwest|tokio' crates; then
  echo "Phase 0 contains a forbidden infrastructure/provider dependency" >&2
  exit 1
fi

echo "Goods Garden architecture boundaries: PASS"
