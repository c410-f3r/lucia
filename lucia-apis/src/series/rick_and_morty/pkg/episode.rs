#[lucia_macros::pkg(
  api(crate::series::rick_and_morty::RickAndMorty),
  data_format(json),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::series::rick_and_morty::{Episode, RickAndMortyHttpPkgsAux, CHARACTER_FRAGMENT};
  use core::fmt::Write;
  use lucia::{
    data_format::{GraphQlRequest, GraphQlResponse},
    network::{transport::TransportParams, HttpMethod},
  };

  #[pkg::aux]
  impl<DRSR> RickAndMortyHttpPkgsAux<DRSR> {
    #[pkg::aux_data]
    fn episode_data<'any>(
      &mut self,
      buffer: &'any mut String,
      id: u32,
    ) -> crate::Result<EpisodeReq<'any>> {
      buffer.clear();
      buffer
        .write_fmt(format_args!(
          r#"
            {CHARACTER_FRAGMENT}
            query {{
              episode(id: "{id}") {{
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
          "#
        ))
        .map_err(|err| lucia::Error::from(err))?;
      self.tp.ext_req_params_mut().method = HttpMethod::Post;
      Ok(EpisodeReq { operation_name: None, query: buffer, variables: None })
    }
  }

  #[pkg::req_data]
  pub type EpisodeReq<'any> = GraphQlRequest<(), &'any str, ()>;

  #[pkg::res_data]
  pub type EpisodeRes = GraphQlResponse<EpisodeData, serde::de::IgnoredAny>;

  #[derive(Debug, serde::Deserialize)]
  #[doc = generic_data_doc!()]
  pub struct EpisodeData {
    /// Episode
    pub episode: Episode,
  }
}
