
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains the user's personal details. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalDetails {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // personalDetails
  /// First name of the user written in English; 1-255 characters.
  first_name: Option<String>,
  /// Middle name of the user written in English; 0-255 characters.
  middle_name: Option<String>,
  /// Last name of the user written in English; 1-255 characters.
  last_name: Option<String>,
  /// Native first name of the user; 1-255 characters.
  native_first_name: Option<String>,
  /// Native middle name of the user; 0-255 characters.
  native_middle_name: Option<String>,
  /// Native last name of the user; 1-255 characters.
  native_last_name: Option<String>,
  /// Birthdate of the user.
  birthdate: Option<Date>,
  /// Gender of the user, "male" or "female".
  gender: Option<String>,
  /// A two-letter ISO 3166-1 alpha-2 country code of the user's country.
  country_code: Option<String>,
  /// A two-letter ISO 3166-1 alpha-2 country code of the user's residence country.
  residence_country_code: Option<String>,
  
}



impl Object for PersonalDetails {}
impl RObject for PersonalDetails {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "personalDetails" }
  fn td_type(&self) -> RTDType { RTDType::PersonalDetails }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl PersonalDetails {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "personalDetails".to_string(),
      first_name: None,
      middle_name: None,
      last_name: None,
      native_first_name: None,
      native_middle_name: None,
      native_last_name: None,
      birthdate: None,
      gender: None,
      country_code: None,
      residence_country_code: None,
      
    }
  }
  
  pub fn first_name(&self) -> Option<String> { self.first_name.clone() }
  #[doc(hidden)] pub fn _set_first_name(&mut self, first_name: String) -> &mut Self { self.first_name = Some(first_name); self }
  
  pub fn middle_name(&self) -> Option<String> { self.middle_name.clone() }
  #[doc(hidden)] pub fn _set_middle_name(&mut self, middle_name: String) -> &mut Self { self.middle_name = Some(middle_name); self }
  
  pub fn last_name(&self) -> Option<String> { self.last_name.clone() }
  #[doc(hidden)] pub fn _set_last_name(&mut self, last_name: String) -> &mut Self { self.last_name = Some(last_name); self }
  
  pub fn native_first_name(&self) -> Option<String> { self.native_first_name.clone() }
  #[doc(hidden)] pub fn _set_native_first_name(&mut self, native_first_name: String) -> &mut Self { self.native_first_name = Some(native_first_name); self }
  
  pub fn native_middle_name(&self) -> Option<String> { self.native_middle_name.clone() }
  #[doc(hidden)] pub fn _set_native_middle_name(&mut self, native_middle_name: String) -> &mut Self { self.native_middle_name = Some(native_middle_name); self }
  
  pub fn native_last_name(&self) -> Option<String> { self.native_last_name.clone() }
  #[doc(hidden)] pub fn _set_native_last_name(&mut self, native_last_name: String) -> &mut Self { self.native_last_name = Some(native_last_name); self }
  
  pub fn birthdate(&self) -> Option<Date> { self.birthdate.clone() }
  #[doc(hidden)] pub fn _set_birthdate(&mut self, birthdate: Date) -> &mut Self { self.birthdate = Some(birthdate); self }
  
  pub fn gender(&self) -> Option<String> { self.gender.clone() }
  #[doc(hidden)] pub fn _set_gender(&mut self, gender: String) -> &mut Self { self.gender = Some(gender); self }
  
  pub fn country_code(&self) -> Option<String> { self.country_code.clone() }
  #[doc(hidden)] pub fn _set_country_code(&mut self, country_code: String) -> &mut Self { self.country_code = Some(country_code); self }
  
  pub fn residence_country_code(&self) -> Option<String> { self.residence_country_code.clone() }
  #[doc(hidden)] pub fn _set_residence_country_code(&mut self, residence_country_code: String) -> &mut Self { self.residence_country_code = Some(residence_country_code); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



