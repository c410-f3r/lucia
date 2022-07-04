use arrayvec::ArrayString;

#[derive(Debug, serde::Deserialize)]
pub struct CountryInfo {
  pub country: Option<ArrayString<32>>,
  pub population: Option<u64>,
  pub sq_km_area: Option<f32>,
  pub continent: Option<ArrayString<16>>,
  pub abbreviation: Option<ArrayString<2>>,
  pub location: Option<ArrayString<28>>,
  pub iso: Option<u32>,
  pub capital_city: Option<ArrayString<24>>,
}
