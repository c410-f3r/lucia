#[lucia_macros::pkg(api(Foo), data_format(json))]
mod pkg {
  #[pkg::after_sending]
  async fn after_sending(foo: i32) -> lucia::Result<()> {
    Ok(())
  }

  #[pkg::req_data]
  struct ReqData;

  #[pkg::res_data]
  struct ResData;
}

fn main() {
}
