# Lucia macros

Convenient macros that enable the fast creation of arbitrary endpoints based on the `lucia` framework.

```rust,ignore
#[lucia::pkg(api(MyApi), data_format(json_rpc("my_endpoint")), transport(http))]
mod my_endpoint {
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
```
