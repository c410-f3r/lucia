#![allow(incomplete_features)]
#![feature(async_fn_in_trait, impl_trait_projections)]

#[lucia_macros::pkg(api(Foo), data_format(json))]
mod pkg {
  #[pkg::aux]
  #[pkg::aux]
  impl Foo {}

  #[derive(Debug)]
  #[pkg::req_data]
  pub struct Req;

  #[pkg::res_data]
  struct Res;
}

#[lucia_macros::pkg(api(Foo), data_format(json))]
mod pkg {
  #[pkg::before_sending]
  #[pkg::before_sending]
  async fn before_sending() -> lucia::Result<()> {
    Ok(())
  }

  #[derive(Debug)]
  #[pkg::req_data]
  pub struct Req;

  #[pkg::res_data]
  struct Res;
}

#[lucia_macros::pkg(api(Foo), data_format(json))]
mod pkg {
  #[derive(Debug)]
  #[pkg::req_data]
  #[pkg::req_data]
  pub struct Req;

  #[pkg::res_data]
  struct Res;
}

fn main() {
}
