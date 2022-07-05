# Lucia

[![CI](https://github.com/c410-f3r/lucia/workflows/Tests/badge.svg)](https://github.com/c410-f3r/lucia/actions/workflows/tests.yaml)
[![crates.io](https://img.shields.io/crates/v/lucia.svg)](https://crates.io/crates/lucia)
[![Documentation](https://docs.rs/lucia/badge.svg)](https://docs.rs/lucia)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Rustc](https://img.shields.io/badge/rustc-1.63-lightgray")](https://blog.rust-lang.org/2022/06/30/Rust-1.62.0.html)

A flexible client API framework as well as a set of API collections written in Rust.

```rust
// lucia = { default-features = false, features = ["reqwest", "solana", "tokio-tungstenite"], version = "0.1" }

use crate::{
  api::{blockchain::solana::Solana, Api},
  network::{reqwest, tokio_tungstenite, Transport},
  Pair,
};

async fn http() -> crate::Result<()> {
  let (mut rb, mut trans) =
    Pair::new(reqwest(), Solana::new("https://api.mainnet-beta.solana.com", None)?).into_parts();

  // Single request
  let req = rb.get_latest_blockhash(None);
  let res = trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await?;
  println!("{:?}", res);

  // Batch request
  let first_req = rb.get_slot(None);
  let second_req = rb.get_slot(None);
  let mut responses = Vec::new();
  trans
    .send_retrieve_and_decode_many(&mut responses, &[first_req, second_req], rb.tp_mut())
    .await?;
  println!("{:?}", responses);

  Ok(())
}

async fn web_socket() -> crate::Result<()> {
  use futures::StreamExt;

  let api = Solana::new("wss://api.mainnet-beta.solana.com", None)?;
  let (mut rb, mut trans) = Pair::new(tokio_tungstenite(&api).await?, api).into_parts();

  let sub = rb.slot_subscribe();
  let sub_id = trans.send_retrieve_and_decode_one(&sub, rb.tp_mut()).await?.result;

  let _data = trans.next().await;
  println!("{:?}", _data);
  let _more_data = trans.next().await;
  println!("{:?}", _more_data);

  let unsub = rb.slot_unsubscribe(sub_id);
  trans.send(&unsub, rb.tp_mut()).await?;

  Ok(())
}
```

## API

### Blockchain

|Name    |URL                                                   |Pct |
|--------|------------------------------------------------------|----|
|Ethereum|https://web3js.readthedocs.io/en/v1.7.4/web3-eth.html |  3%|
|Solana  |https://docs.solana.com/developing/clients/jsonrpc-api| 85%|

### Exchange

|Name  |URL                                                   |Pct |
|------|------------------------------------------------------|----|
|KuCoin|https://docs.kucoin.com                               |  5%|

### Game

|Name             |URL                                                  |Pct |
|-----------------|-----------------------------------------------------|----|
|Age of Empires II|https://age-of-empires-2-api.herokuapp.com/docs      | 50%|

### Health

|Name    |URL                                                |Pct |
|--------|---------------------------------------------------|----|
|Covid-19|https://github.com/M-Media-Group/Covid-19-API      |100%|

### Test data

|Name           |URL                                             |Pct |
|---------------|------------------------------------------------|----|
|JSONPlaceholder|http://jsonplaceholder.typicode.com             |60%|

## Code

- No `expect`, `indexing`, `panic` or `unwrap`.
- No `unsafe`
- Supports `no_std`

## Protocol

|Name        |URL                                  |
|------------|-------------------------------------|
|JSON        |https://www.json.org                 |
|JSON-RPC 2.0|https://www.jsonrpc.org/specification|

## Transport

|Name     |URL                                                      |
|---------|---------------------------------------------------------|
|HTTP     |https://en.wikipedia.org/wiki/Hypertext_Transfer_Protocol|
|WebSocket|https://en.wikipedia.org/wiki/WebSocket                  |

## Transport implementation

|Name             |URL                                          |
|-----------------|---------------------------------------------|
|reqwest          |https://github.com/seanmonstar/reqwest       |
|surf             |https://github.com/http-rs/surf              |
|tokio-tungstenite|https://github.com/snapview/tokio-tungstenite|
