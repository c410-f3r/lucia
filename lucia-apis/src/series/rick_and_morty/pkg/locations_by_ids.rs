#[lucia_macros::pkg(
  api(crate::series::rick_and_morty::RickAndMorty),
  data_format(json),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::{
    misc::SliceByCommas,
    series::rick_and_morty::{Location, RickAndMortyHttpPkgsAux, CHARACTER_FRAGMENT},
  };
  use core::fmt::Write;
  use lucia::{
    data_format::{GraphQlRequest, GraphQlResponse},
    network::{transport::TransportParams, HttpMethod},
  };

  #[pkg::aux]
  impl<DRSR> RickAndMortyHttpPkgsAux<DRSR> {
    #[pkg::aux_data]
    fn locations_by_ids_data<'any>(
      &mut self,
      buffer: &'any mut String,
      ids: &[u32],
    ) -> crate::Result<LocationsByIdsReq<'any>> {
      buffer.clear();
      buffer
        .write_fmt(format_args!(
          r#"
            {CHARACTER_FRAGMENT}
            query {{
              locationsByIds(ids: ["{}"]) {{
                created
                dimension
                id
                name
                residents {{
                  ...CharacterFrag
                }}
                type
              }}
            }}
          "#,
          SliceByCommas(ids)
        ))
        .map_err(|err| lucia::Error::from(err))?;
      self.tp.ext_req_params_mut().method = HttpMethod::Post;
      Ok(LocationsByIdsReq { operation_name: None, query: buffer, variables: None })
    }
  }

  #[pkg::req_data]
  pub type LocationsByIdsReq<'any> = GraphQlRequest<(), &'any str, ()>;

  #[pkg::res_data]
  pub type LocationsByIdsRes = GraphQlResponse<LocationsByIdsData, serde::de::IgnoredAny>;

  #[derive(Debug, serde::Deserialize)]
  #[doc = generic_data_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct LocationsByIdsData {
    /// Locations by ids
    pub locations_by_ids: Vec<Location>,
  }
}
