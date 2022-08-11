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

LUCIA=(
  # Deserialization/Serialization
  miniserde
  serde
  serde_json
  serde-xml-rs

  # Etc
  async-std
  std
  tokio
  tracing

  # Tranport
  reqwest
  "surf,_surf-hack"
  tokio-tungstenite
)

LUCIA_APIS=(
  # API
  age-of-empires-ii
  colour-lovers
  ethereum
  json-placeholder
  ku-coin
  m-media-covid-19
  nager-date
  solana

  # Etc
  default
  serde
  std
)

for feature in "${LUCIA[@]}"
do
	$rt check-with-features lucia $feature
done

for feature in "${LUCIA_APIS[@]}"
do
	$rt check-with-features lucia-apis $feature
done
