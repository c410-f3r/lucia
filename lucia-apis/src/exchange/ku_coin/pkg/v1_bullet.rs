#[lucia_macros::pkg(api(KuCoin), data_format(json), error(crate::Error), transport(http))]
pub(crate) mod pkg {
  use crate::{
    exchange::ku_coin::{HttpResWrapper, KuCoin, KuCoinHttpPkgsAux},
    misc::{_MaxUrl, into_rslt},
  };
  use arrayvec::{ArrayString, ArrayVec};
  use lucia::network::{HttpMethod, HttpReqParams};

  #[pkg::aux]
  impl<DRSR> KuCoinHttpPkgsAux<DRSR> {}

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

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct V1BulletReq;

  #[pkg::res_data]
  pub type V1BulletRes = HttpResWrapper<Box<V1Bullet>>;

  #[derive(Debug, serde::Deserialize, serde::Serialize)]
  #[serde(rename_all = "camelCase")]
  #[doc = _generic_res_data_elem_doc!()]
  pub struct V1Bullet {
    /// Web-socket servers.
    pub instance_servers: ArrayVec<V1BulletInstanceServer, 4>,
    /// Web-socket token connection.
    pub token: ArrayString<256>,
  }

  /// Information about a Web-socket server connection.
  #[derive(Debug, serde::Deserialize, serde::Serialize)]
  #[serde(rename_all = "camelCase")]
  pub struct V1BulletInstanceServer {
    /// If SSL should be used
    pub encrypt: bool,
    /// Websocket server address for establishing connection
    pub endpoint: _MaxUrl,
    /// Recommended internal to send ping requests in milliseconds.
    pub ping_interval: u64,
    /// Time that a connection will be considered expired without a ping.
    pub ping_timeout: u64,
    /// Supported protocol
    pub protocol: ArrayString<12>,
  }
}
