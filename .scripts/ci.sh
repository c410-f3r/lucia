#!/usr/bin/env bash

set -euxo pipefail

.scripts/spin-up-local-instances.sh &> /dev/null &
sleep 10
.scripts/integration-tests.sh
pkill -f "solana-test-validator"
