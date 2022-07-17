# Lucia

[![CI](https://github.com/c410-f3r/lucia/workflows/Tests/badge.svg)](https://github.com/c410-f3r/lucia/actions/workflows/tests.yaml)
[![crates.io](https://img.shields.io/crates/v/lucia.svg)](https://crates.io/crates/lucia)
[![Documentation](https://docs.rs/lucia/badge.svg)](https://docs.rs/lucia)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Rustc](https://img.shields.io/badge/rustc-1.62-lightgray")](https://blog.rust-lang.org/2022/06/30/Rust-1.62.0.html)

A flexible client API framework as well as a set of API collections written in Rust.

```rust,no_run
use lucia::{network::Transport, Pair};

async fn unit() -> lucia::Result<()> {
  let (mut rm, mut trans) = Pair::<(), _, _>::new(
    // Checkout the `network` module to see different transports.
    (),
    // Checkout the `api` module to see different APIs.
    (),
  )
  .into_parts();

  // Checkout the `RequestManager` structure of any API to see different endpoints.
  let req = ();

  // It is also possible to use `send_retrieve_and_decode_many` if a protocol is able
  // to perform batch requests.
  let _res = trans.send_retrieve_and_decode_one(&mut rm, &req, ()).await?;

  Ok(())
}
```

Each API's module has a documentation test for demonstration purposes.

## API

### Blockchain

Name | URL  | Pct |
|---|---|---|
| Ethereum | <https://web3js.readthedocs.io/en/v1.7.4/web3-eth.html> | 3% |
| Solana | <https://docs.solana.com/developing/clients/jsonrpc-api> | 85% |

### Calendar

Name | URL  | Pct |
|---|---|---|
| Nager.Date | <https://date.nager.at> | 100% |

### Exchange

Name | URL  | Pct |
|---|---|---|
| KuCoin | <https://docs.kucoin.com> | 5% |

### Game

Name | URL  | Pct |
|---|---|---|
| Age of Empires II | <https://age-of-empires-2-api.herokuapp.com/docs> | 50% |

### Health

Name | URL  | Pct |
|---|---|---|
| Covid-19 | <https://github.com/M-Media-Group/Covid-19-API>  | 100% |

### Test data

Name | URL  | Pct |
|---|---|---|
| JSONPlaceholder | <http://jsonplaceholder.typicode.com> | 60% |

## Code

- Mostly documented
- No `expect`, `indexing`, `panic` or `unwrap`.
- No `unsafe`
- Supports `no_std`

## Protocol

Name | URL |
|---|---|
| JSON | <https://www.json.org> |
| JSON-RPC 2.0 | <https://www.jsonrpc.org/specification> |

## Transport

Name | URL |
|---|---|
| HTTP | <https://en.wikipedia.org/wiki/Hypertext_Transfer_Protocol> |
| WebSocket | <https://en.wikipedia.org/wiki/WebSocket> |

## Transport implementation

Name | URL |
|---|---|
| reqwest | <https://github.com/seanmonstar/reqwest> |
| surf | <https://github.com/http-rs/surf> |
| tokio-tungstenite | <https://github.com/snapview/tokio-tungstenite> |
