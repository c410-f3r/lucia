use proc_macro2::{Ident, Span, TokenStream};
use syn::{
  punctuated::Punctuated,
  token::{Bracket, Pound},
  AttrStyle, Attribute, Path, PathArguments, PathSegment,
};

pub(crate) fn attrs_by_names<'attrs, const N: usize>(
  attrs: &'attrs [Attribute],
  names: [&str; N],
) -> [Option<&'attrs Attribute>; N] {
  let mut rslt = [None; N];
  for attr in attrs {
    let Some(last) = attr.path.segments.last() else { continue; };
    let s = last.ident.to_string();
    for (name, rslt_attr) in names.iter().zip(&mut rslt) {
      if rslt_attr.is_some() {
        continue;
      }
      if name == &s {
        *rslt_attr = Some(attr);
        break;
      }
    }
  }
  rslt
}

pub(crate) fn has_at_least_one_doc(attrs: &[Attribute]) -> bool {
  attrs_by_names(attrs, ["doc"])[0].is_some()
}

pub(crate) fn push_doc(attrs: &mut Vec<Attribute>, doc: &str) {
  push_attr(attrs, ["doc"], quote::quote!(= #doc));
}

pub(crate) fn push_doc_if_inexistent(attrs: &mut Vec<Attribute>, doc: &str) {
  if !has_at_least_one_doc(attrs) {
    push_doc(attrs, doc);
  }
}

fn push_attr<'any>(
  attrs: &mut Vec<Attribute>,
  idents: impl IntoIterator<Item = &'any str>,
  tokens: TokenStream,
) {
  attrs.push(Attribute {
    pound_token: Pound(Span::mixed_site()),
    style: AttrStyle::Outer,
    bracket_token: Bracket(Span::mixed_site()),
    path: Path {
      leading_colon: None,
      segments: {
        let mut vec = Punctuated::new();
        for ident in idents {
          vec.push(PathSegment {
            ident: Ident::new(ident, Span::mixed_site()),
            arguments: PathArguments::None,
          });
        }
        vec
      },
    },
    tokens,
  })
}

#[cfg(test)]
mod tests {
  use crate::misc::{attrs_by_names, push_attr};
  use proc_macro2::TokenStream;

  #[test]
  fn has_names_in_attrs_has_correct_output() {
    let mut attrs = Vec::new();
    push_attr(&mut attrs, ["foo"], TokenStream::new());
    push_attr(&mut attrs, ["baz"], TokenStream::new());
    assert_eq!(
      attrs_by_names(&attrs, ["foo", "bar", "baz"]),
      [Some(&attrs[0]), None, Some(&attrs[1])]
    );
    let attrs = Vec::new();
    assert_eq!(attrs_by_names(&attrs, ["foo", "bar", "baz"]), [None, None, None]);
  }
}
