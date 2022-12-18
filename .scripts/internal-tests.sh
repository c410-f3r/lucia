#!/usr/bin/env bash

set -euxo pipefail

rt='rust-tools --template you-rust'

export CARGO_TARGET_DIR="$($rt target-dir)"
export RUST_BACKTRACE=1
export RUSTFLAGS="$($rt rust-flags -Aunstable_features,-Asingle_use_lifetimes -Dunused_crate_dependencies)"

$rt rustfmt
$rt clippy -Aclippy::shadow_reuse,-Aclippy::wildcard_in_or_patterns,-Aclippy::pattern_type_mismatch

$rt check-generic .
cargo doc --all-features
cargo test --all-features --doc

LUCIA=(
  # Deserialization/Serialization
  borsh
  miniserde
  "rkyv,_hack"
  serde
  serde_json
  serde_yaml
  serde-xml-rs
  "simd-json,_hack"

  # Etc
  async-std
  std
  tokio
  tracing

  # Tranport
  reqwest
  "surf,_hack"
  std
  tokio-tungstenite
)

LUCIA_APIS=(
  # API
  json-placeholder
  ku-coin
  nager-date
  rick-and-morty
  solana

  # Etc
  default
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

$rt test-with-features lucia-macros ""
