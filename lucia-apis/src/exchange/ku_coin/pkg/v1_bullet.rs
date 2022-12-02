#[lucia_macros::pkg(api(KuCoin), data_format(json), error(crate::Error), transport(http))]
pub(crate) mod pkg {
  use crate::{
    exchange::ku_coin::{GenericDataResponse, KuCoin, KuCoinHttpPackagesAux},
    misc::{_MaxUrl, into_rslt},
  };
  use arrayvec::{ArrayString, ArrayVec};
  use lucia::network::{HttpMethod, HttpReqParams};

  #[pkg::aux]
  impl<DRSR> KuCoinHttpPackagesAux<DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    api: &mut KuCoin,
    params: &mut V1BulletParams,
    req_bytes: &[u8],
    req_params: &mut HttpReqParams,
  ) -> crate::Result<()> {
    req_params.method = HttpMethod::Post;
    match params {
      V1BulletParams::Private => {
        req_params.url.push_path(format_args!("/api/v1/bullet-private"))?;
        into_rslt(api.credentials.as_mut())?.push_headers(req_bytes, req_params)?;
      }
      V1BulletParams::Public => {
        req_params.url.push_path(format_args!("/api/v1/bullet-public"))?;
      }
    }
    Ok(())
  }

  #[derive(Debug)]
  #[pkg::params]
  pub enum V1BulletParams {
    /// Requires user credentials.
    Private,
    /// Does not require authentication.
    Public,
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct V1BulletReqData;

  #[pkg::res_data]
  pub type V1BulletResData = GenericDataResponse<Box<V1BulletElemResData>>;

  #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  pub struct V1BulletElemResData {
    pub instance_servers: ArrayVec<V1BulletInstanceServersResData, 4>,
    pub token: ArrayString<256>,
  }

  #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  pub struct V1BulletInstanceServersResData {
    pub encrypt: bool,
    pub endpoint: _MaxUrl,
    pub ping_interval: u64,
    pub ping_timeout: u64,
    pub protocol: ArrayString<12>,
  }
}
