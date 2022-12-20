# Lucia

[![CI](https://github.com/c410-f3r/lucia/workflows/Tests/badge.svg)](https://github.com/c410-f3r/lucia/actions/workflows/tests.yaml)
[![crates.io](https://img.shields.io/crates/v/lucia.svg)](https://crates.io/crates/lucia)
[![Documentation](https://docs.rs/lucia/badge.svg)](https://docs.rs/lucia)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Rustc](https://img.shields.io/badge/rustc-1.65-lightgray")](https://blog.rust-lang.org/2022/11/03/Rust-1.65.0.html)

A flexible client API framework for writing asynchronous, fast, organizable, scalable and maintainable applications with the Rust programming language. Supports several data formats, transports and custom parameters.

Checkout the `lucia-apis` project to see a collection of APIs based on `lucia`.

## Objective

It is possible to directly decode json responses using built-in methods provided by some transport implementations like `reqwest` or `surf` but as complexity grows, the cost of maintaining large sets of endpoints with ad-hoc solutions usually becomes unsustainable. Based on this scenario, `lucia` comes into play to organize and centralize data flow in a well-defined manner to increase productivity and maintainability.

For API consumers, the calling convention of `lucia` endpoints is based on fluent interfaces which makes the usage more pleasant and intuitive.

Moreover, the project may in the future create automatic bindings for other languages in order to avoid having duplicated API repositories.

## Infrastructure

A structure that implements the `Package` trait describes request data as well as any other additional parameters like HTTP header values.

Auxiliary elements attached to the instance of an API like byte buffers or throttling parameters are declared in a separate mandatory entity called `PkgsAux`, which is responsible for assisting the creation and management of packages and their requests.

Take a look at the following example using `lucia_macros` to create a JSON-RPC request with fluent interfaces, `reqwest` to send data and `serde_json` to decode the returned bytes from the server.

```rust,ignore
use lucia::{dnsn::SerdeJson, network::{transport::Transport, HttpParams}};

lucia::create_packages_aux_wrapper!();

#[derive(Debug)]
pub struct MyApi;

type MyApiPackagesAux = PkgsAux<MyApi, SerdeJson, HttpParams>;

#[lucia::pkg(api(super::MyApi), data_format(json_rpc("my_endpoint")), transport(http))]
mod my_endpoint {
  #[pkg::aux]
  impl super::MyApiPackagesAux {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct MyEndpointReq<'any> {
    pub foo: i64,
    pub bar: &'any str,
  }

  #[derive(Debug, serde::Deserialize)]
  #[pkg::res_data]
  pub struct MyEndpointRes {
    pub data: i32,
  }
}

pub async fn fetch_my_endpoint() -> lucia::Result<i32> {
  let pkgs_aux = &mut MyApiPackagesAux::from_minimum(
    MyApi,
    SerdeJson,
    HttpParams::from_url("https://www.some_url.com/api/v1")?,
  );

  let pkg = &mut pkgs_aux.my_endpoint().data(123, "321").build();

  let trans = &mut reqwest::Client::new();

  Ok(trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await?.result?.data)
}
```

## Data formats

Each request has one or more data formats attached specifically to an API.

They are what will be serialized and deserialized so in order to create yet another data format, it is necessary to implement `Deserialize` and `Serialize` according to the desired serializer.

| Name | URL |
|---|---|
| Borsh | <https://borsh.io/> |
| GraphQL | <https://spec.graphql.org/> |
| JSON | <https://www.json.org/json-en.html> |
| JSON-RPC 2.0 | <https://www.jsonrpc.org/> |
| Protobuf | <https://developers.google.com/protocol-buffers> |
| XML | <https://www.w3.org/TR/xml/> |
| YAML | <https://yaml.org/spec/> |

## De-serializers/Serializers

Can be applied to one or more different data formats.

| Feature | URL |
|---|---|
| borsh | <https://docs.rs/borsh> |
| miniserde | <https://docs.rs/miniserde> |
| rkyv | <https://docs.rs/rkyv> |
| rust-protobuf | <https://docs.rs/protobuf> |
| serde_json | <https://docs.rs/serde_json> |
| serde-xml-rs | <https://docs.rs/serde-xml-rs> |
| serde-yaml | <https://docs.rs/serde_yaml> |
| simd-json | <https://docs.rs/simd-json> |

## Transports

How a request should be deployed. Transports have their own trait conveniently called `Transport`.

Custom transport implementations can be declared in `lucia-macros` using the `transport(custom(SomeTransport))` attribute.

| Name | Feature | URL |
|---|---|---|
| Reqwest | reqwest | <https://docs.rs/reqwest> |
| Surf | surf | <https://docs.rs/surf> |
| TcpStream | std | <https://doc.rust-lang.org/std/net/struct.TcpStream.html> |
| tokio-tungstenite | tokio-tungstenite |<https://docs.rs/tokio-tungstenite> |
| UdpStream | std | <https://doc.rust-lang.org/std/net/struct.UdpSocket.html> |
