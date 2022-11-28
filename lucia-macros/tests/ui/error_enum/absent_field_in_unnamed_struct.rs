#[lucia_macros::pkg(api(Foo), data_format(json))]
mod pkg {
  #[pkg::aux]
  impl Foo {}

  #[pkg::req_data]
  struct ReqData(
    i32
  );

  #[pkg::res_data]
  struct ResData;
}

fn main() {
}
