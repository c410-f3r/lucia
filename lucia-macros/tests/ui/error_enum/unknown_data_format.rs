#![allow(incomplete_features)]
#![feature(async_fn_in_trait, impl_trait_projections)]

#[lucia_macros::pkg(api(Foo), data_format(fdsdfs))]
mod pkg {
  #[pkg::req_data]
  struct Req;

  #[pkg::res_data]
  struct Res;
}

fn main() {
}
