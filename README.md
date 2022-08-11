# Lucia

[![CI](https://github.com/c410-f3r/lucia/workflows/Tests/badge.svg)](https://github.com/c410-f3r/lucia/actions/workflows/tests.yaml)
[![crates.io](https://img.shields.io/crates/v/lucia.svg)](https://crates.io/crates/lucia)
[![Documentation](https://docs.rs/lucia/badge.svg)](https://docs.rs/lucia)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Rustc](https://img.shields.io/badge/rustc-1.63-lightgray")](https://blog.rust-lang.org/2022/08/11/Rust-1.63.0.html)

A flexible client API framework as well as a set of API collections written in Rust.

```rust,no_run
async fn example() -> lucia::Result<()> {
  use lucia::{
    misc::{CommonParams, Pair},
    network::Transport,
    req_res::RequestManager
  };

  let (mut rm, mut trans) = Pair::new(
    RequestManager::new(
      // Checkout the `api`'s module to see different APIs.
      (),
      CommonParams::new(
        // Any transport parameter like HTTP headers
        (),
        // Any custom parameter like request thresholds.
        (),
      ),
      // Checkout the `ds`'s module to see different Deserializers/Serializers.
      ()
    ),
    // Checkout the `network`'s module to see different transports.
    (),
  )
  .into_parts();

  // Checkout the `RequestManager` structure of any API to see different endpoints.
  let req = ();

  // It is also possible to use `send_retrieve_and_decode_many` if a format data is able
  // to perform batch requests.
  let _res = trans.send_retrieve_and_decode_one(&mut rm, &req, ()).await?;

  Ok(())
}
```

Each API's module has a documentation test for demonstration purposes.

## API

| Category | Name | Compile Flag | Serialization<br/>miniserde | Serialization<br/>serde_json | Serialization<br/>serde-xml-rs | Implementation |
|---|---|---|:---:|:---:|:---:|:---:|
| Art & Design | COLOURlovers | colour-lovers | ❌ | ❌ | ✅ |100% |
| Blockchain | Ethereum | ethereum | ❌ | ✅ | ❌ | 3% |
| Blockchain | Solana | solana | ❌ | ✅ | ❌ | 85% |
| Calendar | Nager.Date | nager-date | ❌ | ✅ | ❌ | 100% |
| Exchange | KuCoin | ku-coin | ❌| ✅ | ❌ | 5% |
| Gaming | Age of Empires II | age-of-empires-ii | ❌ | ✅ | ❌ | 100% |
| Health | M Media - Covid-19 | m-media-covid-19 | ❌ | ✅ | ❌ | 100% |
| Test data | JSONPlaceholder | json-placeholder | ❌ | ✅ | ❌ | 100% |

## Features

| Type | Name | URL |
|---|---|---|
| Data Format | JSON | <https://www.json.org> |
| Data Format | JSON-RPC 2.0 | <https://www.jsonrpc.org/specification> |
| Data Format | XML | <https://www.w3.org/TR/xml> |
| Transport (HTTP) | reqwest | <https://github.com/seanmonstar/reqwest> |
| Transport (HTTP) | surf | <https://github.com/http-rs/surf> |
| Transport (WebSocket) | tokio-tungstenite | <https://github.com/snapview/tokio-tungstenite> |

## Code

- Mostly documented
- No `expect`, `indexing`, `panic` or `unwrap`.
- No `unsafe`
- Supports `no_std`
