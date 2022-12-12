//! Custom transport through `transport(Custom)`.

use lucia::pkg::PkgsAux;
use lucia::pkg::Package;
use lucia::network::TransportGroup;
use lucia::network::transport::Transport;
use lucia::network::transport::TransportParams;

struct CustomTransport;

#[async_trait::async_trait]
impl<DRSR> Transport<DRSR> for CustomTransport
where
  DRSR: Send + Sync,
{
  const GROUP: TransportGroup = TransportGroup::Custom("Custom");
  type Params = CustomTransportParams;

  #[inline]
  async fn send<P>(
    &mut self,
    _: &mut P,
    _: &mut PkgsAux<P::Api, DRSR, Self::Params>,
  ) -> Result<(), P::Error>
  where
    P: Package<DRSR, Self::Params> + Send + Sync,
  {
    Ok(())
  }

  #[inline]
  async fn send_and_retrieve<P>(
    &mut self,
    _: &mut P,
    _: &mut PkgsAux<P::Api, DRSR, Self::Params>,
  ) -> Result<usize, P::Error>
  where
    P: Package<DRSR, Self::Params> + Send + Sync,
  {
    Ok(0)
  }
}

struct CustomTransportParams;

impl TransportParams for CustomTransportParams {
  type ExternalRequestParams = ();
  type ExternalResponseParams = ();

  #[inline]
  fn into_parts(self) -> (Self::ExternalRequestParams, Self::ExternalResponseParams) {
    ((), ())
  }
}

type Nothing = ();

#[lucia_macros::pkg(api(super::Nothing), data_format(json), transport(custom(crate::CustomTransport)))]
mod pkg {
  #[pkg::req_data]
  struct Req;

  #[pkg::res_data]
  struct Res;
}

fn main() {
}
