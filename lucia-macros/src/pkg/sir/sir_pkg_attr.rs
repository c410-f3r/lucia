use crate::{
  pkg::{data_format::DataFormat, fir::fir_pkg_attr::FirPkgAttr},
  transport_group::TransportGroup,
};
use alloc::borrow::Cow;
use proc_macro2::{Ident, Span};
use syn::{punctuated::Punctuated, Path, PathArguments, PathSegment};

#[derive(Debug)]
pub(crate) struct SirPkaAttr<'attrs> {
  pub(crate) api: &'attrs Path,
  pub(crate) data_formats: Vec<DataFormat>,
  pub(crate) error: Cow<'attrs, Path>,
  pub(crate) transport_groups: Vec<TransportGroup>,
}

impl<'attrs> TryFrom<FirPkgAttr<'attrs>> for SirPkaAttr<'attrs> {
  type Error = crate::Error;

  fn try_from(fea: FirPkgAttr<'attrs>) -> Result<Self, Self::Error> {
    let data_formats =
      fea.data_formats.into_iter().map(TryInto::try_into).collect::<crate::Result<Vec<_>>>()?;
    if data_formats.is_empty() {
      return Err(crate::Error::MandatoryOuterAttrsAreNotPresent);
    }
    Ok(Self {
      api: fea.api,
      data_formats,
      error: fea.error.map_or_else(
        || {
          let mut segments = Punctuated::new();
          segments.push(PathSegment {
            ident: Ident::new("lucia", Span::mixed_site()),
            arguments: PathArguments::None,
          });
          segments.push(PathSegment {
            ident: Ident::new("Error", Span::mixed_site()),
            arguments: PathArguments::None,
          });
          Cow::Owned(Path { leading_colon: None, segments })
        },
        Cow::Borrowed,
      ),
      transport_groups: fea
        .transports
        .into_iter()
        .map(TryInto::try_into)
        .collect::<crate::Result<_>>()?,
    })
  }
}
