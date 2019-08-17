
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns an IETF language tag of the language preferred in the country, which should be used to fill native fields in Telegram Passport personal details. Returns a 404 error if unknown.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPreferredCountryLanguage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getPreferredCountryLanguage
  /// A two-letter ISO 3166-1 alpha-2 country code.
  country_code: Option<String>,
  
}



impl Object for GetPreferredCountryLanguage {}
impl RObject for GetPreferredCountryLanguage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getPreferredCountryLanguage" }
  fn td_type(&self) -> RTDType { RTDType::GetPreferredCountryLanguage }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetPreferredCountryLanguage {}


impl GetPreferredCountryLanguage {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getPreferredCountryLanguage".to_string(),
      country_code: None,
      
    }
  }
  
  pub fn country_code(&self) -> Option<String> { self.country_code.clone() }
  #[doc(hidden)] pub fn _set_country_code(&mut self, country_code: String) -> &mut Self { self.country_code = Some(country_code); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



