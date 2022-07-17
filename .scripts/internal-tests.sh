#!/usr/bin/env bash

set -euxo pipefail

rt='rust-tools --template you-rust'

export CARGO_TARGET_DIR="$($rt target-dir)"
export RUST_BACKTRACE=1
export RUSTFLAGS="$($rt rust-flags '' -Dunused_crate_dependencies)"

$rt rustfmt
$rt clippy -Aclippy::pub_use

# API

$rt check-with-features . age-of-empires-ii
$rt check-with-features . covid-19
$rt check-with-features . ethereum
$rt check-with-features . json-placeholder
$rt check-with-features . ku-coin
$rt check-with-features . solana

# Transport

$rt check-with-features . reqwest
$rt check-with-features . surf,_surf-hack
$rt check-with-features . tokio-tungstenite

# Etc

$rt check-with-features . async-std
$rt check-with-features . default
$rt check-with-features . std
$rt check-with-features . tokio
$rt check-with-features . tracing
