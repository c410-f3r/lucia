#!/usr/bin/env bash

set -euxo pipefail

rt='rust-tools --template you-rust'

export CARGO_TARGET_DIR="$($rt target-dir)"
export RUST_BACKTRACE=1
export RUSTFLAGS="$($rt rust-flags '' -Dunused_crate_dependencies)"

$rt rustfmt
$rt clippy -Aclippy::pub_use

$rt check-generic .

cargo doc --all-features
cargo test --all-features --doc

FEATURES=(
  # API
  age-of-empires-ii
  colour-lovers
  ethereum
  json-placeholder
  ku-coin
  m-media-covid-19
  nager-date
  solana

  # Deserialization/Serialization
  miniserde
  serde
  serde_json
  serde-xml-rs

# ETC
  async-std
  std
  tokio
  tracing

  # Tranport
  reqwest
  "surf,_surf-hack"
  tokio-tungstenite
)

for feature in "${FEATURES[@]}"
do
	$rt check-with-features . $feature
done
