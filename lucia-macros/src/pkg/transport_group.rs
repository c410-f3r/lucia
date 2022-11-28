use proc_macro2::Span;
use syn::{Meta, NestedMeta};

#[derive(Clone, Copy, Debug)]
pub(crate) enum TransportGroup {
  Http,
  Stub,
  Tcp,
  Udp,
  WebSocket,
}

impl<'attrs> TryFrom<&'attrs NestedMeta> for TransportGroup {
  type Error = crate::Error;

  fn try_from(from: &'attrs NestedMeta) -> Result<Self, Self::Error> {
    let path = if let NestedMeta::Meta(Meta::Path(ref elem)) = *from {
      elem
    } else {
      return Err(crate::Error::UnknownTransport(Span::mixed_site()));
    };
    let mut iter = path.segments.iter();
    let first = iter.next().ok_or_else(|| crate::Error::UnknownTransport(Span::mixed_site()))?;
    if iter.next().is_some() {
      return Err(crate::Error::UnknownTransport(first.ident.span()));
    }
    Ok(match first.ident.to_string().as_str() {
      "http" => TransportGroup::Http,
      "stub" => TransportGroup::Stub,
      "tcp" => TransportGroup::Tcp,
      "udp" => TransportGroup::Udp,
      "ws" => TransportGroup::WebSocket,
      _ => return Err(crate::Error::UnknownTransport(first.ident.span())),
    })
  }
}
