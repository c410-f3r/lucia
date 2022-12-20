#![allow(incomplete_features)]
#![feature(async_fn_in_trait, impl_trait_projections)]

#[lucia_macros::pkg(api(Foo), data_format(json))]
mod pkg {
  #[pkg::before_sending]
  async fn before_sending(foo: i32) -> lucia::Result<()> {
    Ok(())
  }

  #[pkg::req_data]
  struct Req;

  #[pkg::res_data]
  struct Res;
}

fn main() {
}
