#![allow(incomplete_features)]
#![feature(async_fn_in_trait, impl_trait_projections)]

#[lucia_macros::pkg(api(Foo), data_format(json))]
mod pkg {
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct Req;

  #[pkg::req_data]
  struct Res;
}

fn main() {
}
