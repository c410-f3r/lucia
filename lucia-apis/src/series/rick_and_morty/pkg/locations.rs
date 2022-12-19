#[lucia_macros::pkg(
  api(crate::series::rick_and_morty::RickAndMorty),
  data_format(json),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::series::rick_and_morty::{
    Location, Pagination, RickAndMortyHttpPkgsAux, CHARACTER_FRAGMENT,
  };
  use core::fmt::Write;
  use lucia::{
    data_format::{GraphQlRequest, GraphQlResponse},
    network::HttpMethod,
  };

  #[pkg::aux]
  impl<DRSR> RickAndMortyHttpPkgsAux<DRSR> {
    #[pkg::aux_data]
    fn locations_data<'any>(
      &mut self,
      buffer: &'any mut String,
      dimension: &str,
      name: &str,
      page: u32,
      ty: &str,
    ) -> crate::Result<LocationsReq<'any>> {
      buffer.clear();
      buffer
        .write_fmt(format_args!(
          r#"
            {CHARACTER_FRAGMENT}
            query {{
              locations(
                filter: {{
                  dimension: "{dimension}",
                  name: "{name}",
                  type: "{ty}",
                }}
                page: {page},
              ) {{
                info {{
                  prev
                  pages
                  next
                  count
                }}
                results {{
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
            }}
          "#
        ))
        .map_err(|err| lucia::Error::from(err))?;
      self.ext_req_params.method = HttpMethod::Post;
      Ok(LocationsReq { operation_name: None, query: buffer, variables: None })
    }
  }

  #[pkg::req_data]
  pub type LocationsReq<'any> = GraphQlRequest<(), &'any str, ()>;

  #[pkg::res_data]
  pub type LocationsRes = GraphQlResponse<LocationsData, serde::de::IgnoredAny>;

  #[derive(Debug, serde::Deserialize)]
  #[doc = generic_data_doc!()]
  pub struct Locations {
    /// Pagination
    pub info: Pagination,
    /// Locations
    pub results: Vec<Location>,
  }

  #[derive(Debug, serde::Deserialize)]
  #[doc = generic_data_doc!()]
  pub struct LocationsData {
    /// Locations
    pub locations: Locations,
  }
}
