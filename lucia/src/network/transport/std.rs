use crate::{
  misc::manage_before_sending_related,
  network::{
    transport::{Transport, TransportParams},
    TcpParams, TransportGroup, UdpParams,
  },
  package::{Package, PackagesAux},
};
use alloc::boxed::Box;
use std::{
  io::{Read, Write},
  net::{TcpStream, UdpSocket},
};

#[async_trait::async_trait]
impl<DRSR> Transport<DRSR> for TcpStream
where
  DRSR: Send + Sync,
{
  const GROUP: TransportGroup = TransportGroup::TCP;
  type Params = TcpParams;

  #[inline]
  async fn send<P>(
    &mut self,
    pkg: &mut P,
    pkgs_aux: &mut PackagesAux<P::Api, DRSR, Self::Params>,
  ) -> Result<(), P::Error>
  where
    P: Package<DRSR, TcpParams> + Send + Sync,
  {
    send(pkg, pkgs_aux, self, |bytes, _, trans| Ok(trans.write(bytes)?)).await?;
    Ok(())
  }

  #[inline]
  async fn send_and_retrieve<P>(
    &mut self,
    pkg: &mut P,
    pkgs_aux: &mut PackagesAux<P::Api, DRSR, Self::Params>,
  ) -> Result<usize, P::Error>
  where
    P: Package<DRSR, TcpParams> + Send + Sync,
  {
    send_and_retrieve(pkg, pkgs_aux, self, |bytes, _, trans| Ok(trans.read(bytes)?)).await
  }
}

#[async_trait::async_trait]
impl<DRSR> Transport<DRSR> for UdpSocket
where
  DRSR: Send + Sync,
{
  const GROUP: TransportGroup = TransportGroup::UDP;
  type Params = UdpParams;

  #[inline]
  async fn send<P>(
    &mut self,
    pkg: &mut P,
    pkgs_aux: &mut PackagesAux<P::Api, DRSR, Self::Params>,
  ) -> Result<(), P::Error>
  where
    P: Package<DRSR, UdpParams> + Send + Sync,
    P::Api: Send + Sync,
  {
    send(pkg, pkgs_aux, self, |bytes, ext_req_params, trans| {
      Ok(trans.send_to(bytes, ext_req_params.url.url())?)
    })
    .await
  }

  #[inline]
  async fn send_and_retrieve<P>(
    &mut self,
    pkg: &mut P,
    pkgs_aux: &mut PackagesAux<P::Api, DRSR, Self::Params>,
  ) -> Result<usize, P::Error>
  where
    P: Package<DRSR, UdpParams> + Send + Sync,
  {
    Ok(send_and_retrieve(pkg, pkgs_aux, self, |bytes, _, trans| Ok(trans.recv(bytes)?)).await?)
  }
}

async fn send<DRSR, P, T>(
  pkg: &mut P,
  pkgs_aux: &mut PackagesAux<P::Api, DRSR, T::Params>,
  trans: &mut T,
  cb: impl Fn(
    &[u8],
    &<T::Params as TransportParams>::ExternalRequestParams,
    &mut T,
  ) -> crate::Result<usize>,
) -> Result<(), P::Error>
where
  DRSR: Send + Sync,
  P: Package<DRSR, T::Params> + Send + Sync,
  T: Send + Sync + Transport<DRSR>,
{
  pkgs_aux.byte_buffer.clear();
  manage_before_sending_related(pkg, pkgs_aux, trans).await?;
  let mut slice = pkgs_aux.byte_buffer.as_ref();
  let mut everything_was_sent = false;
  for _ in 0..16 {
    let sent = cb(slice, &pkgs_aux.ext_req_params, trans)?;
    if sent == slice.len() {
      everything_was_sent = true;
      break;
    } else {
      slice = slice.get(sent..).unwrap_or_default();
    }
  }
  pkgs_aux.byte_buffer.clear();
  pkgs_aux.byte_buffer.extend((0..pkgs_aux.byte_buffer.capacity()).map(|_| 0));
  pkg.after_sending(&mut pkgs_aux.api, &mut pkgs_aux.ext_res_params).await?;
  if everything_was_sent {
    Ok(())
  } else {
    Err(crate::Error::CouldNotSendTheFullRequestData.into())
  }
}

