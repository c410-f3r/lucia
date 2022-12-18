#[lucia_macros::pkg(
  api(crate::series::rick_and_morty::RickAndMorty),
  data_format(json),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::series::rick_and_morty::{
    Character, Pagination, RickAndMortyHttpPkgsAux, CHARACTER_FRAGMENT,
  };
  use core::fmt::Write;
  use lucia::{
    data_format::{GraphQlRequest, GraphQlResponse},
    network::HttpMethod,
  };

  #[pkg::aux]
  impl<DRSR> RickAndMortyHttpPkgsAux<DRSR> {
    #[pkg::aux_data]
    fn characters_data<'any>(
      &mut self,
      buffer: &'any mut String,
      gender: &str,
      name: &str,
      page: u32,
      species: &str,
      status: &str,
      ty: &str,
    ) -> crate::Result<CharactersReq<'any>> {
      buffer.clear();
      buffer
        .write_fmt(format_args!(
          r#"
            {CHARACTER_FRAGMENT}
            query {{
              characters(
                filter: {{
                  gender: "{gender}",
                  name: "{name}",
                  species: "{species}",
                  status: "{status}",
                  type: "{ty}"
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
                  ...CharacterFrag
                }}
              }}
            }}
          "#
        ))
        .map_err(|err| lucia::Error::from(err))?;
      self.ext_req_params.method = HttpMethod::Post;
      Ok(CharactersReq { operation_name: None, query: buffer, variables: None })
    }
  }

  #[pkg::req_data]
  pub type CharactersReq<'any> = GraphQlRequest<(), &'any str, ()>;

  #[pkg::res_data]
  pub type CharactersRes = GraphQlResponse<CharactersData, serde::de::IgnoredAny>;

  #[derive(Debug, serde::Deserialize)]
  #[doc = generic_data_doc!()]
  pub struct Characters {
    /// Pagination
    pub info: Pagination,
    /// Characters
    pub results: Vec<Character>,
  }

  #[derive(Debug, serde::Deserialize)]
  #[doc = generic_data_doc!()]
  pub struct CharactersData {
    /// Characters
    pub characters: Characters,
  }
}
