#[lucia_macros::pkg(api(KuCoin), data_format(json), error(crate::Error), transport(http))]
pub(crate) mod pkg {
  use crate::{
    exchange::ku_coin::{HttpResWrapper, KuCoin, KuCoinHttpPkgsAux, KuCoinId},
    misc::into_rslt,
  };
  use lucia::{
    misc::SyncDynDebugDisplay,
    network::{HttpMethod, HttpReqParams},
  };

  #[pkg::aux]
  impl<DRSR> KuCoinHttpPkgsAux<DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    api: &mut KuCoin,
    req_bytes: &[u8],
    req_params: &mut HttpReqParams,
  ) -> crate::Result<()> {
    req_params.method = HttpMethod::Post;
    req_params.url.push_path(format_args!("/api/v2/accounts/inner-transfer"))?;
    into_rslt(api.credentials.as_mut())?.push_headers(req_bytes, req_params)?;
    Ok(())
  }

  #[derive(Debug, serde::Serialize)]
  #[serde(rename_all = "camelCase")]
  #[pkg::req_data]
  pub struct V2InnerTransferReq<'any> {
    client_oid: &'any SyncDynDebugDisplay,
    currency: &'any str,
    from: PaymentAccountType,
    to: ReceivingAccountType,
    amount: &'any str,
    #[serde(skip_serializing_if = "Option::is_none")]
    from_tag: Option<&'any str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to_tag: Option<&'any str>,
  }

  #[pkg::res_data]
  pub type V2InnerTransferRes = HttpResWrapper<Box<V2InnerTransfer>>;

  /// Sending account type.
  #[derive(Debug, serde::Serialize)]
  #[serde(rename_all = "lowercase")]
  pub enum PaymentAccountType {
    /// Main
    Main,
    /// Trade
    Trade,
    /// Margin
    Margin,
    /// Isolated
    Isolated,
  }

  /// Receiving account type.
  #[derive(Debug, serde::Serialize)]
  #[serde(rename_all = "lowercase")]
  pub enum ReceivingAccountType {
    /// Main
    Main,
    /// Trade
    Trade,
    /// Margin
    Margin,
    /// Isolated
    Isolated,
    /// Contract
    Contract,
  }

  /// Inner transfer ID
  #[derive(Debug, serde::Deserialize)]
  #[serde(rename_all = "camelCase")]
  pub struct V2InnerTransfer {
    /// ID
    pub order_id: KuCoinId,
  }
}