async fn send_and_retrieve<DRSR, P, T>(
  pkg: &mut P,
  pkgs_aux: &mut PackagesAux<P::Api, DRSR, T::Params>,
  trans: &mut T,
  cb: impl Fn(
    &mut [u8],
    &<T::Params as TransportParams>::ExternalRequestParams,
    &mut T,
  ) -> crate::Result<usize>,
) -> Result<usize, P::Error>
where
  DRSR: Send + Sync,
  P: Package<DRSR, T::Params> + Send + Sync,
  T: Send + Sync + Transport<DRSR>,
{
  trans.send(pkg, pkgs_aux).await?;
  let slice = pkgs_aux.byte_buffer.as_mut();
  let len = cb(slice, &pkgs_aux.ext_req_params, trans)?;
  Ok(len)
}

#[cfg(test)]
mod tests {
  use crate::{
    dnsn::{Deserialize, Serialize},
    misc::{sleep, ByteBuffer},
    network::{
      transport::{Transport, TransportParams},
      TcpParams, UdpParams,
    },
    package::{Package, PackagesAux},
  };
  use core::time::Duration;
  use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream, UdpSocket},
  };

  #[derive(Debug, Eq, PartialEq)]
  struct PingPong(Ping, ());

  impl<DRSR, TP> Package<DRSR, TP> for PingPong
  where
    DRSR: Send + Sync,
    TP: Send + Sync + TransportParams,
  {
    type Api = ();
    type Error = crate::Error;
    type ExternalRequestContent = Ping;
    type ExternalResponseContent = Pong;
    type PackageParams = ();

    fn ext_req_content(&self) -> &Self::ExternalRequestContent {
      &self.0
    }

    fn ext_req_content_mut(&mut self) -> &mut Self::ExternalRequestContent {
      &mut self.0
    }

    fn pkg_params(&self) -> &Self::PackageParams {
      &self.1
    }

    fn pkg_params_mut(&mut self) -> &mut Self::PackageParams {
      &mut self.1
    }
  }

  #[derive(Debug, Eq, PartialEq)]
  struct Ping;

  impl<DRSR> Serialize<DRSR> for Ping {
    fn to_bytes<BB>(&mut self, bytes: &mut BB, _: &mut DRSR) -> crate::Result<()>
    where
      BB: ByteBuffer,
    {
      bytes.extend(*b"ping")?;
      Ok(())
    }
  }

  #[derive(Debug, Eq, PartialEq)]
  struct Pong(&'static str);

  impl<DRSR> Deserialize<DRSR> for Pong {
    #[inline]
    fn from_bytes(bytes: &[u8], _: &mut DRSR) -> crate::Result<Self> {
      assert_eq!(bytes, b"ping");
      Ok(Self("pong"))
    }

    #[inline]
    fn seq_from_bytes<E>(
      _: &[u8],
      _: &mut DRSR,
      _: impl FnMut(Self) -> Result<(), E>,
    ) -> Result<(), E>
    where
      E: From<crate::Error>,
    {
      Ok(())
    }
  }

  #[tokio::test(flavor = "multi_thread")]
  async fn tcp() {
    let addr = "127.0.0.1:12345";
    let _server = tokio::spawn(async move {
      let tcp_listener = TcpListener::bind(addr).unwrap();
      let mut buffer = [0; 8];
      let (mut stream, _) = tcp_listener.accept().unwrap();
      let idx = stream.read(&mut buffer).unwrap();
      stream.write_all(&buffer[..idx]).unwrap();
    });
    sleep(Duration::from_millis(100)).await.unwrap();
    let mut pa = PackagesAux::from_minimum((), (), TcpParams::from_url(addr).unwrap());
    let mut trans = TcpStream::connect(addr).unwrap();
    let res =
      trans.send_retrieve_and_decode_contained(&mut PingPong(Ping, ()), &mut pa).await.unwrap();
    assert_eq!(res, Pong("pong"));
  }

  #[tokio::test]
  async fn udp() {
    let addr = "127.0.0.1:12346";
    let mut pa = PackagesAux::from_minimum((), (), UdpParams::from_url(addr).unwrap());
    let mut trans = UdpSocket::bind(addr).unwrap();
    let res =
      trans.send_retrieve_and_decode_contained(&mut PingPong(Ping, ()), &mut pa).await.unwrap();
    assert_eq!(res, Pong("pong"));
  }
}
