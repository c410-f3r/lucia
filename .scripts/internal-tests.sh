#!/usr/bin/env bash

set -euxo pipefail

rt='rust-tools --template you-rust'

export CARGO_TARGET_DIR="$($rt target-dir)"
export RUST_BACKTRACE=1
export RUSTFLAGS="$($rt rust-flags -Aunstable_features -Dunused_crate_dependencies)"

$rt rustfmt
$rt clippy -Aclippy::pub_use,-Aclippy::shadow_reuse,-Aclippy::wildcard_in_or_patterns,-Aclippy::missing_trait_methods,-Aclippy::pattern_type_mismatch,-Aclippy::partial_pub_fields,-Aclippy::blanket_clippy_restriction_lints

$rt check-generic .
cargo doc --all-features
cargo test --all-features --doc

LUCIA=(
  # Deserialization/Serialization
  miniserde
  serde
  serde_json
  serde_yaml
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
  json-placeholder
  ku-coin
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

$rt test-with-features lucia-macros ""
