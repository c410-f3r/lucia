use crate::{pkg::data_format_elems::DataFormatElems, transport_group::TransportGroup};
use proc_macro2::{Ident, Span, TokenStream};
use syn::{Lit, Meta, NestedMeta};

#[derive(Debug)]
pub(crate) enum DataFormat {
  Borsh,
  Json,
  JsonRpc(String),
  Verbatim,
  Xml,
  Yaml,
}

impl DataFormat {
  pub(crate) fn before_sending_defaults(&self, tg: &TransportGroup) -> TokenStream {
    macro_rules! http_method_and_mime_type {
      ($method:ident, $mime_type:ident) => {
        quote::quote!(
          _ext_req_params.method = lucia::network::HttpMethod::$method;
          _ext_req_params.mime_type = Some(lucia::network::HttpMimeType::$mime_type);
        )
      };
    }
    macro_rules! http_mime_type {
      ($mime_type:ident) => {
        quote::quote!(
          _ext_req_params.mime_type = Some(lucia::network::HttpMimeType::$mime_type);
        )
      };
    }
    macro_rules! rslt {
      ($http_tt:expr) => {
        match *tg {
          TransportGroup::Http => $http_tt,
          TransportGroup::Custom(_)
          | TransportGroup::Stub
          | TransportGroup::Tcp
          | TransportGroup::Udp
          | TransportGroup::WebSocket => TokenStream::new(),
        }
      };
    }
    match *self {
      DataFormat::Borsh => TokenStream::new(),
      DataFormat::Json => rslt!(http_mime_type!(Json)),
      DataFormat::JsonRpc(_) => rslt!(http_method_and_mime_type!(Post, Json)),
      DataFormat::Verbatim => TokenStream::new(),
      DataFormat::Xml => rslt!(http_mime_type!(Xml)),
      DataFormat::Yaml => rslt!(http_mime_type!(Yaml)),
    }
  }

  pub(crate) fn elems(&self) -> DataFormatElems {
    let ident_fn = |name| Ident::new(name, Span::mixed_site());
    match *self {
      DataFormat::Borsh => DataFormatElems {
        dfe_data_format_builder_fn: ident_fn("build_borsh"),
        dfe_ext_req_ctnt_wrapper: ident_fn("BorshRequest"),
        dfe_ext_res_ctnt_wrapper: ident_fn("BorshResponse"),
        dfe_pkgs_aux_call: quote::quote!(borsh_request(data)),
      },
      DataFormat::Json => DataFormatElems {
        dfe_data_format_builder_fn: ident_fn("build_json"),
        dfe_ext_req_ctnt_wrapper: ident_fn("JsonRequest"),
        dfe_ext_res_ctnt_wrapper: ident_fn("JsonResponse"),
        dfe_pkgs_aux_call: quote::quote!(json_request(data)),
      },
      DataFormat::JsonRpc(ref method) => DataFormatElems {
        dfe_data_format_builder_fn: ident_fn("build_json_rpc"),
        dfe_ext_req_ctnt_wrapper: ident_fn("JsonRpcRequest"),
        dfe_ext_res_ctnt_wrapper: ident_fn("JsonRpcResponse"),
        dfe_pkgs_aux_call: quote::quote!(json_rpc_request(#method, data)),
      },
      DataFormat::Verbatim => DataFormatElems {
        dfe_data_format_builder_fn: ident_fn("build_verbatim"),
        dfe_ext_req_ctnt_wrapper: ident_fn("VerbatimRequest"),
        dfe_ext_res_ctnt_wrapper: ident_fn("VerbatimResponse"),
        dfe_pkgs_aux_call: quote::quote!(verbatim_request(data)),
      },
      DataFormat::Xml => DataFormatElems {
        dfe_data_format_builder_fn: ident_fn("build_xml"),
        dfe_ext_req_ctnt_wrapper: ident_fn("XmlRequest"),
        dfe_ext_res_ctnt_wrapper: ident_fn("XmlResponse"),
        dfe_pkgs_aux_call: quote::quote!(xml_request(data)),
      },
      DataFormat::Yaml => DataFormatElems {
        dfe_data_format_builder_fn: ident_fn("build_yaml"),
        dfe_ext_req_ctnt_wrapper: ident_fn("YamlRequest"),
        dfe_ext_res_ctnt_wrapper: ident_fn("YamlResponse"),
        dfe_pkgs_aux_call: quote::quote!(yaml_request(data)),
      },
    }
  }
}

impl<'attrs> TryFrom<&'attrs NestedMeta> for DataFormat {
  type Error = crate::Error;

  fn try_from(from: &'attrs NestedMeta) -> Result<Self, Self::Error> {
    macro_rules! first_path_seg_ident {
      ($path:expr) => {
        if let Some(elem) = $path.segments.first() {
          &elem.ident
        } else {
          return Err(crate::Error::UnknownDataFormat);
        }
      };
    }
    let meta = if let NestedMeta::Meta(ref elem) = *from {
      elem
    } else {
      return Err(crate::Error::UnknownDataFormat);
    };
    if let Meta::List(ref elem) = *meta {
      let first_path_seg_ident = first_path_seg_ident!(elem.path);
      if first_path_seg_ident == "json_rpc" {
        if let Some(&NestedMeta::Lit(Lit::Str(ref elem))) = elem.nested.first() {
          Ok(Self::JsonRpc(elem.value()))
        } else {
          Err(crate::Error::IncorrectJsonRpcDataFormat)
        }
      } else {
        Err(crate::Error::UnknownDataFormat)
      }
    } else if let Meta::Path(ref elem) = *meta {
      match first_path_seg_ident!(elem).to_string().as_str() {
        "borsh" => Ok(Self::Borsh),
        "json" => Ok(Self::Json),
        "xml" => Ok(Self::Xml),
        "yaml" => Ok(Self::Yaml),
        "verbatim" => Ok(Self::Verbatim),
        _ => Err(crate::Error::UnknownDataFormat),
      }
    } else {
      Err(crate::Error::MandatoryOuterAttrsAreNotPresent)
    }
  }
}
