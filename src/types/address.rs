
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Describes an address. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // address
  /// A two-letter ISO 3166-1 alpha-2 country code.
  country_code: Option<String>,
  /// State, if applicable.
  state: Option<String>,
  /// City.
  city: Option<String>,
  /// First line of the address.
  street_line1: Option<String>,
  /// Second line of the address.
  street_line2: Option<String>,
  /// Address postal code.
  postal_code: Option<String>,
  
}



impl Object for Address {}
impl RObject for Address {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "address" }
  fn td_type(&self) -> RTDType { RTDType::Address }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl Address {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "address".to_string(),
      country_code: None,
      state: None,
      city: None,
      street_line1: None,
      street_line2: None,
      postal_code: None,
      
    }
  }
  
  pub fn country_code(&self) -> Option<String> { self.country_code.clone() }
  #[doc(hidden)] pub fn _set_country_code(&mut self, country_code: String) -> &mut Self { self.country_code = Some(country_code); self }
  
  pub fn state(&self) -> Option<String> { self.state.clone() }
  #[doc(hidden)] pub fn _set_state(&mut self, state: String) -> &mut Self { self.state = Some(state); self }
  
  pub fn city(&self) -> Option<String> { self.city.clone() }
  #[doc(hidden)] pub fn _set_city(&mut self, city: String) -> &mut Self { self.city = Some(city); self }
  
  pub fn street_line1(&self) -> Option<String> { self.street_line1.clone() }
  #[doc(hidden)] pub fn _set_street_line1(&mut self, street_line1: String) -> &mut Self { self.street_line1 = Some(street_line1); self }
  
  pub fn street_line2(&self) -> Option<String> { self.street_line2.clone() }
  #[doc(hidden)] pub fn _set_street_line2(&mut self, street_line2: String) -> &mut Self { self.street_line2 = Some(street_line2); self }
  
  pub fn postal_code(&self) -> Option<String> { self.postal_code.clone() }
  #[doc(hidden)] pub fn _set_postal_code(&mut self, postal_code: String) -> &mut Self { self.postal_code = Some(postal_code); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



