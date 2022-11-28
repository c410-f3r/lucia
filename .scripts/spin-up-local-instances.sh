#!/usr/bin/env bash

set -euxo pipefail

trap "trap - SIGTERM && kill -- -$$" SIGINT SIGTERM EXIT

if ! pgrep -x "solana-test-val" > /dev/null
then
  FROM_NORMAL_ACCOUNT=5uRrs5pQeWffpu7LJEBcSTbJv9XaYHGFRMa9CTtR4meu
  TO_NORMAL_ACCOUNT=FiuQrMbFUYka1Goec4wdhoiNq3Ms99cxGrW8JWsWfPnJ
  TO_SOL_TOKEN_ACCOUNT=CDqKzghiixHryqny9r8RPJzYfg3hiiF7e8JecsF6fuJw
  TO_SOL_TOKEN_PROGRAM=So11111111111111111111111111111111111111112

  solana-test-validator \
    --clone $TO_NORMAL_ACCOUNT \
    --clone $TO_SOL_TOKEN_ACCOUNT \
    --clone $TO_SOL_TOKEN_PROGRAM \
    --ledger /tmp/lucia-solana \
    --mint $FROM_NORMAL_ACCOUNT \
    --reset \
    --ticks-per-slot 8 \
    --url https://api.testnet.solana.com
fi
