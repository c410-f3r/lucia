pub(crate) const CHARACTER_FRAGMENT: &str = r#"
  fragment CharacterFrag on Character {
    created
    gender
    id
    image
    location {
      created
      dimension
      id
      name
      type
    }
    name
    origin {
      created
      dimension
      id
      name
      type
    }
    species
    status
    type
  }
"#;

/// Character
#[derive(Debug, serde::Deserialize)]
pub struct Character {
  /// Time at which the character was created in the database.
  pub created: String,
  /// Episodes in which this character appeared.
  pub episode: Option<Vec<Episode>>,
  /// The gender of the character ('Female', 'Male', 'Genderless' or 'unknown').
  pub gender: String,
  /// The id of the character.
  pub id: String,
  /// Link to the character's image.
  pub image: String,
  /// The character's last known location
  pub location: Location,
  /// The name of the character.
  pub name: String,
  /// The character's origin location
  pub origin: Location,
  /// The species of the character.
  pub species: String,
  /// The status of the character ('Alive', 'Dead' or 'unknown').
  pub status: String,
  /// The type or subspecies of the character.
  pub r#type: String,
}

/// Episode
#[derive(Debug, serde::Deserialize)]
pub struct Episode {
  /// The air date of the episode.
  pub air_date: String,
  /// List of characters who have been seen in the episode.
  pub characters: Vec<Character>,
  /// Time at which the episode was created in the database.
  pub created: String,
  /// The code of the episode.
  pub episode: String,
  /// The id of the episode.
  pub id: String,
  /// The name of the episode.
  pub name: String,
}

/// Location
#[derive(Debug, serde::Deserialize)]
pub struct Location {
  /// Time at which the location was created in the database.
  pub created: Option<String>,
  /// The dimension in which the location is located.
  pub dimension: Option<String>,
  /// The id of the location.
  pub id: Option<String>,
  /// The name of the location.
  pub name: String,
  /// List of characters who have been last seen in the location.
  pub residents: Option<Vec<Character>>,
  /// The type of the location.
  pub r#type: Option<String>,
}

/// Pagination
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Pagination {
  /// Total number of entries
  pub count: u32,
  /// Next page
  pub next: Option<u32>,
  /// Total number of pages
  pub pages: u32,
  /// Previous page
  pub prev: Option<String>,
}
