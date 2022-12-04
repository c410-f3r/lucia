#[lucia_macros::pkg(api(KuCoin), data_format(json), error(crate::Error), transport(http))]
pub(crate) mod pkg {
  use crate::{
    exchange::ku_coin::{
      manage_paginated_params, KuCoin, KuCoinHttpPackagesAux, Order, OrderSide, PaginatedResponse,
      ResponseWrapper,
    },
    misc::into_rslt,
  };
  use core::fmt::{Display, Formatter};
  use lucia::network::HttpReqParams;

  #[pkg::aux]
  impl<DRSR> KuCoinHttpPackagesAux<DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    api: &mut KuCoin,
    params: &mut V1GetOrdersParams<'_>,
    req_bytes: &[u8],
    req_params: &mut HttpReqParams,
  ) -> crate::Result<()> {
    api.orders_rt.rc.update_params(&api.orders_rt.rl).await?;
    req_params.url.push_path(format_args!("/api/v1/orders"))?;
    let qw = req_params
      .url
      .query_writer()?
      .write_opt("status", params.status)?
      .write_opt("symbol", params.symbol)?
      .write_opt("side", params.side)?;
    manage_paginated_params(qw, params.pagination)?;
    into_rslt(api.credentials.as_mut())?.push_headers(req_bytes, req_params)?;
    Ok(())
  }

  #[derive(Debug)]
  #[pkg::params]
  pub struct V1GetOrdersParams<'any> {
    pagination: Option<[u32; 2]>,
    side: Option<OrderSide>,
    status: Option<V1GetOrderStatus>,
    symbol: Option<&'any str>,
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct V1GetOrdersReq;

  #[pkg::res_data]
  pub type V1GetOrdersRes = ResponseWrapper<PaginatedResponse<Order>>;

  /// Order trading status.
  #[derive(Clone, Copy, Debug)]
  pub enum V1GetOrderStatus {
    /// Is being traded.
    Active,
    /// Finished or canceled
    Done,
  }

  impl Display for V1GetOrderStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
      f.write_str(match *self {
        Self::Active => "active",
        Self::Done => "done",
      })
    }
  }
}
