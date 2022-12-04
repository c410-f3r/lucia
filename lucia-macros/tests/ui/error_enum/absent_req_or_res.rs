#[lucia_macros::pkg]
mod a {
}

#[lucia_macros::pkg]
mod b {
  #[pkg::req_data]
  struct Req(
    i32
  );
}

#[lucia_macros::pkg]
mod c {
  #[pkg::res_data]
  struct Res;
}

fn main() {
}
