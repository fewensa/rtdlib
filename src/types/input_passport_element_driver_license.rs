
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A Telegram Passport element to be saved containing the user's driver license. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputPassportElementDriverLicense {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputPassportElementDriverLicense
  /// The driver license to be saved.
  driver_license: Option<InputIdentityDocument>,
  
}



impl Object for InputPassportElementDriverLicense {}
impl RObject for InputPassportElementDriverLicense {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementDriverLicense" }
  fn td_type(&self) -> RTDType { RTDType::InputPassportElementDriverLicense }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputPassportElement for InputPassportElementDriverLicense {}


impl InputPassportElementDriverLicense {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputPassportElementDriverLicense".to_string(),
      driver_license: None,
      
    }
  }
  
  pub fn driver_license(&self) -> Option<InputIdentityDocument> { self.driver_license.clone() }
  #[doc(hidden)] pub fn _set_driver_license(&mut self, driver_license: InputIdentityDocument) -> &mut Self { self.driver_license = Some(driver_license); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



