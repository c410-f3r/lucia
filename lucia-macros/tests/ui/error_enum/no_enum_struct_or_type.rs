#[lucia_macros::pkg(api(Foo), data_format(json))]
mod pkg {
  #[pkg::req_data]
  fn foo() {}

  #[pkg::res_data]
  struct Res;
}

#[lucia_macros::pkg(api(Foo), data_format(json))]
mod pkg {
  #[pkg::req_data]
  struct Req;

  #[pkg::res_data]
  fn foo() {}
}

#[lucia_macros::pkg(api(Foo), data_format(json))]
mod pkg {
  #[pkg::params]
  fn foo() {}

  #[pkg::req_data]
  struct Req;

  #[pkg::res_data]
  struct Res;
}

fn main() {
}
