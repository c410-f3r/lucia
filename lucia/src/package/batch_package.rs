use crate::{
  dnsn::{Deserialize, Serialize},
  network::transport::TransportParams,
  package::Package,
  Id,
};
use cl_aux::DynContigColl;
use core::{borrow::Borrow, marker::PhantomData};

/// Used to perform batch requests with multiple packages.
#[derive(Debug)]
pub struct BatchPackage<'slice, DRSR, P, TP>(BatchElems<'slice, DRSR, P, TP>, ());

impl<'slice, DRSR, P, TP> BatchPackage<'slice, DRSR, P, TP> {
  /// Currently, only slices of packages are allowed to perform batch requests.
  #[inline]
  pub fn new(slice: &'slice mut [P]) -> Self {
    Self(BatchElems(slice, PhantomData), ())
  }
}

impl<'slice, DRSR, P, TP> BatchPackage<'slice, DRSR, P, TP>
where
  P: Package<DRSR, TP>,
  P::ExternalRequestContent: Borrow<Id> + Ord,
  P::ExternalResponseContent: Borrow<Id> + Ord,
  TP: TransportParams,
{
  /// Deserializes a sequence of bytes and then pushes them to the provided buffer.
  #[inline]
  pub fn decode_and_push_from_bytes<B>(
    &mut self,
    buffer: &mut B,
    bytes: &[u8],
    drsr: &mut DRSR,
  ) -> Result<(), P::Error>
  where
    B: DynContigColl<P::ExternalResponseContent>,
  {
    if self.0 .0.is_empty() {
      return Ok(());
    }
    Self::is_sorted(self.0 .0.iter().map(|elem| elem.ext_req_ctnt().borrow()))?;
    let mut pkgs_idx = 0;
    let mut responses_are_not_sorted = false;
    P::ExternalResponseContent::seq_from_bytes(
      bytes,
      drsr,
      |eresc: P::ExternalResponseContent| {
        let eresc_id = *eresc.borrow();
        let found_pkgs_idx = Self::search_slice(pkgs_idx, eresc_id, self.0 .0)?;
        if pkgs_idx != found_pkgs_idx {
          responses_are_not_sorted = true;
        }
        buffer.push(eresc).map_err(Into::into)?;
        pkgs_idx = pkgs_idx.wrapping_add(1);
        Ok::<_, P::Error>(())
      },
    )?;
    if responses_are_not_sorted {
      buffer.sort_unstable();
    }
    Ok(())
  }

  #[inline]
  fn is_sorted<T>(mut iter: impl Iterator<Item = T>) -> crate::Result<()>
  where
    T: PartialOrd,
  {
    let mut is_sorted = true;
    let mut previous = if let Some(elem) = iter.next() { elem } else { return Ok(()) };
    for curr in iter {
      if previous > curr {
        is_sorted = false;
        break;
      } else {
        previous = curr;
      }
    }
    if is_sorted {
      Ok(())
    } else {
      Err(crate::Error::BatchPackagesAreNotSorted)
    }
  }

  // First try indexing and then falls back to binary search
  fn search_slice(idx: usize, eresc_id: Id, pkgs: &[P]) -> crate::Result<usize> {
    if pkgs.get(idx).map(|pkg| *pkg.ext_req_ctnt().borrow() == eresc_id).unwrap_or_default() {
      return Ok(idx);
    }
    pkgs
      .binary_search_by(|req| req.ext_req_ctnt().borrow().cmp(&eresc_id))
      .ok()
      .ok_or(crate::Error::ResponseIdIsNotPresentInTheOfSentBatchPackages(eresc_id))
  }
}

impl<'slice, DRSR, P, TP> Package<DRSR, TP> for BatchPackage<'slice, DRSR, P, TP>
where
  BatchElems<'slice, DRSR, P, TP>: Serialize<DRSR>,
  P: Package<DRSR, TP>,
  TP: TransportParams,
{
  type Api = P::Api;
  type Error = P::Error;
  type ExternalRequestContent = BatchElems<'slice, DRSR, P, TP>;
  type ExternalResponseContent = ();
  type PackageParams = ();

  #[inline]
  fn after_sending(
    &mut self,
    api: &mut Self::Api,
    ext_res_params: &mut TP::ExternalResponseParams,
  ) -> Result<(), Self::Error> {
    for elem in self.0 .0.iter_mut() {
      elem.after_sending(api, ext_res_params)?;
    }
    Ok(())
  }

  #[inline]
  fn before_sending(
    &mut self,
    api: &mut Self::Api,
    ext_req_params: &mut TP::ExternalRequestParams,
  ) -> Result<(), Self::Error> {
    for elem in self.0 .0.iter_mut() {
      elem.before_sending(api, ext_req_params)?;
    }
    Ok(())
  }

  #[inline]
  fn ext_req_ctnt(&self) -> &Self::ExternalRequestContent {
    &self.0
  }

  #[inline]
  fn ext_req_ctnt_mut(&mut self) -> &mut Self::ExternalRequestContent {
    &mut self.0
  }

  #[inline]
  fn pkg_params(&self) -> &Self::PackageParams {
    &self.1
  }

  #[inline]
  fn pkg_params_mut(&mut self) -> &mut Self::PackageParams {
    &mut self.1
  }
}

/// Used internally and exclusively by [BatchPackage]. Not intended for public usage.
#[derive(Debug)]
pub struct BatchElems<'slice, DRSR, P, T>(&'slice mut [P], PhantomData<(DRSR, T)>);

#[cfg(feature = "serde_json")]
mod serde_json {
  use crate::{
    dnsn::SerdeJson,
    misc::ByteBuffer,
    network::transport::TransportParams,
    package::{BatchElems, Package},
  };
  use serde::Serializer;

  impl<'slice, DRSR, P, TP> crate::dnsn::Serialize<SerdeJson> for BatchElems<'slice, DRSR, P, TP>
  where
    P: Package<DRSR, TP>,
    P::ExternalRequestContent: serde::Serialize,
    TP: TransportParams,
  {
    #[inline]
    fn to_bytes<BB>(&mut self, bytes: &mut BB, _: &mut SerdeJson) -> crate::Result<()>
    where
      BB: ByteBuffer,
    {
      serde_json::Serializer::new(bytes).collect_seq(self.0.iter().map(|el| el.ext_req_ctnt()))?;
      Ok(())
    }
  }
}
