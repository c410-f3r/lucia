#![allow(incomplete_features)]
#![feature(async_fn_in_trait, impl_trait_projections)]

#[lucia_macros::pkg(api(Foo), data_format(json))]
mod pkg {
  #[pkg::aux]
  impl Foo {}

  #[pkg::req_data]
  struct Req(
    i32
  );

  #[pkg::res_data]
  struct Res;
}

fn main() {
}
