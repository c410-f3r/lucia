#[lucia_macros::pkg(
  api(crate::series::rick_and_morty::RickAndMorty),
  data_format(json),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::series::rick_and_morty::{
    Episode, Pagination, RickAndMortyHttpPkgsAux, CHARACTER_FRAGMENT,
  };
  use core::fmt::Write;
  use lucia::{
    data_format::{GraphQlRequest, GraphQlResponse},
    network::{transport::TransportParams, HttpMethod},
  };

  #[pkg::aux]
  impl<DRSR> RickAndMortyHttpPkgsAux<DRSR> {
    #[pkg::aux_data]
    fn episodes_data<'any>(
      &mut self,
      buffer: &'any mut String,
      episode: &str,
      name: &str,
      page: u32,
    ) -> crate::Result<EpisodesReq<'any>> {
      buffer.clear();
      buffer
        .write_fmt(format_args!(
          r#"
            {CHARACTER_FRAGMENT}
            query {{
              episodes(
                filter: {{
                  episode: "{episode}",
                  name: "{name}",
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
                  air_date
                  characters {{
                    ...CharacterFrag
                  }}
                  created
                  episode
                  id
                  name
                }}
              }}
            }}
          "#
        ))
        .map_err(lucia::Error::from)?;
      self.tp.ext_req_params_mut().method = HttpMethod::Post;
      Ok(EpisodesReq { operation_name: None, query: buffer, variables: None })
    }
  }

  #[pkg::req_data]
  pub type EpisodesReq<'any> = GraphQlRequest<(), &'any str, ()>;

  #[pkg::res_data]
  pub type EpisodesRes = GraphQlResponse<EpisodesData, serde::de::IgnoredAny>;

  #[derive(Debug, serde::Deserialize)]
  #[doc = generic_data_doc!()]
  pub struct Episodes {
    /// Pagination
    pub info: Pagination,
    /// Episodes
    pub results: Vec<Episode>,
  }

  #[derive(Debug, serde::Deserialize)]
  #[doc = generic_data_doc!()]
  pub struct EpisodesData {
    /// Episodes
    pub episodes: Episodes,
  }
}
