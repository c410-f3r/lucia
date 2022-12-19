macro_rules! generic_data_doc {
  () => {
    "Expected format of the returned query."
  };
}

mod character;
mod characters;
mod characters_by_ids;
mod common;
mod episode;
mod episodes;
mod episodes_by_ids;
mod location;
mod locations;
mod locations_by_ids;

pub use character::pkg::*;
pub use characters::pkg::*;
pub use characters_by_ids::pkg::*;
pub use common::*;
pub use episode::pkg::*;
pub use episodes::pkg::*;
pub use episodes_by_ids::pkg::*;
pub use location::pkg::*;
pub use locations::pkg::*;
pub use locations_by_ids::pkg::*;
