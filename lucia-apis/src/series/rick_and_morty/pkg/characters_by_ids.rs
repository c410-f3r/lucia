#[lucia_macros::pkg(
  api(crate::series::rick_and_morty::RickAndMorty),
  data_format(json),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::{
    misc::SliceByCommas,
    series::rick_and_morty::{Character, RickAndMortyHttpPkgsAux, CHARACTER_FRAGMENT},
  };
  use core::fmt::Write;
  use lucia::{
    data_format::{GraphQlRequest, GraphQlResponse},
    network::{transport::TransportParams, HttpMethod},
  };

  #[pkg::aux]
  impl<DRSR> RickAndMortyHttpPkgsAux<DRSR> {
    #[pkg::aux_data]
    fn characters_by_ids_data<'any>(
      &mut self,
      buffer: &'any mut String,
      ids: &[u32],
    ) -> crate::Result<CharactersByIdsReq<'any>> {
      buffer.clear();
      buffer
        .write_fmt(format_args!(
          r#"
            {CHARACTER_FRAGMENT}
            query {{
              charactersByIds(ids: ["{}"]) {{
                ...CharacterFrag
              }}
            }}
          "#,
          SliceByCommas(ids)
        ))
        .map_err(|err| lucia::Error::from(err))?;
      self.tp.ext_req_params_mut().method = HttpMethod::Post;
      Ok(CharactersByIdsReq { operation_name: None, query: buffer, variables: None })
    }
  }

  #[pkg::req_data]
  pub type CharactersByIdsReq<'any> = GraphQlRequest<(), &'any str, ()>;

  #[pkg::res_data]
  pub type CharactersByIdsRes = GraphQlResponse<CharactersByIdsData, serde::de::IgnoredAny>;

  #[derive(Debug, serde::Deserialize)]
  #[doc = generic_data_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct CharactersByIdsData {
    /// Characters by ids
    pub characters_by_ids: Vec<Character>,
  }
}
