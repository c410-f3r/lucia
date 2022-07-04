#!/usr/bin/env bash

set -euxo pipefail

RUST_BACKTRACE=1 RUST_LOG=debug cargo test --all-features
