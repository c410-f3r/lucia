//! Any request with lifetimes and custom data methods must have lifetimes that match
//! both request and method signatures.

#![feature(async_fn_in_trait)]

lucia::create_packages_aux_wrapper!();

type Api = ();

#[lucia_macros::pkg(api(super::Api), data_format(json), transport(http))]
mod pkg {
  #[pkg::aux]
  impl super::PkgsAux<(), (), ()> {
    #[pkg::aux_data]
    fn foo_data<'any>(&mut self, param: &'any ()) -> lucia::Result<FooReq<'any>> {
      Ok(param)
    }
  }

  #[pkg::req_data]
  pub(crate) type FooReq<'any> = &'any ();

  #[pkg::res_data]
  pub(crate) type FooRes = ();
}

fn main() {
}
