mod v3_available_countries;
mod v3_country_info;
mod v3_is_today_public_holiday;
mod v3_long_weekend;
mod v3_next_public_holidays;
mod v3_next_public_holidays_worldwide;
mod v3_public_holidays;

pub use v3_available_countries::pkg::*;
pub use v3_country_info::pkg::*;
pub use v3_is_today_public_holiday::pkg::*;
pub use v3_long_weekend::pkg::*;
pub use v3_next_public_holidays::pkg::*;
pub use v3_next_public_holidays_worldwide::pkg::*;
pub use v3_public_holidays::pkg::*;

use alloc::vec::Vec;
use arrayvec::ArrayString;

/// The type of a public holiday
#[derive(Debug, serde::Deserialize)]
pub enum V3HolidayTy {
  /// Authorities are closed
  Authorities,
  /// Bank holiday, banks and offices are closed
  Bank,
  /// Optional festivity, no paid day off
  Observance,
  /// Majority of people take a day off
  Optional,
  /// Public holiday
  Public,
  /// School holiday, schools are closed
  School,
}

/// Used by all the endpoints that return holydays.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V3PublicHoliday {
  /// The date.
  pub date: ArrayString<10>,
  /// Local name.
  pub local_name: Option<ArrayString<96>>,
  /// English name.
  pub name: Option<ArrayString<96>>,
  /// ISO 3166-1 alpha-2.
  pub country_code: Option<ArrayString<2>>,
  /// Is this public holiday every year on the same date.
  pub fixed: bool,
  /// Is this public holiday in every county (federal state).
  pub global: bool,
  /// ISO-3166-2 - Federal states.
  pub counties: Option<Vec<ArrayString<8>>>,
  /// The launch year of the public holiday
  pub launch_year: Option<i32>,
  /// A list of types the public holiday it is valid
  pub types: Option<Vec<V3HolidayTy>>,
}
