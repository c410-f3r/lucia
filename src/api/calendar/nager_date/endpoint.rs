mod v3_available_countries;
mod v3_country_info;
mod v3_is_today_public_holiday;
mod v3_long_weekend;
mod v3_next_public_holidays;
mod v3_next_public_holidays_worldwide;
mod v3_public_holidays;

pub use v3_available_countries::*;
pub use v3_country_info::*;
pub use v3_is_today_public_holiday::*;
pub use v3_long_weekend::*;
pub use v3_next_public_holidays::*;
pub use v3_next_public_holidays_worldwide::*;
pub use v3_public_holidays::*;

use alloc::vec::Vec;
use arrayvec::ArrayString;

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Debug)]
pub enum PublicHolidayType {
  Authorities,
  Bank,
  Observance,
  Optional,
  Public,
  School,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct V3PublicHoliday {
  pub date: ArrayString<10>,
  pub local_name: Option<ArrayString<72>>,
  pub name: Option<ArrayString<72>>,
  pub country_code: Option<ArrayString<2>>,
  pub fixed: bool,
  pub global: bool,
  pub counties: Option<Vec<ArrayString<8>>>,
  pub launch_year: Option<i32>,
  pub types: Option<Vec<PublicHolidayType>>,
}
