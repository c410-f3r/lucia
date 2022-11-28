use proc_macro2::Span;

#[allow(
  // `syn::Error` is composed by a single vector
  variant_size_differences
)]
#[derive(Debug)]
pub(crate) enum Error {
  AbsentFieldInUnnamedStruct(Span),
  AbsentReqOrRes(Span),
  BadAfterSending(Span),
  BadAux(Span),
  BadAuxData(Span, String),
  BadBeforeSending(Span),
  BadField(Span),
  BadParams(Span),
  BadReqData(Span),
  BadResData(Span),
  DuplicatedAttribute(Span),
  DuplicatedPkg,
  ElemHasDoc(Span),
  EmptyDataFormats,
  IncorrectJsonRpcDataFormat,
  MandatoryOuterAttrsAreNotPresent,
  NoEnumStructOrType(Span),
  ResponsesCanNotHaveTypeParams(Span),
  Syn(syn::Error),
  UnknownDataFormat,
  UnknownTransport(Span),
  WrongPkgPlace(Span),
}

impl From<syn::Error> for Error {
  fn from(from: syn::Error) -> Self {
    Self::Syn(from)
  }
}

impl From<Error> for syn::Error {
  fn from(from: Error) -> Self {
    match from {
      Error::AbsentFieldInUnnamedStruct(span) => syn::Error::new(
        span,
        "Unnamed structures must have a `#[pkg::field]` attribute on each field.",
      ),
      Error::AbsentReqOrRes(span) => syn::Error::new(
        span,
        "The `#[pkg]` module must have an inner `#[pkg::req_data]` element and an inner \
          `#[pkg::res_data]` element.",
      ),
      Error::BadAux(span) => syn::Error::new(
        span,
        "#[pkg::aux] must be an item implementation with none, one `#[pkg::aux_data]`, one \
          `#[pkg::aux_params]` or both `#[pkg::aux_data]` and `#[pkg::aux_params]`",
      ),
      Error::BadAfterSending(span) => syn::Error::new(
        span,
        "#[pkg::after_sending] must have be a function with signature of type \
        `after_sending(&mut ResponseParameters) -> Result<(), DefinedError>` or \
        `after_sending(&mut Parameters, &mut ResponseParameters) -> Result<(), DefinedError>`",
      ),
      Error::BadBeforeSending(span) => syn::Error::new(
        span,
        "#[pkg::before_sending] must have be a function with signature of type \
        `before_sending(&mut ResponseParameters) -> Result<(), DefinedError>` or \
        `before_sending(&mut Parameters, &mut ResponseParameters) -> Result<(), DefinedError>`",
      ),
      Error::BadReqData(span) => {
        syn::Error::new(span, "Request data must end with the `ReqData` suffix.")
      }
      Error::BadAuxData(span, name) => {
        syn::Error::new(span, format!("This method must be named as `{name}`"))
      }
      Error::BadParams(span) => {
        syn::Error::new(span, "Parameters must end with the `Params` suffix.")
      }
      Error::BadField(span) => {
        syn::Error::new(span, "#[pkg::field] expects a mandatory `name` parameter")
      }
      Error::BadResData(span) => {
        syn::Error::new(span, "Response data must end with the `ResData` suffix.")
      }
      Error::DuplicatedAttribute(span) => {
        syn::Error::new(span, "It is not possible to have more than one of this attribute.")
      }
      Error::DuplicatedPkg => syn::Error::new(
        Span::call_site(),
        "It is not possible to have more than one `pkg` attribute in this structure.",
      ),
      Error::ElemHasDoc(span) => syn::Error::new(span, "Element expects no documentation"),
      Error::EmptyDataFormats => {
        syn::Error::new(Span::call_site(), "`#[pkg]` requires at least one data format.")
      }
      Error::IncorrectJsonRpcDataFormat => syn::Error::new(
        Span::call_site(),
        "JSON-RPC expects the name of its method. For example, \
          `#[pkg(data_format(json_rpc(\"method\")))]`",
      ),
      Error::MandatoryOuterAttrsAreNotPresent => syn::Error::new(
        Span::call_site(),
        "All packages must have an `api` and a `data_format` attribute. For example, \
          #[pkg(api(SomeApi), data_format(json))]",
      ),
      Error::NoEnumStructOrType(span) => {
        syn::Error::new(span, "Invalid type. Expected enum, struct or type.")
      }
      Error::ResponsesCanNotHaveTypeParams(span) => {
        syn::Error::new(span, "Responses can not have type parameters")
      }
      Error::Syn(error) => error,
      Error::UnknownDataFormat => syn::Error::new(Span::call_site(), "Unknown data format."),
      Error::UnknownTransport(span) => syn::Error::new(span, "Unknown transport."),
      Error::WrongPkgPlace(span) => {
        syn::Error::new(span, "#[pkg] must be placed above an inline module")
      }
    }
  }
}
