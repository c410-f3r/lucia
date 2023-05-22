#[lucia_macros::pkg(
  api(crate::series::rick_and_morty::RickAndMorty),
  data_format(json),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::series::rick_and_morty::{Character, RickAndMortyHttpPkgsAux, CHARACTER_FRAGMENT};
  use core::fmt::Write;
  use lucia::{
    data_format::{GraphQlRequest, GraphQlResponse},
    network::{transport::TransportParams, HttpMethod},
  };

  #[pkg::aux]
  impl<DRSR> RickAndMortyHttpPkgsAux<DRSR> {
    #[pkg::aux_data]
    fn character_data<'any>(
      &mut self,
      buffer: &'any mut String,
      id: u32,
    ) -> crate::Result<CharacterReq<'any>> {
      buffer.clear();
      buffer
        .write_fmt(format_args!(
          r#"
            {CHARACTER_FRAGMENT}
            query {{
              character(id: "{id}") {{
                ...CharacterFrag
              }}
            }}
          "#
        ))
        .map_err(lucia::Error::from)?;
      self.tp.ext_req_params_mut().method = HttpMethod::Post;
      Ok(CharacterReq { operation_name: None, query: buffer, variables: None })
    }
  }

  #[pkg::req_data]
  pub type CharacterReq<'any> = GraphQlRequest<(), &'any str, ()>;

  #[pkg::res_data]
  pub type CharacterRes = GraphQlResponse<CharacterData, serde::de::IgnoredAny>;

  #[derive(Debug, serde::Deserialize)]
  #[doc = generic_data_doc!()]
  pub struct CharacterData {
    /// Character
    pub character: Character,
  }
}
