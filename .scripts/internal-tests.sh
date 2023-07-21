#!/usr/bin/env bash

set -euxo pipefail

cargo install rust-tools --git https://github.com/c410-f3r/regular-crates

rt='rust-tools --template you-rust'

export CARGO_TARGET_DIR="$($rt target-dir)"
export RUST_BACKTRACE=1
export RUSTFLAGS="$($rt rust-flags -Aunstable_features,-Asingle_use_lifetimes -Dunused_crate_dependencies)"

$rt rustfmt
$rt clippy -Aclippy::shadow_reuse,-Aclippy::wildcard_in_or_patterns,-Aclippy::pattern_type_mismatch,-Aclippy::unused_async

$rt check-generic lucia
$rt check-generic lucia-apis
$rt check-generic lucia-macros

cargo doc --all-features
cargo test --all-features --doc

LUCIA=(
  # Deserialization/Serialization
  borsh
  miniserde
  protobuf
  "rkyv,_hack"
  serde
  serde_json
  serde_yaml
  serde-xml-rs
  "simd-json,_hack"

  # Etc
  arrayvec
  async-std
  async-trait
  futures
  macros
  std
  tokio
  tracing

  # Tranport
  reqwest
  soketto
  "surf,_hack"
  std
  tokio-tungstenite
  tungstenite
)
for feature in "${LUCIA[@]}"
do
	$rt check-with-features lucia $feature
done

LUCIA_APIS=(
  # API
  aptos
  ethereum
  json-placeholder
  nager-date
  rick-and-morty
  solana

  # Etc
  async-trait
  base64
  bs58
  default
  ed25519-dalek
  std

  # Ethereum
  ethabi
  ethereum-types
  hex
  primitive-types

  # Solana
  bincode
  solana-program
)
for feature in "${LUCIA_APIS[@]}"
do
	$rt check-with-features lucia-apis $feature
done

$rt test-with-features lucia-macros ""
