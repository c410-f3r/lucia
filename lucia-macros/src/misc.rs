use proc_macro2::{Ident, Span, TokenStream};
use syn::{
  punctuated::Punctuated,
  token::{Bracket, Pound},
  AttrStyle, Attribute, Path, PathArguments, PathSegment,
};

pub(crate) fn has_at_least_one_doc(attrs: &[Attribute]) -> bool {
  attrs.iter().any(|attr| {
    if let Some(last) = attr.path.segments.last() {
      last.ident == "doc"
    } else {
      false
    }
  })
}

pub(crate) fn push_allow_missing_docs(attrs: &mut Vec<Attribute>) {
  push_attr(attrs, ["allow"], quote::quote!((missing_docs)));
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
