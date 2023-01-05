#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getBlockTime")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::SolanaHttpPkgsAux;

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetBlockTimeReq(
    #[serde(serialize_with = "crate::misc::serialize_as_tuple")]
    #[pkg::field(name = "block")]
    u64,
  );

  #[pkg::res_data]
  pub type GetBlockTimeRes = Option<i64>;
}
