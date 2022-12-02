#[lucia_macros::pkg]
mod a {
}

#[lucia_macros::pkg]
mod b {
  #[pkg::req_data]
  struct ReqData(
    i32
  );
}

#[lucia_macros::pkg]
mod c {
  #[pkg::res_data]
  struct ResData;
}

fn main() {
}
